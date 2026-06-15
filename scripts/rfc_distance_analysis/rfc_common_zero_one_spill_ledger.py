#!/usr/bin/env python3
"""One-spill mixed-shape ledger for common-zero subcode buckets.

This is the next diagnostic after the pure all-paired ledger.  A shape is paired
for `lift_levels` below one spill level.  At the spill level, the compressed zero
set has:

  * `p0` paired lower positions, contributing two sibling coordinates each;
  * `s0` singleton spill coordinates, contributing one sibling coordinate each.

The original zero set size and kernel dimension are exact lifts:

    z = 2^lift_levels * (2*p0 + s0)
    h = 2^lift_levels * h0

The subcode-zero exponent is a one-step exact-rank random-matrix model at the
spill level:

    D0(p0-k1+D0) + h0(s0-2D0+h0),

where `D0` is the exact lower paired-kernel dimension.  This is not a final RFC
certificate; it is a stress test for whether the first mixed branch is already
safe once exact kernel lifting is enforced.
"""

from __future__ import annotations

import argparse
import math
from dataclasses import dataclass
from functools import lru_cache

from rfc_tensor_triple_segre_classifier import INF, triple_codim


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


@lru_cache(maxsize=None)
def triple_structural_min_profile(depth: int, expansion: int) -> tuple[int, int]:
    n = expansion * (1 << depth)
    counts: dict[int, int] = {}
    for c0 in range(n):
        for c1 in range(c0 + 1, n):
            for c2 in range(c1 + 1, n):
                codim, _free_level, _level_costs = triple_codim(depth, expansion, (c0, c1, c2))
                if codim >= INF:
                    continue
                counts[codim] = counts.get(codim, 0) + 1
    if not counts:
        return -1, 0
    gamma = min(counts)
    return gamma, counts[gamma]


@dataclass(frozen=True)
class Row:
    h: int
    flat_excess: int
    r: int
    a: int
    z: int
    lift_levels: int
    spill_depth: int
    h0: int
    z0: int
    p0: int
    s0: int
    d0: int
    lower_q: int
    lower_log2_count: float
    spill_q: int
    root_residual_q: int
    total_q: int
    lower_model: str
    label_model: str
    label_log2_count: float
    log2_shape: float
    log2_term: float

    def extra_q_needed(self, q_log2: float, security_bits: float) -> float:
        return max(0.0, (self.log2_term + security_bits) / q_log2)


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
    lift_levels: int,
    p0: int,
    s0: int,
    d0: int,
    triple_rank_codim_override: int | None,
    triple_rank_structural_profile: bool,
    label_model: str,
) -> Row | None:
    child_depth = depth - 1
    spill_depth = child_depth - lift_levels
    if spill_depth <= 0:
        return None

    lift = 1 << lift_levels
    if h % lift != 0:
        return None
    h0 = h // lift

    r = d - h
    a = 2 * r + flat_excess
    z = p + a
    if h < 1 or r < 0 or a < 1 or a > t:
        return None
    if z % lift != 0:
        return None
    z0 = z // lift
    if 2 * p0 + s0 != z0 or s0 < 1:
        return None

    k1 = 1 << (spill_depth - 1)
    n1 = expansion * k1
    if p0 < 0 or s0 < 0 or p0 + s0 > n1:
        return None
    if d0 < 0 or d0 > k1:
        return None

    # Exact lower paired-kernel dimension D0 is possible only when the lower
    # restriction can have rank k1-D0 using p0 columns.
    if p0 < k1 - d0:
        return None
    # RFC columns are nonzero, so a nonempty lower paired set cannot have rank 0.
    if p0 > 0 and d0 >= k1:
        return None

    # Exact spill kernel h0 inside K_P^2 requires singleton rank 2D0-h0.
    if h0 < 0 or h0 > 2 * d0:
        return None
    if s0 < 2 * d0 - h0:
        return None

    lower_q = d0 * (p0 - k1 + d0)
    lower_model = "random_rank"
    lower_log2_count = log2_comb(n1, p0)
    if triple_rank_codim_override is not None and p0 == 3 and k1 - d0 == 2:
        lower_q = triple_rank_codim_override
        lower_model = "triple_override"
    if triple_rank_structural_profile and p0 == 3 and k1 - d0 == 2:
        profile_depth = spill_depth - 1
        gamma, count = triple_structural_min_profile(profile_depth, expansion)
        if gamma < 0:
            return None
        lower_q = gamma
        lower_log2_count = math.log2(count)
        lower_model = "triple_structural"
    spill_q = h0 * (s0 - 2 * d0 + h0)
    root_residual_q = t - a - 2 * h + 1
    if lower_q < 0 or spill_q < 0 or root_residual_q < 0:
        return None
    total_q = lower_q + spill_q + root_residual_q

    if label_model == "arbitrary":
        label_log2_count = log2_comb(z, a)
    elif label_model == "block_constant":
        lift_blocks = z // lift
        if a % lift != 0:
            return None
        label_log2_count = log2_comb(lift_blocks, a // lift)
    else:
        raise ValueError(f"unknown label model {label_model}")

    n_child = expansion * (1 << child_depth)
    log2_shape = (
        lower_log2_count
        + log2_comb(n1 - p0, s0)
        + s0
        + label_log2_count
        + log2_comb(n_child - z, t - a)
        + t
    )
    log2_term = log2_shape - q_log2 * total_q
    return Row(
        h=h,
        flat_excess=flat_excess,
        r=r,
        a=a,
        z=z,
        lift_levels=lift_levels,
        spill_depth=spill_depth,
        h0=h0,
        z0=z0,
        p0=p0,
        s0=s0,
        d0=d0,
        lower_q=lower_q,
        lower_log2_count=lower_log2_count,
        spill_q=spill_q,
        root_residual_q=root_residual_q,
        total_q=total_q,
        lower_model=lower_model,
        label_model=label_model,
        label_log2_count=label_log2_count,
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
    parser.add_argument("--min-lift-levels", type=int, default=0)
    parser.add_argument("--max-lift-levels", type=int, default=None)
    parser.add_argument("--max-spill-singletons", type=int, default=None)
    parser.add_argument(
        "--triple-rank-codim-override",
        type=int,
        default=None,
        help=(
            "Override the lower q-exponent for p0=3, rank=2 compressed triple events. "
            "Use this to stress the one-spill row with tensor-specific codimension estimates."
        ),
    )
    parser.add_argument(
        "--triple-rank-structural-profile",
        action="store_true",
        help=(
            "For p0=3, rank=2 lower events, replace arbitrary triple entropy by "
            "the count of triples with minimum Segre-line codimension."
        ),
    )
    parser.add_argument(
        "--label-model",
        choices=["arbitrary", "block_constant"],
        default="arbitrary",
        help=(
            "How to count top P/C labels inside the lifted one-spill zero set. "
            "block_constant requires each 2^lift-level block to be entirely P or entirely C."
        ),
    )
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--top", type=int, default=12)
    args = parser.parse_args()

    if args.p is None or args.t is None or args.D is None:
        p, t, d = dominant_surplus_profile(args.depth, args.expansion, args.excess, args.q_log2)
    else:
        p, t, d = args.p, args.t, args.D

    surplus = t - 2 * d + 1
    max_h = d if args.max_h is None else min(args.max_h, d)
    max_f = surplus if args.max_flat_excess is None else min(args.max_flat_excess, surplus)
    child_depth = args.depth - 1
    max_lift = child_depth - 1 if args.max_lift_levels is None else min(args.max_lift_levels, child_depth - 1)

    rows: list[Row] = []
    total = NEG_INF
    for lift_levels in range(args.min_lift_levels, max_lift + 1):
        lift = 1 << lift_levels
        spill_depth = child_depth - lift_levels
        k1 = 1 << (spill_depth - 1)
        n1 = args.expansion * k1
        h_start = ((args.min_h + lift - 1) // lift) * lift
        for h in range(h_start, max_h + 1, lift):
            h0 = h // lift
            for flat_excess in range(args.min_flat_excess, max_f + 1):
                r = d - h
                a = 2 * r + flat_excess
                z = p + a
                if r < 0 or a < 1 or a > t or z % lift != 0:
                    continue
                z0 = z // lift
                max_s0 = z0 if args.max_spill_singletons is None else min(args.max_spill_singletons, z0)
                for s0 in range(1, max_s0 + 1):
                    rest = z0 - s0
                    if rest < 0 or rest % 2 != 0:
                        continue
                    p0 = rest // 2
                    if p0 + s0 > n1:
                        continue
                    d0_min = max((h0 + 1) // 2, k1 - p0)
                    d0_max = min(k1, (h0 + s0) // 2)
                    for d0 in range(d0_min, d0_max + 1):
                        row = row_for(
                            depth=args.depth,
                            expansion=args.expansion,
                            q_log2=args.q_log2,
                            p=p,
                            t=t,
                            d=d,
                            h=h,
                            flat_excess=flat_excess,
                            lift_levels=lift_levels,
                            p0=p0,
                            s0=s0,
                            d0=d0,
                            triple_rank_codim_override=args.triple_rank_codim_override,
                            triple_rank_structural_profile=args.triple_rank_structural_profile,
                            label_model=args.label_model,
                        )
                        if row is None:
                            continue
                        rows.append(row)
                        total = log2_add(total, row.log2_term)

    rows.sort(key=lambda row: row.log2_term, reverse=True)
    print(
        "summary,depth,expansion,q_log2,excess,p,t,D,S,row_count,log2_sum,"
        "dominant_h,dominant_flat_excess,dominant_lift_levels,dominant_z,"
        "dominant_spill_depth,dominant_h0,dominant_z0,dominant_p0,dominant_s0,"
        "dominant_d0,dominant_lower_model,dominant_label_model,dominant_total_q,"
        "dominant_log2_shape,dominant_log2_term,"
        "extra_q_needed_for_sum,dominant_extra_q_needed"
    )
    if not rows:
        print(
            f"summary,{args.depth},{args.expansion},{args.q_log2:g},{args.excess},"
            f"{p},{t},{d},{surplus},0,-inf,,,,,,,,,,,,,,0,0"
        )
        return

    best = rows[0]
    extra_sum = max(0.0, (total + args.security_bits) / args.q_log2)
    extra_best = best.extra_q_needed(args.q_log2, args.security_bits)
    print(
        f"summary,{args.depth},{args.expansion},{args.q_log2:g},{args.excess},"
        f"{p},{t},{d},{surplus},{len(rows)},{total:.6f},"
        f"{best.h},{best.flat_excess},{best.lift_levels},{best.z},"
        f"{best.spill_depth},{best.h0},{best.z0},{best.p0},{best.s0},"
        f"{best.d0},{best.lower_model},{best.label_model},{best.total_q},"
        f"{best.log2_shape:.6f},{best.log2_term:.6f},"
        f"{extra_sum:.6f},{extra_best:.6f}"
    )
    print(
        "rank,h,flat_excess,r,a,z,lift_levels,spill_depth,h0,z0,p0,s0,d0,"
        "lower_model,lower_q,lower_log2_count,spill_q,root_residual_q,total_q,"
        "label_model,label_log2_count,log2_shape,log2_term,extra_q_needed"
    )
    for rank, row in enumerate(rows[: max(0, args.top)], start=1):
        print(
            f"{rank},{row.h},{row.flat_excess},{row.r},{row.a},{row.z},"
            f"{row.lift_levels},{row.spill_depth},{row.h0},{row.z0},"
            f"{row.p0},{row.s0},{row.d0},{row.lower_model},{row.lower_q},"
            f"{row.lower_log2_count:.6f},{row.spill_q},"
            f"{row.root_residual_q},{row.total_q},{row.label_model},"
            f"{row.label_log2_count:.6f},{row.log2_shape:.6f},"
            f"{row.log2_term:.6f},{row.extra_q_needed(args.q_log2, args.security_bits):.6f}"
        )


if __name__ == "__main__":
    main()
