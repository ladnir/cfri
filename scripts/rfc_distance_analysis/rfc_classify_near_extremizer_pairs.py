#!/usr/bin/env python3
"""Classify near-extremizer pair CSVs by matched block/stride cores."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def parse_ints(text: str) -> tuple[int, ...]:
    return tuple(int(value) for value in text.split(":") if value != "")


def row_block_index(rows: tuple[int, ...]) -> int | None:
    if not rows:
        return None
    m = len(rows)
    first = rows[0]
    if first % m != 0:
        return None
    if rows != tuple(range(first, first + m)):
        return None
    return first // m


def matched_stride_residues(outputs: tuple[int, ...], m: int, k: int) -> list[int]:
    output_set = set(outputs)
    residues = []
    for residue in range(m):
        stride = set(range(residue, k, m))
        if stride <= output_set:
            residues.append(residue)
    return residues


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("pairs_csv")
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    rows_total = 0
    row_block_count = 0
    contains_core_count = 0
    multi_core_count = 0
    first_missing = ""
    by_extra: dict[int, int] = {}

    with Path(args.pairs_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            rows_total += 1
            rows = parse_ints(row["rows"])
            outputs = parse_ints(row["outputs"])
            m = len(rows)
            block = row_block_index(rows)
            if block is not None:
                row_block_count += 1
            residues = matched_stride_residues(outputs, m, k)
            if residues:
                contains_core_count += 1
            elif first_missing == "":
                first_missing = row["rows"] + "|" + row["outputs"]
            if len(residues) > 1:
                multi_core_count += 1
            extra = len(outputs) - k // m
            by_extra[extra] = by_extra.get(extra, 0) + 1

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "pairs_csv",
                "depth",
                "k",
                "pairs",
                "row_block_count",
                "contains_matched_stride_core_count",
                "multi_core_count",
                "first_missing_core",
            ]
        )
        writer.writerow(
            [
                args.pairs_csv,
                args.depth,
                k,
                rows_total,
                row_block_count,
                contains_core_count,
                multi_core_count,
                first_missing,
            ]
        )
        writer.writerow([])
        writer.writerow(["extra_outputs", "pair_count"])
        for extra, count in sorted(by_extra.items()):
            writer.writerow([extra, count])

    print(f"pairs={rows_total} contains_core={contains_core_count} out={args.out}")


if __name__ == "__main__":
    main()
