#!/usr/bin/env python3
"""Test selected rank shapes across independent RFC generator samples."""

from __future__ import annotations

import argparse
import csv
import random
from pathlib import Path

from sample_rfc_rank_failure import rank_selected_columns, systematic_generator_prime


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--seed-start", type=int, default=1)
    parser.add_argument("--samples", type=int, default=20)
    parser.add_argument("--shape", action="append", default=[], help="name=colon-separated columns")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    shapes: list[tuple[str, list[int]]] = []
    for raw in args.shape:
        name, columns_text = raw.split("=", 1)
        shapes.append((name, [int(value) for value in columns_text.split(":") if value]))

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["shape", "columns", "rank", "count"])
        ranks: dict[tuple[str, str, int], int] = {}
        for seed in range(args.seed_start, args.seed_start + args.samples):
            rng = random.Random(seed)
            generator = systematic_generator_prime(args.depth, args.total_expansion, args.prime, rng)
            for name, columns in shapes:
                rank = rank_selected_columns(generator, columns, args.prime)
                key = (name, ":".join(str(column) for column in columns), rank)
                ranks[key] = ranks.get(key, 0) + 1
        for (name, columns, rank), count in sorted(ranks.items()):
            writer.writerow([name, columns, rank, count])

    print(f"shapes={len(shapes)} samples={args.samples} out={args.out}")


if __name__ == "__main__":
    main()
