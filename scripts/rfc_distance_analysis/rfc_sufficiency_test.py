#!/usr/bin/env python3
"""Sufficiency test: is (zero-count, rank) a closing statistic for the first-moment fold?

The exact top relation says the parent zero distribution of a fold is determined by the child
column types: per child column jj with a_jj = (left codewords)[jj] in F^R and b_jj = (right
codewords)[jj] in F^R,
  - common  : a_jj = b_jj = 0                      (contributes 2 to every parent zero count; count u)
  - g2       : rank{a_jj,b_jj}=1, lambda not in {0,-1}   (hittable on both roots; count n_g2)
  - g1       : rank{a_jj,b_jj}=1, lambda in {0,-1}        (hittable on one root;  count n_g1)
  - dead     : rank{a_jj,b_jj}=2, OR b_jj=0!=a_jj         (never zero)
The triple (u, n_g1, n_g2) fixes the parent zero-count distribution exactly (= dist of 2u + X,
X = Bin(n_g1,1/(q-1)) + Bin(n_g2,2/(q-1))).

For a tractable recurrence we need a SMALL per-instance summary S that (a) determines (u,n_g1,n_g2)
and (b) itself recurses.  This script tests (a): enumerate instances (child code + 2R-tuple of
depth-h messages, first R = left halves, last R = right halves), compute (u,n_g1,n_g2) and several
candidate summaries S, and report for each S whether (u,n_g1,n_g2) is constant within S-groups
(i.e. whether S is a SUFFICIENT statistic for the parent zero distribution).

Run: python rfc_sufficiency_test.py --depth 1 --replica 1 --q 3
"""

from __future__ import annotations

import argparse
import itertools
import sys
from collections import defaultdict
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_brute_force_moment import encode, num_challenges  # noqa: E402


def mat_rank_modq(rows: list[list[int]], q: int) -> int:
    """Rank over F_q (q prime) of the given rows."""
    M = [row[:] for row in rows]
    nrows = len(M)
    ncols = len(M[0]) if nrows else 0
    r = 0
    for col in range(ncols):
        piv = None
        for i in range(r, nrows):
            if M[i][col] % q != 0:
                piv = i
                break
        if piv is None:
            continue
        M[r], M[piv] = M[piv], M[r]
        inv = pow(M[r][col], q - 2, q)
        M[r] = [(x * inv) % q for x in M[r]]
        for i in range(nrows):
            if i != r and M[i][col] % q != 0:
                f = M[i][col] % q
                M[i] = [(M[i][j] - f * M[r][j]) % q for j in range(ncols)]
        r += 1
        if r == nrows:
            break
    return r


def classify_columns(a_rows: list[list[int]], b_rows: list[list[int]], q: int):
    """Return (u, n_g1, n_g2, n_dead) over columns; a_rows/b_rows are R x ncols."""
    R = len(a_rows)
    ncols = len(a_rows[0]) if R else 0
    u = n_g1 = n_g2 = n_dead = 0
    for jj in range(ncols):
        a = [a_rows[i][jj] % q for i in range(R)]
        b = [b_rows[i][jj] % q for i in range(R)]
        az = all(x == 0 for x in a)
        bz = all(x == 0 for x in b)
        if az and bz:
            u += 1
            continue
        # rank{a,b}: dim span of the two vectors a,b in F^R
        rk = mat_rank_modq([a, b], q)
        if rk >= 2:
            n_dead += 1
            continue
        # rank 1: a,b proportional. find lambda with a = -t b style; classify by g = [a!=0]+[a!=-b]
        # g counts roots t in F^* that zero some side: left t=-a/b (a!=0), right t=-a/b-1 (a!=-b).
        if bz:  # b=0, a!=0 -> never zero
            n_dead += 1
            continue
        a_nonzero = not az
        a_eq_negb = all((a[i] + b[i]) % q == 0 for i in range(R))
        g = (1 if a_nonzero else 0) + (0 if a_eq_negb else 1)
        if g == 2:
            n_g2 += 1
        elif g == 1:
            n_g1 += 1
        else:
            n_dead += 1
    return u, n_g1, n_g2, n_dead


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--depth", type=int, required=True, help="depth h of the child messages")
    ap.add_argument("--replica", type=int, default=1, help="R (parent replica); child is 2R-tuple")
    ap.add_argument("--q", type=int, required=True)
    ap.add_argument("--expansion", type=int, default=2)
    args = ap.parse_args()
    h, R, q, c = args.depth, args.replica, args.q, args.expansion
    twoR = 2 * R
    nh = c * (1 << h)
    chal_count = num_challenges(h, c)
    t_dom = list(range(1, q))
    msgs = list(itertools.product(range(q), repeat=1 << h))

    # candidate summaries -> dict S -> set of (u,n_g1,n_g2) observed
    cand = {"u": defaultdict(set), "u,rank": defaultdict(set),
            "u,rank,nz": defaultdict(set), "full_typehist": defaultdict(set)}
    total = 0
    for chal in itertools.product(t_dom, repeat=chal_count):
        chal = list(chal)
        # codewords of all messages under this code
        cw = {m: encode(m, h, c, q, chal) for m in msgs}
        # enumerate 2R-tuples (first R = left halves a, last R = right halves b); exclude all-zero
        for tup in itertools.product(msgs, repeat=twoR):
            if all(all(x == 0 for x in cw[t]) for t in tup):
                continue
            a_rows = [cw[tup[i]] for i in range(R)]
            b_rows = [cw[tup[R + i]] for i in range(R)]
            u, g1, g2, dead = classify_columns(a_rows, b_rows, q)
            target = (u, g1, g2)
            # the full 2R x nh codeword matrix rank
            rank = mat_rank_modq([cw[t] for t in tup], q)
            nz = sum(1 for jj in range(nh)
                     if any(cw[tup[i]][jj] % q for i in range(twoR)))  # #nonzero (non-common) cols
            cand["u"][(u,)].add(target)
            cand["u,rank"][(u, rank)].add(target)
            cand["u,rank,nz"][(u, rank, nz)].add(target)
            cand["full_typehist"][(u, g1, g2, dead)].add(target)
            total += 1

    print(f"# sufficiency test: depth_h={h} R={R} (child 2R={twoR}-tuple) q={q} c={c} "
          f"n_h={nh} instances={total}")
    print("# a summary S is SUFFICIENT iff (u,n_g1,n_g2) is constant within every S-group.")
    print("summary,distinct_groups,nonconstant_groups,verdict")
    for name, groups in cand.items():
        nonconst = sum(1 for s in groups.values() if len(s) > 1)
        verdict = "SUFFICIENT" if nonconst == 0 else f"INSUFFICIENT ({nonconst} ambiguous)"
        print(f"{name},{len(groups)},{nonconst},{verdict}")


if __name__ == "__main__":
    main()
