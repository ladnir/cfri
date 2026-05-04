#!/usr/bin/env python3
"""Enumerate one systematic rank-shape row and summarize good/bad counts by tree signature."""

from __future__ import annotations

import argparse
import csv
import itertools
from collections import Counter
from pathlib import Path
import random

from sample_rfc_rank_failure import rank_selected_columns, systematic_generator_prime
from summarize_rank_bad_shapes import signature


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--s-identity", type=int, required=True)
    parser.add_argument("--z-parity", type=int, required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    rng = random.Random(args.seed)
    k = 1 << args.depth
    n = args.total_expansion * k
    parity_n = n - k
    generator = systematic_generator_prime(args.depth, args.total_expansion, args.prime, rng)

    total_by_signature: Counter[tuple[int, int, int, int, int, int]] = Counter()
    bad_by_signature: Counter[tuple[int, int, int, int, int, int]] = Counter()
    first_bad: dict[tuple[int, int, int, int, int, int], str] = {}

    for identity_columns in itertools.combinations(range(k), args.s_identity):
        for parity_columns in itertools.combinations(range(parity_n), args.z_parity):
            columns = tuple(identity_columns) + tuple(k + column for column in parity_columns)
            sig = signature(list(columns), k, parity_n)
            total_by_signature[sig] += 1
            if rank_selected_columns(generator, list(columns), args.prime) < k:
                bad_by_signature[sig] += 1
                first_bad.setdefault(sig, ":".join(str(column) for column in columns))

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
                "total_count",
                "bad_count",
                "bad_fraction",
                "first_bad",
            ]
        )
        for sig, total in total_by_signature.most_common():
            bad = bad_by_signature[sig]
            writer.writerow([*sig, total, bad, bad / total if total else 0.0, first_bad.get(sig, "")])

    print(
        f"depth={args.depth} s_identity={args.s_identity} z_parity={args.z_parity} "
        f"signatures={len(total_by_signature)} bad={sum(bad_by_signature.values())} out={args.out}"
    )


if __name__ == "__main__":
    main()
