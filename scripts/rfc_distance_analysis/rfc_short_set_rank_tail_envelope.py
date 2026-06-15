#!/usr/bin/env python3
"""Paired-compression envelope for short-set RFC rank tails.

This is a diagnostic, not a theorem.  It compares the random-matrix
rank-deficiency exponent for a short coordinate set against the obvious RFC
all-paired compression branch:

    B_d(2u, s) <- B_{d-1}(u, ceil(s/2)).

The output is useful for flat-excess profiles, where a witness gives a child
rank-tail event below k with parameters z=|P|+2r+F and s=rho+r+F.
"""

from __future__ import annotations

import argparse
import math
from dataclasses import dataclass


@dataclass(frozen=True)
class EnvelopePoint:
    level: int
    k: int
    n: int
    z: int
    s: int
    q_exponent: int
    log2_choices: float
    log2_slack: float
    paired_steps: int


def log2_binom(n: int, k: int) -> float:
    if k < 0 or k > n:
        return float("-inf")
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


def random_matrix_q_exponent(k: int, z: int, s: int) -> int:
    """q-exponent for rank <= min(k,z)-s in a random k x z matrix."""
    if s <= 0:
        return 0
    if z <= k:
        return s * (k - z + s)
    return s * (z - k + s)


def paired_envelope(*, depth: int, expansion: int, z: int, s: int, q_log2: float) -> list[EnvelopePoint]:
    points: list[EnvelopePoint] = []
    cur_depth = depth
    cur_z = z
    cur_s = s
    paired_steps = 0

    while cur_depth >= 0 and cur_z > 0 and cur_s > 0:
        k = 1 << cur_depth
        n = expansion * k
        q_exp = random_matrix_q_exponent(k, cur_z, cur_s)
        choices = log2_binom(n, cur_z)
        points.append(
            EnvelopePoint(
                level=cur_depth,
                k=k,
                n=n,
                z=cur_z,
                s=cur_s,
                q_exponent=q_exp,
                log2_choices=choices,
                log2_slack=q_exp * q_log2 - choices,
                paired_steps=paired_steps,
            )
        )

        if cur_z % 2 != 0 or cur_depth == 0:
            break
        cur_depth -= 1
        cur_z //= 2
        cur_s = (cur_s + 1) // 2
        paired_steps += 1

    return points


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--depth", type=int, default=10, help="child depth for the short-set event")
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--z", type=int, required=True, help="short coordinate-set size")
    ap.add_argument("--s", type=int, required=True, help="rank deficit from full short-set rank")
    args = ap.parse_args()

    points = paired_envelope(
        depth=args.depth,
        expansion=args.expansion,
        z=args.z,
        s=args.s,
        q_log2=args.q_log2,
    )

    print("level,k,n,z,s,paired_steps,q_exponent,log2_choices,log2_slack")
    for p in points:
        print(
            f"{p.level},{p.k},{p.n},{p.z},{p.s},{p.paired_steps},"
            f"{p.q_exponent},{p.log2_choices:.6f},{p.log2_slack:.6f}"
        )

    worst = min(points, key=lambda p: p.log2_slack)
    print(
        "# worst_after_paired_compression="
        f"level:{worst.level},z:{worst.z},s:{worst.s},"
        f"q_exponent:{worst.q_exponent},log2_slack:{worst.log2_slack:.6f}"
    )


if __name__ == "__main__":
    main()
