#!/usr/bin/env python3
"""Sample marked incremental rank events for small RFC instances.

This is an experiment driver for docs/rfc_distance_analysis/rfc_conjectural_distance_program.md.
It samples small RFC generators, random disjoint marked sets (P,A), and reports:

    rank(P), rank(P union A)-rank(P), and the top-fold PP/PA/P0/A0/AA profile.

It supports the current linked fold and the independent-distinct variant:

    linked:               (u+t w, u+(t+1)w), t in F_q^*
    independent_distinct: (u+a w, u+b w), a,b in F_q^*, a != b

Keep depths small; this is a diagnostic sampler, not a benchmark.
"""

from __future__ import annotations

import argparse
import random
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_sufficiency_test import mat_rank_modq  # noqa: E402


def level_lengths(depth: int, c: int) -> list[int]:
    return [c * (1 << i) for i in range(depth)]


def sample_linked_challenges(depth: int, c: int, q: int, rng: random.Random) -> list[list[tuple[int, int]]]:
    levels: list[list[tuple[int, int]]] = []
    for level_len in level_lengths(depth, c):
        level = []
        for _ in range(level_len):
            t = rng.randrange(1, q)
            level.append((t, (t + 1) % q))
        levels.append(level)
    return levels


def sample_independent_distinct_challenges(
    depth: int,
    c: int,
    q: int,
    rng: random.Random,
) -> list[list[tuple[int, int]]]:
    levels: list[list[tuple[int, int]]] = []
    for level_len in level_lengths(depth, c):
        level = []
        for _ in range(level_len):
            a = rng.randrange(1, q)
            b = rng.randrange(1, q)
            while b == a:
                b = rng.randrange(1, q)
            level.append((a, b))
        levels.append(level)
    return levels


def encode_with_pair_slopes(
    message: tuple[int, ...],
    depth: int,
    c: int,
    q: int,
    levels: list[list[tuple[int, int]]],
) -> list[int]:
    k = 1 << depth
    n = c * k
    arr = [0] * n
    for i, value in enumerate(message):
        for j in range(c):
            arr[i * c + j] = value % q

    chunk_size = c
    for level_index in range(depth):
        level = levels[level_index]
        chunk_size <<= 1
        half = chunk_size >> 1
        for base in range(0, n, chunk_size):
            for jj in range(half):
                a, b = level[jj]
                u = arr[base + jj]
                w = arr[base + half + jj]
                arr[base + jj] = (u + a * w) % q
                arr[base + half + jj] = (u + b * w) % q
    return arr


def generator_rows(depth: int, c: int, q: int, fold_mode: str, rng: random.Random) -> list[list[int]]:
    if fold_mode == "linked":
        levels = sample_linked_challenges(depth, c, q, rng)
    elif fold_mode == "independent_distinct":
        levels = sample_independent_distinct_challenges(depth, c, q, rng)
    else:
        raise ValueError(fold_mode)

    k = 1 << depth
    rows = []
    for i in range(k):
        msg = [0] * k
        msg[i] = 1
        rows.append(encode_with_pair_slopes(tuple(msg), depth, c, q, levels))
    return rows


def restricted_rank(rows: list[list[int]], columns: list[int], q: int) -> int:
    if not columns:
        return 0
    return mat_rank_modq([[row[j] for j in columns] for row in rows], q)


def top_marked_profile(n: int, p_set: set[int], a_set: set[int]) -> dict[str, int]:
    half = n // 2
    profile = {"00": 0, "PP": 0, "AA": 0, "PA": 0, "P0": 0, "A0": 0}
    for j in range(half):
        left = j
        right = j + half
        marks = []
        for col in (left, right):
            if col in p_set:
                marks.append("P")
            elif col in a_set:
                marks.append("A")
            else:
                marks.append("0")
        key = "".join(sorted(marks))
        if key == "0P":
            key = "P0"
        elif key == "0A":
            key = "A0"
        elif key == "AP":
            key = "PA"
        profile[key] += 1
    return profile


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--depth", type=int, default=3)
    ap.add_argument("--expansion", type=int, default=4)
    ap.add_argument("--q", type=int, default=7)
    ap.add_argument("--fold-mode", choices=["linked", "independent_distinct"], default="linked")
    ap.add_argument("--generator-samples", type=int, default=20)
    ap.add_argument("--marked-samples", type=int, default=200)
    ap.add_argument("--p-size", type=int, default=4)
    ap.add_argument("--a-size", type=int, default=2)
    ap.add_argument("--seed", type=int, default=1)
    args = ap.parse_args()

    rng = random.Random(args.seed)
    k = 1 << args.depth
    n = args.expansion * k
    if args.p_size + args.a_size > n:
        raise SystemExit("p-size + a-size must be <= n")

    print(
        "depth,expansion,k,n,q,fold_mode,generator_index,marked_index,p_size,a_size,"
        "rank_p,rank_increment,rank_union,prof_00,prof_PP,prof_AA,prof_PA,prof_P0,prof_A0"
    )
    for gen_index in range(args.generator_samples):
        rows = generator_rows(args.depth, args.expansion, args.q, args.fold_mode, rng)
        for marked_index in range(args.marked_samples):
            chosen = rng.sample(range(n), args.p_size + args.a_size)
            p_cols = sorted(chosen[: args.p_size])
            a_cols = sorted(chosen[args.p_size :])
            p_set = set(p_cols)
            a_set = set(a_cols)
            rank_p = restricted_rank(rows, p_cols, args.q)
            rank_union = restricted_rank(rows, sorted(p_cols + a_cols), args.q)
            inc = rank_union - rank_p
            prof = top_marked_profile(n, p_set, a_set)
            print(
                f"{args.depth},{args.expansion},{k},{n},{args.q},{args.fold_mode},"
                f"{gen_index},{marked_index},{args.p_size},{args.a_size},"
                f"{rank_p},{inc},{rank_union},{prof['00']},{prof['PP']},{prof['AA']},"
                f"{prof['PA']},{prof['P0']},{prof['A0']}"
            )


if __name__ == "__main__":
    main()
