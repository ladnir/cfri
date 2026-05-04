#!/usr/bin/env python3
"""Exhaustive toy distances for systematic RFC candidates over small prime fields.

This is a research sanity-check script. It builds paper-style RFC generator matrices with
T' = -T over GF(p), then compares:

  non-systematic RFC at total expansion c
  systematic code (m, RFC parity at expansion c-1)

Only use this for tiny k and p.
"""

from __future__ import annotations

import argparse
import csv
import itertools
import random


def matmul_mod(lhs: list[list[int]], rhs: list[list[int]], p: int) -> list[list[int]]:
    rows = len(lhs)
    mid = len(rhs)
    cols = len(rhs[0])
    out = [[0] * cols for _ in range(rows)]
    for i in range(rows):
        for k in range(mid):
            a = lhs[i][k]
            if a == 0:
                continue
            rhs_k = rhs[k]
            for j in range(cols):
                out[i][j] = (out[i][j] + a * rhs_k[j]) % p
    return out


def rfc_generator(k0: int, depth: int, expansion: int, p: int, rng: random.Random) -> list[list[int]]:
    if k0 != 1:
        raise ValueError("only k0=1 repetition base is implemented")
    g = [[1 for _ in range(expansion)]]
    n = expansion
    for _ in range(depth):
        t = [rng.randrange(p) for _ in range(n)]
        top = []
        bottom = []
        for row in g:
            top.append(
                [((1 - t[j]) * row[j]) % p for j in range(n)]
                + [((1 - (t[j] + 1)) * row[j]) % p for j in range(n)]
            )
            bottom.append(
                [(t[j] * row[j]) % p for j in range(n)]
                + [((t[j] + 1) * row[j]) % p for j in range(n)]
            )
        g = top + bottom
        n *= 2
    return g


def encode(message: tuple[int, ...], generator: list[list[int]], p: int) -> list[int]:
    out = [0] * len(generator[0])
    for i, value in enumerate(message):
        if value == 0:
            continue
        row = generator[i]
        for j in range(len(out)):
            out[j] = (out[j] + value * row[j]) % p
    return out


def weight(values: list[int] | tuple[int, ...]) -> int:
    return sum(1 for value in values if value != 0)


def exhaustive_distance(generator: list[list[int]], p: int, systematic: bool) -> tuple[int, tuple[int, ...]]:
    k = len(generator)
    best = 10**18
    best_message: tuple[int, ...] = ()
    for message in itertools.product(range(p), repeat=k):
        if all(value == 0 for value in message):
            continue
        codeword = encode(message, generator, p)
        candidate = weight(codeword)
        if systematic:
            candidate += weight(message)
        if candidate < best:
            best = candidate
            best_message = message
    return best, best_message


def support_zero_profile(generator: list[list[int]], p: int) -> list[int]:
    k = len(generator)
    n = len(generator[0])
    profile = [-1] * (k + 1)
    profile[0] = n
    for message in itertools.product(range(p), repeat=k):
        support = weight(message)
        if support == 0:
            continue
        zeros = n - weight(encode(message, generator, p))
        if zeros > profile[support]:
            profile[support] = zeros
    return profile


def read_round_thresholds(path: str, round_index: int) -> list[int]:
    rows: list[tuple[int, int, int]] = []
    with open(path, newline="") as handle:
        for row in csv.DictReader(handle):
            if int(row["round"]) == round_index:
                rows.append((int(row["support"]), int(row["threshold"]), int(row["parity_n"])))
    if not rows:
        raise ValueError(f"round {round_index} not found in {path}")
    rows.sort()
    parity_n = rows[0][2]
    if [support for support, _, _ in rows] != list(range(1, len(rows) + 1)):
        raise ValueError(f"round {round_index} supports are not contiguous")
    return [parity_n + 1] + [threshold for _, threshold, _ in rows]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--p", type=int, default=5)
    parser.add_argument("--depth", type=int, default=3)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--thresholds-csv", default=None)
    parser.add_argument("--threshold-round", type=int, default=None)
    args = parser.parse_args()

    if args.total_expansion <= 1:
        raise SystemExit("total expansion must be greater than one")

    rng = random.Random(args.seed)
    old = rfc_generator(1, args.depth, args.total_expansion, args.p, rng)
    sys_parity = rfc_generator(1, args.depth, args.total_expansion - 1, args.p, rng)

    old_distance, old_message = exhaustive_distance(old, args.p, systematic=False)
    sys_distance, sys_message = exhaustive_distance(sys_parity, args.p, systematic=True)

    k = 1 << args.depth
    old_n = args.total_expansion * k
    sys_n = args.total_expansion * k
    print(f"p={args.p} depth={args.depth} k={k} total_expansion={args.total_expansion}")
    print(
        f"non_systematic_distance={old_distance} relative={old_distance / old_n:.8f} "
        f"support={weight(old_message)} message={old_message}"
    )
    print(
        f"systematic_distance={sys_distance} relative={sys_distance / sys_n:.8f} "
        f"support={weight(sys_message)} message={sys_message}"
    )

    if args.thresholds_csv is not None:
        threshold_round = args.depth if args.threshold_round is None else args.threshold_round
        thresholds = read_round_thresholds(args.thresholds_csv, threshold_round)
        profile = support_zero_profile(sys_parity, args.p)
        violations = []
        for support in range(1, len(profile)):
            if profile[support] >= thresholds[support]:
                violations.append((support, profile[support], thresholds[support]))
        print(f"threshold_round={threshold_round}")
        print(f"profile_ok={str(not violations).lower()}")
        if violations:
            for support, zeros, threshold in violations[:20]:
                print(f"violation support={support} zeros={zeros} threshold={threshold}")


if __name__ == "__main__":
    main()
