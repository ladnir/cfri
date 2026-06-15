#!/usr/bin/env python3
"""Multi-mark closure-tail recurrence diagnostic for RFC.

This is a theorem-candidate stress model, not a certificate.

It models

    C_d(p,a) = E[# {(P,A): |P|=p, |A|=a, A subset cl(P)}].

The recurrence permits correlated PA chains:

    - b marked coordinates are routed through PA projection to a child closure
      event C_{d-1}(y,b);
    - the remaining a-b marked coordinates are charged directly at the parent
      level as A0-style quotient closure events.

The lift counts are intentionally loose, so a passing result is meaningful but
a failing result may only identify overcount.
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


def fmt(value: float) -> str:
    if value <= NEG_INF / 2:
        return "-inf"
    return f"{value:.6f}"


@dataclass(frozen=True)
class Term:
    kind: str
    log2_value: float
    child_p: int
    child_a: int
    pa_marks: int
    direct_marks: int
    direct_q_exp: int
    child_value: float
    lift_log2: float


class ClosureMultiMarkRecurrence:
    def __init__(self, *, expansion: int, q_log2: float) -> None:
        self.expansion = expansion
        self.q_log2 = q_log2
        self.best: dict[tuple[int, int, int], Term] = {}

    def base_count(self, n: int, p: int, a: int) -> float:
        if a < 0 or p < 0 or p + a > n:
            return NEG_INF
        if a == 0:
            return log2_comb(n, p)
        if p <= 0:
            return NEG_INF
        return log2_comb(n, a) + log2_comb(n - a, p)

    @functools.cache
    def log2_C(self, depth: int, p: int, a: int) -> float:
        k = 1 << depth
        n = self.expansion * k
        if a < 0 or p < 0 or p + a > n:
            return NEG_INF
        if a == 0:
            return log2_comb(n, p)
        if depth == 0:
            value = self.base_count(n, p, a)
            self.best[(depth, p, a)] = Term("base", value, 0, 0, 0, a, 0, NEG_INF, NEG_INF)
            return value

        total = NEG_INF
        best = Term("none", NEG_INF, 0, 0, 0, 0, 0, NEG_INF, NEG_INF)

        # Direct all-A0-style term. This is the random quotient closure scale.
        direct_q = max(0, a * (k - p))
        direct_count = log2_comb(n, a) + log2_comb(n - a, p)
        direct = direct_count - direct_q * self.q_log2
        total = log2_add(total, direct)
        best = Term("direct", direct, 0, 0, 0, a, direct_q, NEG_INF, direct_count)

        n_child = n // 2
        # Route b marked coordinates through PA projection to a child closure event.
        for b in range(1, a + 1):
            direct_marks = a - b
            max_y = min(p - b, n_child - b)
            for y in range(max_y + 1):
                child = self.log2_C(depth - 1, y, b)
                if child <= NEG_INF / 2:
                    continue
                remaining_p = p - b - y
                if remaining_p < 0:
                    continue

                # Child C chooses child core Q and marked child set J.
                # Lift PA marks: choose marked side over every j in J, sibling goes to P.
                # Lift Q: choose one P side over every q in Q.
                # Remaining parent P coordinates are chosen from all still-available siblings.
                available_for_extra_p = 2 * (n_child - b) - y
                pa_lift = b + y + log2_comb(available_for_extra_p, remaining_p)

                # Remaining direct marks are parent-level marked coordinates. This overcounts by
                # allowing arbitrary positions; the direct q-exponent charges their closure.
                direct_q_exp = max(0, direct_marks * (k - p))
                direct_lift = log2_comb(n, direct_marks) - direct_q_exp * self.q_log2
                lift = pa_lift + direct_lift
                term = child + lift
                total = log2_add(total, term)
                if term > best.log2_value:
                    best = Term(
                        "hybrid_pa_direct" if direct_marks else "all_pa_child",
                        term,
                        y,
                        b,
                        b,
                        direct_marks,
                        direct_q_exp,
                        child,
                        lift,
                    )

        self.best[(depth, p, a)] = best
        return total


def trace(
    rec: ClosureMultiMarkRecurrence, depth: int, p: int, a: int
) -> list[tuple[int, int, int, float, Term]]:
    out: list[tuple[int, int, int, float, Term]] = []
    cur = (depth, p, a)
    seen: set[tuple[int, int, int]] = set()
    while cur not in seen:
        seen.add(cur)
        value = rec.log2_C(*cur)
        term = rec.best.get(cur, Term("none", NEG_INF, 0, 0, 0, 0, 0, NEG_INF, NEG_INF))
        out.append((cur[0], cur[1], cur[2], value, term))
        if term.kind not in ("all_pa_child", "hybrid_pa_direct"):
            break
        if term.pa_marks <= 0:
            break
        cur = (cur[0] - 1, term.child_p, term.child_a)
        if cur[0] < 0:
            break
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--depth", type=int, default=10)
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--p", type=int, default=137)
    ap.add_argument("--a", type=int, required=True)
    ap.add_argument("--residual-q-exp", type=int, default=0)
    args = ap.parse_args()

    rec = ClosureMultiMarkRecurrence(expansion=args.expansion, q_log2=args.q_log2)
    value = rec.log2_C(args.depth, args.p, args.a)
    combined = value - args.residual_q_exp * args.q_log2
    status = "pass" if combined <= -1.0 else "fail"
    print("depth,expansion,q_log2,p,a,residual_q_exp,log2_C,combined_log2,status")
    print(
        f"{args.depth},{args.expansion},{args.q_log2:g},{args.p},{args.a},"
        f"{args.residual_q_exp},{fmt(value)},{fmt(combined)},{status}"
    )
    print(
        "trace_depth,trace_p,trace_a,log2_C,kind,child_p,child_a,"
        "pa_marks,direct_marks,direct_q_exp,child_value,lift_log2,log2_term"
    )
    for d, p, a, val, term in trace(rec, args.depth, args.p, args.a):
        print(
            f"{d},{p},{a},{fmt(val)},{term.kind},{term.child_p},{term.child_a},"
            f"{term.pa_marks},{term.direct_marks},{term.direct_q_exp},"
            f"{fmt(term.child_value)},{fmt(term.lift_log2)},{fmt(term.log2_value)}"
        )


if __name__ == "__main__":
    main()
