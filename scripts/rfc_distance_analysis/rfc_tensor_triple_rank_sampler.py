#!/usr/bin/env python3
"""Sample rank-deficient triples of RFC tensor columns.

This targets the lower event in the one-spill common-zero ledger.  It samples
only the requested columns using the RFC tensor formula, avoiding construction
of the full generator matrix.
"""

from __future__ import annotations

import argparse
import random


def column(depth: int, expansion: int, q: int, levels: list[list[int]], col: int) -> list[int]:
    copy = col % expansion
    path = col // expansion
    idx = copy
    values = [1]
    for level in range(depth):
        bit = (path >> level) & 1
        t = levels[level][idx]
        if bit == 0:
            top = [((1 - t) * value) % q for value in values]
            bottom = [(t * value) % q for value in values]
        else:
            top = [((-t) * value) % q for value in values]
            bottom = [((t + 1) * value) % q for value in values]
        values = top + bottom
        idx += bit * (expansion << level)
    return values


def rank_columns(columns: list[list[int]], q: int) -> int:
    rows = [[columns[col][row] % q for col in range(len(columns))] for row in range(len(columns[0]))]
    rank = 0
    for col in range(len(columns)):
        pivot = rank
        while pivot < len(rows) and rows[pivot][col] == 0:
            pivot += 1
        if pivot == len(rows):
            continue
        rows[rank], rows[pivot] = rows[pivot], rows[rank]
        inv = pow(rows[rank][col], q - 2, q)
        rows[rank] = [(value * inv) % q for value in rows[rank]]
        for row_idx, row in enumerate(rows):
            if row_idx == rank or row[col] == 0:
                continue
            factor = row[col]
            rows[row_idx] = [
                (value - factor * pivot_value) % q
                for value, pivot_value in zip(row, rows[rank])
            ]
        rank += 1
        if rank == len(columns):
            break
    return rank


def parse_columns(text: str) -> list[int]:
    out = [int(part) for part in text.split(",") if part]
    if len(out) < 1:
        raise argparse.ArgumentTypeError("provide at least one column")
    return out


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--depth", type=int, default=4)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q", type=int, action="append", required=True)
    parser.add_argument("--columns", type=parse_columns, action="append", required=True)
    parser.add_argument("--samples", type=int, default=50000)
    parser.add_argument("--seed", type=int, default=12345)
    args = parser.parse_args()

    print("q,depth,expansion,columns,samples,bad,rate,min_rank")
    for q in args.q:
        for columns in args.columns:
            rng = random.Random(args.seed)
            bad = 0
            min_rank = len(columns)
            for _ in range(args.samples):
                levels = [
                    [rng.randrange(1, q) for _ in range(args.expansion * (1 << level))]
                    for level in range(args.depth)
                ]
                sampled_columns = [
                    column(args.depth, args.expansion, q, levels, col)
                    for col in columns
                ]
                rank = rank_columns(sampled_columns, q)
                min_rank = min(min_rank, rank)
                if rank < len(columns):
                    bad += 1
            print(
                f"{q},{args.depth},{args.expansion},"
                f"{':'.join(str(col) for col in columns)},"
                f"{args.samples},{bad},{bad / args.samples:.12g},{min_rank}"
            )


if __name__ == "__main__":
    main()
