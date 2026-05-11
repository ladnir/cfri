#!/usr/bin/env python3
"""Check whether one-copy extremizer vectors are dense in an independent copy."""

from __future__ import annotations

import argparse
import csv
import random
from pathlib import Path

from sample_rfc_rank_failure import rfc_generator_prime


def null_vector_left(matrix: list[list[int]], prime: int) -> list[int]:
    """Return nonzero x with x^T matrix = 0 for an m x n rank-(m-1) matrix."""

    if not matrix:
        return []
    rows = len(matrix)
    cols = len(matrix[0])
    work = [[matrix[r][c] % prime for r in range(rows)] for c in range(cols)]
    rank = 0
    pivots: list[int] = []
    for col in range(rows):
        pivot = rank
        while pivot < cols and work[pivot][col] == 0:
            pivot += 1
        if pivot == cols:
            continue
        work[rank], work[pivot] = work[pivot], work[rank]
        inv = pow(work[rank][col], prime - 2, prime)
        work[rank] = [(value * inv) % prime for value in work[rank]]
        for row_index, row in enumerate(work):
            if row_index == rank or row[col] == 0:
                continue
            factor = row[col]
            work[row_index] = [(value - factor * pivot_value) % prime for value, pivot_value in zip(row, work[rank])]
        pivots.append(col)
        rank += 1
        if rank == rows:
            break

    if rank >= rows:
        raise ValueError("matrix has no left kernel")

    free = next(col for col in range(rows) if col not in pivots)
    vector = [0] * rows
    vector[free] = 1
    for pivot_row in range(rank - 1, -1, -1):
        pivot_col = pivots[pivot_row]
        value = 0
        for col in range(pivot_col + 1, rows):
            value = (value + work[pivot_row][col] * vector[col]) % prime
        vector[pivot_col] = (-value) % prime
    return vector


def encode_support(rows: list[int], values: list[int], generator: list[list[int]], prime: int) -> int:
    k = len(generator[0])
    out = [0] * k
    for row, value in zip(rows, values):
        gen_row = generator[row]
        for col in range(k):
            out[col] = (out[col] + value * gen_row[col]) % prime
    return sum(1 for value in out if value != 0)


def parse_ints(text: str) -> list[int]:
    return [int(value) for value in text.split(":") if value != ""]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--pairs", required=True)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--first-seed", type=int, default=1)
    parser.add_argument("--second-seed", type=int, default=2)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    rng_first = random.Random(args.first_seed)
    rng_second = random.Random(args.second_seed)
    first = rfc_generator_prime(args.depth, 1, args.prime, rng_first)
    second = rfc_generator_prime(args.depth, 1, args.prime, rng_second)
    k = 1 << args.depth

    with Path(args.pairs).open(newline="") as input_handle, Path(args.out).open("w", newline="") as output_handle:
        reader = csv.DictReader(input_handle)
        writer = csv.writer(output_handle)
        writer.writerow(
            [
                "rows",
                "outputs",
                "kernel_support",
                "first_output_weight",
                "second_output_weight",
                "second_full",
            ]
        )
        for row in reader:
            rows = parse_ints(row["rows"])
            outputs = set(parse_ints(row["outputs"]))
            zero_columns = [col for col in range(k) if col not in outputs]
            restricted = [[first[r][col] for col in zero_columns] for r in rows]
            kernel = null_vector_left(restricted, args.prime)
            first_weight = encode_support(rows, kernel, first, args.prime)
            second_weight = encode_support(rows, kernel, second, args.prime)
            writer.writerow(
                [
                    row["rows"],
                    row["outputs"],
                    sum(1 for value in kernel if value != 0),
                    first_weight,
                    second_weight,
                    int(second_weight == k),
                ]
            )

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
