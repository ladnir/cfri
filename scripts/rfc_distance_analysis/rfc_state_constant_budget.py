#!/usr/bin/env python3
"""Compute q-dimensional budgets for hard-trace state constants.

This is a deterministic bookkeeping helper for the RFC distance notes.  It
converts simple state-count models into log_q units so they can be compared
with hard-trace margins such as:

    9 * hard_steps + rho_terminal - E_anc.

It is not an enumerator and does not sample codes.
"""

from __future__ import annotations

import argparse
import math


def log2_int(value: int) -> float:
    if value <= 0:
        raise ValueError("value must be positive")
    return math.log2(value)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--N", type=int, default=16384)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--poly-degree", type=float, default=0.0)
    parser.add_argument("--q-factors", type=float, default=0.0)
    parser.add_argument("--finite-labels", type=int, default=1)
    parser.add_argument("--label-power", type=int, default=1)
    parser.add_argument("--margin", type=float, default=7.0)
    args = parser.parse_args()

    log2_total = args.poly_degree * log2_int(args.N)
    log2_total += args.label_power * log2_int(args.finite_labels)
    qdim_poly_labels = log2_total / args.q_log2
    qdim_total = qdim_poly_labels + args.q_factors
    print("N,q_log2,poly_degree,q_factors,finite_labels,label_power,qdim_total,margin,slack")
    print(
        f"{args.N},{args.q_log2},{args.poly_degree},{args.q_factors},"
        f"{args.finite_labels},{args.label_power},{qdim_total:.8f},"
        f"{args.margin:.8f},{args.margin - qdim_total:.8f}"
    )


if __name__ == "__main__":
    main()
