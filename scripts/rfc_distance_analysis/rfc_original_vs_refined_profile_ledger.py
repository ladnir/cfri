#!/usr/bin/env python3
"""Ledger comparing original repair accounting to explicit refined buckets.

This is not a certificate. It is an honesty check for incremental upgrades:
for a fixed top survivor profile (p,t,D), compare the original one-minor repair
row against explicitly enumerated refined h-dimensional common-zero buckets.

The script deliberately reports two baselines:

  old_full_profile:
      Original proof cost for the whole (p,t,D) profile.

  old_same_bucket_enumerated:
      What the original one-minor cost would be if we also enumerated the same
      common-zero witness C. This is useful for local apples-to-apples
      comparison, but it is not the original global proof.

A refined bucket is a useful local upgrade only if it beats
old_same_bucket_enumerated. It is a plausible global refinement only if it is
also small relative to old_full_profile and we can prove a restricted
complement bound. The script does not fake that complement subtraction.
"""

from __future__ import annotations

import argparse
import math
from dataclasses import dataclass


NEG_INF = -1.0e300


@dataclass(frozen=True)
class LedgerRow:
    flat_excess: int
    h: int
    r: int
    a: int
    z_child: int
    subcode_q: int
    root_residual_q: int
    total_q: int
    bucket_profile: float
    old_same: float
    refined: float
    gain_vs_same: float
    refined_minus_old: float
    local_status: str
    global_status: str


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


def dominant_surplus_profile(
    *, depth: int, expansion: int, excess: int, q_log2: float
) -> tuple[int, int, int]:
    k = 1 << depth
    k_child = k >> 1
    n_child = expansion * k_child
    survivor_size = k + excess
    best_score = NEG_INF
    best = (0, 0, 0)
    for p in range(min(k_child, n_child) + 1):
        t = survivor_size - 2 * p
        if t < 0 or t > n_child - p:
            continue
        d = max(0, k_child - p)
        if d <= 0 or t < 2 * d:
            continue
        profile = log2_comb(n_child, p) + log2_comb(n_child - p, t) + t
        surplus = t - 2 * d + 1
        score = profile - q_log2 * surplus
        if score > best_score:
            best_score = score
            best = (p, t, d)
    return best


def profile_log2(n_child: int, p: int, t: int) -> float:
    return log2_comb(n_child, p) + log2_comb(n_child - p, t) + t


def kernel_tail_q_exp(k: int, z: int, h: int) -> int:
    return max(0, h * (z - k + h))


def subcode_q_exp(model: str, *, depth: int, z: int, h: int) -> int:
    if model == "random":
        return kernel_tail_q_exp(1 << depth, z, h)
    if model != "paired_envelope":
        raise ValueError(model)

    best = kernel_tail_q_exp(1 << depth, z, h)
    cur_depth = depth
    cur_z = z
    cur_h = h
    while cur_depth > 0 and cur_z % 2 == 0:
        cur_depth -= 1
        cur_z //= 2
        cur_h = (cur_h + 1) // 2
        best = min(best, kernel_tail_q_exp(1 << cur_depth, cur_z, cur_h))
    return best


def refined_h_log2(
    *,
    child_depth: int,
    n_child: int,
    p: int,
    t: int,
    d: int,
    h: int,
    flat_excess: int,
    q_log2: float,
    subcode_model: str,
) -> tuple[int, int, int, int, int, int, float, float]:
    r = d - h
    a = 2 * r + flat_excess
    z_child = p + a
    if h < 1 or r < 0 or a < 1 or a > t or z_child > n_child:
        return r, a, z_child, 0, 0, 0, NEG_INF, NEG_INF

    subcode_q = subcode_q_exp(subcode_model, depth=child_depth, z=z_child, h=h)
    root_residual_q = max(0, t - a - 2 * h + 1)
    total_q = subcode_q + root_residual_q
    bucket_profile = (
        log2_comb(n_child, p)
        + log2_comb(n_child - p, a)
        + log2_comb(n_child - p - a, t - a)
        + t
    )
    refined = bucket_profile - total_q * q_log2
    return r, a, z_child, subcode_q, root_residual_q, total_q, bucket_profile, refined


def row_status(refined: float, refined_minus_old: float, gain_vs_same: float) -> tuple[str, str]:
    local_status = "win" if gain_vs_same > 0 else "loss"
    if refined <= NEG_INF / 2:
        global_status = "invalid_bucket"
    elif refined_minus_old <= -80:
        global_status = "cheap_bucket_needs_complement_bound"
    elif refined_minus_old <= 0:
        global_status = "bucket_below_old_full_needs_complement_bound"
    else:
        global_status = "bucket_exceeds_old_full_discard_for_now"
    return local_status, global_status


def print_row(
    *,
    args: argparse.Namespace,
    k_child: int,
    n_child: int,
    p: int,
    t: int,
    d: int,
    surplus: int,
    whole_profile: float,
    old_local: float,
    old_full: float,
    row: LedgerRow,
) -> None:
    print(
        f"{args.depth},{args.expansion},{args.q_log2:g},{args.subcode_model},{args.excess},"
        f"{k_child},{n_child},{p},{t},{d},{surplus},{row.flat_excess},"
        f"{row.h},{row.r},{row.a},{row.z_child},{row.subcode_q},"
        f"{row.root_residual_q},{row.total_q},"
        f"{whole_profile:.6f},{old_local:.6f},{old_full:.6f},"
        f"{row.bucket_profile:.6f},{row.old_same:.6f},{row.refined:.6f},"
        f"{row.gain_vs_same:.6f},{row.refined_minus_old:.6f},"
        f"{row.local_status},{row.global_status},"
        "1,no_gain_under_hall_only"
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--depth", type=int, default=11)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--excess", type=int, default=71)
    parser.add_argument("--p", type=int, default=None)
    parser.add_argument("--t", type=int, default=None)
    parser.add_argument("--D", type=int, default=None)
    parser.add_argument("--h", type=int, action="append", default=None)
    parser.add_argument("--all-h", action="store_true")
    parser.add_argument("--min-h", type=int, default=1)
    parser.add_argument("--max-h", type=int, default=None)
    parser.add_argument("--flat-excess", type=int, action="append", default=None)
    parser.add_argument("--min-flat-excess", type=int, default=1)
    parser.add_argument("--max-flat-excess", type=int, default=None)
    parser.add_argument("--summary-only", action="store_true")
    parser.add_argument("--top", type=int, default=10)
    parser.add_argument("--subcode-model", choices=["random", "paired_envelope"], default="random")
    args = parser.parse_args()

    k_parent = 1 << args.depth
    k_child = k_parent >> 1
    n_child = args.expansion * k_child
    if args.p is None or args.t is None or args.D is None:
        p, t, d = dominant_surplus_profile(
            depth=args.depth,
            expansion=args.expansion,
            excess=args.excess,
            q_log2=args.q_log2,
        )
    else:
        p, t, d = args.p, args.t, args.D

    whole_profile = profile_log2(n_child, p, t)
    old_local = math.log2(max(1, d)) - args.q_log2
    old_full = whole_profile + old_local
    surplus = t - 2 * d + 1

    if args.all_h:
        max_h = d if args.max_h is None else min(args.max_h, d)
        hs = list(range(max(1, args.min_h), max_h + 1))
    else:
        hs = args.h if args.h is not None else [1]

    if args.flat_excess is not None:
        flat_excesses = args.flat_excess
    else:
        max_flat_excess = surplus if args.max_flat_excess is None else args.max_flat_excess
        flat_excesses = list(range(args.min_flat_excess, max_flat_excess + 1))

    header = (
        "depth,expansion,q_log2,subcode_model,excess,k_child,n_child,p,t,D,S,flat_excess,"
        "h,r,a,z_child,subcode_q_exp,root_residual_q_exp,total_q_exp,"
        "old_full_profile_log2,old_local_log2,old_full_log2,"
        "bucket_profile_log2,old_same_bucket_log2,refined_bucket_log2,"
        "gain_vs_old_same_bucket_bits,refined_minus_old_full_bits,"
        "local_bucket_status,global_refinement_status,"
        "complement_min_q_exp_under_hall,complement_verdict"
    )
    if not args.summary_only:
        print(header)

    rows: list[LedgerRow] = []
    for h in hs:
        for flat_excess in flat_excesses:
            r, a, z_child, subcode_q, root_residual_q, total_q, bucket_profile, refined = refined_h_log2(
                child_depth=args.depth - 1,
                n_child=n_child,
                p=p,
                t=t,
                d=d,
                h=h,
                flat_excess=flat_excess,
                q_log2=args.q_log2,
                subcode_model=args.subcode_model,
            )
            old_same = bucket_profile + old_local if bucket_profile > NEG_INF / 2 else NEG_INF
            gain_vs_same = old_same - refined if refined > NEG_INF / 2 else NEG_INF
            refined_minus_old = refined - old_full if refined > NEG_INF / 2 else NEG_INF
            local_status, global_status = row_status(refined, refined_minus_old, gain_vs_same)
            row = LedgerRow(
                flat_excess=flat_excess,
                h=h,
                r=r,
                a=a,
                z_child=z_child,
                subcode_q=subcode_q,
                root_residual_q=root_residual_q,
                total_q=total_q,
                bucket_profile=bucket_profile,
                old_same=old_same,
                refined=refined,
                gain_vs_same=gain_vs_same,
                refined_minus_old=refined_minus_old,
                local_status=local_status,
                global_status=global_status,
            )
            rows.append(row)
            if not args.summary_only:
                print_row(
                    args=args,
                    k_child=k_child,
                    n_child=n_child,
                    p=p,
                    t=t,
                    d=d,
                    surplus=surplus,
                    whole_profile=whole_profile,
                    old_local=old_local,
                    old_full=old_full,
                    row=row,
                )

    valid_rows = [row for row in rows if row.refined > NEG_INF / 2]
    valid_rows.sort(key=lambda row: row.refined, reverse=True)
    refined_logsum = NEG_INF
    old_same_logsum = NEG_INF
    for row in valid_rows:
        refined_logsum = log2_add(refined_logsum, row.refined)
        old_same_logsum = log2_add(old_same_logsum, row.old_same)
    refined_total_minus_old_full = (
        refined_logsum - old_full if refined_logsum > NEG_INF / 2 else NEG_INF
    )
    total_gain_vs_old_same = (
        old_same_logsum - refined_logsum
        if old_same_logsum > NEG_INF / 2 and refined_logsum > NEG_INF / 2
        else NEG_INF
    )
    if not args.summary_only:
        print(
            f"# refined_bucket_logsum={refined_logsum:.6f} "
            f"old_same_bucket_logsum={old_same_logsum:.6f} "
            f"gain_vs_old_same_logsum={total_gain_vs_old_same:.6f} "
            f"refined_logsum_minus_old_full={refined_total_minus_old_full:.6f}"
        )
        return

    print(
        "summary,depth,expansion,q_log2,subcode_model,excess,k_child,n_child,p,t,D,S,"
        "old_full_profile_log2,old_local_log2,old_full_log2,"
        "old_same_bucket_logsum,refined_bucket_logsum,"
        "gain_vs_old_same_logsum,refined_logsum_minus_old_full,"
        "h_count,flat_excess_count,valid_bucket_count,"
        "dominant_h,dominant_flat_excess,dominant_r,dominant_a,"
        "dominant_z_child,dominant_total_q_exp,dominant_refined_log2,"
        "dominant_gain_vs_old_same_bits,dominant_refined_minus_old_full_bits,"
        "dominant_status,complement_verdict"
    )
    if not valid_rows:
        print(
            f"summary,{args.depth},{args.expansion},{args.q_log2:g},{args.subcode_model},{args.excess},"
            f"{k_child},{n_child},{p},{t},{d},{surplus},"
            f"{whole_profile:.6f},{old_local:.6f},{old_full:.6f},"
            "-inf,-inf,-inf,-inf,"
            f"{len(hs)},{len(flat_excesses)},0,,,,,,,,,no_valid_bucket,"
            "no_gain_under_hall_only"
        )
        return

    best = valid_rows[0]
    print(
        f"summary,{args.depth},{args.expansion},{args.q_log2:g},{args.subcode_model},{args.excess},"
        f"{k_child},{n_child},{p},{t},{d},{surplus},"
        f"{whole_profile:.6f},{old_local:.6f},{old_full:.6f},"
        f"{old_same_logsum:.6f},{refined_logsum:.6f},"
        f"{total_gain_vs_old_same:.6f},{refined_total_minus_old_full:.6f},"
        f"{len(hs)},{len(flat_excesses)},{len(valid_rows)},"
        f"{best.h},{best.flat_excess},{best.r},{best.a},"
        f"{best.z_child},{best.total_q},{best.refined:.6f},"
        f"{best.gain_vs_same:.6f},{best.refined_minus_old:.6f},"
        f"{best.global_status},no_gain_under_hall_only"
    )
    print(
        "rank,flat_excess,h,r,a,z_child,total_q_exp,refined_log2,"
        "gain_vs_old_same_bits,refined_minus_old_full_bits,global_status"
    )
    for rank, row in enumerate(valid_rows[: max(0, args.top)], start=1):
        print(
            f"{rank},{row.flat_excess},{row.h},{row.r},{row.a},"
            f"{row.z_child},{row.total_q},{row.refined:.6f},"
            f"{row.gain_vs_same:.6f},{row.refined_minus_old:.6f},"
            f"{row.global_status}"
        )


if __name__ == "__main__":
    main()
