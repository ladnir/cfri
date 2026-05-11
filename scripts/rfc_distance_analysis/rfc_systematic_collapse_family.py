#!/usr/bin/env python3
"""Emit the explicit all-level systematic RFC collapse family."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def collapse_columns(
    depth: int,
    total_expansion: int,
    copy: int,
    suffix: int,
    live_bits: int,
    missing_direction: int,
) -> tuple[list[int], list[int], list[int]]:
    k = 1 << depth
    parity_expansion = total_expansion - 1
    if not 0 <= copy < parity_expansion:
        raise ValueError("copy must be in [0,total_expansion-2]")
    if not 1 <= live_bits <= depth:
        raise ValueError("live-bits must be in [1,depth]")
    live_size = 1 << live_bits
    fixed_bits = depth - live_bits
    if not 0 <= suffix < (1 << fixed_bits):
        raise ValueError("suffix must fit in depth-live_bits bits")
    if not 0 <= missing_direction < live_size:
        raise ValueError("missing direction must fit in live_bits")

    live_rows = [(suffix << live_bits) | row for row in range(live_size)]
    systematic = [row for row in range(k) if row not in live_rows]
    parity = [
        k + ((((upper_path << live_bits) | direction) * parity_expansion) + copy)
        for upper_path in range(1 << fixed_bits)
        for direction in range(live_size)
        if direction != missing_direction
    ]
    return live_rows, systematic, parity


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--copy", type=int, default=0)
    parser.add_argument("--live-bits", type=int, default=1)
    parser.add_argument("--missing-direction", type=int, default=0)
    parser.add_argument("--suffix", type=int, default=None)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    fixed_bits = args.depth - args.live_bits
    suffix = (1 << fixed_bits) - 1 if args.suffix is None else args.suffix
    live_rows, systematic, parity = collapse_columns(
        args.depth,
        args.total_expansion,
        args.copy,
        suffix,
        args.live_bits,
        args.missing_direction,
    )
    columns = systematic + parity
    k = 1 << args.depth

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "total_expansion",
                "k",
                "live_rows",
                "live_bits",
                "missing_direction",
                "systematic_count",
                "parity_count",
                "zero_count",
                "columns",
            ]
        )
        writer.writerow(
            [
                args.depth,
                args.total_expansion,
                k,
                ":".join(str(row) for row in live_rows),
                args.live_bits,
                args.missing_direction,
                len(systematic),
                len(parity),
                len(columns),
                ":".join(str(column) for column in columns),
            ]
        )

    print(
        f"depth={args.depth} k={k} live={live_rows} zero_count={len(columns)} "
        f"out={args.out}"
    )


if __name__ == "__main__":
    main()
