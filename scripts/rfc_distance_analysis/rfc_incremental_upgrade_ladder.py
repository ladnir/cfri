#!/usr/bin/env python3
"""Incremental RFC proof-upgrade scale checks.

This is a small theorem-facing calculator for the crawl-before-run ladder.
It currently implements Tier 1: high-rank flat excess, h=1, reduced to an
ordinary child line-zero event plus residual root repair.
"""

from __future__ import annotations

import argparse
import math


NEG_INF = -1.0e300


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


def dominant_top_profile(depth: int, expansion: int, excess: int, q_log2: float) -> tuple[int, int, int, int]:
    k = 1 << depth
    k_child = k >> 1
    n_child = expansion * k_child
    survivor_size = k + excess
    best_score = NEG_INF
    best_p = best_t = best_d = 0

    for p in range(min(k_child, n_child) + 1):
        t = survivor_size - 2 * p
        if t < 0 or t > n_child - p:
            continue
        d = max(0, k_child - p)
        if d <= 0 or t < 2 * d:
            continue
        profile = log2_comb(n_child, p) + log2_comb(n_child - p, t) + t
        repair_exp = t - 2 * d + 1
        score = profile - q_log2 * repair_exp
        if score > best_score:
            best_score = score
            best_p, best_t, best_d = p, t, d

    return best_p, best_t, best_d, survivor_size


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--depth", type=int, default=11)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--excess", type=int, default=71)
    parser.add_argument("--flat-excess", type=int, action="append", required=True)
    parser.add_argument("--p", type=int, default=None)
    parser.add_argument("--t", type=int, default=None)
    parser.add_argument("--D", type=int, default=None)
    args = parser.parse_args()

    k = 1 << args.depth
    k_child = k >> 1
    n_child = args.expansion * k_child

    if args.p is None or args.t is None or args.D is None:
        p, t, d, survivor_size = dominant_top_profile(
            args.depth, args.expansion, args.excess, args.q_log2
        )
    else:
        p, t, d = args.p, args.t, args.D
        survivor_size = k + args.excess

    surplus = t - 2 * d + 1

    print(
        "depth,expansion,q_log2,excess,k,k_child,n_child,survivor_size,"
        "p,t,D,S,flat_excess,h,r,a,zero_set_size,line_zero_q_exp,"
        "residual_repair_q_exp,total_q_exp,total_bits,status"
    )
    for flat_excess in args.flat_excess:
        h = 1
        r = d - h
        a = 2 * r + flat_excess
        zero_set_size = p + a
        line_zero_q_exp = zero_set_size - k_child + 1
        residual_repair_q_exp = max(0, surplus - flat_excess)
        total_q_exp = line_zero_q_exp + residual_repair_q_exp
        total_bits = total_q_exp * args.q_log2
        status = "valid_h1" if 0 <= a <= t and zero_set_size <= n_child else "invalid_profile"
        print(
            f"{args.depth},{args.expansion},{args.q_log2:g},{args.excess},"
            f"{k},{k_child},{n_child},{survivor_size},{p},{t},{d},{surplus},"
            f"{flat_excess},{h},{r},{a},{zero_set_size},{line_zero_q_exp},"
            f"{residual_repair_q_exp},{total_q_exp},{total_bits:.6f},{status}"
        )


if __name__ == "__main__":
    main()
