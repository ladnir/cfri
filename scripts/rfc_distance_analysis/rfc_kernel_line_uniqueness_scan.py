#!/usr/bin/env python3
"""Scan kernel dimensions for one-copy support/output pairs."""

from __future__ import annotations

import argparse
import csv
import itertools
import random
from collections import Counter
from pathlib import Path

from sample_rfc_rank_failure import rfc_generator_prime


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


def is_matched(rows: tuple[int, ...], outputs: tuple[int, ...], k: int) -> bool:
    m = len(rows)
    if m == 0 or k % m != 0:
        return False
    if len(outputs) != k // m:
        return False
    first = rows[0]
    if first % m != 0 or rows != tuple(range(first, first + m)):
        return False
    residue = outputs[0] % m
    return outputs == tuple(range(residue, k, m))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--live-rows", type=int, required=True)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    m = args.live_rows
    if k % m != 0:
        raise SystemExit("--live-rows must divide k")
    output_support = k // m
    generator = rfc_generator_prime(args.depth, 1, args.prime, random.Random(args.seed))

    by_kind_dim: Counter[tuple[int, int]] = Counter()
    checked = 0
    first_bad_matched = ""
    first_bad_nonmatched = ""
    for rows in itertools.combinations(range(k), m):
        for outputs in itertools.combinations(range(k), output_support):
            zero_columns = tuple(col for col in range(k) if col not in outputs)
            rank = rank_rows_columns(generator, rows, zero_columns, args.prime)
            kernel_dim = m - rank
            matched = int(is_matched(rows, outputs, k))
            by_kind_dim[(matched, kernel_dim)] += 1
            checked += 1
            if matched and kernel_dim != 1 and first_bad_matched == "":
                first_bad_matched = ":".join(str(value) for value in rows) + "|" + ":".join(str(value) for value in outputs)
            if not matched and kernel_dim != 0 and first_bad_nonmatched == "":
                first_bad_nonmatched = ":".join(str(value) for value in rows) + "|" + ":".join(str(value) for value in outputs)

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "live_rows",
                "output_support",
                "checked_pairs",
                "first_bad_matched",
                "first_bad_nonmatched",
            ]
        )
        writer.writerow([args.depth, k, m, output_support, checked, first_bad_matched, first_bad_nonmatched])
        writer.writerow([])
        writer.writerow(["matched", "kernel_dim", "pair_count"])
        for (matched, kernel_dim), count in sorted(by_kind_dim.items()):
            writer.writerow([matched, kernel_dim, count])

    print(f"checked={checked} out={args.out}")


if __name__ == "__main__":
    main()
