"""Probe simple dual potentials over the RFC canonical block grammar.

This script is deterministic proof bookkeeping, not a recurrence profiler.  It
uses the current stress transitions recorded in the canonical diagram plan and
asks for a coarse potential of the form

    Phi_h(D) = level_weight * h
             + dim_weight * sum(node dimensions)
             - zero_weight * sum(node zero budgets)
             + node_weight * number_of_nodes.

For every transition we require, in q-dimensional units,

    Phi(parent) >= local_qdim + Phi(child) + slack.

The `level_weight` is a placeholder per-fold budget.  A good future proof should
replace it by actual local/root/incidence accounting; here it is useful because
the bottleneck transition tells us which grammar block is asking for the most
unexplained per-level budget.
"""

from __future__ import annotations

import argparse
import csv
import sys
from dataclasses import dataclass
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_block_grammar_ledger import BLOCKS  # noqa: E402


Q_LOG2 = 128.0


@dataclass(frozen=True)
class DiagramState:
    level: int
    nodes: tuple[tuple[int, int], ...]

    @property
    def dim_sum(self) -> int:
        return sum(dim for dim, _zero in self.nodes)

    @property
    def zero_sum(self) -> int:
        return sum(zero for _dim, zero in self.nodes)

    @property
    def node_count(self) -> int:
        return len(self.nodes)

    @property
    def label(self) -> str:
        return ">=".join(f"({dim},{zero})" for dim, zero in self.nodes)


@dataclass(frozen=True)
class Transition:
    transition_id: str
    parent: DiagramState
    child: DiagramState
    local_log2: float
    blocks: tuple[str, ...]
    note: str

    @property
    def local_qdim(self) -> float:
        return self.local_log2 / Q_LOG2

    @property
    def level_drop(self) -> int:
        return self.parent.level - self.child.level


@dataclass(frozen=True)
class Weights:
    dim: float
    zero: float
    node: float


@dataclass(frozen=True)
class SearchResult:
    level_weight: float
    worst_margin: float
    weights: Weights
    bottleneck: str


TRANSITIONS: tuple[Transition, ...] = (
    Transition(
        "top_tau1_to_2_15",
        DiagramState(5, ((1, 34),)),
        DiagramState(4, ((2, 15),)),
        -101.38165304,
        ("tau1_line", "tau1_full_line_carry", "kernel_fiber_cover"),
        "top stratified checkpoint row feeding (2,15)",
    ),
    Transition(
        "support3_stratified_to_3_6",
        DiagramState(4, ((2, 15),)),
        DiagramState(3, ((3, 6),)),
        273.91326343,
        ("support3_stratified",),
        "support-three rank-defect-incidence row",
    ),
    Transition(
        "support2_diamond_to_3_2_ge_1_4",
        DiagramState(3, ((3, 6),)),
        DiagramState(2, ((3, 2), (1, 4))),
        266.76487159,
        ("support2_diamond",),
        "kerneled support-two quotient-diamond row",
    ),
    Transition(
        "flag_tau1_tau0_to_base_flag",
        DiagramState(2, ((3, 2), (1, 4))),
        DiagramState(1, ((2, 0), (1, 2))),
        -119.09310940,
        ("tau1_line", "tau0_container"),
        "top child-table row for (3,2)>=(1,4)",
    ),
)


def parse_grid(spec: str) -> list[float]:
    parts = spec.split(":")
    if len(parts) != 3:
        raise argparse.ArgumentTypeError("grid must be start:stop:step")
    start, stop, step = (float(part) for part in parts)
    if step <= 0:
        raise argparse.ArgumentTypeError("grid step must be positive")
    values: list[float] = []
    value = start
    # Include the endpoint up to a small floating tolerance.
    while value <= stop + step * 1.0e-9:
        values.append(round(value, 10))
        value += step
    if not values:
        raise argparse.ArgumentTypeError("grid is empty")
    return values


def feature_score(state: DiagramState, weights: Weights) -> float:
    return (
        weights.dim * state.dim_sum
        - weights.zero * state.zero_sum
        + weights.node * state.node_count
    )


def required_level_weight(
    transition: Transition,
    weights: Weights,
    slack: float,
) -> float:
    if transition.level_drop <= 0:
        raise ValueError(f"{transition.transition_id} does not descend in level")
    parent_feature = feature_score(transition.parent, weights)
    child_feature = feature_score(transition.child, weights)
    requirement = transition.local_qdim + child_feature - parent_feature + slack
    return requirement / transition.level_drop


def margin(transition: Transition, weights: Weights, level_weight: float) -> float:
    parent_phi = level_weight * transition.parent.level + feature_score(transition.parent, weights)
    child_phi = level_weight * transition.child.level + feature_score(transition.child, weights)
    return parent_phi - transition.local_qdim - child_phi


def validate_transitions() -> None:
    block_ids = {block.block_id for block in BLOCKS}
    for transition in TRANSITIONS:
        unknown = sorted(set(transition.blocks) - block_ids)
        if unknown:
            raise ValueError(f"{transition.transition_id} references unknown blocks: {unknown}")
        if transition.level_drop <= 0:
            raise ValueError(f"{transition.transition_id} has nonpositive level drop")


def evaluate(weights: Weights, slack: float) -> SearchResult:
    requirements = [
        max(0.0, required_level_weight(transition, weights, slack))
        for transition in TRANSITIONS
    ]
    level_weight = max(requirements)
    margins = [margin(transition, weights, level_weight) for transition in TRANSITIONS]
    worst_margin = min(margins)
    bottleneck_index = margins.index(worst_margin)
    return SearchResult(
        level_weight=level_weight,
        worst_margin=worst_margin,
        weights=weights,
        bottleneck=TRANSITIONS[bottleneck_index].transition_id,
    )


def search(
    dim_grid: list[float],
    zero_grid: list[float],
    node_grid: list[float],
    slack: float,
    top: int,
) -> list[SearchResult]:
    results: list[SearchResult] = []
    for dim_weight in dim_grid:
        for zero_weight in zero_grid:
            for node_weight in node_grid:
                weights = Weights(dim=dim_weight, zero=zero_weight, node=node_weight)
                results.append(evaluate(weights, slack))
    results.sort(
        key=lambda result: (
            result.level_weight,
            -result.weights.zero,
            result.weights.dim,
            result.weights.node,
        )
    )
    return results[:top]


def transition_rows(weights: Weights, level_weight: float) -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    for transition in TRANSITIONS:
        rows.append(
            {
                "transition_id": transition.transition_id,
                "parent": transition.parent.label,
                "child": transition.child.label,
                "level": transition.parent.level,
                "child_level": transition.child.level,
                "local_qdim": f"{transition.local_qdim:.8f}",
                "margin_qdim": f"{margin(transition, weights, level_weight):.8f}",
                "blocks": ";".join(transition.blocks),
                "note": transition.note,
            }
        )
    return rows


def emit_csv(rows: list[dict[str, object]]) -> None:
    if not rows:
        return
    fieldnames: list[str] = []
    for row in rows:
        for key in row:
            if key not in fieldnames:
                fieldnames.append(key)
    writer = csv.DictWriter(sys.stdout, fieldnames=fieldnames, lineterminator="\n")
    writer.writeheader()
    writer.writerows(rows)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dim-grid", default="0:4:0.25", type=parse_grid)
    parser.add_argument("--zero-grid", default="0:2:0.05", type=parse_grid)
    parser.add_argument("--node-grid", default="0:2:0.25", type=parse_grid)
    parser.add_argument("--slack", default=0.0, type=float, help="required margin in qdims")
    parser.add_argument("--top", default=10, type=int)
    parser.add_argument(
        "--show-transitions",
        action="store_true",
        help="print per-transition margins for the best result instead of the search table",
    )
    parser.add_argument("--validate-only", action="store_true")
    args = parser.parse_args()

    validate_transitions()
    if args.validate_only:
        return

    results = search(args.dim_grid, args.zero_grid, args.node_grid, args.slack, args.top)
    if args.show_transitions:
        best = results[0]
        emit_csv(transition_rows(best.weights, best.level_weight))
        return

    rows = [
        {
            "rank": index + 1,
            "level_weight_qdim": f"{result.level_weight:.8f}",
            "worst_margin_qdim": f"{result.worst_margin:.8f}",
            "dim_weight": f"{result.weights.dim:.8f}",
            "zero_weight": f"{result.weights.zero:.8f}",
            "node_weight": f"{result.weights.node:.8f}",
            "bottleneck": result.bottleneck,
        }
        for index, result in enumerate(results)
    ]
    emit_csv(rows)


if __name__ == "__main__":
    main()
