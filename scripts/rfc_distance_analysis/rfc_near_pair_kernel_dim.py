#!/usr/bin/env python3
"""Compute kernel dimensions for saved near-extremizer support pairs."""

from __future__ import annotations

import argparse
import csv
import random
from collections import Counter
from pathlib import Path

from sample_rfc_rank_failure import rfc_generator_prime


def parse_ints(text: str) -> tuple[int, ...]:
    return tuple(int(value) for value in text.split(":") if value != "")


def rank_rows_columns(matrix: list[list[int]], rows: tuple[int, ...], columns: tuple[int, ...], prime: int) -> int:
    work = [[matrix[row][col] % prime for col in columns] for row in rows]
    rank = 0
    for col in range(len(columns)):
        pivot = rank
        while pivot < len(work) and work[pivot][col] == 0:
            pivot += 1
        if pivot == len(work):
            continue
        work[rank], work[pivot] = work[pivot], work[rank]
        inv = pow(work[rank][col], prime - 2, prime)
        work[rank] = [(value * inv) % prime for value in work[rank]]
        for row_index, row in enumerate(work):
            if row_index == rank or row[col] == 0:
                continue
            factor = row[col]
            work[row_index] = [(value - factor * pivot_value) % prime for value, pivot_value in zip(row, work[rank])]
        rank += 1
        if rank == len(rows):
            break
    return rank


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
    by_dim: Counter[int] = Counter()
    by_extra_dim: Counter[tuple[int, int]] = Counter()
    first_by_dim: dict[int, str] = {}
    total = 0

    with Path(args.pairs_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            rows = parse_ints(row["rows"])
            outputs = parse_ints(row["outputs"])
            zero_columns = tuple(col for col in range(k) if col not in outputs)
            rank = rank_rows_columns(generator, rows, zero_columns, args.prime)
            kernel_dim = len(rows) - rank
            exact_output = k // len(rows)
            extra = len(outputs) - exact_output
            by_dim[kernel_dim] += 1
            by_extra_dim[(extra, kernel_dim)] += 1
            first_by_dim.setdefault(kernel_dim, row["rows"] + "|" + row["outputs"])
            total += 1

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["pairs_csv", "depth", "k", "pairs"])
        writer.writerow([args.pairs_csv, args.depth, k, total])
        writer.writerow([])
        writer.writerow(["kernel_dim", "pair_count", "first_pair"])
        for kernel_dim, count in sorted(by_dim.items()):
            writer.writerow([kernel_dim, count, first_by_dim[kernel_dim]])
        writer.writerow([])
        writer.writerow(["extra_outputs", "kernel_dim", "pair_count"])
        for (extra, kernel_dim), count in sorted(by_extra_dim.items()):
            writer.writerow([extra, kernel_dim, count])

    print(f"pairs={total} out={args.out}")


if __name__ == "__main__":
    main()
