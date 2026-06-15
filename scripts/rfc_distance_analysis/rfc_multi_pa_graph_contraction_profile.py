#!/usr/bin/env python3
"""Profile multi-PA graph-contraction ranks over small fields.

Condition on:

    U <= H+H       from non-mixed P columns,
    h_j in H       for mixed child positions.

For mixed PA coordinates, P contributes graph lines

    p_j = (h_j, alpha_j h_j)

and the complementary A columns are equivalent modulo p_j to

    y_j = (0, h_j).

This script enumerates alpha_j in F_q^* for small m and records

    rank({y_j} modulo U + span{p_j}).

It is a local algebra profiler for the multi-PA incidence target, not a benchmark.
"""

from __future__ import annotations

import argparse
import itertools
import math
import random
from collections import Counter


def mod_inv(x: int, q: int) -> int:
    return pow(x % q, q - 2, q)


def rank_rows(rows: list[list[int]], q: int) -> int:
    if not rows:
        return 0
    mat = [row[:] for row in rows]
    m = len(mat)
    n = len(mat[0])
    rank = 0
    for col in range(n):
        pivot = None
        for r in range(rank, m):
            if mat[r][col] % q:
                pivot = r
                break
        if pivot is None:
            continue
        mat[rank], mat[pivot] = mat[pivot], mat[rank]
        inv = mod_inv(mat[rank][col], q)
        mat[rank] = [(v * inv) % q for v in mat[rank]]
        for r in range(m):
            if r == rank:
                continue
            factor = mat[r][col] % q
            if factor:
                mat[r] = [(a - factor * b) % q for a, b in zip(mat[r], mat[rank])]
        rank += 1
        if rank == m:
            break
    return rank


def rank_increment(base: list[list[int]], extra: list[list[int]], q: int) -> int:
    return rank_rows(base + extra, q) - rank_rows(base, q)


def rado_generic_rank(base: list[list[int]], subspaces: list[list[list[int]]], q: int) -> int:
    """Generic rank increment from one vector chosen in each listed subspace."""
    m = len(subspaces)
    best = m
    for mask in range(1 << m):
        active_rows: list[list[int]] = []
        active_count = 0
        for i, rows in enumerate(subspaces):
            if (mask >> i) & 1:
                active_count += 1
                active_rows.extend(rows)
        term = (m - active_count) + rank_increment(base, active_rows, q)
        best = min(best, term)
    return best


def scale(c: int, v: list[int], q: int) -> list[int]:
    return [(c * x) % q for x in v]


def add(u: list[int], v: list[int], q: int) -> list[int]:
    return [(a + b) % q for a, b in zip(u, v)]


def x_col(h: list[int], q: int) -> list[int]:
    return h + [0] * len(h)


def y_col(h: list[int], q: int) -> list[int]:
    return [0] * len(h) + h


def graph_col(h: list[int], alpha: int, q: int) -> list[int]:
    return add(x_col(h, q), scale(alpha, y_col(h, q), q), q)


def random_nonzero_vec(k: int, q: int, rng: random.Random) -> list[int]:
    while True:
        v = [rng.randrange(q) for _ in range(k)]
        if any(v):
            return v


def random_u_rows(k: int, q: int, u_rows: int, rng: random.Random) -> list[list[int]]:
    return [[rng.randrange(q) for _ in range(2 * k)] for _ in range(u_rows)]


def random_graph_u_rows(k: int, q: int, u_rows: int, rng: random.Random) -> list[list[int]]:
    rows: list[list[int]] = []
    for _ in range(u_rows):
        h = random_nonzero_vec(k, q, rng)
        alpha = rng.randrange(1, q)
        rows.append(graph_col(h, alpha, q))
    return rows


def log_q_count(count: int, q: int) -> float:
    if count <= 0:
        return float("-inf")
    return math.log(count, q)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--q", type=int, default=7)
    ap.add_argument("--k", type=int, default=5)
    ap.add_argument("--mixed", type=int, default=3)
    ap.add_argument("--u-rows", type=int, default=4)
    ap.add_argument("--u-mode", choices=["graph", "random"], default="graph")
    ap.add_argument("--instances", type=int, default=20)
    ap.add_argument("--summary-only", action="store_true")
    ap.add_argument("--seed", type=int, default=1)
    args = ap.parse_args()

    rng = random.Random(args.seed)
    domain = list(range(1, args.q))
    total_roots = len(domain) ** args.mixed

    if not args.summary_only:
        print(
            "q,k,mixed,u_rows,u_mode,instance,u_rank,child_rank,full_rank,min_rank_a,"
            "max_rank_a,min_p_rank,max_p_rank,formula_generic,rank,hist_count,"
            "probability,neg_logq_probability"
        )
    summary: Counter[tuple[int, int, int]] = Counter()
    for inst in range(args.instances):
        if args.u_mode == "random":
            u = random_u_rows(args.k, args.q, args.u_rows, rng)
        else:
            u = random_graph_u_rows(args.k, args.q, args.u_rows, rng)
        h_cols = [random_nonzero_vec(args.k, args.q, rng) for _ in range(args.mixed)]
        y_cols = [y_col(h, args.q) for h in h_cols]
        line_subspaces = [[x_col(h, args.q), y_col(h, args.q)] for h in h_cols]
        child_rank = rank_increment(
            [row[: args.k] for row in u] + [row[args.k :] for row in u],
            h_cols,
            args.q,
        )
        generic_p = rado_generic_rank(u, line_subspaces, args.q)
        y_rank = rank_increment(u, y_cols, args.q)
        generic_py = y_rank + rado_generic_rank(u + y_cols, line_subspaces, args.q)
        formula_generic = generic_py - generic_p
        full_rank = rank_increment(
            u,
            [col for h in h_cols for col in (x_col(h, args.q), y_col(h, args.q))],
            args.q,
        )

        hist: Counter[int] = Counter()
        max_rank_a = 0
        min_rank_a = args.mixed + 1
        max_p_rank = 0
        min_p_rank = args.mixed + 1
        for alphas in itertools.product(domain, repeat=args.mixed):
            p_cols = [graph_col(h, alpha, args.q) for h, alpha in zip(h_cols, alphas)]
            p_rank = rank_increment(u, p_cols, args.q)
            r = rank_increment(u + p_cols, y_cols, args.q)
            hist[r] += 1
            max_rank_a = max(max_rank_a, r)
            min_rank_a = min(min_rank_a, r)
            max_p_rank = max(max_p_rank, p_rank)
            min_p_rank = min(min_p_rank, p_rank)

        u_rank = rank_rows(u, args.q)
        for rank, count in sorted(hist.items()):
            summary[(child_rank, full_rank, min_rank_a, max_rank_a, max_p_rank, rank)] += count
            if args.summary_only:
                continue
            probability = count / total_roots
            codim = -log_q_count(count, args.q) + math.log(total_roots, args.q)
            print(
                f"{args.q},{args.k},{args.mixed},{args.u_rows},{args.u_mode},{inst},{u_rank},"
                f"{child_rank},{full_rank},{min_rank_a},{max_rank_a},{min_p_rank},"
                f"{max_p_rank},{formula_generic},{rank},{count},"
                f"{probability:.12g},{codim:.6f}"
            )
    if args.summary_only:
        total = args.instances * total_roots
        print(
            "q,k,mixed,u_rows,u_mode,instances,total_roots,child_rank,"
            "full_rank,min_rank_a,max_rank_a,max_p_rank,rank,count,probability"
        )
        for (child_rank, full_rank, min_rank_a, max_rank_a, max_p_rank, rank), count in sorted(summary.items()):
            probability = count / total
            print(
                f"{args.q},{args.k},{args.mixed},{args.u_rows},{args.u_mode},{args.instances},"
                f"{total},{child_rank},{full_rank},{min_rank_a},{max_rank_a},"
                f"{max_p_rank},{rank},{count},{probability:.12g}"
            )


if __name__ == "__main__":
    main()
