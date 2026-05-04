#!/usr/bin/env python3
"""Classify bad zero-set shapes as persistent or accidental by resampling generators."""

from __future__ import annotations

import argparse
import csv
import random
from collections import Counter
from pathlib import Path

from sample_rfc_rank_failure import rank_selected_columns, systematic_generator_prime


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("bad_shapes_csv")
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--seed-start", type=int, default=1)
    parser.add_argument("--samples", type=int, default=20)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    shapes: list[tuple[int, list[int]]] = []
    with Path(args.bad_shapes_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            shapes.append((int(row.get("index", len(shapes))), [int(value) for value in row["columns"].split(":")]))

    deficient_counts = [0] * len(shapes)
    for seed in range(args.seed_start, args.seed_start + args.samples):
        rng = random.Random(seed)
        generator = systematic_generator_prime(args.depth, args.total_expansion, args.prime, rng)
        for idx, (_shape_id, columns) in enumerate(shapes):
            if rank_selected_columns(generator, columns, args.prime) < (1 << args.depth):
                deficient_counts[idx] += 1

    histogram = Counter(deficient_counts)
    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["deficient_samples", "shape_count"])
        for deficient_samples, count in sorted(histogram.items()):
            writer.writerow([deficient_samples, count])

    persistent = histogram.get(args.samples, 0)
    accidental = len(shapes) - persistent
    print(
        f"shapes={len(shapes)} samples={args.samples} persistent={persistent} "
        f"accidental_or_unstable={accidental} out={args.out}"
    )


if __name__ == "__main__":
    main()
