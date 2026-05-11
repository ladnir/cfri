#!/usr/bin/env python3
"""Inspect parity-column relations after quotienting selected systematic rows."""

from __future__ import annotations

import argparse
import csv
import random
from pathlib import Path

from sample_rfc_rank_failure import systematic_generator_prime


def rank_rows(rows: list[list[int]], prime: int) -> int:
    work = [row[:] for row in rows]
    rank = 0
    if not work:
        return 0
    cols = len(work[0])
    for col in range(cols):
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
        if rank == len(work):
            break
    return rank


def normalize_projective(vector: list[int], prime: int) -> tuple[int, ...]:
    for value in vector:
        if value != 0:
            inv = pow(value, prime - 2, prime)
            return tuple((entry * inv) % prime for entry in vector)
    return tuple(vector)


def relation_summary(columns: list[int], depth: int, total_expansion: int, prime: int, seed: int) -> tuple[int, int, int, int, int]:
    k = 1 << depth
    rng = random.Random(seed)
    generator = systematic_generator_prime(depth, total_expansion, prime, rng)
    identities = [column for column in columns if column < k]
    parity_columns = [column for column in columns if column >= k]
    remaining_rows = [row for row in range(k) if row not in identities]
    restricted = [
        [generator[row][column] % prime for row in remaining_rows]
        for column in parity_columns
    ]
    projective_classes = {normalize_projective(vector, prime) for vector in restricted}
    zero_columns = sum(1 for vector in restricted if all(value == 0 for value in vector))
    restricted_rank = rank_rows([[vector[row] for vector in restricted] for row in range(len(remaining_rows))], prime)
    return len(remaining_rows), len(parity_columns), restricted_rank, len(projective_classes), zero_columns


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--shape", action="append", default=[], help="name=colon-separated full columns")
    parser.add_argument("--bad-shapes-csv", default=None)
    parser.add_argument("--out", required=True)
    parser.add_argument("--max-rows", type=int, default=1000)
    args = parser.parse_args()

    shapes: list[tuple[str, list[int]]] = []
    for raw in args.shape:
        name, columns_text = raw.split("=", 1)
        shapes.append((name, [int(value) for value in columns_text.split(":") if value]))
    if args.bad_shapes_csv is not None:
        with Path(args.bad_shapes_csv).open(newline="") as handle:
            for index, row in enumerate(csv.DictReader(handle)):
                if index >= args.max_rows:
                    break
                shapes.append((row.get("index", str(index)), [int(value) for value in row["columns"].split(":")]))

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "shape",
                "columns",
                "remaining_rows",
                "parity_columns",
                "restricted_rank",
                "projective_classes",
                "zero_columns",
            ]
        )
        for name, columns in shapes:
            remaining_rows, parity_count, restricted_rank, projective_count, zero_count = relation_summary(
                columns,
                args.depth,
                args.total_expansion,
                args.prime,
                args.seed,
            )
            writer.writerow(
                [
                    name,
                    ":".join(str(column) for column in columns),
                    remaining_rows,
                    parity_count,
                    restricted_rank,
                    projective_count,
                    zero_count,
                ]
            )

    print(f"shapes={len(shapes)} out={args.out}")


if __name__ == "__main__":
    main()
