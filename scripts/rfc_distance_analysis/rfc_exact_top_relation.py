#!/usr/bin/env python3
"""Exact top-level relation for B_d(1,z) — the tight replacement for the binom-placement recurrence.

Derived from the corrected (shared-challenge) encoder.  The depth-d codeword of message m splits into
two half-codewords W_L = enc_{d-1}(m_L), W_R = enc_{d-1}(m_R) under the SAME child code (shared
challenges).  The top fold pairs column jj with column n/2+jj using an independent challenge t_jj:
  left_jj  = W_L[jj] + t_jj * W_R[jj]
  right_jj = W_L[jj] + (t_jj+1) * W_R[jj]
For a single message (R=1), per child column (a,b) = (W_L[jj], W_R[jj]):
  - (0,0): both parent sides zero for every t  -> contributes 2 to the parent zero count (a "common" u).
  - b != 0: left zero iff t = -a/b, right zero iff t = -a/b - 1; these are distinct, each in F^* or
    not.  #good t in F^* is g = [a!=0] + [a!=-b].  Contributes 1 with prob g/(q-1).
  - b == 0, a != 0: never zero (g=0, dead).
So parent_zeros = 2u + X, X = sum of independent Bernoulli(g_jj/(q-1)) over non-common columns.
Hence the EXACT identity (no binom placement double-counting):
  B_d(1,z) = E_childcode[ sum_{m != 0} E_top[ binom(2u + X, z) ] ].

This script computes that RHS by enumerating the child code (levels 0..d-2) and all messages, and
averaging X analytically, then compares to the full brute-force oracle B_d(1,z).  If they match, the
exact top relation is validated and the child statistic the recurrence must propagate is the joint
distribution of (u, m1, m2) over (child code, message), where m1/m2 = #non-common columns with g=1/2.
"""

from __future__ import annotations

import argparse
import itertools
import math
import sys
from fractions import Fraction
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_brute_force_moment import encode, num_challenges, brute_force_moment  # noqa: E402


def x_distribution(m1: int, m2: int, q: int) -> list[Fraction]:
    """Distribution of X = Bin(m1, 1/(q-1)) + Bin(m2, 2/(q-1)) as a list P[x], x=0..m1+m2."""
    p1 = Fraction(1, q - 1)
    p2 = Fraction(2, q - 1)
    dist = [Fraction(1)]
    for _ in range(m1):
        dist = _convolve_bernoulli(dist, p1)
    for _ in range(m2):
        dist = _convolve_bernoulli(dist, p2)
    return dist


def _convolve_bernoulli(dist: list[Fraction], p: Fraction) -> list[Fraction]:
    out = [Fraction(0)] * (len(dist) + 1)
    for x, pr in enumerate(dist):
        out[x] += pr * (1 - p)
        out[x + 1] += pr * p
    return out


def exact_top_prediction(depth: int, c: int, q: int, max_z: int) -> list[Fraction]:
    """Predicted B_d(1,z) via the exact top relation, enumerating child code + messages."""
    k = 1 << depth
    n = c * k
    half_msg = 1 << (depth - 1)
    child_chal_count = num_challenges(depth - 1, c)
    t_dom = list(range(1, q))  # F^* (proof model)
    # binom table
    binom = [[0] * (n + 1) for _ in range(n + 1)]
    for w in range(n + 1):
        binom[w][0] = 1
        for z in range(1, w + 1):
            binom[w][z] = binom[w - 1][z - 1] + binom[w - 1][z]

    messages = [m for m in itertools.product(range(q), repeat=k) if any(m)]
    acc = [Fraction(0)] * (n + 1)
    num_child_codes = 0
    for chal in itertools.product(t_dom, repeat=child_chal_count):
        num_child_codes += 1
        chal = list(chal)
        for m in messages:
            w_l = encode(m[:half_msg], depth - 1, c, q, chal)
            w_r = encode(m[half_msg:], depth - 1, c, q, chal)
            u = 0
            m1 = 0
            m2 = 0
            for a, b in zip(w_l, w_r):
                if a == 0 and b == 0:
                    u += 1
                elif b != 0:
                    g = (1 if a != 0 else 0) + (1 if a != (-b) % q else 0)
                    if g == 1:
                        m1 += 1
                    elif g == 2:
                        m2 += 1
                    # g==0 impossible when b!=0 (a==0 gives [0]+[a!=-b]=1; a==-b!=0 gives 1+0=1)
                # else b==0,a!=0: dead, g=0
            xdist = x_distribution(m1, m2, q)
            for z in range(0, max_z + 1):
                s = Fraction(0)
                for x, pr in enumerate(xdist):
                    w = 2 * u + x
                    if w >= z:
                        s += pr * binom[w][z]
                acc[z] += s
    return [acc[z] / num_child_codes for z in range(max_z + 1)]


def log2_frac(x: Fraction) -> float:
    if x == 0:
        return float("-inf")
    return math.log2(x.numerator) - math.log2(x.denominator)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--depth", type=int, required=True)
    ap.add_argument("--expansion", type=int, default=2)
    ap.add_argument("--q", type=int, required=True)
    args = ap.parse_args()
    d, c, q = args.depth, args.expansion, args.q
    n = c * (1 << d)
    max_z = n

    pred = exact_top_prediction(d, c, q, max_z)
    # full oracle (exact), R=1
    orc, _hist, _n, _nt, _ex = brute_force_moment(d, c, q, True, max_z, None, 0, 1)

    print(f"# exact top relation vs full oracle: depth={d} c={c} q={q} n={n}")
    print("z,exact_top_pred,oracle,match")
    worst = 0.0
    for z in range(0, n + 1):
        p, o = pred[z], orc[z]
        same = (p == o)
        if not same:
            d2 = abs(log2_frac(p) - log2_frac(o)) if (p != 0 and o != 0) else float("inf")
            worst = max(worst, d2)
        ps = log2_frac(p)
        os = log2_frac(o)
        pss = "-inf" if ps == float("-inf") else f"{ps:.6f}"
        oss = "-inf" if os == float("-inf") else f"{os:.6f}"
        print(f"{z},{pss},{oss},{'EXACT' if same else 'DIFF'}")
    print(f"# VERDICT: {'exact top relation reproduces oracle EXACTLY' if worst == 0 else f'mismatch up to {worst:.6f} bits'}")


if __name__ == "__main__":
    main()
