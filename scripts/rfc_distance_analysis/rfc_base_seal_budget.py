#!/usr/bin/env python3
"""Budget the finite depth-5 RFC base seal against child-bound losses.

This is deterministic bookkeeping for the hybrid top-boundary route.  It uses
the optimistic scalar replica recurrence only as a calibration baseline and
answers:

    How many bits can the depth-4 B_4(2,u) child bounds lose before the
    top-level B_5(1,34) target exceeds 2^-80?

The answer is intentionally sobering: the top boundary has only tens of bits of
slack, so a theorem-grade finite flag recurrence must be nearly as sharp as the
scalar calibration on the dominant u values.
"""

from __future__ import annotations

import argparse
import csv
import math
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_replica_zero_moment import (  # noqa: E402
    NEG_INF,
    initial_nonzero_moment,
    lift_moment,
    lift_terms_for_z,
    log2_add,
    log2_comb_table,
)


def build_levels(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    singleton_charge: str,
) -> list[list[float]]:
    replica_count = 1 << depth
    values = initial_nonzero_moment(expansion, replica_count, q_log2)
    levels = [values]
    comb = log2_comb_table(expansion * (1 << depth))
    for level in range(1, depth + 1):
        parent_replica_count = replica_count // 2
        values, _choices = lift_moment(
            values,
            comb,
            q_log2,
            None,
            parent_replica_count,
            singleton_charge,
            1 << (level - 1),
        )
        levels.append(values)
        replica_count = parent_replica_count
    return levels


def log2_sum(values: list[float]) -> float:
    total = NEG_INF
    for value in values:
        total = log2_add(total, value)
    return total


def add_loss_to_group(
    terms: list[tuple[float, int, int, int, int]],
    *,
    target_u: int | None,
    loss_bits: float,
) -> float:
    shifted: list[float] = []
    for term, _p, _s, _c, u in terms:
        if target_u is None or u == target_u:
            shifted.append(term + loss_bits)
        else:
            shifted.append(term)
    return log2_sum(shifted)


def max_loss_for_group(
    terms: list[tuple[float, int, int, int, int]],
    *,
    target_u: int | None,
    target_log2: float,
) -> float:
    if add_loss_to_group(terms, target_u=target_u, loss_bits=0.0) > target_log2:
        return 0.0
    lo = 0.0
    hi = 1.0
    while add_loss_to_group(terms, target_u=target_u, loss_bits=hi) <= target_log2:
        hi *= 2.0
        if hi > 100000.0:
            return hi
    for _ in range(80):
        mid = (lo + hi) / 2.0
        if add_loss_to_group(terms, target_u=target_u, loss_bits=mid) <= target_log2:
            lo = mid
        else:
            hi = mid
    return lo


def rows_for_budget(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    security_bits: float,
    z: int,
    singleton_charge: str,
) -> list[dict[str, object]]:
    levels = build_levels(
        depth=depth,
        expansion=expansion,
        q_log2=q_log2,
        singleton_charge=singleton_charge,
    )
    comb = log2_comb_table(expansion * (1 << depth))
    child = levels[depth - 1]
    terms = lift_terms_for_z(
        child,
        comb,
        q_log2,
        None,
        1,
        singleton_charge,
        1 << (depth - 1),
        z,
    )
    total = log2_sum([term for term, _p, _s, _c, _u in terms])
    target_log2 = -security_bits
    uniform_budget = max_loss_for_group(terms, target_u=None, target_log2=target_log2)
    rows: list[dict[str, object]] = [
        {
            "section": "summary",
            "z": z,
            "log2_moment": f"{total:.8f}",
            "target_log2": f"{target_log2:.8f}",
            "slack_bits": f"{target_log2 - total:.8f}",
            "uniform_child_loss_budget_bits": f"{uniform_budget:.8f}",
            "uniform_child_loss_budget_qdim": f"{uniform_budget / q_log2:.8f}",
        }
    ]

    by_u: dict[int, list[float]] = {}
    best_by_u: dict[int, tuple[float, int, int, int, int]] = {}
    for term in terms:
        term_log2, p, s, c, u = term
        by_u.setdefault(u, []).append(term_log2)
        if u not in best_by_u or term_log2 > best_by_u[u][0]:
            best_by_u[u] = (term_log2, p, s, c, u)
    best_term = terms[0][0]
    for u in sorted(by_u):
        group_log2 = log2_sum(by_u[u])
        budget = max_loss_for_group(terms, target_u=u, target_log2=target_log2)
        best_u_term, p, s, c, _u = best_by_u[u]
        child_log2 = child[u] if 0 <= u < len(child) else NEG_INF
        rows.append(
            {
                "section": "u_budget",
                "u": u,
                "child_log2": "-inf" if child_log2 <= NEG_INF / 2 else f"{child_log2:.8f}",
                "term_count": len(by_u[u]),
                "group_log2": f"{group_log2:.8f}",
                "group_gap_bits": f"{best_term - group_log2:.8f}",
                "best_term_log2": f"{best_u_term:.8f}",
                "best_term_gap_bits": f"{best_term - best_u_term:.8f}",
                "best_p": p,
                "best_s": s,
                "best_c": c,
                "loss_budget_bits": f"{budget:.8f}",
                "loss_budget_qdim": f"{budget / q_log2:.8f}",
                "child_plus_loss_ceiling_log2": (
                    "-inf"
                    if child_log2 <= NEG_INF / 2
                    else f"{child_log2 + budget:.8f}"
                ),
            }
        )
    return rows


def emit_csv(rows: list[dict[str, object]]) -> None:
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
    parser.add_argument("--depth", type=int, default=5)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--z", type=int, default=34)
    parser.add_argument(
        "--singleton-charge",
        choices=("replica", "component-uniform", "loose"),
        default="replica",
    )
    args = parser.parse_args()
    emit_csv(
        rows_for_budget(
            depth=args.depth,
            expansion=args.expansion,
            q_log2=args.q_log2,
            security_bits=args.security_bits,
            z=args.z,
            singleton_charge=args.singleton_charge,
        )
    )


if __name__ == "__main__":
    main()
