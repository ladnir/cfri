#!/usr/bin/env python3
"""Random-code scale for marked incremental flat-rank events.

For disjoint marked sets (P,A), this estimates the first-moment exponent for

    rank(P union A) - rank(P) <= r.

Conditioned on rank(P)=p-u, the quotient ambient dimension is

    D = k - p + u,

and the random-matrix q-exponent for A of size a having quotient rank <= r is

    (a-r)(D-r).

This is the marked version of the small-flat charge; unlike aggregate B_d(z,s), it does not count
dependencies contained entirely inside P as flat witnesses.
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
    ap.add_argument("--k", type=int, default=1024)
    ap.add_argument("--n", type=int, default=8192)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--p", type=int, default=137, help="marked core size |P|")
    ap.add_argument("--a", type=int, required=True, help="marked flat-witness size |A|")
    ap.add_argument("--rank", type=int, required=True, help="target quotient rank bound r")
    ap.add_argument("--core-defect", type=int, default=0, help="u=p-rank(P)")
    args = ap.parse_args()

    quotient_dim = args.k - args.p + args.core_defect
    if args.rank > args.a:
        raise SystemExit("rank bound cannot exceed |A|")
    if args.rank > quotient_dim:
        q_exp = 0
    else:
        q_exp = (args.a - args.rank) * (quotient_dim - args.rank)

    core_choices = log2_comb(args.n, args.p)
    witness_choices = log2_comb(args.n - args.p, args.a)
    log2_moment = core_choices + witness_choices - q_exp * args.q_log2

    print(
        "k,n,q_log2,p,a,rank_bound,core_defect,quotient_dim,"
        "q_exponent,log2_core_choices,log2_witness_choices,log2_first_moment"
    )
    print(
        f"{args.k},{args.n},{args.q_log2:g},{args.p},{args.a},{args.rank},"
        f"{args.core_defect},{quotient_dim},{q_exp},{core_choices:.6f},"
        f"{witness_choices:.6f},{log2_moment:.6f}"
    )


if __name__ == "__main__":
    main()
