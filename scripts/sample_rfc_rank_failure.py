#!/usr/bin/env python3
"""Sample row-subset rank failures for tiny RFC generators.

A linear code has distance greater than D iff every set of N-D coordinate rows spans the message
space. This script samples coordinate subsets and estimates the first moment of rank-deficient zero
sets:

    E[# Z, |Z|=z, rank(G_Z) < k].

It is a design/calibration tool for a possible rank-first-moment certificate.
"""

from __future__ import annotations

import argparse
import itertools
import math
import random


def mod_prime(value: int, prime: int) -> int:
    return value % prime


def rfc_generator_prime(depth: int, expansion: int, prime: int, rng: random.Random) -> list[list[int]]:
    k = 1
    n = expansion
    generator = [[1 for _ in range(n)]]

    for _round in range(depth):
        next_n = 2 * n
        next_generator = [[0 for _ in range(next_n)] for _ in range(2 * k)]
        diagonal = [rng.randrange(1, prime) for _ in range(n)]
        for row in range(k):
            for j, entry in enumerate(generator[row]):
                t = diagonal[j]
                next_generator[row][j] = mod_prime((1 - t) * entry, prime)
                next_generator[row][n + j] = mod_prime((1 - (t + 1)) * entry, prime)
                next_generator[row + k][j] = mod_prime(t * entry, prime)
                next_generator[row + k][n + j] = mod_prime((t + 1) * entry, prime)
        generator = next_generator
        k *= 2
        n = next_n

    return generator


def systematic_generator_prime(depth: int, total_expansion: int, prime: int, rng: random.Random) -> list[list[int]]:
    parity = rfc_generator_prime(depth, total_expansion - 1, prime, rng)
    k = len(parity)
    out: list[list[int]] = []
    for i, row in enumerate(parity):
        identity = [0] * k
        identity[i] = 1
        out.append(identity + row)
    return out


def rank_selected_columns(generator: list[list[int]], columns: list[int], prime: int) -> int:
    rows = [[row[col] % prime for col in columns] for row in generator]
    rank = 0
    col_count = len(columns)
    for col in range(col_count):
        pivot = rank
        while pivot < len(rows) and rows[pivot][col] == 0:
            pivot += 1
        if pivot == len(rows):
            continue
        rows[rank], rows[pivot] = rows[pivot], rows[rank]
        inv = pow(rows[rank][col], prime - 2, prime)
        rows[rank] = [(value * inv) % prime for value in rows[rank]]
        for r, row in enumerate(rows):
            if r == rank or row[col] == 0:
                continue
            factor = row[col]
            rows[r] = [(value - factor * pivot_value) % prime for value, pivot_value in zip(row, rows[rank])]
        rank += 1
        if rank == len(rows):
            break
    return rank


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return -math.inf
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--depth", type=int, default=3)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--zero-count", type=int, required=True)
    parser.add_argument("--code-samples", type=int, default=20)
    parser.add_argument("--subset-samples", type=int, default=1000)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--systematic", action="store_true")
    parser.add_argument("--exact-k-subsets", action="store_true")
    args = parser.parse_args()

    rng = random.Random(args.seed)
    k = 1 << args.depth
    n = args.total_expansion * k
    if args.zero_count < 1 or args.zero_count > n:
        raise SystemExit("--zero-count must be in 1..N")

    if args.exact_k_subsets:
        generator = (
            systematic_generator_prime(args.depth, args.total_expansion, args.prime, rng)
            if args.systematic
            else rfc_generator_prime(args.depth, args.total_expansion, args.prime, rng)
        )
        failures = 0
        first_bad: tuple[int, ...] | None = None
        checked = 0
        for columns_tuple in itertools.combinations(range(n), k):
            checked += 1
            if rank_selected_columns(generator, list(columns_tuple), args.prime) < k:
                failures += 1
                if first_bad is None:
                    first_bad = columns_tuple
        ensemble = "systematic" if args.systematic else "original"
        print("ensemble,prime,depth,k,n,checked_k_subsets,rank_deficient_k_subsets,first_bad")
        first_bad_text = "" if first_bad is None else ":".join(str(column) for column in first_bad)
        print(f"{ensemble},{args.prime},{args.depth},{k},{n},{checked},{failures},{first_bad_text}")
        return

    failures = 0
    trials = 0
    min_rank = k
    for code_sample in range(args.code_samples):
        generator = (
            systematic_generator_prime(args.depth, args.total_expansion, args.prime, rng)
            if args.systematic
            else rfc_generator_prime(args.depth, args.total_expansion, args.prime, rng)
        )
        for _ in range(args.subset_samples):
            columns = rng.sample(range(n), args.zero_count)
            rank = rank_selected_columns(generator, columns, args.prime)
            min_rank = min(min_rank, rank)
            failures += int(rank < k)
            trials += 1
        if args.code_samples >= 10 and (code_sample + 1) % max(1, args.code_samples // 10) == 0:
            print(f"code_sample={code_sample + 1}/{args.code_samples}", flush=True)

    failure_rate = failures / trials if trials else 0.0
    log2_expected = -math.inf if failures == 0 else log2_comb(n, args.zero_count) + math.log2(failure_rate)
    ensemble = "systematic" if args.systematic else "original"
    print(
        "ensemble,prime,depth,k,n,zero_count,code_samples,subset_samples,"
        "failures,trials,failure_rate,min_rank,log2_expected_deficient_sets"
    )
    print(
        f"{ensemble},{args.prime},{args.depth},{k},{n},{args.zero_count},"
        f"{args.code_samples},{args.subset_samples},{failures},{trials},"
        f"{failure_rate:.12g},{min_rank},{log2_expected:.12g}"
    )


if __name__ == "__main__":
    main()
