#!/usr/bin/env python3
"""Normal-slice checker for theta=-1 kernel-chain truncation.

This is a deterministic exponent calculator. It does not enumerate codes or run
field experiments. Given a nested flag shape and zero witnesses, it computes the
shortened-ambient ancestor exponent

    E_anc = sum_i (t_i - t_{i+1}) * (D_i - t_i)

where D_i is bounded on a normal rank slice by max(k_child - z_i, 0)+b_i.
The fourth-layer truncation test is

    E_anc + split_const <= charge.

The default examples are deliberately small hard-case sketches for pure
kernel-following theta=-1 rows: every step has a=5 and local charge 9.
"""

from __future__ import annotations

import argparse
import csv
import math
import sys
from dataclasses import dataclass
from functools import lru_cache

from rfc_shortened_rank_recurrence import INF, ShortenedRankRecurrence


@dataclass(frozen=True)
class Scenario:
    name: str
    child_k: int
    expansion: int
    dims: tuple[int, ...]
    zeros: tuple[int, ...]
    b: tuple[int, ...]
    charge: int
    split_const: float


def parse_int_tuple(text: str, *, name: str) -> tuple[int, ...]:
    try:
        values = tuple(int(part.strip()) for part in text.split(",") if part.strip())
    except ValueError as exc:
        raise argparse.ArgumentTypeError(f"{name} must be a comma-separated integer list") from exc
    if not values:
        raise argparse.ArgumentTypeError(f"{name} must not be empty")
    return values


def normal_dims(child_k: int, zeros: tuple[int, ...], b: tuple[int, ...]) -> tuple[int, ...]:
    if len(zeros) != len(b):
        raise ValueError("zeros and b must have the same length")
    return tuple(max(child_k - z, 0) + bi for z, bi in zip(zeros, b))


def ancestor_exponent(dims: tuple[int, ...], short_dims: tuple[int, ...]) -> tuple[int, bool]:
    if len(dims) < 2:
        raise ValueError("dims must contain at least two layers")
    if any(dims[i] < dims[i + 1] for i in range(len(dims) - 1)):
        raise ValueError("dims must be nonincreasing")
    if len(short_dims) != len(dims) - 1:
        raise ValueError("short_dims must have one entry per ancestor edge")

    total = 0
    feasible = True
    for i, ambient in enumerate(short_dims):
        upper = dims[i]
        lower = dims[i + 1]
        if ambient < upper:
            # On this normal slice the requested ancestor cannot exist.
            feasible = False
            continue
        total += (upper - lower) * (ambient - upper)
    return total, feasible


def defect_codim(child_k: int, zero_count: int, short_dim: int) -> int:
    """Random-rank codimension proxy for dim H(B) >= short_dim.

    For a k x z restriction matrix, dim H(B)=D means rank <= k-D.
    The determinantal codimension is D * (z-k+D), which is zero at
    the MDS dimension max(k-z, 0).
    """
    return max(0, short_dim * (zero_count - child_k + short_dim))


def log2_exact_power(value: int) -> int:
    if value <= 0 or value & (value - 1):
        raise ValueError("child_k must be a positive power of two for rho diagnostics")
    return int(math.log2(value))


@lru_cache(maxsize=None)
def rho_recurrence(depth: int, expansion: int) -> ShortenedRankRecurrence:
    return ShortenedRankRecurrence(depth=depth, expansion=expansion)


def optimistic_rho_costs(scenario: Scenario, edge_zeros: tuple[int, ...], short: tuple[int, ...]) -> tuple[int, ...]:
    depth = log2_exact_power(scenario.child_k)
    recurrence = rho_recurrence(depth, scenario.expansion)
    costs = []
    for zero_count, short_dim in zip(edge_zeros, short):
        if short_dim > scenario.child_k:
            costs.append(INF)
        else:
            costs.append(recurrence.cost(depth, short_dim, zero_count))
    return tuple(costs)


def evaluate(scenario: Scenario) -> dict[str, str]:
    edge_zeros = scenario.zeros[: len(scenario.dims) - 1]
    edge_b = scenario.b[: len(scenario.dims) - 1]
    short = normal_dims(scenario.child_k, edge_zeros, edge_b)
    e_anc, feasible = ancestor_exponent(scenario.dims, short)
    defect_charges = tuple(
        defect_codim(scenario.child_k, z, d) for z, d in zip(edge_zeros, short)
    )
    rho_costs = optimistic_rho_costs(scenario, edge_zeros, short)
    max_defect_charge = max(defect_charges) if defect_charges else 0
    finite_rho = [cost for cost in rho_costs if cost < INF]
    max_rho_cost = max(finite_rho) if finite_rho else INF
    lhs = e_anc + scenario.split_const
    margin = scenario.charge - lhs
    defect_margin = scenario.charge + max_defect_charge - lhs
    rho_margin = scenario.charge + max_rho_cost - lhs if max_rho_cost < INF else INF
    safe = (not feasible) or margin >= 0
    defect_safe = safe or defect_margin >= 0
    rho_safe = safe or max_rho_cost >= INF or rho_margin >= 0
    return {
        "name": scenario.name,
        "child_k": str(scenario.child_k),
        "expansion": str(scenario.expansion),
        "dims": ",".join(map(str, scenario.dims)),
        "zeros": ",".join(map(str, edge_zeros)),
        "b": ",".join(map(str, edge_b)),
        "short_dims": ",".join(map(str, short)),
        "normal_slice_feasible": "yes" if feasible else "no",
        "e_anc_logq": str(e_anc),
        "defect_codim_logq": ",".join(map(str, defect_charges)),
        "max_defect_charge_logq": str(max_defect_charge),
        "optimistic_rho_logq": ",".join("inf" if cost >= INF else str(cost) for cost in rho_costs),
        "max_optimistic_rho_logq": "inf" if max_rho_cost >= INF else str(max_rho_cost),
        "split_const_logq": f"{scenario.split_const:.6g}",
        "charge_logq": str(scenario.charge),
        "margin_logq": "infeasible" if not feasible else f"{margin:.6g}",
        "normal_slice_safe": "yes" if safe else "no",
        "defect_routed_margin_logq": "infeasible" if not feasible else f"{defect_margin:.6g}",
        "defect_routed_safe": "yes" if defect_safe else "no",
        "rho_routed_margin_logq": (
            "infeasible" if not feasible else "inf" if rho_margin >= INF else f"{rho_margin:.6g}"
        ),
        "rho_routed_safe": "yes" if rho_safe else "no",
    }


def default_scenarios() -> list[Scenario]:
    # These are not observed consecutive chains; the current best-transition
    # diagnostics have chain length one. They are hard-case sketches used to
    # sanity-check the truncation inequality using the level-local child_k
    # values from the depth-6/7 reports.
    return [
        Scenario(
            name="observed_level4_inner_shape_child_k8",
            child_k=8,
            expansion=8,
            dims=(4, 3, 2, 1),
            zeros=(21, 26, 31),
            b=(0, 0, 0),
            charge=9,
            split_const=0.0,
        ),
        Scenario(
            name="observed_level5_inner_shape_child_k16",
            child_k=16,
            expansion=8,
            dims=(4, 3, 2, 1),
            zeros=(101, 106, 111),
            b=(0, 0, 0),
            charge=9,
            split_const=0.0,
        ),
        Scenario(
            name="toy_child32_gap_near_dimension",
            child_k=32,
            expansion=8,
            dims=(4, 3, 2, 1),
            zeros=(21, 26, 31),
            b=(0, 0, 0),
            charge=9,
            split_const=0.0,
        ),
    ]


def build_arg_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--child-k", type=int, default=None)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--dims", type=lambda s: parse_int_tuple(s, name="dims"), default=None)
    parser.add_argument("--zeros", type=lambda s: parse_int_tuple(s, name="zeros"), default=None)
    parser.add_argument("--b", type=lambda s: parse_int_tuple(s, name="b"), default=None)
    parser.add_argument("--charge", type=int, default=9)
    parser.add_argument("--split-const", type=float, default=0.0)
    parser.add_argument("--name", default="custom")
    parser.add_argument("--sweep-b", type=int, default=None, help="emit custom rows with b=0..MAX on every edge")
    parser.add_argument("--output-csv", default=None)
    return parser


def main() -> int:
    args = build_arg_parser().parse_args()

    if args.child_k is None and args.dims is None and args.zeros is None:
        scenarios = default_scenarios()
    else:
        if args.child_k is None or args.dims is None or args.zeros is None:
            raise SystemExit("--child-k, --dims, and --zeros must be supplied together")
        if len(args.zeros) != len(args.dims) - 1:
            raise SystemExit("--zeros must have one entry per ancestor edge")
        if args.sweep_b is not None:
            if args.sweep_b < 0:
                raise SystemExit("--sweep-b must be nonnegative")
            scenarios = [
                Scenario(
                    name=f"{args.name}_b{bi}",
                    child_k=args.child_k,
                    expansion=args.expansion,
                    dims=args.dims,
                    zeros=args.zeros,
                    b=tuple(bi for _ in args.zeros),
                    charge=args.charge,
                    split_const=args.split_const,
                )
                for bi in range(args.sweep_b + 1)
            ]
        else:
            b = args.b if args.b is not None else tuple(0 for _ in args.zeros)
            if len(b) != len(args.zeros):
                raise SystemExit("--b must have the same length as --zeros")
            scenarios = [
                Scenario(
                    name=args.name,
                    child_k=args.child_k,
                    expansion=args.expansion,
                    dims=args.dims,
                    zeros=args.zeros,
                    b=b,
                    charge=args.charge,
                    split_const=args.split_const,
                )
            ]

    rows = [evaluate(scenario) for scenario in scenarios]
    fields = list(rows[0])

    if args.output_csv:
        with open(args.output_csv, "w", newline="", encoding="utf-8") as handle:
            writer = csv.DictWriter(handle, fieldnames=fields)
            writer.writeheader()
            writer.writerows(rows)

    writer = csv.DictWriter(sys.stdout, fieldnames=fields)
    writer.writeheader()
    writer.writerows(rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
