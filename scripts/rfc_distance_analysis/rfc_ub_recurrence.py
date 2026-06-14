#!/usr/bin/env python3
"""Tractable RIGOROUS upper bound on B_d(1,z) via the exact 2u+X structure.

From the validated exact top relation (rfc_exact_top_relation.py): a parent codeword's zero count is
2u + X, where u = common zeros of the child 2-tuple and X = sum over non-common child columns of
independent Bernoulli(g/(q-1)), g in {0,1,2}.  Since g <= 2, stochastically X <= Bin(M, 2/(q-1))
with M = #non-common columns.  This upper bound depends ONLY on the child common-zero count, so the
state collapses to the exact zero-count distribution A_h(R, .) (a vector), avoiding the column-type
state explosion while keeping the 2u+X structure (no binom placement double-count).

Recurrence (all counts in log2):
  A_0(R, .):  mass q^R - 1 at w=0, else 0           (repetition base: nonzero R-tuple has 0 common zeros)
  A_d(R, w) = (+)_{u'} A_{d-1}(2R, u') * P[2u' + Bin(n_{d-1}-u', 2/(q-1)) = w]      (UPPER bound on X)
  where n_{d-1} = c*2^{d-1} = number of child columns (= half the parent length).
Top: B_d(1,z) = sum_w A_d(1,w) * binom(w,z).  Smallest z with B <= 2^-lambda gives distance >= n-z+1.

The chain is A_0(2^d) -> A_1(2^{d-1}) -> ... -> A_d(1).  This is an UPPER bound on the true first
moment, hence a valid distance LOWER bound, and it is tractable at production depth.
"""

from __future__ import annotations

import argparse
import math

NEG = float("-inf")


def logsumexp2(vals: list[float]) -> float:
    m = max((v for v in vals if v > NEG), default=NEG)
    if m == NEG:
        return NEG
    return m + math.log2(sum(2.0 ** (v - m) for v in vals if v > NEG))


def log2_binom_table(n: int) -> list[list[float]]:
    # returns function-ish: we only need rows up to n; build lazily per-need to save memory.
    raise NotImplementedError


def log2_binom_row(nval: int, kmax: int) -> list[float]:
    """log2 binom(nval, x) for x=0..kmax."""
    row = [NEG] * (kmax + 1)
    row[0] = 0.0
    acc = 0.0
    for x in range(1, kmax + 1):
        # binom(n,x) = binom(n,x-1) * (n-x+1)/x
        num = nval - x + 1
        if num <= 0:
            break
        acc += math.log2(num) - math.log2(x)
        row[x] = acc
    return row


def step(child: list[float], n_child_cols: int, log2p: float, log2_1mp: float,
         w_cap: int, x_cap: int) -> list[float]:
    """One fold step: child = A_{h-1}(2R, .) over u'=0..n_child_cols; return A_h(R, .) over w=0..w_cap.

    A_h(R, w) = logsumexp over u',x with 2u'+x=w of
        child[u'] + log2 C(M, x) + x*log2p + (M-x)*log2_1mp,   M = n_child_cols - u'.
    """
    parent = [NEG] * (w_cap + 1)
    for up in range(min(n_child_cols, w_cap // 2) + 1):
        cu = child[up]
        if cu <= NEG:
            continue
        M = n_child_cols - up
        if M < 0:
            continue
        xmax = min(x_cap, M, w_cap - 2 * up)
        if xmax < 0:
            continue
        brow = log2_binom_row(M, xmax)
        base2u = 2 * up
        for x in range(xmax + 1):
            if brow[x] <= NEG:
                continue
            miss = M - x
            if miss == 0:
                tail = 0.0
            elif log2_1mp <= NEG:
                continue  # p=1: misses impossible, only x=M survives
            else:
                tail = miss * log2_1mp
            term = cu + brow[x] + x * log2p + tail
            w = base2u + x
            pv = parent[w]
            parent[w] = term if pv <= NEG else (max(pv, term) + math.log2(1 + 2.0 ** (-abs(pv - term))))
    return parent


def run(depth: int, c: int, q_log2: float, w_cap: int, x_cap: int) -> list[float]:
    q = 2.0 ** q_log2
    p = 2.0 / (q - 1.0)
    if p >= 1.0:
        # degenerate small-q case (q<=3): the g=2 upper bound forces a hit on every non-common
        # column. Clamp to p=1 (log2_1mp = -inf): only the all-hit term survives.
        p = 1.0
        log2p = 0.0
        log2_1mp = NEG
    else:
        log2p = math.log2(p)
        log2_1mp = math.log2(1.0 - p)
    # base level 0: replica R0 = 2^depth, codeword length n0 = c, mass q^{R0}-1 at w=0
    R0_log2_count = (1 << depth) * q_log2  # log2(q^{2^depth}) ~ log2(q^{2^depth}-1)
    n0 = c
    A = [NEG] * (n0 + 1)
    A[0] = R0_log2_count
    n_cols = n0  # columns at current level = codeword length
    for h in range(1, depth + 1):
        # parent length = 2 * current; child columns = current codeword length = n_cols
        parent_len = 2 * n_cols
        wcap_h = min(w_cap, parent_len)
        A = step(A, n_cols, log2p, log2_1mp, wcap_h, x_cap)
        n_cols = parent_len
    return A  # A_depth(1, .) over w=0..min(w_cap, n)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--depth", type=int, required=True)
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--security-bits", type=float, default=80.0)
    ap.add_argument("--w-cap", type=int, default=4000, help="max zero-count tracked")
    ap.add_argument("--x-cap", type=int, default=400, help="max hits per fold step")
    ap.add_argument("--print-window", type=int, default=6)
    args = ap.parse_args()

    k = 1 << args.depth
    n = args.expansion * k
    A = run(args.depth, args.expansion, args.q_log2, args.w_cap, args.x_cap)
    wmax = len(A) - 1

    # B(z) = logsumexp_w [ A[w] + log2 binom(w,z) ].  Build B for z=0..wmax.
    # log2 binom(w,z): precompute per w as we go.
    B = [NEG] * (wmax + 1)
    for w in range(wmax + 1):
        if A[w] <= NEG:
            continue
        brow = log2_binom_row(w, w)  # binom(w,z) for z=0..w
        for z in range(w + 1):
            term = A[w] + brow[z]
            pv = B[z]
            B[z] = term if pv <= NEG else (max(pv, term) + math.log2(1 + 2.0 ** (-abs(pv - term))))

    crossing = None
    for z in range(wmax + 1):
        if B[z] <= -args.security_bits:
            crossing = z
            break

    print(f"# UB recurrence: depth={args.depth} k={k} c={args.expansion} n={n} q=2^{args.q_log2:.0f} "
          f"lambda={args.security_bits} w_cap={args.w_cap} x_cap={args.x_cap}")
    print("z,excess,log2_B_upper")
    if crossing is None:
        center = wmax
        note = "NO CROSSING within w_cap (increase --w-cap)"
    else:
        center = crossing
        note = ""
    lo = max(0, center - args.print_window)
    hi = min(wmax, center + args.print_window)
    for z in range(lo, hi + 1):
        bs = "-inf" if B[z] <= NEG else f"{B[z]:.6f}"
        print(f"{z},{z-k},{bs}")
    if crossing is None:
        print(f"# {note}")
    else:
        dist = n - crossing + 1
        print(f"crossing_z={crossing} crossing_excess={crossing-k} "
              f"distance>={dist} rel_dist>={dist/n:.5f} rel_gap_to_MDS={((n-k+1)-dist)/n:.5f}")


if __name__ == "__main__":
    main()
