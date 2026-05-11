#!/usr/bin/env python3
"""Exact tiny-field first moment via independent single-tree products.

For expansion `e`, RFC parity coordinates are `e` independent copies of the same single-tree random
linear map. For a fixed message m, if `G_m(x)` is the single-tree weight distribution, then the
parity-weight distribution at expansion e is `G_m(x)^e`.

This script computes that exactly for tiny fields/depths by enumerating all single-tree T
assignments. It is meant to validate the product formulation against sampled full-expansion runs.
"""

from __future__ import annotations

import argparse
import itertools


def weight(values: list[int] | tuple[int, ...]) -> int:
    return sum(1 for value in values if value != 0)


def single_tree_generator(depth: int, ts: tuple[int, ...], p: int) -> list[list[int]]:
    """Build the one-expansion RFC generator using the same level order as the C++ sampler."""
    cursor = 0
    k = 1
    n = 1
    generator = [[1]]
    for _round in range(depth):
        diagonal = ts[cursor: cursor + n]
        cursor += n
        next_n = 2 * n
        next_generator = [[0] * next_n for _ in range(2 * k)]
        for row in range(k):
            for j, entry in enumerate(generator[row]):
                t = diagonal[j]
                next_generator[row][j] = ((1 - t) * entry) % p
                next_generator[row][n + j] = ((1 - (t + 1)) * entry) % p
                next_generator[row + k][j] = (t * entry) % p
                next_generator[row + k][n + j] = ((t + 1) * entry) % p
        generator = next_generator
        k *= 2
        n = next_n

    if cursor != len(ts):
        raise AssertionError((cursor, len(ts)))
    return generator


def encode_with_generator(message: tuple[int, ...], generator: list[list[int]], p: int) -> list[int]:
    out = [0] * len(generator[0])
    for value, row in zip(message, generator):
        if value == 0:
            continue
        for j, entry in enumerate(row):
            out[j] = (out[j] + value * entry) % p
    return out


def convolve(lhs: list[float], rhs: list[float], max_degree: int) -> list[float]:
    out = [0.0] * (max_degree + 1)
    for i, left in enumerate(lhs):
        if left == 0.0:
            continue
        for j, right in enumerate(rhs):
            if right == 0.0 or i + j > max_degree:
                continue
            out[i + j] += left * right
    return out


def power_distribution(base: list[float], exponent: int, max_degree: int) -> list[float]:
    out = [0.0] * (max_degree + 1)
    out[0] = 1.0
    cur = base[:]
    exp = exponent
    while exp:
        if exp & 1:
            out = convolve(out, cur, max_degree)
        exp >>= 1
        if exp:
            cur = convolve(cur, cur, max_degree)
    return out


def first_crossing(spectrum: list[float]) -> int | None:
    total = 0.0
    for i, value in enumerate(spectrum):
        if i == 0:
            continue
        total += value
        if total >= 1.0:
            return i
    return None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--p", type=int, default=5)
    parser.add_argument("--depth", type=int, default=2)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--max-print-weight", type=int, default=32)
    args = parser.parse_args()

    if args.p <= 2:
        raise SystemExit("use an odd prime p > 2")
    if args.total_expansion <= 1:
        raise SystemExit("total expansion must be greater than one")

    k = 1 << args.depth
    tree_n = k
    total_n = args.total_expansion * k
    num_tree_randomizers = tree_n - 1
    t_assignments = list(itertools.product(range(1, args.p), repeat=num_tree_randomizers))
    generators = [single_tree_generator(args.depth, ts, args.p) for ts in t_assignments]
    t_scale = 1.0 / len(t_assignments)

    old_spectrum = [0.0] * (total_n + 1)
    systematic_spectrum = [0.0] * (total_n + 1)

    for message in itertools.product(range(args.p), repeat=k):
        support = weight(message)
        if support == 0:
            continue
        single_tree = [0.0] * (tree_n + 1)
        for generator in generators:
            single_tree[weight(encode_with_generator(message, generator, args.p))] += t_scale

        old_dist = power_distribution(single_tree, args.total_expansion, total_n)
        sys_dist = power_distribution(single_tree, args.total_expansion - 1, total_n)
        for h, value in enumerate(old_dist):
            old_spectrum[h] += value
        for h, value in enumerate(sys_dist):
            if support + h <= total_n:
                systematic_spectrum[support + h] += value

    print(
        f"p={args.p} depth={args.depth} k={k} total_n={total_n} "
        f"tree_randomizers={num_tree_randomizers} t_assignments={len(t_assignments)}"
    )
    print(f"old_crossing={first_crossing(old_spectrum)}")
    print(f"systematic_crossing={first_crossing(systematic_spectrum)}")
    print("weight,old_expected_count,systematic_expected_count")
    for h in range(min(total_n, args.max_print_weight) + 1):
        if old_spectrum[h] or systematic_spectrum[h]:
            print(f"{h},{old_spectrum[h]:.12g},{systematic_spectrum[h]:.12g}")


if __name__ == "__main__":
    main()
