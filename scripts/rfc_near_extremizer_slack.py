#!/usr/bin/env python3
"""Combine near-extremizer counts with conditioned-copy zero tails."""

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
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--field-bits", type=float, default=128.0)
    parser.add_argument("--max-extra", type=int, default=128)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    m = args.live_rows
    if m <= 0 or k % m != 0:
        raise SystemExit("--live-rows must divide k")
    parity_copies = args.total_expansion - 1
    other_copies = parity_copies - 1
    if other_copies <= 0:
        raise SystemExit("--total-expansion must leave at least one other parity copy")
    exact_output = k // m
    extras_available = k - exact_output

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "live_rows",
                "extra_outputs",
                "near_output_support",
                "near_count_log2",
                "needed_other_copy_zeros",
                "conditioned_tail_log2",
                "union_log2",
            ]
        )
        for extra in range(args.max_extra + 1):
            if extra > extras_available:
                break
            near_count_log = math.log2(parity_copies) + math.log2(k) + log2_comb(extras_available, extra)
            needed_zeros = extra + 1
            tail_log = math.log2(other_copies) + log2_comb(k, needed_zeros) - needed_zeros * args.field_bits
            writer.writerow(
                [
                    args.depth,
                    k,
                    m,
                    extra,
                    exact_output + extra,
                    f"{near_count_log:.8f}",
                    needed_zeros,
                    f"{tail_log:.8f}",
                    f"{near_count_log + tail_log:.8f}",
                ]
            )

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
