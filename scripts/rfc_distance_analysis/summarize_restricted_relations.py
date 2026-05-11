#!/usr/bin/env python3
"""Summarize restricted parity ranks/projective classes for a list of shapes."""

from __future__ import annotations

import argparse
import csv
from collections import Counter
from pathlib import Path

from restricted_parity_relations import relation_summary


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("shapes_csv")
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    counts: Counter[tuple[int, int, int, int, int]] = Counter()
    first: dict[tuple[int, int, int, int, int], str] = {}
    with Path(args.shapes_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            columns = [int(value) for value in row["columns"].split(":")]
            summary = relation_summary(columns, args.depth, args.total_expansion, args.prime, args.seed)
            counts[summary] += 1
            first.setdefault(summary, row["columns"])

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "remaining_rows",
                "parity_columns",
                "restricted_rank",
                "projective_classes",
                "zero_columns",
                "count",
                "first_shape",
            ]
        )
        for summary, count in counts.most_common():
            writer.writerow([*summary, count, first[summary]])
    print(f"shapes={sum(counts.values())} classes={len(counts)} out={args.out}")


if __name__ == "__main__":
    main()
