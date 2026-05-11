#!/usr/bin/env python3
"""Summarize tree signatures of rank-deficient systematic zero-set shapes."""

from __future__ import annotations

import argparse
import csv
from collections import Counter
from pathlib import Path


def signature(columns: list[int], k: int, parity_n: int) -> tuple[int, int, int, int, int, int]:
    identities = [column for column in columns if column < k]
    parity = [column - k for column in columns if column >= k]
    identity_pairs = sum(1 for start in range(0, k, 2) if start in identities and start + 1 in identities)
    identity_quads = sum(
        1 for start in range(0, k, 4) if all(column in identities for column in range(start, start + 4))
    )
    parity_half = parity_n // 2
    parity_quarter = parity_n // 4
    parity_half_pairs = sum(1 for column in parity if parity_half and column + parity_half in parity)
    parity_quarter_pairs = sum(1 for column in parity if parity_quarter and column + parity_quarter in parity)
    parity_mod_half_collisions = len(parity) - len(set(column % parity_half for column in parity)) if parity_half else 0
    parity_mod_quarter_collisions = (
        len(parity) - len(set(column % parity_quarter for column in parity)) if parity_quarter else 0
    )
    return (
        identity_pairs,
        identity_quads,
        parity_half_pairs,
        parity_quarter_pairs,
        parity_mod_half_collisions,
        parity_mod_quarter_collisions,
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("bad_shapes_csv")
    parser.add_argument("--k", type=int, required=True)
    parser.add_argument("--parity-n", type=int, required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    counts: Counter[tuple[int, int, int, int, int, int]] = Counter()
    first: dict[tuple[int, int, int, int, int, int], str] = {}
    with Path(args.bad_shapes_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            columns = [int(value) for value in row["columns"].split(":")]
            sig = signature(columns, args.k, args.parity_n)
            counts[sig] += 1
            first.setdefault(sig, row["columns"])

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "identity_pairs",
                "identity_quads",
                "parity_half_pairs",
                "parity_quarter_pairs",
                "parity_mod_half_collisions",
                "parity_mod_quarter_collisions",
                "count",
                "first_bad",
            ]
        )
        for sig, count in counts.most_common():
            writer.writerow([*sig, count, first[sig]])

    print(f"bad_shapes={sum(counts.values())} signatures={len(counts)} out={args.out}")


if __name__ == "__main__":
    main()
