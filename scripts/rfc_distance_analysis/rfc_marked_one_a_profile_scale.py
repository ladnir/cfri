#!/usr/bin/env python3
"""Crude scale check for one marked A coordinate in incremental flat-rank.

This compares the two top shapes for I_d(p,1,0):

    A0: A is alone in its sibling pair.
    PA: A's sibling is in P.

The exponents are random-projection heuristics, not theorem bounds.  The point is to see whether
the mixed PA category could plausibly threaten the small-flat slack.
"""

from __future__ import annotations

import argparse
import math


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return float("-inf")
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--depth", type=int, default=10)
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--p", type=int, default=137)
    args = ap.parse_args()

    k = 1 << args.depth
    n = args.expansion * k
    k_child = k >> 1
    n_child = n >> 1

    # A0: choose the A coordinate, forbid its sibling from P, choose P elsewhere.
    a0_log2_count = math.log2(n) + log2_comb(n - 2, args.p)
    a0_q_exp = max(0, k - args.p)
    a0_log2_moment = a0_log2_count - a0_q_exp * args.q_log2

    # PA: choose child pair and which side is A, the other side is P, choose remaining P elsewhere.
    pa_log2_count = math.log2(n_child) + 1.0 + log2_comb(n - 2, args.p - 1)
    # Mixed estimate from the PA projection lemma: the complementary root-line is recoverable only
    # if the child direction is in the projection span of the *other* P columns. The same-coordinate
    # P sibling is excluded, so the projection rank is at most p-1.
    pa_q_exp = max(0, k_child - (args.p - 1))
    pa_log2_moment = pa_log2_count - pa_q_exp * args.q_log2

    print("depth,expansion,k,n,q_log2,p,shape,q_exponent,log2_count,log2_moment")
    print(
        f"{args.depth},{args.expansion},{k},{n},{args.q_log2:g},{args.p},"
        f"A0,{a0_q_exp},{a0_log2_count:.6f},{a0_log2_moment:.6f}"
    )
    print(
        f"{args.depth},{args.expansion},{k},{n},{args.q_log2:g},{args.p},"
        f"PA_crude,{pa_q_exp},{pa_log2_count:.6f},{pa_log2_moment:.6f}"
    )


if __name__ == "__main__":
    main()
