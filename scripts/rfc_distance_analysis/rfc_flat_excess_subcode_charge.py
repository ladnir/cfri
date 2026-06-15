#!/usr/bin/env python3
"""Subcode first-moment exponent for flat-excess witnesses.

A rank-r quotient flat A with flat excess F gives an h=(D-r)-dimensional subcode that vanishes on
z=|P|+2r+F child coordinates.  This script reports the random-code q-exponent:

    [k choose h]_q * binom(n,z) * q^{-h z}

as a sanity target for an RFC subcode-zero theorem.
"""

from __future__ import annotations

import argparse
import math


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return -math.inf
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--child-k", type=int, default=1024)
    ap.add_argument("--child-n", type=int, default=8192)
    ap.add_argument("--paired", type=int, default=137)
    ap.add_argument("--pair-deficit", type=int, default=887)
    ap.add_argument("--flat-excess", type=int, default=1)
    ap.add_argument("--max-rank", type=int, default=16)
    ap.add_argument("--q-log2", type=float, default=128.0)
    args = ap.parse_args()

    print(
        "child_k,child_n,paired,pair_deficit,flat_excess,flat_rank,"
        "subcode_dim,zero_count,q_exponent,log2_coordinate_choices,log2_first_moment"
    )
    for r in range(args.max_rank + 1):
        h = args.pair_deficit - r
        z = args.paired + 2 * r + args.flat_excess
        # Gaussian binomial exponent h(k-h), then z zero constraints on an h-dimensional subspace.
        q_exponent = h * (args.child_k - h - z)
        log2_coord = log2_comb(args.child_n, z)
        log2_moment = q_exponent * args.q_log2 + log2_coord
        print(
            f"{args.child_k},{args.child_n},{args.paired},{args.pair_deficit},"
            f"{args.flat_excess},{r},{h},{z},{q_exponent},{log2_coord:.6f},"
            f"{log2_moment:.6f}"
        )


if __name__ == "__main__":
    main()
