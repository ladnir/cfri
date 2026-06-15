#!/usr/bin/env python3
"""One-mark closure-tail recurrence diagnostic for RFC.

This is a theorem-candidate calculator, not a certificate.

It models

    C_d(p) = E[# {(P,a): |P|=p, a notin P, a in cl(P)}]

for one marked coordinate.  The split is:

    A0: the sibling of a is not in P; pay a direct quotient closure cost q^{-(k-p)}.
    PA: the sibling of a is in P; use the PA projection lemma to route to a child
        closure event, then pay a conservative lift count for the remaining P coordinates.

The recurrence deliberately overcounts PA lifts.  Its purpose is to check whether the
low-rank flat endpoint has enough slack under the new original-proof upgrade route.
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
    q_exponent: int
    child_value: float
    lift_log2: float


class ClosureOneMarkRecurrence:
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
            # The depth-0 generator has one message dimension and every coordinate is nonzero
            # multiple of it, so every marked coordinate lies in the closure of any nonempty P.
            if p <= 0:
                return NEG_INF
            return math.log2(n) + log2_comb(n - 1, p)

        total = NEG_INF
        best = Term("none", NEG_INF, 0, 0, NEG_INF, NEG_INF)

        # A0: choose marked parent coordinate, forbid its sibling from P, choose P elsewhere.
        # Direct random quotient scale. This is the intended direct closure cost.
        direct_q = max(0, k - p)
        a0_count = math.log2(n) + log2_comb(n - 2, p)
        a0 = a0_count - direct_q * self.q_log2
        total = log2_add(total, a0)
        best = Term("A0_direct", a0, 0, direct_q, NEG_INF, a0_count)

        # PA: choose a child closure witness (Q,j) of size y in the child.  The parent marked
        # coordinate is one sibling over j, and the other sibling is in P.  For every q in Q,
        # require at least one parent P coordinate over q, then choose the remaining P coordinates
        # arbitrarily from the other available parent coordinates. This intentionally overcounts.
        n_child = n // 2
        max_y = min(p - 1, n_child - 1)
        for y in range(max_y + 1):
            child = self.log2_C(depth - 1, y)
            if child <= NEG_INF / 2:
                continue
            remaining = p - 1 - y
            # child C already chooses (Q,j). Lift choices:
            # - choose marked side over j: 2
            # - choose one parent side over each q in Q: 2^y
            # - choose remaining P' coords from all non-j parent coords not already selected.
            lift = 1.0 + y + log2_comb(2 * (n_child - 1) - y, remaining)
            term = child + lift
            total = log2_add(total, term)
            if term > best.log2_value:
                best = Term("PA_child_closure", term, y, 0, child, lift)

        self.best[(depth, p)] = best
        return total


def trace(rec: ClosureOneMarkRecurrence, depth: int, p: int) -> list[tuple[int, int, float, Term]]:
    out: list[tuple[int, int, float, Term]] = []
    cur_depth, cur_p = depth, p
    seen: set[tuple[int, int]] = set()
    while (cur_depth, cur_p) not in seen:
        seen.add((cur_depth, cur_p))
        value = rec.log2_C(cur_depth, cur_p)
        term = rec.best.get((cur_depth, cur_p), Term("base", value, 0, 0, NEG_INF, NEG_INF))
        out.append((cur_depth, cur_p, value, term))
        if term.kind != "PA_child_closure":
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
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--depth", type=int, default=10)
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--p", type=int, default=137)
    ap.add_argument(
        "--residual-q-exp",
        type=int,
        default=0,
        help="Optional residual repair q-exponent multiplied after the closure event.",
    )
    args = ap.parse_args()

    rec = ClosureOneMarkRecurrence(expansion=args.expansion, q_log2=args.q_log2)
    value = rec.log2_C(args.depth, args.p)
    combined = value - args.residual_q_exp * args.q_log2
    print("depth,expansion,q_log2,p,residual_q_exp,log2_C,combined_log2,status")
    status = "pass" if combined <= -1.0 else "fail"
    print(
        f"{args.depth},{args.expansion},{args.q_log2:g},{args.p},"
        f"{args.residual_q_exp},{value:.6f},{combined:.6f},{status}"
    )
    print("trace_depth,trace_p,log2_C,kind,y,q_exponent,child_value,lift_log2,log2_term")
    for d, p, val, term in trace(rec, args.depth, args.p):
        print(
            f"{d},{p},{val:.6f},{term.kind},{term.y},{term.q_exponent},"
            f"{fmt(term.child_value)},{fmt(term.lift_log2)},{fmt(term.log2_value)}"
        )


if __name__ == "__main__":
    main()
