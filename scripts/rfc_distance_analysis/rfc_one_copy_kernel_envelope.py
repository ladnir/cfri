#!/usr/bin/env python3
"""Compute the one-copy kernel-dimension envelope implied by uncertainty."""

from __future__ import annotations

import argparse
import csv
import math
from pathlib import Path


def kernel_dim_bound(k: int, live_rows: int, zero_count: int) -> int:
    if zero_count >= k:
        return 0
    output_budget = k - zero_count
    required_input_support = math.ceil(k / output_budget)
    return max(0, live_rows + 1 - required_input_support)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "live_bits",
                "live_rows",
                "parity_zero_count",
                "output_budget",
                "kernel_dim_bound",
            ]
        )
        for live_bits in range(1, args.depth + 1):
            live_rows = 1 << live_bits
            extremal_zero = k - k // live_rows
            window_lo = max(0, extremal_zero - live_rows)
            window_hi = min(k, extremal_zero + live_rows)
            for zero_count in range(window_lo, window_hi + 1):
                writer.writerow(
                    [
                        args.depth,
                        k,
                        live_bits,
                        live_rows,
                        zero_count,
                        k - zero_count,
                        kernel_dim_bound(k, live_rows, zero_count),
                    ]
                )

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
