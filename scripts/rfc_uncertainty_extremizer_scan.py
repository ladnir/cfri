#!/usr/bin/env python3
"""Scan small one-copy RFC extremizer support pairs."""

from __future__ import annotations

import argparse
import csv
import itertools
import random
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


def is_aligned_subcube(values: tuple[int, ...], depth: int) -> bool:
    size = len(values)
    if size == 0 or size & (size - 1) != 0:
        return False
    live_bits = size.bit_length() - 1
    value_set = set(values)
    for live_positions in itertools.combinations(range(depth), live_bits):
        live_mask = sum(1 << bit for bit in live_positions)
        fixed_positions = [bit for bit in range(depth) if bit not in live_positions]
        for fixed_values in range(1 << len(fixed_positions)):
            base = 0
            for index, bit in enumerate(fixed_positions):
                if (fixed_values >> index) & 1:
                    base |= 1 << bit
            subcube = {
                base | sum(((direction >> index) & 1) << bit for index, bit in enumerate(live_positions))
                for direction in range(size)
            }
            if subcube == value_set:
                return True
    return False


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--live-rows", type=int, required=True)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--max-pairs", type=int, default=0)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    if k % args.live_rows != 0:
        raise SystemExit("--live-rows must divide k")
    output_support = k // args.live_rows
    zero_count = k - output_support
    rng = random.Random(args.seed)
    generator = rfc_generator_prime(args.depth, 1, args.prime, rng)

    checked = 0
    extremal = 0
    extremal_both_subcube = 0
    first_non_subcube = ""
    by_kind: dict[tuple[int, int], int] = {}

    row_supports = list(itertools.combinations(range(k), args.live_rows))
    output_supports = list(itertools.combinations(range(k), output_support))
    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "live_rows",
                "output_support",
                "zero_count",
                "checked_pairs",
                "extremal_pairs",
                "extremal_both_subcube",
                "first_non_subcube_extremal",
            ]
        )
        for rows in row_supports:
            rows_subcube = is_aligned_subcube(rows, args.depth)
            for outputs in output_supports:
                zero_columns = tuple(col for col in range(k) if col not in outputs)
                rank = rank_rows_columns(generator, rows, zero_columns, args.prime)
                checked += 1
                if rank < args.live_rows:
                    outputs_subcube = is_aligned_subcube(outputs, args.depth)
                    extremal += 1
                    if rows_subcube and outputs_subcube:
                        extremal_both_subcube += 1
                    elif first_non_subcube == "":
                        first_non_subcube = (
                            ":".join(str(value) for value in rows)
                            + "|"
                            + ":".join(str(value) for value in outputs)
                        )
                    key = (int(rows_subcube), int(outputs_subcube))
                    by_kind[key] = by_kind.get(key, 0) + 1
                if args.max_pairs > 0 and checked >= args.max_pairs:
                    break
            if args.max_pairs > 0 and checked >= args.max_pairs:
                break

        writer.writerow(
            [
                args.depth,
                k,
                args.live_rows,
                output_support,
                zero_count,
                checked,
                extremal,
                extremal_both_subcube,
                first_non_subcube,
            ]
        )
        writer.writerow([])
        writer.writerow(["rows_subcube", "outputs_subcube", "extremal_pairs"])
        for (rows_subcube, outputs_subcube), count in sorted(by_kind.items()):
            writer.writerow([rows_subcube, outputs_subcube, count])

    print(f"checked={checked} extremal={extremal} out={args.out}")


if __name__ == "__main__":
    main()
