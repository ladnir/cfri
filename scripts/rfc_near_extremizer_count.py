#!/usr/bin/env python3
"""Count the matched-plus-extra near-extremizer model."""

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
    parser.add_argument("--live-rows", type=int, required=True)
    parser.add_argument("--max-extra", type=int, default=32)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    m = args.live_rows
    if m <= 0 or k % m != 0:
        raise SystemExit("--live-rows must divide k")
    exact_output = k // m
    extras_available = k - exact_output

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "live_rows",
                "exact_output_support",
                "extra_outputs",
                "near_output_support",
                "model_count_log2",
                "model_count",
            ]
        )
        for extra in range(args.max_extra + 1):
            if extra > extras_available:
                break
            log_count = math.log2(k) + log2_comb(extras_available, extra)
            count = "" if log_count > 60 else str(int(round(2**log_count)))
            writer.writerow([args.depth, k, m, exact_output, extra, exact_output + extra, f"{log_count:.8f}", count])

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
