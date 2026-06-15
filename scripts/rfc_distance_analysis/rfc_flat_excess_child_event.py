#!/usr/bin/env python3
"""Translate a flat-excess witness into child rank-event parameters.

This is deterministic bookkeeping for the fixed-survivor route.  If A singleton coordinates have
rank r in K_P^* and flat excess F=|A|-2r, then P union A is a child rank event.  The script reports
whether that child event is large enough to be visible to near-MDS rank-tail bounds.
"""

from __future__ import annotations

import argparse


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--child-k", type=int, default=1024)
    ap.add_argument("--paired", type=int, default=137)
    ap.add_argument("--pair-deficit", type=int, default=887)
    ap.add_argument("--flat-excess", type=int, default=0)
    ap.add_argument("--max-rank", type=int, default=None)
    ap.add_argument("--step", type=int, default=32)
    args = ap.parse_args()

    child_k = args.child_k
    p = args.paired
    d = args.pair_deficit
    f = args.flat_excess
    rho = p + d - child_k
    max_rank = args.max_rank if args.max_rank is not None else d - 1

    print(
        "child_k,paired,pair_deficit,pair_nullity_slack,flat_excess,"
        "flat_rank,a_size,child_set_size,child_rank_bound,child_rank_deficit,"
        "child_set_excess_over_k,excess_over_rank_bound,ordinary_distance_visible"
    )
    for r in range(0, max_rank + 1, args.step):
        a_size = 2 * r + f
        child_set_size = p + a_size
        child_rank_bound = child_k - d + r
        child_rank_deficit = d - r
        child_set_excess_over_k = child_set_size - child_k
        excess_over_rank_bound = child_set_size - child_rank_bound
        ordinary_visible = child_set_size >= child_k
        print(
            f"{child_k},{p},{d},{rho},{f},{r},{a_size},{child_set_size},"
            f"{child_rank_bound},{child_rank_deficit},{child_set_excess_over_k},"
            f"{excess_over_rank_bound},{int(ordinary_visible)}"
        )


if __name__ == "__main__":
    main()
