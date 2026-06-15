#!/usr/bin/env python3
"""Top-profile scale for charging flat excess by marked incremental rank.

This is a random-matrix scale check, not a certificate.

For a fixed top survivor profile (P,T), a flat-excess witness is a subset
A <= T with

    a = |A| = 2r + F,
    rank(A modulo P) <= r,

where F is flat_excess. The rank condition is exactly a marked incremental
rank event for the child restriction.

This script combines:

    profile choices for P, A, and T\\A,
    random-matrix marked-rank q-exponent for A modulo P,
    the residual root-repair failure exponent t - 2D + 1 - F.

It sweeps r to find the largest remaining first-moment term.
"""

from __future__ import annotations

import argparse
import math
from dataclasses import dataclass


NEG_INF = -1.0e300


def log2_add(left: float, right: float) -> float:
    if left <= NEG_INF / 2:
        return right
    if right <= NEG_INF / 2:
        return left
    if right > left:
        left, right = right, left
    return left + math.log2(1.0 + 2.0 ** (right - left))


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


@dataclass
class Row:
    flat_excess: int
    r: int
    a: int
    marked_q_exp: int
    repair_q_exp: int
    total_q_exp: int
    log2_profile: float
    log2_term: float


def dominant_top_profile(depth: int, expansion: int, excess: int) -> tuple[int, int, int, int]:
    k = 1 << depth
    k_child = k >> 1
    n_child = expansion * k_child
    survivor_size = k + excess
    best = (NEG_INF, 0, 0, 0)
    for p in range(0, min(k_child, n_child) + 1):
        t = survivor_size - 2 * p
        if t < 0 or t > n_child - p:
            continue
        d_min = max(0, k_child - p)
        if d_min <= 0 or t < 2 * d_min:
            continue
        # Ideal surplus term. This locates the same top profile as the tolerance script.
        profile = log2_comb(n_child, p) + log2_comb(n_child - p, t) + t
        q_exp = t - 2 * d_min + 1
        # q_log2 is not needed to identify the profile for fixed q; return maximum profile-q shape.
        score = profile - 128.0 * q_exp
        if score > best[0]:
            best = (score, p, t, d_min)
    return n_child, best[1], best[2], best[3]


def row_for(
    n_child: int,
    p: int,
    t: int,
    d: int,
    q_log2: float,
    flat_excess: int,
    r: int,
    side_choices: bool,
) -> Row | None:
    a = 2 * r + flat_excess
    if r < 0 or r >= d:
        return None
    if a < 1 or a > t:
        return None
    if p + a > n_child:
        return None

    marked_q_exp = (a - r) * (d - r)
    repair_q_exp = max(0, t - 2 * d + 1 - flat_excess)
    total_q_exp = marked_q_exp + repair_q_exp

    # Choose paired core P, marked flat witness A, and the remaining singleton set.
    log2_profile = (
        log2_comb(n_child, p)
        + log2_comb(n_child - p, a)
        + log2_comb(n_child - p - a, t - a)
    )
    if side_choices:
        log2_profile += t

    log2_term = log2_profile - q_log2 * total_q_exp
    return Row(
        flat_excess=flat_excess,
        r=r,
        a=a,
        marked_q_exp=marked_q_exp,
        repair_q_exp=repair_q_exp,
        total_q_exp=total_q_exp,
        log2_profile=log2_profile,
        log2_term=log2_term,
    )


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--depth", type=int, default=11)
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--excess", type=int, default=71)
    ap.add_argument("--flat-excess", type=int, action="append", required=True)
    ap.add_argument("--p", type=int, default=None)
    ap.add_argument("--t", type=int, default=None)
    ap.add_argument("--d", type=int, default=None)
    ap.add_argument("--side-choices", action="store_true", default=True)
    ap.add_argument("--top", type=int, default=8)
    args = ap.parse_args()

    if args.p is None or args.t is None or args.d is None:
        n_child, p, t, d = dominant_top_profile(args.depth, args.expansion, args.excess)
    else:
        k_child = 1 << (args.depth - 1)
        n_child = args.expansion * k_child
        p, t, d = args.p, args.t, args.d

    print(
        "depth,expansion,q_log2,excess,n_child,p,t,D,flat_excess,"
        "dominant_r,a,marked_q_exp,repair_q_exp,total_q_exp,"
        "log2_profile,log2_term,status"
    )
    for f in args.flat_excess:
        total = NEG_INF
        rows: list[Row] = []
        for r in range(d):
            row = row_for(n_child, p, t, d, args.q_log2, f, r, args.side_choices)
            if row is None:
                continue
            rows.append(row)
            total = log2_add(total, row.log2_term)
        rows.sort(key=lambda row: row.log2_term, reverse=True)
        if not rows:
            print(
                f"{args.depth},{args.expansion},{args.q_log2:g},{args.excess},"
                f"{n_child},{p},{t},{d},{f},,,,,,,,-inf,no_rows"
            )
            continue
        best = rows[0]
        status = "pass" if total <= -1.0 else "fail"
        print(
            f"{args.depth},{args.expansion},{args.q_log2:g},{args.excess},"
            f"{n_child},{p},{t},{d},{f},{best.r},{best.a},"
            f"{best.marked_q_exp},{best.repair_q_exp},{best.total_q_exp},"
            f"{best.log2_profile:.6f},{best.log2_term:.6f},{status}"
        )
        for row in rows[: max(0, args.top - 1)]:
            print(
                f"# row f={f} r={row.r} a={row.a} marked_q={row.marked_q_exp} "
                f"repair_q={row.repair_q_exp} total_q={row.total_q_exp} "
                f"log2_profile={row.log2_profile:.6f} log2_term={row.log2_term:.6f}"
            )
        print(f"# total f={f} log2_sum={total:.6f} status={status}")


if __name__ == "__main__":
    main()
