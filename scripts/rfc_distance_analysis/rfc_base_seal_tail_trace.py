#!/usr/bin/env python3
"""Trace high-common-zero child rows that can break the depth-5 base seal."""

from __future__ import annotations

import argparse
import csv
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_base_seal_budget import rows_for_budget  # noqa: E402
from rfc_replica_zero_moment import (  # noqa: E402
    NEG_INF,
    initial_nonzero_moment,
    lift_moment,
    log2_comb_table,
)


def finite_label(value: float) -> str:
    return "-inf" if value <= NEG_INF / 2 else f"{value:.8f}"


def component_uniform_charge(*, replica_count: int, child_k: int, u: int, extras: int) -> int:
    if replica_count <= 1:
        return extras
    common_rank = min(u, child_k)
    quotient_rank = max(0, child_k - common_rank)
    if extras == 0:
        return 0
    if quotient_rank == 0:
        return 0
    if extras <= quotient_rank:
        return replica_count * extras
    return extras + replica_count * quotient_rank - 1


def scalar_replica_charge(*, replica_count: int, extras: int) -> int:
    return replica_count * extras


def build_levels_and_choices(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    singleton_charge: str,
) -> tuple[list[list[float]], list[list[tuple[int, int, int, int] | None]]]:
    replica_count = 1 << depth
    values = initial_nonzero_moment(expansion, replica_count, q_log2)
    levels = [values]
    choices_by_level: list[list[tuple[int, int, int, int] | None]] = []
    comb = log2_comb_table(expansion * (1 << depth))
    for level in range(1, depth + 1):
        parent_replica_count = replica_count // 2
        values, choices = lift_moment(
            values,
            comb,
            q_log2,
            None,
            parent_replica_count,
            singleton_charge,
            1 << (level - 1),
        )
        levels.append(values)
        choices_by_level.append(choices)
        replica_count = parent_replica_count
    return levels, choices_by_level


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=5)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--z", type=int, default=34)
    parser.add_argument("--min-u", type=int, default=9)
    parser.add_argument("--max-u", type=int, default=17)
    parser.add_argument(
        "--candidate-charge",
        choices=("component-uniform", "loose", "replica"),
        default="component-uniform",
    )
    args = parser.parse_args()

    scalar_rows = rows_for_budget(
        depth=args.depth,
        expansion=args.expansion,
        q_log2=args.q_log2,
        security_bits=args.security_bits,
        z=args.z,
        singleton_charge="replica",
    )
    ceiling_by_u: dict[int, float] = {}
    top_shape_by_u: dict[int, tuple[int, int, int]] = {}
    for row in scalar_rows:
        if row.get("section") != "u_budget":
            continue
        u = int(row["u"])
        ceiling_by_u[u] = float(row["child_plus_loss_ceiling_log2"])
        top_shape_by_u[u] = (int(row["best_p"]), int(row["best_s"]), int(row["best_c"]))

    levels, choices_by_level = build_levels_and_choices(
        depth=args.depth,
        expansion=args.expansion,
        q_log2=args.q_log2,
        singleton_charge=args.candidate_charge,
    )
    child_level = args.depth - 1
    candidate_child = levels[child_level]

    writer = csv.DictWriter(
        sys.stdout,
        fieldnames=[
            "u_target",
            "candidate_child_log2",
            "one_u_ceiling_log2",
            "excess_bits",
            "excess_qdim",
            "top_best_p",
            "top_best_s",
            "top_best_c",
            "trace_level",
            "replica_count",
            "parent_z",
            "p",
            "s",
            "c",
            "child_u",
            "extras",
            "child_k",
            "quotient_rank",
            "component_uniform_charge",
            "scalar_replica_charge",
            "missing_charge_qdim",
        ],
        lineterminator="\n",
    )
    writer.writeheader()
    for u_target in range(args.min_u, args.max_u + 1):
        if u_target >= len(candidate_child):
            continue
        candidate = candidate_child[u_target]
        ceiling = ceiling_by_u.get(u_target, NEG_INF)
        if candidate <= NEG_INF / 2 or ceiling <= NEG_INF / 2:
            continue
        top_p, top_s, top_c = top_shape_by_u.get(u_target, (-1, -1, -1))
        z = u_target
        for level in range(child_level, 0, -1):
            choice = choices_by_level[level - 1][z]
            if choice is None:
                break
            p, s, c, child_u = choice
            extras = s - c
            replica_count = 1 << (args.depth - level)
            child_k = 1 << (level - 1)
            quotient_rank = max(0, child_k - min(child_u, child_k))
            charge = component_uniform_charge(
                replica_count=replica_count,
                child_k=child_k,
                u=child_u,
                extras=extras,
            )
            scalar_charge = scalar_replica_charge(
                replica_count=replica_count,
                extras=extras,
            )
            writer.writerow(
                {
                    "u_target": u_target,
                    "candidate_child_log2": finite_label(candidate),
                    "one_u_ceiling_log2": finite_label(ceiling),
                    "excess_bits": f"{candidate - ceiling:.8f}",
                    "excess_qdim": f"{(candidate - ceiling) / args.q_log2:.8f}",
                    "top_best_p": top_p,
                    "top_best_s": top_s,
                    "top_best_c": top_c,
                    "trace_level": level,
                    "replica_count": replica_count,
                    "parent_z": z,
                    "p": p,
                    "s": s,
                    "c": c,
                    "child_u": child_u,
                    "extras": extras,
                    "child_k": child_k,
                    "quotient_rank": quotient_rank,
                    "component_uniform_charge": charge,
                    "scalar_replica_charge": scalar_charge,
                    "missing_charge_qdim": scalar_charge - charge,
                }
            )
            z = child_u


if __name__ == "__main__":
    main()
