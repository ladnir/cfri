#!/usr/bin/env python3
"""Compute the best generalized collapse-family ceiling by depth."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def best_collapse(depth: int) -> tuple[int, int]:
    k = 1 << depth
    best_m = 2
    best_z = -1
    for live_bits in range(1, depth + 1):
        m = 1 << live_bits
        z = 2 * k - m - (k // m)
        if z > best_z:
            best_z = z
            best_m = m
    return best_m, best_z


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--max-depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "total_n",
                "best_live_rows",
                "zero_count",
                "distance_upper_bound",
                "relative_distance_upper_bound",
            ]
        )
        for depth in range(1, args.max_depth + 1):
            k = 1 << depth
            total_n = args.total_expansion * k
            best_m, zero_count = best_collapse(depth)
            distance = total_n - zero_count
            writer.writerow(
                [
                    depth,
                    k,
                    total_n,
                    best_m,
                    zero_count,
                    distance,
                    f"{distance / total_n:.8f}",
                ]
            )

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
