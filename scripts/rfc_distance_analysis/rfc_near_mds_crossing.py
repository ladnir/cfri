#!/usr/bin/env python3
"""Compute first-moment near-MDS crossings for RFC rank-tail targets."""

from __future__ import annotations

import argparse
import math


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return -math.inf
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


def find_crossing(n: int, k: int, q_bits: float, security_bits: float, log2_c_factor: float) -> tuple[int, float]:
    for z in range(k, n + 1):
        e = z - k
        log2_bad = log2_comb(n, z) + log2_c_factor - (e + 1) * q_bits
        if log2_bad <= -security_bits:
            return z, log2_bad
    raise SystemExit("no crossing before z=N")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--k", type=int, required=True)
    parser.add_argument("--expansion", type=int, required=True)
    parser.add_argument("--q-bits", type=float, required=True)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--log2-c-factor", type=float, default=0.0)
    args = parser.parse_args()

    n = args.expansion * args.k
    z, log2_bad = find_crossing(n, args.k, args.q_bits, args.security_bits, args.log2_c_factor)
    e = z - args.k
    distance = n - z + 1
    relative_distance = distance / n
    mds_relative_distance = (n - args.k + 1) / n
    print("k,n,expansion,q_bits,security_bits,log2_c_factor,z,e,log2_bad,distance,relative_distance,mds_relative_distance,relative_gap")
    print(
        f"{args.k},{n},{args.expansion},{args.q_bits:g},{args.security_bits:g},{args.log2_c_factor:g},"
        f"{z},{e},{log2_bad:.6f},{distance},{relative_distance:.12f},"
        f"{mds_relative_distance:.12f},{(mds_relative_distance - relative_distance):.12f}"
    )


if __name__ == "__main__":
    main()
