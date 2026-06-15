#!/usr/bin/env python3
"""Two-parameter short-set rank-tail recurrence diagnostic for RFC.

This script is a theorem-candidate calculator, not a certificate.

It studies

    B_d(z,s) = E[# {Z : |Z|=z and rank(G_d[Z]) <= min(k_d,z)-s}].

The recurrence has two parts:

1. exact top-shape accounting for paired compression and child structural witnesses;
2. a selectable finite-root tail model for mixed singleton shapes.

The goal is to see whether the short-set/flat-excess route has enough entropy slack once the
obvious RFC shape recursion is included.  The finite-root model is the remaining theorem obligation.
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


def random_matrix_q_exponent(k: int, z: int, s: int) -> int:
    """q-exponent for rank <= min(k,z)-s in a random k x z matrix."""
    if s <= 0:
        return 0
    if z <= k:
        return s * (k - z + s)
    return s * (z - k + s)


def finite_root_q_exponent(model: str, *, k: int, z: int, s: int, t: int) -> int | None:
    """Return q-exponent for mixed singleton finite-root drops.

    None means this top profile gets no direct finite-root term and must be charged recursively.
    """
    if s <= 0:
        return 0
    if t <= 0:
        return None
    if model == "none":
        return None
    if model == "one_minor":
        return s
    if model == "singleton_surplus":
        # A deliberately cautious surplus-style model: each rank drop needs one equation, and
        # extra singleton columns beyond the requested rank drop can provide extra equations.
        return max(s, t - s + 1)
    if model == "random_matrix":
        return random_matrix_q_exponent(k, z, s)
    raise ValueError(model)


@dataclass(frozen=True)
class Term:
    kind: str
    log2_value: float
    p: int
    t: int
    a: int
    child_z: int
    child_s: int
    q_exponent: int


class ShortSetRecurrence:
    def __init__(self, *, expansion: int, q_log2: float, finite_root_model: str) -> None:
        self.expansion = expansion
        self.q_log2 = q_log2
        self.finite_root_model = finite_root_model
        self.best: dict[tuple[int, int, int], Term] = {}

    def profile_count(self, n_child: int, p: int, t: int) -> float:
        # Choose paired child positions, singleton child positions, and singleton sides.
        return log2_comb(n_child, p) + log2_comb(n_child - p, t) + t

    def extension_count(self, n_child: int, y: int, p: int, t: int, a: int) -> float:
        # Given a bad child set Y=P union A of size y=p+a:
        # choose P inside Y, choose the other singleton child positions outside Y, then choose
        # left/right sides for all singleton positions.
        return log2_comb(y, p) + log2_comb(n_child - y, t - a) + t

    @functools.cache
    def log2_B(self, depth: int, z: int, s: int) -> float:
        if s <= 0:
            return log2_comb(self.expansion * (1 << depth), z)
        k = 1 << depth
        n = self.expansion * k
        if z < 0 or z > n or s > min(k, z):
            return NEG_INF
        if depth == 0:
            # The depth-0 repetition generator has rank one on every nonempty set.
            return NEG_INF

        n_child = n // 2
        total = NEG_INF
        best = Term("none", NEG_INF, 0, 0, 0, 0, 0, 0)

        p_min = max(0, z - n_child)
        p_max = min(n_child, z // 2)
        for p in range(p_min, p_max + 1):
            t = z - 2 * p
            if t < 0 or t > n_child - p:
                continue

            q_exp = finite_root_q_exponent(
                self.finite_root_model,
                k=k,
                z=z,
                s=s,
                t=t,
            )
            if q_exp is not None:
                term = self.profile_count(n_child, p, t) - q_exp * self.q_log2
                total = log2_add(total, term)
                if term > best.log2_value:
                    best = Term("finite_root", term, p, t, -1, 0, 0, q_exp)

            # Structural child witnesses.  If A subset T has size a, then the matroid-union
            # generic rank formula implies parent deficit s when the child set P union A has
            # short-set rank deficit ceil((a+s)/2).
            for a in range(0, t + 1):
                y = p + a
                child_s = (a + s + 1) // 2
                child = self.log2_B(depth - 1, y, child_s)
                if child <= NEG_INF / 2:
                    continue
                term = child + self.extension_count(n_child, y, p, t, a)
                total = log2_add(total, term)
                if term > best.log2_value:
                    best = Term("child_witness", term, p, t, a, y, child_s, 0)

        self.best[(depth, z, s)] = best
        return total


def trace(rec: ShortSetRecurrence, depth: int, z: int, s: int) -> list[tuple[int, int, int, Term, float]]:
    out: list[tuple[int, int, int, Term, float]] = []
    cur = (depth, z, s)
    seen: set[tuple[int, int, int]] = set()
    while cur not in seen:
        seen.add(cur)
        value = rec.log2_B(*cur)
        term = rec.best.get(cur, Term("none", NEG_INF, 0, 0, 0, 0, 0, 0))
        out.append((cur[0], cur[1], cur[2], term, value))
        if term.kind != "child_witness":
            break
        cur = (cur[0] - 1, term.child_z, term.child_s)
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--depth", type=int, default=10)
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--z", type=int, required=True)
    ap.add_argument("--s", type=int, required=True)
    ap.add_argument(
        "--finite-root-model",
        choices=["none", "one_minor", "singleton_surplus", "random_matrix"],
        default="random_matrix",
    )
    args = ap.parse_args()

    rec = ShortSetRecurrence(
        expansion=args.expansion,
        q_log2=args.q_log2,
        finite_root_model=args.finite_root_model,
    )
    value = rec.log2_B(args.depth, args.z, args.s)
    print("depth,expansion,q_log2,finite_root_model,z,s,log2_B,status")
    status = "empty" if value <= NEG_INF / 2 else "nonempty"
    print(
        f"{args.depth},{args.expansion},{args.q_log2:g},{args.finite_root_model},"
        f"{args.z},{args.s},{value:.6f},{status}"
    )
    print("trace_depth,trace_z,trace_s,log2_B,kind,p,t,a,child_z,child_s,q_exponent,log2_term")
    for d, z, s, term, val in trace(rec, args.depth, args.z, args.s):
        print(
            f"{d},{z},{s},{val:.6f},{term.kind},{term.p},{term.t},{term.a},"
            f"{term.child_z},{term.child_s},{term.q_exponent},{term.log2_value:.6f}"
        )


if __name__ == "__main__":
    main()
