#!/usr/bin/env python3
"""Compute near-extremizer slack summaries over all power-of-two live sizes."""

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
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--field-bits", type=float, default=128.0)
    parser.add_argument("--max-extra", type=int, default=128)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    parity_copies = args.total_expansion - 1
    other_copies = parity_copies - 1
    if other_copies <= 0:
        raise SystemExit("--total-expansion must leave at least one other parity copy")

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "live_bits",
                "live_rows",
                "worst_extra_outputs",
                "worst_near_output_support",
                "worst_union_log2",
                "exact_union_log2",
                "max_extra_checked",
            ]
        )
        for live_bits in range(1, args.depth + 1):
            m = 1 << live_bits
            exact_output = k // m
            extras_available = k - exact_output
            max_extra = min(args.max_extra, extras_available)
            worst_extra = 0
            worst_union = -math.inf
            exact_union = None
            for extra in range(max_extra + 1):
                near_count_log = math.log2(parity_copies) + math.log2(k) + log2_comb(extras_available, extra)
                needed_zeros = extra + 1
                tail_log = log2_comb(other_copies * k, needed_zeros) - needed_zeros * args.field_bits
                union_log = near_count_log + tail_log
                if extra == 0:
                    exact_union = union_log
                if union_log > worst_union:
                    worst_union = union_log
                    worst_extra = extra
            writer.writerow(
                [
                    args.depth,
                    k,
                    live_bits,
                    m,
                    worst_extra,
                    exact_output + worst_extra,
                    f"{worst_union:.8f}",
                    f"{exact_union:.8f}",
                    max_extra,
                ]
            )

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
