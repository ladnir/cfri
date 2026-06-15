#!/usr/bin/env python3
"""Search for gaps in the naive multi-PA projection lemma.

For one mixed PA coordinate, A-in-span(P) implies a child projection dependence.  A tempting
multi-coordinate generalization is:

    rank(A mixed siblings modulo P) >= rank(child h_j modulo projection(P_other)).

This script tests that inequality in random root-line models.  A failure means the multi-PA proof
needs a richer root-line state rather than a direct child-projection reduction.
"""

from __future__ import annotations

import argparse
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


def left_col(h: list[int], t: int, q: int) -> list[int]:
    return h + scale(t, h, q)


def right_col(h: list[int], t: int, q: int) -> list[int]:
    return h + scale(t + 1, h, q)


def random_nonzero_vec(k: int, q: int, rng: random.Random) -> list[int]:
    while True:
        v = [rng.randrange(q) for _ in range(k)]
        if any(v):
            return v


def projections(parent_rows: list[list[int]], k: int) -> list[list[int]]:
    out: list[list[int]] = []
    for row in parent_rows:
        out.append(row[:k])
        out.append(row[k:])
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--q", type=int, default=5)
    ap.add_argument("--k", type=int, default=4)
    ap.add_argument("--n", type=int, default=12)
    ap.add_argument("--p-other", type=int, default=3)
    ap.add_argument("--mixed", type=int, default=3)
    ap.add_argument("--trials", type=int, default=50000)
    ap.add_argument("--seed", type=int, default=1)
    args = ap.parse_args()

    rng = random.Random(args.seed)
    failures = 0
    checked = 0
    first = ""
    hist: Counter[tuple[int, int]] = Counter()

    for _ in range(args.trials):
        if args.p_other + args.mixed > args.n:
            raise SystemExit("p-other + mixed must be <= n")
        cols = [random_nonzero_vec(args.k, args.q, rng) for _ in range(args.n)]
        roots = [rng.randrange(1, args.q) for _ in range(args.n)]
        selected = rng.sample(range(args.n), args.p_other + args.mixed)
        other = selected[: args.p_other]
        mixed = selected[args.p_other :]

        u_rows: list[list[int]] = []
        for i in other:
            side = rng.randrange(2)
            u_rows.append(left_col(cols[i], roots[i], args.q) if side == 0 else right_col(cols[i], roots[i], args.q))

        p_mixed: list[list[int]] = []
        a_mixed: list[list[int]] = []
        for j in mixed:
            if rng.randrange(2) == 0:
                p_mixed.append(left_col(cols[j], roots[j], args.q))
                a_mixed.append(right_col(cols[j], roots[j], args.q))
            else:
                p_mixed.append(right_col(cols[j], roots[j], args.q))
                a_mixed.append(left_col(cols[j], roots[j], args.q))

        child_rank = rank_increment(projections(u_rows, args.k), [cols[j] for j in mixed], args.q)
        marked_rank = rank_increment(u_rows + p_mixed, a_mixed, args.q)
        hist[(child_rank, marked_rank)] += 1
        checked += 1
        if marked_rank < child_rank:
            failures += 1
            if not first:
                first = (
                    f"child_rank={child_rank} marked_rank={marked_rank} "
                    f"other={other} mixed={mixed}"
                )

    print("q,k,n,p_other,mixed,trials,checked,failures,status,first_failure")
    status = "pass" if failures == 0 else "fail"
    print(
        f"{args.q},{args.k},{args.n},{args.p_other},{args.mixed},{args.trials},"
        f"{checked},{failures},{status},{first}"
    )
    print("child_rank,marked_rank,count")
    for (child_rank, marked_rank), count in sorted(hist.items()):
        print(f"{child_rank},{marked_rank},{count}")


if __name__ == "__main__":
    main()
