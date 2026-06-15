#!/usr/bin/env python3
"""Top-level tolerance check for fixed-survivor root-line repair exponents.

This is not a certificate.  It asks whether a proposed local repair exponent can even beat the
number of top-level survivor profiles with too few paired child positions.
"""

from __future__ import annotations

import argparse
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
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


@dataclass
class DominantProfile:
    log2_sum: float
    log2_term: float
    p: int
    t: int
    d_min: int
    q_exponent: int
    log2_profile_count: float


def local_q_exponent(model: str, d_min: int, t: int, flat_excess: int) -> int:
    if d_min <= 0:
        return 0
    if t < 2 * d_min:
        return 0
    if model == "single_minor":
        return 1
    if model == "surplus_rank":
        return t - 2 * d_min + 1
    if model == "flat_corrected_surplus":
        return max(0, t - 2 * d_min + 1 - flat_excess)
    raise ValueError(model)


def top_profile_count_log2(n_child: int, p: int, t: int) -> float:
    # Choose p paired child positions, then t singleton child positions, then a left/right side
    # for each singleton.
    return log2_comb(n_child, p) + log2_comb(n_child - p, t) + t


def dominant_for_excess(depth: int, expansion: int, q_log2: float, excess: int,
                        model: str, flat_excess: int) -> DominantProfile:
    k = 1 << depth
    k_child = k >> 1
    n_child = expansion * k_child
    survivor_size = k + excess

    total = NEG_INF
    best = DominantProfile(NEG_INF, NEG_INF, 0, 0, 0, 0, NEG_INF)
    # This stress test only accounts for top profiles with p<k_child, where the paired child block
    # has a deterministic cardinality deficit.  Profiles with p>=k_child recurse into child rank
    # tails and are intentionally not counted here.
    for p in range(0, min(k_child, n_child) + 1):
        t = survivor_size - 2 * p
        if t < 0 or t > n_child - p:
            continue
        d_min = max(0, k_child - p)
        if d_min <= 0:
            continue
        profile_log2 = top_profile_count_log2(n_child, p, t)
        q_exp = local_q_exponent(model, d_min, t, flat_excess)
        if q_exp == 0:
            fail_log2 = 0.0
        elif model == "single_minor":
            fail_log2 = math.log2(max(1, d_min)) - q_log2
        else:
            fail_log2 = -q_exp * q_log2
        term = profile_log2 + fail_log2
        total = log2_add(total, term)
        if term > best.log2_term:
            best = DominantProfile(total, term, p, t, d_min, q_exp, profile_log2)
        else:
            best.log2_sum = total
    return best


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--depth", type=int, default=11)
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--model", choices=["single_minor", "surplus_rank", "flat_corrected_surplus"],
                    default="single_minor")
    ap.add_argument("--flat-excess", type=int, default=0,
                    help="q-exponent loss for flat_corrected_surplus")
    ap.add_argument("--min-excess", type=int, default=0)
    ap.add_argument("--max-excess", type=int, default=160)
    ap.add_argument("--target-log2-bad", type=float, default=-1.0,
                    help="default -1 corresponds to expected bad count <= 1/2")
    args = ap.parse_args()

    print(
        "depth,expansion,k,n,q_log2,model,flat_excess,excess,log2_bad_top_deficit,"
        "dominant_p,dominant_t,dominant_D_min,dominant_q_exponent,"
        "dominant_log2_profile_count,dominant_log2_term,status"
    )
    k = 1 << args.depth
    n = args.expansion * k
    first_crossing: tuple[int, float] | None = None
    for excess in range(args.min_excess, args.max_excess + 1):
        dom = dominant_for_excess(
            args.depth, args.expansion, args.q_log2, excess, args.model, args.flat_excess
        )
        status = "pass" if dom.log2_sum <= args.target_log2_bad else "fail"
        if status == "pass" and first_crossing is None:
            first_crossing = (excess, dom.log2_sum)
        print(
            f"{args.depth},{args.expansion},{k},{n},{args.q_log2:g},{args.model},"
            f"{args.flat_excess},{excess},{dom.log2_sum:.6f},{dom.p},{dom.t},{dom.d_min},"
            f"{dom.q_exponent},{dom.log2_profile_count:.6f},{dom.log2_term:.6f},{status}"
        )
    if first_crossing is None:
        print(f"# crossing none up to excess={args.max_excess}")
    else:
        print(f"# crossing excess={first_crossing[0]} log2_bad={first_crossing[1]:.6f}")


if __name__ == "__main__":
    main()
