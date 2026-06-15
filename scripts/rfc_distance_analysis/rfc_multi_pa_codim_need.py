#!/usr/bin/env python3
"""Codimension budget for all-mixed PA flat witnesses.

This is a simple first-moment scale calculator.  It asks how much finite-root codimension is needed
for a marked all-PA witness of size a and target quotient rank r, given a core P of size p.

It is not a certificate; it is a pressure check for whether a one-minor q^-1 bound can be enough.
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
    ap.add_argument("--child-k", type=int, default=1024)
    ap.add_argument("--child-n", type=int, default=8192)
    ap.add_argument("--paired-core", type=int, default=137)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--flat-rank", type=int, required=True, help="target quotient rank r")
    ap.add_argument("--flat-excess", type=int, required=True, help="F where a=2r+F")
    ap.add_argument("--security-log2", type=float, default=-1.0)
    args = ap.parse_args()

    r = args.flat_rank
    a = 2 * r + args.flat_excess
    # All-PA means a of the core P siblings are paired with A siblings at the same child positions.
    # Choose a mixed child positions, choose their sides, then choose remaining P positions away
    # from those child positions.  This is slightly conservative because it treats non-mixed P as
    # singleton P positions.
    if a > args.paired_core:
        raise SystemExit("all-PA witness cannot have a > paired-core in this simple profile")
    log2_profiles = (
        log2_comb(args.child_n, a)
        + a
        + log2_comb(args.child_n - a, args.paired_core - a)
    )
    needed_qdim = max(0.0, (log2_profiles - args.security_log2) / args.q_log2)
    random_like_qdim = max(0, a - r) * max(0, args.child_k // 2 - (args.paired_core - a) - r)
    one_minor_log2 = log2_profiles - args.q_log2

    print(
        "child_k,child_n,paired_core,q_log2,flat_rank,flat_excess,a,"
        "log2_profiles,needed_qdim_for_target,one_minor_log2,random_like_qdim"
    )
    print(
        f"{args.child_k},{args.child_n},{args.paired_core},{args.q_log2:g},{r},"
        f"{args.flat_excess},{a},{log2_profiles:.6f},{needed_qdim:.6f},"
        f"{one_minor_log2:.6f},{random_like_qdim}"
    )


if __name__ == "__main__":
    main()
