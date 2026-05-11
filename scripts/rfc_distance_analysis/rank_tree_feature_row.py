#!/usr/bin/env python3
"""Summarize exact rank-shape rows by recursive tree-collision features."""

from __future__ import annotations

import argparse
import csv
import itertools
from collections import Counter
from pathlib import Path
import random

from sample_rfc_rank_failure import rank_selected_columns, systematic_generator_prime


def full_identity_subtrees(identities: set[int], k: int, level: int) -> int:
    size = 1 << level
    return sum(
        1
        for start in range(0, k, size)
        if all(column in identities for column in range(start, start + size))
    )


def parity_projection_collisions(parity_local: list[int], expansion: int, level: int) -> int:
    modulus = expansion * (1 << level)
    counts: dict[int, int] = {}
    for column in parity_local:
        key = column % modulus
        counts[key] = counts.get(key, 0) + 1
    return sum(count - 1 for count in counts.values() if count > 1)


def feature(columns: tuple[int, ...], k: int, parity_n: int, expansion: int) -> tuple[int, ...]:
    depth = (k.bit_length() - 1)
    identities = {column for column in columns if column < k}
    parity_local = [column - k for column in columns if column >= k]
    identity_features = [full_identity_subtrees(identities, k, level) for level in range(1, depth + 1)]
    parity_features = [parity_projection_collisions(parity_local, expansion, level) for level in range(1, depth + 1)]
    return tuple(identity_features + parity_features)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--s-identity", type=int, required=True)
    parser.add_argument("--z-parity", type=int, required=True)
    parser.add_argument("--shape-samples", type=int, default=0)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    rng = random.Random(args.seed)
    k = 1 << args.depth
    n = args.total_expansion * k
    parity_expansion = args.total_expansion - 1
    parity_n = n - k
    generator = systematic_generator_prime(args.depth, args.total_expansion, args.prime, rng)

    total_by_feature: Counter[tuple[int, ...]] = Counter()
    bad_by_feature: Counter[tuple[int, ...]] = Counter()
    first_bad: dict[tuple[int, ...], str] = {}

    if args.shape_samples > 0:
        shape_iter = (
            (
                tuple(sorted(rng.sample(range(k), args.s_identity))),
                tuple(sorted(rng.sample(range(parity_n), args.z_parity))),
            )
            for _ in range(args.shape_samples)
        )
    else:
        shape_iter = itertools.product(
            itertools.combinations(range(k), args.s_identity),
            itertools.combinations(range(parity_n), args.z_parity),
        )

    for identity_columns, parity_columns in shape_iter:
        columns = tuple(identity_columns) + tuple(k + column for column in parity_columns)
        key = feature(columns, k, parity_n, parity_expansion)
        total_by_feature[key] += 1
        if rank_selected_columns(generator, list(columns), args.prime) < k:
            bad_by_feature[key] += 1
            first_bad.setdefault(key, ":".join(str(column) for column in columns))

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        identity_headers = [f"identity_full_level_{level}" for level in range(1, args.depth + 1)]
        parity_headers = [f"parity_collision_level_{level}" for level in range(1, args.depth + 1)]
        writer.writerow(
            ["s_identity", "z_parity"]
            + identity_headers
            + parity_headers
            + ["total_count", "bad_count", "bad_fraction", "first_bad"]
        )
        for key, total in total_by_feature.most_common():
            bad = bad_by_feature[key]
            writer.writerow(
                [
                    args.s_identity,
                    args.z_parity,
                    *key,
                    total,
                    bad,
                    bad / total if total else 0.0,
                    first_bad.get(key, ""),
                ]
            )

    print(
        f"depth={args.depth} s_identity={args.s_identity} z_parity={args.z_parity} "
        f"features={len(total_by_feature)} bad={sum(bad_by_feature.values())} out={args.out}"
    )


if __name__ == "__main__":
    main()
