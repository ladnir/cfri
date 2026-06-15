#!/usr/bin/env python3
"""Self-test for the PA mixed projection lemma.

The lemma says: if one sibling at child coordinate j is in P and the complementary sibling is in
span(P), then the child column h_j lies in the projection span of the other P columns.

This script samples random linear configurations over a small prime field and checks the
implication directly.  It is a sanity test for the deterministic algebra, not a benchmark.
"""

from __future__ import annotations

import argparse
import random


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


def in_span(vec: list[int], basis: list[list[int]], q: int) -> bool:
    return rank_rows(basis + [vec], q) == rank_rows(basis, q)


def add(u: list[int], v: list[int], q: int) -> list[int]:
    return [(a + b) % q for a, b in zip(u, v)]


def scale(c: int, v: list[int], q: int) -> list[int]:
    return [(c * x) % q for x in v]


def left_col(h: list[int], t: int, q: int) -> list[int]:
    return h + scale(t, h, q)


def right_col(h: list[int], t: int, q: int) -> list[int]:
    return h + scale(t + 1, h, q)


def random_nonzero_vec(k: int, q: int, rng: random.Random) -> list[int]:
    while True:
        v = [rng.randrange(q) for _ in range(k)]
        if any(v):
            return v


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--q", type=int, default=7)
    ap.add_argument("--k", type=int, default=5)
    ap.add_argument("--n", type=int, default=12)
    ap.add_argument("--p-other", type=int, default=4)
    ap.add_argument("--trials", type=int, default=20000)
    ap.add_argument("--seed", type=int, default=1)
    args = ap.parse_args()

    rng = random.Random(args.seed)
    failures = 0
    containments = 0

    for _ in range(args.trials):
        cols = [random_nonzero_vec(args.k, args.q, rng) for _ in range(args.n)]
        roots = [rng.randrange(1, args.q) for _ in range(args.n)]
        j = rng.randrange(args.n)
        candidates = [i for i in range(args.n) if i != j]
        chosen = rng.sample(candidates, min(args.p_other, len(candidates)))

        p_other: list[list[int]] = []
        projection_cols: list[list[int]] = []
        for i in chosen:
            side = rng.randrange(2)
            col = left_col(cols[i], roots[i], args.q) if side == 0 else right_col(cols[i], roots[i], args.q)
            p_other.append(col)
            projection_cols.append(cols[i])

        if rng.randrange(2) == 0:
            p_sibling = left_col(cols[j], roots[j], args.q)
            a_sibling = right_col(cols[j], roots[j], args.q)
        else:
            p_sibling = right_col(cols[j], roots[j], args.q)
            a_sibling = left_col(cols[j], roots[j], args.q)

        p_basis = p_other + [p_sibling]
        if in_span(a_sibling, p_basis, args.q):
            containments += 1
            if not in_span(cols[j], projection_cols, args.q):
                failures += 1
                break

    print("q,k,n,p_other,trials,containments,failures,status")
    status = "pass" if failures == 0 else "fail"
    print(
        f"{args.q},{args.k},{args.n},{args.p_other},{args.trials},"
        f"{containments},{failures},{status}"
    )


if __name__ == "__main__":
    main()
