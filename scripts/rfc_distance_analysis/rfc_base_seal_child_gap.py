#!/usr/bin/env python3
"""Compare depth-4 child bounds against the depth-5 base-seal calibration.

This is the first narrow check for the finite base-seal route.  The top
boundary budget says each aggregate child value B_4(2,u) must stay very close
to the optimistic scalar replica value.  This script joins that calibration
table against another child-bound mode and reports the per-u gap.

The default comparison mode is `component-uniform`, which is intentionally
coarse.  It should fail; the point is to make the size and location of that
failure explicit before replacing it with the finite exact-support flag DP.
"""

from __future__ import annotations

import argparse
import csv
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_base_seal_budget import build_levels, rows_for_budget  # noqa: E402
from rfc_replica_zero_moment import NEG_INF  # noqa: E402


def finite_label(value: float) -> str:
    return "-inf" if value <= NEG_INF / 2 else f"{value:.8f}"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=5)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--z", type=int, default=34)
    parser.add_argument(
        "--candidate-charge",
        choices=("component-uniform", "loose", "replica"),
        default="component-uniform",
        help="child-bound mode to compare against the optimistic scalar ceiling",
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
    ceilings: dict[int, tuple[float, float, float]] = {}
    for row in scalar_rows:
        if row.get("section") != "u_budget":
            continue
        u = int(row["u"])
        scalar_child = float(row["child_log2"])
        ceiling = float(row["child_plus_loss_ceiling_log2"])
        loss_budget = float(row["loss_budget_bits"])
        ceilings[u] = (scalar_child, ceiling, loss_budget)

    candidate_levels = build_levels(
        depth=args.depth,
        expansion=args.expansion,
        q_log2=args.q_log2,
        singleton_charge=args.candidate_charge,
    )
    candidate_child = candidate_levels[args.depth - 1]

    writer = csv.DictWriter(
        sys.stdout,
        fieldnames=[
            "u",
            "scalar_child_log2",
            "candidate_child_log2",
            "one_u_ceiling_log2",
            "loss_budget_bits",
            "candidate_minus_scalar_bits",
            "candidate_minus_ceiling_bits",
            "candidate_minus_ceiling_qdim",
            "status",
        ],
        lineterminator="\n",
    )
    writer.writeheader()
    for u in sorted(ceilings):
        scalar_child, ceiling, loss_budget = ceilings[u]
        candidate = candidate_child[u] if 0 <= u < len(candidate_child) else NEG_INF
        if candidate <= NEG_INF / 2:
            delta_scalar = NEG_INF
            delta_ceiling = NEG_INF
            status = "no_candidate"
        else:
            delta_scalar = candidate - scalar_child
            delta_ceiling = candidate - ceiling
            status = "passes_one_u_ceiling" if delta_ceiling <= 0 else "fails_one_u_ceiling"
        writer.writerow(
            {
                "u": u,
                "scalar_child_log2": finite_label(scalar_child),
                "candidate_child_log2": finite_label(candidate),
                "one_u_ceiling_log2": finite_label(ceiling),
                "loss_budget_bits": f"{loss_budget:.8f}",
                "candidate_minus_scalar_bits": finite_label(delta_scalar),
                "candidate_minus_ceiling_bits": finite_label(delta_ceiling),
                "candidate_minus_ceiling_qdim": (
                    "-inf" if delta_ceiling <= NEG_INF / 2 else f"{delta_ceiling / args.q_log2:.8f}"
                ),
                "status": status,
            }
        )


if __name__ == "__main__":
    main()
