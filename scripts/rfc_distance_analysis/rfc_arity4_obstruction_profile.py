#!/usr/bin/env python3
"""Profile whether binary RFC mixed-PA obstructions survive arity-4 blocks.

This is a local algebra diagnostic, not a benchmark.

For each child coordinate j, choose four child-copy columns h_j in H^4. A true
arity-4 fold stores four invertible linear combinations of these child copies.
We compare P/A mixed profiles:

    P contributes some local output rows.
    A contributes disjoint local output rows from the same block.

The binary all-mixed obstruction works because P and A cover the whole local
two-output block, so low A-rank modulo P is exactly covered-span deficiency.
In arity 4, P/A may cover only part of the block; this script measures both
the covered-span rank and the full four-copy rank.
"""

from __future__ import annotations

import argparse
import itertools
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


def scale(c: int, v: list[int], q: int) -> list[int]:
    return [(c * x) % q for x in v]


def add_many(rows: list[list[int]], q: int) -> list[int]:
    if not rows:
        return []
    out = [0] * len(rows[0])
    for row in rows:
        out = [(a + b) % q for a, b in zip(out, row)]
    return out


def child_copy(h: list[int], copy: int, arity: int) -> list[int]:
    k = len(h)
    out = [0] * (arity * k)
    out[copy * k : (copy + 1) * k] = h
    return out


def lagrange_eval_matrix(beta: list[int], theta: list[int], q: int) -> list[list[int]]:
    mat: list[list[int]] = []
    for x in theta:
        row: list[int] = []
        for s, b_s in enumerate(beta):
            num = 1
            den = 1
            for t, b_t in enumerate(beta):
                if t == s:
                    continue
                num = (num * (x - b_t)) % q
                den = (den * (b_s - b_t)) % q
            row.append((num * mod_inv(den, q)) % q)
        mat.append(row)
    return mat


def random_invertible_matrix(n: int, q: int, rng: random.Random) -> list[list[int]]:
    while True:
        mat = [[rng.randrange(q) for _ in range(n)] for _ in range(n)]
        if rank_rows(mat, q) == n:
            return mat


def random_nonzero_vec(k: int, q: int, rng: random.Random) -> list[int]:
    while True:
        v = [rng.randrange(q) for _ in range(k)]
        if any(v):
            return v


def output_col(h: list[int], coeffs: list[int], q: int) -> list[int]:
    copies = [scale(c, child_copy(h, s, len(coeffs)), q) for s, c in enumerate(coeffs)]
    return add_many(copies, q)


def random_output_u_rows(
    k: int,
    q: int,
    u_rows: int,
    matrix: list[list[int]],
    rng: random.Random,
) -> list[list[int]]:
    rows: list[list[int]] = []
    for _ in range(u_rows):
        h = random_nonzero_vec(k, q, rng)
        out_idx = rng.randrange(len(matrix))
        rows.append(output_col(h, matrix[out_idx], q))
    return rows


def parse_indices(text: str) -> list[int]:
    if not text:
        return []
    return [int(x) for x in text.split(",")]


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--q", type=int, default=7)
    ap.add_argument("--k", type=int, default=6)
    ap.add_argument("--mixed", type=int, default=3)
    ap.add_argument("--u-rows", type=int, default=3)
    ap.add_argument("--instances", type=int, default=200)
    ap.add_argument("--matrix", choices=["rs", "random"], default="rs")
    ap.add_argument("--p-indices", default="0")
    ap.add_argument("--a-indices", default="1")
    ap.add_argument("--seed", type=int, default=1)
    args = ap.parse_args()

    rng = random.Random(args.seed)
    arity = 4
    p_indices = parse_indices(args.p_indices)
    a_indices = parse_indices(args.a_indices)
    if set(p_indices) & set(a_indices):
        raise SystemExit("P and A indices must be disjoint")
    if any(i < 0 or i >= arity for i in p_indices + a_indices):
        raise SystemExit("indices must lie in 0..3")

    if args.matrix == "rs":
        beta = [0, 1, 2, 3]
        theta = [4, 5, 6, 7]
        if args.q <= 7:
            raise SystemExit("--matrix rs needs q > 7 for distinct beta/theta defaults")
        matrix = lagrange_eval_matrix(beta, theta, args.q)
    else:
        matrix = random_invertible_matrix(arity, args.q, rng)

    hist: Counter[tuple[int, int, int, int, int]] = Counter()
    for _ in range(args.instances):
        u = random_output_u_rows(args.k, args.q, args.u_rows, matrix, rng)
        h_cols = [random_nonzero_vec(args.k, args.q, rng) for _ in range(args.mixed)]

        p_cols: list[list[int]] = []
        a_cols: list[list[int]] = []
        covered_cols: list[list[int]] = []
        full_cols: list[list[int]] = []
        for h in h_cols:
            local_outputs = [output_col(h, matrix[i], args.q) for i in range(arity)]
            p_cols.extend(local_outputs[i] for i in p_indices)
            a_cols.extend(local_outputs[i] for i in a_indices)
            covered_cols.extend(local_outputs[i] for i in p_indices + a_indices)
            full_cols.extend(child_copy(h, s, arity) for s in range(arity))

        p_rank = rank_increment(u, p_cols, args.q)
        a_rank = rank_increment(u + p_cols, a_cols, args.q)
        covered_rank = rank_increment(u, covered_cols, args.q)
        full_rank = rank_increment(u, full_cols, args.q)
        child_rank = rank_increment(
            [[*row[i * args.k : (i + 1) * args.k]] for row in u for i in range(arity)],
            h_cols,
            args.q,
        )
        hist[(child_rank, p_rank, a_rank, covered_rank, full_rank)] += 1

    print(
        "q,k,mixed,u_rows,matrix,p_indices,a_indices,instances,"
        "child_rank,p_rank,a_rank,covered_rank,full_rank,count"
    )
    for key, count in sorted(hist.items()):
        child_rank, p_rank, a_rank, covered_rank, full_rank = key
        print(
            f"{args.q},{args.k},{args.mixed},{args.u_rows},{args.matrix},"
            f"{args.p_indices},{args.a_indices},{args.instances},"
            f"{child_rank},{p_rank},{a_rank},{covered_rank},{full_rank},{count}"
        )


if __name__ == "__main__":
    main()
