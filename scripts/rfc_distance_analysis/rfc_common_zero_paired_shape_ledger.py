#!/usr/bin/env python3
"""Shape-sensitive all-paired ledger for common-zero subcode buckets.

This diagnostic repairs the deliberate unfairness in the paired-envelope stress
test.  The paired envelope used a compressed subcode exponent but kept arbitrary
set entropy.  Here, if Z=P union C is all-paired for m levels, we also count it
with the entropy of choosing z/2^m lower positions.

For the canonical common-zero bucket, h is an exact kernel dimension.  Pure
all-paired compression doubles rank at each lifted level, so it also doubles
kernel dimension.  The default mode therefore enforces h=2^m h'.  The old
ceil(h/2^m) event is kept behind --relaxed-kernel-ceil as a diagnostic for the
looser "kernel at least h" overcount.

The script is still not a certificate: it only models pure all-paired shapes.
Mixed shapes need a full recurrence.
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
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


@dataclass(frozen=True)
class Row:
    h: int
    flat_excess: int
    r: int
    a: int
    z: int
    paired_levels: int
    compressed_depth: int
    compressed_k: int
    compressed_n: int
    compressed_h: int
    compressed_z: int
    subcode_q: int
    root_residual_q: int
    total_q: int
    log2_shape: float
    log2_term: float

    def extra_q_needed(self, q_log2: float, security_bits: float) -> float:
        """Extra q-exponent needed to push this term below 2^-security_bits."""
        return max(0.0, (self.log2_term + security_bits) / q_log2)


def kernel_tail_q_exp(k: int, z: int, h: int) -> int:
    return max(0, h * (z - k + h))


def dominant_surplus_profile(depth: int, expansion: int, excess: int, q_log2: float) -> tuple[int, int, int]:
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


def compressed_params(
    child_depth: int,
    expansion: int,
    z: int,
    h: int,
    paired_levels: int,
    *,
    exact_lift_kernel: bool,
) -> tuple[int, int, int, int, int]:
    cur_depth = child_depth
    cur_z = z
    cur_h = h
    for _ in range(paired_levels):
        if cur_depth <= 0 or cur_z % 2 != 0:
            return -1, -1, -1, -1, -1
        if exact_lift_kernel and cur_h % 2 != 0:
            return -1, -1, -1, -1, -1
        cur_depth -= 1
        cur_z //= 2
        cur_h = cur_h // 2 if exact_lift_kernel else (cur_h + 1) // 2
    cur_k = 1 << cur_depth
    cur_n = expansion * cur_k
    return cur_depth, cur_k, cur_n, cur_z, cur_h


def row_for(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    p: int,
    t: int,
    d: int,
    h: int,
    flat_excess: int,
    paired_levels: int,
    exact_lift_kernel: bool,
) -> Row | None:
    child_depth = depth - 1
    r = d - h
    a = 2 * r + flat_excess
    z = p + a
    if h < 1 or r < 0 or a < 1 or a > t:
        return None
    c_depth, c_k, c_n, c_z, c_h = compressed_params(
        child_depth,
        expansion,
        z,
        h,
        paired_levels,
        exact_lift_kernel=exact_lift_kernel,
    )
    if c_depth < 0 or c_z > c_n:
        return None
    # RFC columns are never the zero vector. A nonempty compressed zero set therefore has
    # rank at least one, so its kernel dimension is at most k-1. The random-matrix tail
    # would charge the zero-column event by q^-z, but that event is structurally impossible.
    if c_z > 0 and c_h >= c_k:
        return None

    subcode_q = kernel_tail_q_exp(c_k, c_z, c_h)
    root_residual_q = max(0, t - a - 2 * h + 1)
    total_q = subcode_q + root_residual_q

    # Shape-sensitive entropy for pure all-paired Z=P union C:
    # choose the compressed lower zero set, then choose which a of the z lifted child
    # positions are C rather than P, then choose the remaining singleton positions and sides.
    n_child = expansion * (1 << child_depth)
    log2_shape = (
        log2_comb(c_n, c_z)
        + log2_comb(z, a)
        + log2_comb(n_child - z, t - a)
        + t
    )
    log2_term = log2_shape - total_q * q_log2
    return Row(
        h=h,
        flat_excess=flat_excess,
        r=r,
        a=a,
        z=z,
        paired_levels=paired_levels,
        compressed_depth=c_depth,
        compressed_k=c_k,
        compressed_n=c_n,
        compressed_h=c_h,
        compressed_z=c_z,
        subcode_q=subcode_q,
        root_residual_q=root_residual_q,
        total_q=total_q,
        log2_shape=log2_shape,
        log2_term=log2_term,
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
    parser.add_argument("--min-h", type=int, default=1)
    parser.add_argument("--max-h", type=int, default=None)
    parser.add_argument("--min-flat-excess", type=int, default=1)
    parser.add_argument("--max-flat-excess", type=int, default=None)
    parser.add_argument("--max-paired-levels", type=int, default=None)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument(
        "--relaxed-kernel-ceil",
        action="store_true",
        help=(
            "Use the old ceil(h/2^m) compression for a noncanonical dim>=h event. "
            "By default the ledger enforces exact all-paired lifting h=2^m h'."
        ),
    )
    parser.add_argument("--top", type=int, default=12)
    args = parser.parse_args()

    if args.p is None or args.t is None or args.D is None:
        p, t, d = dominant_surplus_profile(args.depth, args.expansion, args.excess, args.q_log2)
    else:
        p, t, d = args.p, args.t, args.D

    surplus = t - 2 * d + 1
    max_h = d if args.max_h is None else min(args.max_h, d)
    max_f = surplus if args.max_flat_excess is None else args.max_flat_excess
    max_levels = args.depth - 1 if args.max_paired_levels is None else args.max_paired_levels

    rows: list[Row] = []
    total = NEG_INF
    for h in range(args.min_h, max_h + 1):
        for flat_excess in range(args.min_flat_excess, max_f + 1):
            for paired_levels in range(max_levels + 1):
                row = row_for(
                    depth=args.depth,
                    expansion=args.expansion,
                    q_log2=args.q_log2,
                    p=p,
                    t=t,
                    d=d,
                    h=h,
                    flat_excess=flat_excess,
                    paired_levels=paired_levels,
                    exact_lift_kernel=not args.relaxed_kernel_ceil,
                )
                if row is None:
                    continue
                rows.append(row)
                total = log2_add(total, row.log2_term)

    rows.sort(key=lambda row: row.log2_term, reverse=True)
    print(
        "summary,depth,expansion,q_log2,excess,p,t,D,S,row_count,log2_sum,"
        "dominant_h,dominant_flat_excess,dominant_paired_levels,dominant_z,"
        "dominant_compressed_depth,dominant_compressed_z,dominant_compressed_h,"
        "dominant_total_q,dominant_log2_shape,dominant_log2_term,"
        "extra_q_needed_for_sum,dominant_extra_q_needed"
    )
    if not rows:
        print(
            f"summary,{args.depth},{args.expansion},{args.q_log2:g},{args.excess},"
            f"{p},{t},{d},{surplus},0,-inf,,,,,,,,,,0,0"
        )
        return

    best = rows[0]
    extra_sum = max(0.0, (total + args.security_bits) / args.q_log2)
    extra_best = best.extra_q_needed(args.q_log2, args.security_bits)
    print(
        f"summary,{args.depth},{args.expansion},{args.q_log2:g},{args.excess},"
        f"{p},{t},{d},{surplus},{len(rows)},{total:.6f},"
        f"{best.h},{best.flat_excess},{best.paired_levels},{best.z},"
        f"{best.compressed_depth},{best.compressed_z},{best.compressed_h},"
        f"{best.total_q},{best.log2_shape:.6f},{best.log2_term:.6f},"
        f"{extra_sum:.6f},{extra_best:.6f}"
    )
    print(
        "rank,h,flat_excess,r,a,z,paired_levels,compressed_depth,compressed_z,"
        "compressed_h,subcode_q,root_residual_q,total_q,log2_shape,log2_term,"
        "extra_q_needed"
    )
    for rank, row in enumerate(rows[: max(0, args.top)], start=1):
        print(
            f"{rank},{row.h},{row.flat_excess},{row.r},{row.a},{row.z},"
            f"{row.paired_levels},{row.compressed_depth},{row.compressed_z},"
            f"{row.compressed_h},{row.subcode_q},{row.root_residual_q},"
            f"{row.total_q},{row.log2_shape:.6f},{row.log2_term:.6f},"
            f"{row.extra_q_needed(args.q_log2, args.security_bits):.6f}"
        )


if __name__ == "__main__":
    main()
