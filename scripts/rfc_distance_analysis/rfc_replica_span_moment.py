#!/usr/bin/env python3
"""Toy span-aware replica moment recurrence for RFC distance.

This is a calibration model, not a certificate.  It extends rfc_replica_zero_moment.py by tracking
the span dimension t of the ordered replica tuple.  The default component-aware singleton charge is
modeled as

    |E| + t * rank(E) - comp(E)

under a uniform quotient matroid approximation.

The endpoint-tau2 mode keeps that approximation except at visible span t=2, where it uses the
corrected generic/component endpoint charge

    min(4delta - 2g, |E| + 2delta - comp(E)).

The endpoint-tau2-layer mode instead uses the layer-codimension charge

    4delta - 4 - max_h(2h - 4 - gamma_h)

with the uniform-matroid heuristic `gamma_h=(h-g)^2` for intermediate layers and
`gamma_delta=|E|-comp(E)` for the full-kernel layer.

The recurrence deliberately overbounds parent tuples by allowing any child 2r-tuple whose span is
at most 2t.  This keeps the model simple while testing whether replica-span tracking removes the
bottom parallel-class blow-up seen in the scalar component-aware recurrence.
"""

from __future__ import annotations

import argparse
import math


NEG_INF = -1.0e300
INF = 10**9


def log2_add(left: float, right: float) -> float:
    if left <= NEG_INF / 2:
        return right
    if right <= NEG_INF / 2:
        return left
    if right > left:
        left, right = right, left
    return left + math.log2(1.0 + 2.0 ** (right - left))


def log2_comb_table(n: int) -> list[list[float]]:
    table: list[list[float]] = []
    for row_n in range(n + 1):
        row = [NEG_INF] * (row_n + 1)
        row[0] = 0.0
        row[row_n] = 0.0
        for k in range(1, row_n):
            row[k] = log2_add(table[row_n - 1][k - 1], table[row_n - 1][k])
        table.append(row)
    return table


def uniform_component_count(size: int, rank: int) -> int:
    if size == 0 or rank == 0:
        return 0
    if rank == size:
        return size
    return 1


def endpoint_tau2_charge(size: int, delta: int, components: int) -> int:
    generic_kernel_dim = max(0, 2 * delta - size)
    if delta < 2 and generic_kernel_dim < 2:
        return INF
    component_charge = INF
    if delta >= 2:
        component_charge = size + 2 * delta - components
    generic_charge = INF
    if generic_kernel_dim >= 2:
        generic_charge = 4 * delta - 2 * generic_kernel_dim
    return min(component_charge, generic_charge)


def endpoint_tau2_layer_charge(size: int, delta: int, components: int) -> int:
    if delta < 2:
        return INF
    generic_kernel_dim = max(0, 2 * delta - size)
    full_component_codim = max(0, size - components)
    theta = -INF
    for h in range(2, delta + 1):
        if h == delta:
            gamma_h = full_component_codim
        else:
            gamma_h = max(0, h - generic_kernel_dim) ** 2
        theta = max(theta, 2 * h - 4 - gamma_h)
    return int(4 * delta - 4 - theta)


def singleton_charge(
    parent_replica_span: int,
    child_k: int,
    common_size: int,
    extras: int,
    mode: str,
) -> int:
    if extras == 0:
        return 0
    common_rank = min(common_size, child_k)
    quotient_rank = max(0, child_k - common_rank)
    if quotient_rank == 0:
        return 0
    rho = min(extras, quotient_rank)
    components = uniform_component_count(extras, rho)
    if mode == "endpoint-tau2" and parent_replica_span == 2:
        return endpoint_tau2_charge(extras, rho, components)
    if mode == "endpoint-tau2-layer" and parent_replica_span == 2:
        return endpoint_tau2_layer_charge(extras, rho, components)
    return extras + parent_replica_span * rho - components


def lift_span_moment(
    child_by_span: dict[int, list[float]],
    comb: list[list[float]],
    q_log2: float,
    parent_replica_count: int,
    child_k: int,
    singleton_charge_mode: str,
) -> tuple[dict[int, list[float]], dict[tuple[int, int], tuple[int, int, int, int, int] | None]]:
    child_n = len(next(iter(child_by_span.values()))) - 1
    parent_n = 2 * child_n
    parent_k = 2 * child_k
    max_parent_span = min(parent_replica_count, parent_k)
    parent_by_span: dict[int, list[float]] = {
        span: [NEG_INF] * (parent_n + 1) for span in range(1, max_parent_span + 1)
    }
    choices: dict[tuple[int, int], tuple[int, int, int, int, int] | None] = {}

    for parent_span in range(1, max_parent_span + 1):
        max_child_span = 2 * parent_span
        available_child_spans = [
            span for span in child_by_span.keys() if span <= max_child_span
        ]
        for z in range(parent_n + 1):
            total = NEG_INF
            best = NEG_INF
            best_choice: tuple[int, int, int, int, int] | None = None
            for p in range(z // 2 + 1):
                s = z - 2 * p
                if p + s > child_n:
                    continue
                orientation_log = float(s)
                max_common_singletons = min(s, child_n - p)
                for common_singletons in range(max_common_singletons + 1):
                    common_size = p + common_singletons
                    extras = s - common_singletons
                    if extras > child_n - common_size:
                        continue
                    charge = singleton_charge(
                        parent_span,
                        child_k,
                        common_size,
                        extras,
                        singleton_charge_mode,
                    )
                    if charge >= INF:
                        continue
                    shape_log = (
                        orientation_log
                        + comb[common_size][p]
                        + comb[child_n - common_size][extras]
                    )
                    charge_log = -charge * q_log2
                    for child_span in available_child_spans:
                        child_values = child_by_span[child_span]
                        if child_values[common_size] <= NEG_INF / 2:
                            continue
                        term = shape_log + charge_log + child_values[common_size]
                        total = log2_add(total, term)
                        if term > best:
                            best = term
                            best_choice = (p, s, common_singletons, common_size, child_span)
            parent_by_span[parent_span][z] = total
            choices[(parent_span, z)] = best_choice
    return parent_by_span, choices


def first_crossing(values: list[float], security_bits: float) -> int | None:
    target = -security_bits
    for z, value in enumerate(values):
        if value <= target:
            return z
    return None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--print-window", type=int, default=5)
    parser.add_argument("--trace-z", type=int, default=-1)
    parser.add_argument(
        "--singleton-charge",
        choices=["component-uniform", "endpoint-tau2", "endpoint-tau2-layer"],
        default="component-uniform",
    )
    args = parser.parse_args()

    if args.depth < 0:
        raise SystemExit("--depth must be nonnegative")
    if args.expansion < 1:
        raise SystemExit("--expansion must be positive")

    total_n = args.expansion * (1 << args.depth)
    comb = log2_comb_table(total_n)

    replica_count = 1 << args.depth
    values_by_span: dict[int, list[float]] = {1: [NEG_INF] * (args.expansion + 1)}
    values_by_span[1][0] = replica_count * args.q_log2
    trace: list[dict[tuple[int, int], tuple[int, int, int, int, int] | None]] = []

    print("level,replica_count,k,n,span_count,min_log2,max_log2", flush=True)
    print(
        f"0,{replica_count},1,{args.expansion},1,"
        f"{values_by_span[1][0]:.8f},{values_by_span[1][0]:.8f}",
        flush=True,
    )

    for level in range(1, args.depth + 1):
        parent_replica_count = replica_count // 2
        values_by_span, choices = lift_span_moment(
            values_by_span,
            comb,
            args.q_log2,
            parent_replica_count,
            1 << (level - 1),
            args.singleton_charge,
        )
        trace.append(choices)
        replica_count = parent_replica_count
        finite = [
            value
            for values in values_by_span.values()
            for value in values
            if value > NEG_INF / 2
        ]
        print(
            f"{level},{replica_count},{1 << level},{args.expansion * (1 << level)},"
            f"{len(values_by_span)},{min(finite):.8f},{max(finite):.8f}",
            flush=True,
        )

    final = values_by_span[1]
    crossing = first_crossing(final, args.security_bits)
    k = 1 << args.depth
    center = k if crossing is None else crossing
    start = max(0, center - args.print_window)
    stop = min(total_n, center + args.print_window)
    print("z,excess,log2_span1_moment")
    for z in range(start, stop + 1):
        label = "-inf" if final[z] <= NEG_INF / 2 else f"{final[z]:.8f}"
        print(f"{z},{z - k},{label}")
    print(
        f"crossing_z={crossing} crossing_excess="
        f"{'' if crossing is None else crossing - k}"
    )

    if args.trace_z >= 0:
        print("trace_level,span,z,p,s,c,u,child_span")
        span = 1
        z = args.trace_z
        for level in range(args.depth, 0, -1):
            choice = trace[level - 1].get((span, z))
            if choice is None:
                print(f"{level},{span},{z},,,,,")
                break
            p, s, common_singletons, common_size, child_span = choice
            print(f"{level},{span},{z},{p},{s},{common_singletons},{common_size},{child_span}")
            span = child_span
            z = common_size


if __name__ == "__main__":
    main()
