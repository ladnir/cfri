#!/usr/bin/env python3
"""Sum the matched-plus-extra near-extremizer union model over all m and e."""

from __future__ import annotations

import argparse
import csv
import math
from pathlib import Path


NEG_INF = -math.inf


def log2_add(lhs: float, rhs: float) -> float:
    if lhs == NEG_INF:
        return rhs
    if rhs == NEG_INF:
        return lhs
    if rhs > lhs:
        lhs, rhs = rhs, lhs
    return lhs + math.log2(1.0 + 2.0 ** (rhs - lhs))


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--field-bits", type=float, default=128.0)
    parser.add_argument("--max-extra", type=int, default=-1)
    parser.add_argument("--cumulative-count", action="store_true")
    parser.add_argument(
        "--target-sparse-sum",
        type=int,
        default=-1,
        help="If set, require enough aggregate zeros to beat this target for m+k/m+extra.",
    )
    parser.add_argument(
        "--stride-dimension-bound",
        action="store_true",
        help="Charge q^(floor(extra/(k/m))) for complete extra stride classes.",
    )
    parser.add_argument(
        "--charge-overhead-log2",
        type=float,
        default=0.0,
        help="Additional log2 overhead per extra/charged defect in the near support-pair count.",
    )
    parser.add_argument(
        "--virtual-core-holes-max",
        type=int,
        default=0,
        help="Experimental: allow up to this many holes in the selected core and compensate with extras outside it.",
    )
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    parity_copies = args.total_expansion - 1
    other_copies = parity_copies - 1
    if other_copies <= 0:
        raise SystemExit("--total-expansion must leave at least one other parity copy")

    total_log = NEG_INF
    rows: list[list[str | int]] = []
    for live_bits in range(1, args.depth + 1):
        m = 1 << live_bits
        exact_output = k // m
        extras_available = k - exact_output
        max_extra = extras_available if args.max_extra < 0 else min(args.max_extra, extras_available)
        subtotal = NEG_INF
        worst_extra = 0
        worst_needed_zeros = 0
        worst_dimension_excess = 0
        worst_log = NEG_INF
        cumulative_near_count_log = NEG_INF
        for extra in range(max_extra + 1):
            exact_extra_log = NEG_INF
            max_holes = min(args.virtual_core_holes_max, exact_output, extras_available - extra)
            for holes in range(max_holes + 1):
                exact_extra_log = log2_add(
                    exact_extra_log,
                    log2_comb(exact_output, holes) + log2_comb(extras_available, extra + holes),
                )
            cumulative_near_count_log = log2_add(cumulative_near_count_log, exact_extra_log)
            chosen_extra_log = cumulative_near_count_log if args.cumulative_count else exact_extra_log
            charge_overhead_log = extra * args.charge_overhead_log2
            near_count_log = math.log2(parity_copies) + math.log2(k) + chosen_extra_log + charge_overhead_log
            if args.target_sparse_sum >= 0:
                needed_zeros = max(1, m + exact_output + extra - args.target_sparse_sum + 1)
            else:
                needed_zeros = extra + 1
            dimension_excess = extra // exact_output if args.stride_dimension_bound else 0
            dimension_log = dimension_excess * args.field_bits
            aggregate_tail_log = log2_comb(other_copies * k, needed_zeros) - needed_zeros * args.field_bits
            term = near_count_log + dimension_log + aggregate_tail_log
            subtotal = log2_add(subtotal, term)
            if term > worst_log:
                worst_log = term
                worst_extra = extra
                worst_needed_zeros = needed_zeros
                worst_dimension_excess = dimension_excess
        total_log = log2_add(total_log, subtotal)
        rows.append(
            [
                args.depth,
                k,
                live_bits,
                m,
                max_extra,
                worst_extra,
                worst_needed_zeros,
                worst_dimension_excess,
                f"{worst_log:.8f}",
                f"{subtotal:.8f}",
            ]
        )

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "summary",
                "depth",
                "k",
                "total_expansion",
                "field_bits",
                "max_extra",
                "cumulative_count",
                "target_sparse_sum",
                "stride_dimension_bound",
                "charge_overhead_log2",
                "virtual_core_holes_max",
                "total_log2_union",
            ]
        )
        writer.writerow(
            [
                "total",
                args.depth,
                k,
                args.total_expansion,
                f"{args.field_bits:.8f}",
                args.max_extra,
                int(args.cumulative_count),
                args.target_sparse_sum,
                int(args.stride_dimension_bound),
                f"{args.charge_overhead_log2:.8f}",
                args.virtual_core_holes_max,
                f"{total_log:.8f}",
            ]
        )
        writer.writerow([])
        writer.writerow(
            [
                "depth",
                "k",
                "live_bits",
                "live_rows",
                "max_extra_checked",
                "worst_extra_outputs",
                "worst_needed_zeros",
                "worst_dimension_excess",
                "worst_term_log2",
                "subtotal_log2",
            ]
        )
        writer.writerows(rows)

    print(f"total_log2_union={total_log:.8f} out={args.out}")


if __name__ == "__main__":
    main()
