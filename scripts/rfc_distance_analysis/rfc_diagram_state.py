#!/usr/bin/env python3
r"""Small incidence-diagram state for RFC distance diagnostics.

This module is intentionally tiny.  It does not try to evaluate the full RFC
recurrence; it only represents the child incidence objects that the recurrence
must stop collapsing into chains.

Containment edges are written child <= parent.  If an edge connects two nodes of
the same dimension, the nodes are forced equal and are merged, keeping the
strongest zero budget.
"""

from __future__ import annotations

from dataclasses import dataclass, replace


@dataclass(frozen=True, order=True)
class DiagramNode:
    name: str
    dim: int
    zero_budget: int


@dataclass(frozen=True, order=True)
class ContainmentEdge:
    child: str
    parent: str


@dataclass(frozen=True)
class DiagramState:
    nodes: tuple[DiagramNode, ...]
    edges: tuple[ContainmentEdge, ...]

    @staticmethod
    def empty() -> "DiagramState":
        return DiagramState(nodes=(), edges=())

    def node_map(self) -> dict[str, DiagramNode]:
        return {node.name: node for node in self.nodes}

    def node(self, name: str) -> DiagramNode:
        nodes = self.node_map()
        if name not in nodes:
            raise KeyError(f"unknown diagram node {name!r}")
        return nodes[name]

    def with_node(self, name: str, dim: int, zero_budget: int) -> "DiagramState":
        nodes = self.node_map()
        if name in nodes:
            old = nodes[name]
            if old.dim != dim:
                raise ValueError(f"node {name!r} already has dimension {old.dim}, not {dim}")
            nodes[name] = replace(old, zero_budget=max(old.zero_budget, zero_budget))
        else:
            nodes[name] = DiagramNode(name=name, dim=dim, zero_budget=zero_budget)
        return DiagramState(tuple(sorted(nodes.values())), self.edges).canonicalized()

    def with_edge(self, child: str, parent: str) -> "DiagramState":
        nodes = self.node_map()
        if child not in nodes:
            raise KeyError(f"unknown child node {child!r}")
        if parent not in nodes:
            raise KeyError(f"unknown parent node {parent!r}")
        edges = set(self.edges)
        if child != parent:
            edges.add(ContainmentEdge(child=child, parent=parent))
        return DiagramState(self.nodes, tuple(sorted(edges))).canonicalized()

    def merge_nodes(self, keep: str, drop: str) -> "DiagramState":
        if keep == drop:
            return self
        nodes = self.node_map()
        if keep not in nodes or drop not in nodes:
            raise KeyError(f"cannot merge missing nodes {keep!r}, {drop!r}")
        keep_node = nodes[keep]
        drop_node = nodes[drop]
        if keep_node.dim != drop_node.dim:
            raise ValueError("only equal-dimension diagram nodes can be merged")
        nodes[keep] = replace(
            keep_node,
            zero_budget=max(keep_node.zero_budget, drop_node.zero_budget),
        )
        del nodes[drop]
        edges = {
            ContainmentEdge(
                child=keep if edge.child == drop else edge.child,
                parent=keep if edge.parent == drop else edge.parent,
            )
            for edge in self.edges
        }
        edges = {edge for edge in edges if edge.child != edge.parent}
        return DiagramState(tuple(sorted(nodes.values())), tuple(sorted(edges))).canonicalized()

    def canonicalized(self) -> "DiagramState":
        state = self
        while True:
            nodes = state.node_map()
            forced_edge: ContainmentEdge | None = None
            for edge in state.edges:
                child = nodes[edge.child]
                parent = nodes[edge.parent]
                if child.dim > parent.dim:
                    raise ValueError(
                        f"invalid containment {child.name}<={parent.name}: "
                        f"{child.dim}>{parent.dim}"
                    )
                if child.dim == parent.dim:
                    forced_edge = edge
                    break
            if forced_edge is None:
                edges = tuple(sorted(set(state.edges)))
                nodes_tuple = tuple(sorted(state.nodes))
                if edges == state.edges and nodes_tuple == state.nodes:
                    return state
                state = DiagramState(nodes_tuple, edges)
                continue
            keep = min(forced_edge.child, forced_edge.parent)
            drop = max(forced_edge.child, forced_edge.parent)
            state = state.merge_nodes(keep, drop)

    def key(self) -> str:
        node_part = ";".join(
            f"{node.name}:d{node.dim}:z{node.zero_budget}" for node in self.nodes
        )
        edge_part = ";".join(f"{edge.child}<={edge.parent}" for edge in self.edges)
        if not edge_part:
            return node_part
        return f"{node_part}|{edge_part}"

    def zero_budget(self, name: str) -> int:
        return self.node(name).zero_budget


def carrier_plane_with_line(
    *,
    plane_name: str,
    line_name: str,
    plane_zeros: int,
    line_zeros: int,
) -> DiagramState:
    """Return the carrier diagram line <= plane."""

    return (
        DiagramState.empty()
        .with_node(plane_name, 2, plane_zeros)
        .with_node(line_name, 1, line_zeros)
        .with_edge(line_name, plane_name)
    )


def add_marked_line_in_plane(
    state: DiagramState,
    *,
    plane_name: str,
    line_name: str,
    line_zeros: int,
) -> tuple[DiagramState, int, str]:
    """Add an ordered marked line under a fixed plane.

    The returned integer is the q-dimension of the safe line-choice factor.
    The finite factor is `q+1`; the q-dimensional diagnostic charge is one.
    """

    plane = state.node(plane_name)
    if plane.dim != 2:
        raise ValueError("marked-line insertion currently requires a child 2-plane")
    next_state = (
        state.with_node(line_name, 1, line_zeros)
        .with_edge(line_name, plane_name)
        .canonicalized()
    )
    return next_state, 1, "q+1"
