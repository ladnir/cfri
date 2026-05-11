#!/usr/bin/env python3
"""Classify matched-core near pairs by complete stride classes and kernel dimension."""

from __future__ import annotations

import argparse
import csv
import random
from collections import Counter
from pathlib import Path

from rfc_near_pair_kernel_dim import parse_ints, rank_rows_columns
from sample_rfc_rank_failure import rfc_generator_prime


def complete_extra_stride_count(outputs: tuple[int, ...], k: int, m: int) -> int:
    """Return complete stride classes beyond the mandatory matched core."""
    class_size = k // m
    counts = [0] * m
    for value in outputs:
        counts[value % m] += 1
    full_classes = sum(1 for count in counts if count == class_size)
    return max(0, full_classes - 1)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("pairs_csv")
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    generator = rfc_generator_prime(args.depth, 1, args.prime, random.Random(args.seed))
    by_stride_dim: Counter[tuple[int, int]] = Counter()
    by_bound_gap: Counter[int] = Counter()
    examples: dict[tuple[int, int], str] = {}
    total = 0

    with Path(args.pairs_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            rows = parse_ints(row["rows"])
            outputs = parse_ints(row["outputs"])
            m = len(rows)
            zero_columns = tuple(col for col in range(k) if col not in outputs)
            rank = rank_rows_columns(generator, rows, zero_columns, args.prime)
            kernel_dim = m - rank
            complete_extras = complete_extra_stride_count(outputs, k, m)
            key = (complete_extras, kernel_dim)
            by_stride_dim[key] += 1
            by_bound_gap[complete_extras - (kernel_dim - 1)] += 1
            examples.setdefault(key, row["rows"] + "|" + row["outputs"])
            total += 1

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["pairs_csv", "depth", "k", "pairs"])
        writer.writerow([args.pairs_csv, args.depth, k, total])
        writer.writerow([])
        writer.writerow(["complete_extra_strides", "kernel_dim", "pair_count", "first_pair"])
        for key, count in sorted(by_stride_dim.items()):
            writer.writerow([key[0], key[1], count, examples[key]])
        writer.writerow([])
        writer.writerow(["complete_extra_strides_minus_dimension_excess", "pair_count"])
        for gap, count in sorted(by_bound_gap.items()):
            writer.writerow([gap, count])

    print(f"pairs={total} out={args.out}")


if __name__ == "__main__":
    main()
