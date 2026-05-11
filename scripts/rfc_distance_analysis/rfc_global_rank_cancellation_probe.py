#!/usr/bin/env python3
"""Probe global rank-budget cancellation accounting on small RFC supports.

This is a diagnostic, not a proof.  For a fixed matched row block and a virtual output support
Y=(C\\H) union E, it finds final output coordinates that look like *excess*
sibling-cancellation zeros in the output tree: at some node, exactly one sibling output is present,
but the absent sibling is not part of the selected exact matched core's own cancellation pattern
and is not a virtual hole inside the selected core.  It then adds those cancelled siblings back to
form a relaxed support Y_plus and measures how many independent zero constraints the cancelled
siblings impose:

    dim ker(A[R, complement(Y_plus)]) - dim ker(A[R, complement(Y)]).

The global-rank-budget heuristic predicts this contribution should cover the number of such
cancellations after the final admissible projective dimension is spent.
"""

from __future__ import annotations

import argparse
import csv
import itertools
import random
from pathlib import Path

from rfc_near_pair_kernel_dim import rank_rows_columns
from sample_rfc_rank_failure import rfc_generator_prime


def matched_rows(block_start: int, m: int) -> tuple[int, ...]:
    return tuple(range(block_start, block_start + m))


def matched_core(k: int, m: int, residue: int) -> tuple[int, ...]:
    return tuple(range(residue, k, m))


def tree_cancellation_zeros(k: int, support: set[int]) -> set[int]:
    """Return absent final leaves whose sibling at some tree node is present."""
    cancelled: set[int] = set()

    def visit(start: int, size: int) -> None:
        if size <= 1:
            return
        half = size // 2
        for offset in range(half):
            left = start + offset
            right = start + half + offset
            left_live = left in support
            right_live = right in support
            if left_live ^ right_live:
                cancelled.add(right if left_live else left)
        visit(start, half)
        visit(start + half, half)

    visit(0, k)
    return cancelled


def kernel_dim(
    generator: list[list[int]],
    rows: tuple[int, ...],
    outputs: set[int],
    k: int,
    prime: int,
) -> int:
    zero_columns = tuple(column for column in range(k) if column not in outputs)
    rank = rank_rows_columns(generator, rows, zero_columns, prime)
    return len(rows) - rank


def limited_combinations(
    values: tuple[int, ...],
    choose: int,
    limit: int,
    rng: random.Random,
    randomize: bool,
) -> list[tuple[int, ...]]:
    if choose == 0:
        return [()]
    if choose > len(values):
        return []
    if randomize:
        out_set: set[tuple[int, ...]] = set()
        attempts = 0
        max_attempts = max(100, 20 * limit)
        while len(out_set) < limit and attempts < max_attempts:
            out_set.add(tuple(sorted(rng.sample(values, choose))))
            attempts += 1
        return sorted(out_set)
    out: list[tuple[int, ...]] = []
    for combo in itertools.combinations(values, choose):
        out.append(combo)
        if limit > 0 and len(out) >= limit:
            break
    return out


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=4)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--live-bits", type=int, default=-1)
    parser.add_argument("--max-extra", type=int, default=2)
    parser.add_argument("--max-holes", type=int, default=2)
    parser.add_argument("--combo-limit", type=int, default=200)
    parser.add_argument("--case-limit", type=int, default=0)
    parser.add_argument("--random-combos", action="store_true")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    rng = random.Random(args.seed)
    generator = rfc_generator_prime(args.depth, 1, args.prime, rng)
    live_bits_values = [args.live_bits] if args.live_bits >= 0 else list(range(1, args.depth + 1))
    rows_out: list[list[int | str]] = []
    failures = 0
    checked = 0
    worst_gap = 0

    for live_bits in live_bits_values:
        m = 1 << live_bits
        for block_start in range(0, k, m):
            rows = matched_rows(block_start, m)
            for residue in range(m):
                core = matched_core(k, m, residue)
                core_set = set(core)
                outside = tuple(column for column in range(k) if column not in core_set)
                for h in range(args.max_holes + 1):
                    for holes in limited_combinations(core, h, args.combo_limit, rng, args.random_combos):
                        hole_set = set(holes)
                        for e in range(args.max_extra + 1):
                            for extras in limited_combinations(outside, e + h, args.combo_limit, rng, args.random_combos):
                                support = (core_set - hole_set) | set(extras)
                                dim_final = kernel_dim(generator, rows, support, k, args.prime)
                                if dim_final <= 0:
                                    continue
                                core_cancelled = tree_cancellation_zeros(k, core_set) - core_set
                                cancelled = (
                                    (tree_cancellation_zeros(k, support) - support)
                                    - core_cancelled
                                    - core_set
                                )
                                active_cancelled = set()
                                for column in cancelled:
                                    if kernel_dim(generator, rows, support | {column}, k, args.prime) > dim_final:
                                        active_cancelled.add(column)
                                relaxed = support | active_cancelled
                                dim_relaxed = kernel_dim(generator, rows, relaxed, k, args.prime)
                                imposed_rank = dim_relaxed - dim_final
                                projective_excess = dim_final - 1
                                required_rank = max(0, len(active_cancelled) - projective_excess - 1)
                                gap = imposed_rank - required_rank
                                worst_gap = min(worst_gap, gap)
                                checked += 1
                                if gap < 0:
                                    failures += 1
                                rows_out.append(
                                    [
                                        args.depth,
                                        k,
                                        live_bits,
                                        m,
                                        block_start,
                                        residue,
                                        h,
                                        e,
                                        ":".join(str(value) for value in holes),
                                        ":".join(str(value) for value in extras),
                                        dim_final,
                                        dim_relaxed,
                                        len(cancelled),
                                        len(active_cancelled),
                                        ":".join(str(value) for value in sorted(active_cancelled)),
                                        imposed_rank,
                                        projective_excess,
                                        required_rank,
                                        gap,
                                    ]
                                )
                                if args.case_limit > 0 and checked >= args.case_limit:
                                    break
                            if args.case_limit > 0 and checked >= args.case_limit:
                                break
                        if args.case_limit > 0 and checked >= args.case_limit:
                            break
                    if args.case_limit > 0 and checked >= args.case_limit:
                        break
                if args.case_limit > 0 and checked >= args.case_limit:
                    break
            if args.case_limit > 0 and checked >= args.case_limit:
                break
        if args.case_limit > 0 and checked >= args.case_limit:
            break

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "k",
                "live_bits",
                "m",
                "block_start",
                "residue",
                "holes",
                "extra_defect",
                "hole_positions",
                "extra_positions",
                "dim_final",
                "dim_relaxed",
                "raw_tree_cancellation_zeros",
                "active_tree_cancellation_zeros",
                "active_cancelled_positions",
                "imposed_rank",
                "projective_excess",
                "required_rank",
                "gap",
            ]
        )
        writer.writerows(rows_out)

    print(f"checked={checked} failures={failures} worst_gap={worst_gap} out={args.out}")
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
