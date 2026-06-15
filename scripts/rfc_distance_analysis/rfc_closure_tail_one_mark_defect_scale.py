#!/usr/bin/env python3
"""One-mark closure recurrence with direct A0 core-defect stratification.

This is a scale diagnostic for the Tier-2 crawl step.  It modifies the older
one-mark closure recurrence only in the A0 branch:

    old A0: q^{-(k-p)}

    defect-safe A0:
        union over u >= 0 of the ordinary rank-tail event
        rank(P union {a}) <= p-u,
        with random-matrix exponent (u+1)(k-p+u).

The PA branch is deliberately left as a total child one-mark closure overcount.
That avoids trying to close an exact C_d(p,1;u) recurrence before we know it is
needed.
"""

from __future__ import annotations

import argparse
import functools
import math
from dataclasses import dataclass


NEG_INF = -1.0e300


def log2_add(left: float, right: float) -> float:
    if left <= NEG_INF / 2:
        return right
    if right <= NEG_INF / 2:
        return left
    if right > left:
        left, right = right, left
    return left + math.log2(1.0 + 2.0 ** (right - left))


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


@dataclass(frozen=True)
class Term:
    kind: str
    log2_value: float
    y: int
    u: int
    q_exponent: int
    child_value: float
    lift_log2: float


class ClosureOneMarkDefectScale:
    def __init__(self, *, expansion: int, q_log2: float) -> None:
        self.expansion = expansion
        self.q_log2 = q_log2
        self.best: dict[tuple[int, int], Term] = {}

    @functools.cache
    def log2_C(self, depth: int, p: int) -> float:
        k = 1 << depth
        n = self.expansion * k
        if p < 0 or p >= n:
            return NEG_INF
        if depth == 0:
            if p <= 0:
                return NEG_INF
            return math.log2(n) + log2_comb(n - 1, p)

        total = NEG_INF
        best = Term("none", NEG_INF, 0, 0, 0, NEG_INF, NEG_INF)

        # A0: fixed marked coordinate, sibling not in P.  If P has rank p-u and
        # a is in cl(P), then rank(P union {a}) <= p-u.  In the random-matrix
        # scale, a fixed (p+1)-set having rank at most p-u costs
        # (u+1)(k-p+u) q-dimensions.
        a0_count = math.log2(n) + log2_comb(n - 2, p)
        max_u = min(p, k)
        for u in range(max_u + 1):
            rank_bound = p - u
            if rank_bound < 0 or rank_bound > min(k, p + 1):
                continue
            q_exp = (u + 1) * (k - p + u)
            term = a0_count - q_exp * self.q_log2
            total = log2_add(total, term)
            if term > best.log2_value:
                best = Term("A0_defect_rank_tail", term, 0, u, q_exp, NEG_INF, a0_count)

        # PA: same conservative lift as the older diagnostic.  We intentionally
        # do not try to preserve parent rank defect here.
        n_child = n // 2
        max_y = min(p - 1, n_child - 1)
        for y in range(max_y + 1):
            child = self.log2_C(depth - 1, y)
            if child <= NEG_INF / 2:
                continue
            remaining = p - 1 - y
            lift = 1.0 + y + log2_comb(2 * (n_child - 1) - y, remaining)
            term = child + lift
            total = log2_add(total, term)
            if term > best.log2_value:
                best = Term("PA_child_total_closure", term, y, 0, 0, child, lift)

        self.best[(depth, p)] = best
        return total


def trace(rec: ClosureOneMarkDefectScale, depth: int, p: int) -> list[tuple[int, int, float, Term]]:
    out: list[tuple[int, int, float, Term]] = []
    cur_depth, cur_p = depth, p
    seen: set[tuple[int, int]] = set()
    while (cur_depth, cur_p) not in seen:
        seen.add((cur_depth, cur_p))
        value = rec.log2_C(cur_depth, cur_p)
        term = rec.best.get((cur_depth, cur_p), Term("base", value, 0, 0, 0, NEG_INF, NEG_INF))
        out.append((cur_depth, cur_p, value, term))
        if term.kind != "PA_child_total_closure":
            break
        cur_depth -= 1
        cur_p = term.y
        if cur_depth < 0:
            break
    return out


def fmt(value: float) -> str:
    if value <= NEG_INF / 2:
        return "-inf"
    return f"{value:.6f}"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--depth", type=int, default=10)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--p", type=int, default=137)
    parser.add_argument("--residual-q-exp", type=int, default=0)
    args = parser.parse_args()

    rec = ClosureOneMarkDefectScale(expansion=args.expansion, q_log2=args.q_log2)
    value = rec.log2_C(args.depth, args.p)
    combined = value - args.residual_q_exp * args.q_log2
    status = "pass" if combined <= -1.0 else "fail"
    print("depth,expansion,q_log2,p,residual_q_exp,log2_C,combined_log2,status")
    print(
        f"{args.depth},{args.expansion},{args.q_log2:g},{args.p},"
        f"{args.residual_q_exp},{value:.6f},{combined:.6f},{status}"
    )
    print("trace_depth,trace_p,log2_C,kind,y,u,q_exponent,child_value,lift_log2,log2_term")
    for d, p, value, term in trace(rec, args.depth, args.p):
        print(
            f"{d},{p},{value:.6f},{term.kind},{term.y},{term.u},{term.q_exponent},"
            f"{fmt(term.child_value)},{fmt(term.lift_log2)},{fmt(term.log2_value)}"
        )


if __name__ == "__main__":
    main()
