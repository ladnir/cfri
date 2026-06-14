#!/usr/bin/env python3
"""One-step lift validator (M2 tool).

Isolates whether the recurrence structure + a given singleton-charge rule is correct, by feeding
the lift the EXACT child moments B_{d-1}(2r, u) from the brute-force oracle and comparing the lifted
prediction to the EXACT parent B_d(r, z).  This removes compounding error: any discrepancy is the
one-step transition's fault, not accumulated child error.

Usage:
  python rfc_lift_validator.py --depth 2 --replica 1 --q 3 --expansion 2 --charge component-uniform
"""

from __future__ import annotations

import argparse
import math
import sys
from fractions import Fraction
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_brute_force_moment import brute_force_moment  # noqa: E402
from rfc_replica_zero_moment import (  # noqa: E402
    NEG_INF, log2_add, log2_comb_table, lift_moment,
)


def log2_frac(x: Fraction) -> float:
    if x == 0:
        return NEG_INF
    return math.log2(x.numerator) - math.log2(x.denominator)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--depth", type=int, required=True, help="parent depth d")
    ap.add_argument("--replica", type=int, default=1, help="parent replica r")
    ap.add_argument("--q", type=int, required=True)
    ap.add_argument("--expansion", type=int, default=2)
    ap.add_argument("--charge", default="component-uniform",
                    choices=["loose", "replica", "component-uniform"])
    ap.add_argument("--t-domain", choices=["nonzero", "all"], default="nonzero")
    ap.add_argument("--child-samples", type=int, default=None)
    ap.add_argument("--parent-samples", type=int, default=None)
    args = ap.parse_args()

    d, r, q, c = args.depth, args.replica, args.q, args.expansion
    nonzero = args.t_domain == "nonzero"

    # Exact child B_{d-1}(2r, u) and exact parent B_d(r, z) from the oracle.
    child_mom, child_n, _, child_exact = brute_force_moment(
        d - 1, c, q, nonzero, None, args.child_samples, 1, 2 * r)
    parent_mom, parent_n, _, parent_exact = brute_force_moment(
        d, c, q, nonzero, None, args.parent_samples, 1, r)

    child_log = [log2_frac(x) for x in child_mom]
    qlog = math.log2(q)
    comb = log2_comb_table(parent_n)
    # child_k for this single lift = 2^{d-1}; lift_moment uses q_exact for (q-1),(q-2) factors.
    lifted, _choices = lift_moment(child_log, comb, qlog, q, r, args.charge, 1 << (d - 1))

    print(f"# parent B_{d}({r}, z): oracle vs lift(charge={args.charge}) fed EXACT child B_{d-1}({2*r}, u)")
    print(f"# q={q} expansion={c} child_exact={child_exact} parent_exact={parent_exact}")
    print(f"# child B_{d-1}({2*r},u) exact = {[str(x) for x in child_mom]}")
    print("z,excess,oracle_log2,lift_log2,lift_minus_oracle_bits,safe_upper_bound?")
    k = 1 << d
    worst = None
    for z in range(parent_n + 1):
        o = log2_frac(parent_mom[z])
        l = lifted[z]
        os = "-inf" if o <= NEG_INF / 2 else f"{o:.6f}"
        ls = "-inf" if l <= NEG_INF / 2 else f"{l:.6f}"
        if o <= NEG_INF / 2 and l <= NEG_INF / 2:
            diff = 0.0
        elif o <= NEG_INF / 2:
            diff = float("inf")  # lift positive where truth is 0: very loose but safe
        elif l <= NEG_INF / 2:
            diff = float("-inf")  # lift says 0 where truth positive: UNSAFE
        else:
            diff = l - o
        safe = (l + 1e-9 >= o) if not (o <= NEG_INF / 2) else True
        if not safe and (worst is None or diff < worst):
            worst = diff
        ds = ("+inf" if diff == float("inf")
              else "-inf(UNSAFE)" if diff == float("-inf")
              else f"{diff:+.6f}")
        print(f"{z},{z-k},{os},{ls},{ds},{'yes' if safe else 'NO'}")
    if worst is None:
        print("# VERDICT: lift is a valid upper bound on the exact parent at every z (safe).")
    else:
        print(f"# VERDICT: lift UNDERSHOOTS truth (UNSAFE) by up to {worst:.6f} bits.")


if __name__ == "__main__":
    main()
