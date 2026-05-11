#!/usr/bin/env python3
"""Estimate low-output tails for an independent copy conditioned on a fixed message."""

from __future__ import annotations

import argparse
import csv
import math
from pathlib import Path


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return -math.inf
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--field-bits", type=float, default=128.0)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["depth", "k", "output_weight_bound", "zero_count", "log2_union_tail"])
        for output_weight in range(0, k + 1):
            zero_count = k - output_weight
            log_tail = log2_comb(k, output_weight) - zero_count * args.field_bits
            writer.writerow([args.depth, k, output_weight, zero_count, f"{log_tail:.8f}"])

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
