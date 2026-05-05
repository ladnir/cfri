#!/usr/bin/env python3
"""Union-bound the aligned live-subcube two-copy collapse intersection event."""

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
    parser.add_argument("--max-depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--field-bits", type=float, default=128.0)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    parity_expansion = args.total_expansion - 1
    if parity_expansion < 2:
        raise SystemExit("--total-expansion must be at least 3 for two-copy intersections")

    copy_pair_log = log2_comb(parity_expansion, 2)

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "live_bits",
                "live_rows",
                "subcube_log2_count",
                "direction_pair_log2_count",
                "copy_pair_log2_count",
                "line_collision_log2_prob",
                "union_log2_bound",
            ]
        )

        for depth in range(1, args.max_depth + 1):
            k = 1 << depth
            for live_bits in range(1, depth + 1):
                live_rows = 1 << live_bits
                # Axis-aligned live subcubes: choose live coordinates and fix the rest.
                subcube_log = log2_comb(depth, live_bits) + (depth - live_bits)
                # One omitted live direction per copy.
                direction_pair_log = 2 * live_bits
                # Two independent random lines in F^m meet nontrivially only when equal.
                # The projective line count is (q^m-1)/(q-1), so this is q^{-(m-1)}
                # up to a negligible factor at q=2^field_bits.
                collision_log = -(live_rows - 1) * args.field_bits
                union_log = subcube_log + direction_pair_log + copy_pair_log + collision_log
                writer.writerow(
                    [
                        depth,
                        k,
                        live_bits,
                        live_rows,
                        f"{subcube_log:.8f}",
                        f"{direction_pair_log:.8f}",
                        f"{copy_pair_log:.8f}",
                        f"{collision_log:.8f}",
                        f"{union_log:.8f}",
                    ]
                )

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
