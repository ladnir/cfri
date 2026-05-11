#!/usr/bin/env python3
"""Check whether saved support pairs contain a matched stride core."""

from __future__ import annotations

import argparse
import csv
from collections import Counter
from pathlib import Path

from rfc_near_pair_kernel_dim import parse_ints


def containing_residues(outputs: tuple[int, ...], k: int, m: int) -> tuple[int, ...]:
    output_set = set(outputs)
    residues: list[int] = []
    for residue in range(m):
        core = range(residue, k, m)
        if all(value in output_set for value in core):
            residues.append(residue)
    return tuple(residues)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("pairs_csv")
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    total = 0
    by_contained_count: Counter[int] = Counter()
    by_extra_contained: Counter[tuple[int, int]] = Counter()
    first_missing = ""

    with Path(args.pairs_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            rows = parse_ints(row["rows"])
            outputs = parse_ints(row["outputs"])
            m = len(rows)
            exact_output = k // m
            extra = len(outputs) - exact_output
            residues = containing_residues(outputs, k, m)
            contained_count = len(residues)
            by_contained_count[contained_count] += 1
            by_extra_contained[(extra, contained_count)] += 1
            if contained_count == 0 and not first_missing:
                first_missing = row["rows"] + "|" + row["outputs"]
            total += 1

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["pairs_csv", "depth", "k", "pairs", "first_missing_core"])
        writer.writerow([args.pairs_csv, args.depth, k, total, first_missing])
        writer.writerow([])
        writer.writerow(["contained_core_count", "pair_count"])
        for contained_count, count in sorted(by_contained_count.items()):
            writer.writerow([contained_count, count])
        writer.writerow([])
        writer.writerow(["extra_outputs", "contained_core_count", "pair_count"])
        for (extra, contained_count), count in sorted(by_extra_contained.items()):
            writer.writerow([extra, contained_count, count])

    print(f"pairs={total} out={args.out}")


if __name__ == "__main__":
    main()
