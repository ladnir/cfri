#!/usr/bin/env python3
"""Combinatorial occupancy proxy for larger-block RFC obstructions.

This is not a distance certificate. It asks a local structural question:

    Given arity b and total P/A zero requests over local blocks, do high-count
    profiles naturally cover whole blocks, or do they remain sparse?

For binary b=2, one P plus one A already covers the whole local block. For
arity b=4, one P plus one A covers only half the block. The old full-span
obstruction reappears only when P and A collectively cover all b outputs.

The script enumerates local occupancy profiles and reports the largest
combinatorial masses grouped by number of full-cover A-active blocks.
"""

from __future__ import annotations

import argparse
import math
from collections import defaultdict
from dataclasses import dataclass


NEG_INF = -1.0e300


def log2_add(a: float, b: float) -> float:
    if a <= NEG_INF / 2:
        return b
    if b <= NEG_INF / 2:
        return a
    if a < b:
        a, b = b, a
    return a + math.log2(1.0 + 2.0 ** (b - a))


def log2_binom(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return math.log2(math.comb(n, k))


@dataclass(frozen=True)
class LocalShape:
    p: int
    a: int
    ell: int
    full_cover: int
    a_active: int
    log2_mult: float


@dataclass(frozen=True)
class TraceState:
    log2_mult: float
    counts: tuple[tuple[int, int], ...]


def local_shapes(arity: int, require_p_for_a: bool) -> list[LocalShape]:
    shapes: list[LocalShape] = []
    for p in range(arity + 1):
        for a in range(arity - p + 1):
            if require_p_for_a and a > 0 and p == 0:
                continue
            ell = p + a
            log2_mult = log2_binom(arity, p) + log2_binom(arity - p, a)
            shapes.append(
                LocalShape(
                    p=p,
                    a=a,
                    ell=ell,
                    full_cover=int(a > 0 and ell == arity),
                    a_active=int(a > 0),
                    log2_mult=log2_mult,
                )
            )
    return shapes


def add_shape_to_trace(trace: TraceState, shape: LocalShape) -> TraceState:
    counts = dict(trace.counts)
    key = (shape.p, shape.a)
    counts[key] = counts.get(key, 0) + 1
    return TraceState(
        log2_mult=trace.log2_mult + shape.log2_mult,
        counts=tuple(sorted(counts.items())),
    )


def better_trace(a: TraceState | None, b: TraceState) -> TraceState:
    if a is None or b.log2_mult > a.log2_mult:
        return b
    return a


def enumerate_profiles(
    arity: int,
    blocks: int,
    p_total: int,
    a_total: int,
    require_p_for_a: bool,
) -> tuple[dict[tuple[int, int, int], float], dict[tuple[int, int, int], TraceState]]:
    shapes = local_shapes(arity, require_p_for_a)
    dp: dict[tuple[int, int, int, int, int], float] = {(0, 0, 0, 0, 0): 0.0}
    trace: dict[tuple[int, int, int, int, int], TraceState] = {
        (0, 0, 0, 0, 0): TraceState(0.0, ())
    }
    # State: used_p, used_a, covered_ell_on_A_blocks, full_cover_A_blocks, A_active_blocks.
    for _ in range(blocks):
        nxt: dict[tuple[int, int, int, int, int], float] = defaultdict(lambda: NEG_INF)
        nxt_trace: dict[tuple[int, int, int, int, int], TraceState] = {}
        for state, log_mass in dp.items():
            used_p, used_a, ell_sum, full_count, active_count = state
            base_trace = trace[state]
            for shape in shapes:
                np = used_p + shape.p
                na = used_a + shape.a
                if np > p_total or na > a_total:
                    continue
                nell = ell_sum + (shape.ell if shape.a_active else 0)
                nfull = full_count + shape.full_cover
                nactive = active_count + shape.a_active
                key = (np, na, nell, nfull, nactive)
                value = log_mass + shape.log2_mult
                nxt[key] = log2_add(nxt[key], value)
                candidate = add_shape_to_trace(base_trace, shape)
                nxt_trace[key] = better_trace(nxt_trace.get(key), candidate)
        dp = dict(nxt)
        trace = nxt_trace

    grouped: dict[tuple[int, int, int], float] = defaultdict(lambda: NEG_INF)
    best_trace: dict[tuple[int, int, int], TraceState] = {}
    for (used_p, used_a, ell_sum, full_count, active_count), log_mass in dp.items():
        if used_p != p_total or used_a != a_total:
            continue
        key = (ell_sum, full_count, active_count)
        grouped[key] = log2_add(grouped[key], log_mass)
        best_trace[key] = better_trace(
            best_trace.get(key),
            trace[(used_p, used_a, ell_sum, full_count, active_count)],
        )
    return dict(grouped), best_trace


def format_counts(counts: tuple[tuple[int, int], ...]) -> str:
    return " ".join(f"({p},{a})x{n}" for (p, a), n in counts)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--arity", type=int, default=4)
    ap.add_argument("--blocks", type=int, default=16)
    ap.add_argument("--p-total", type=int, required=True)
    ap.add_argument("--a-total", type=int, required=True)
    ap.add_argument(
        "--require-p-for-a",
        action="store_true",
        help="Only allow A-active blocks when the same block has at least one P output.",
    )
    ap.add_argument("--top", type=int, default=12)
    args = ap.parse_args()

    grouped, traces = enumerate_profiles(
        args.arity,
        args.blocks,
        args.p_total,
        args.a_total,
        args.require_p_for_a,
    )
    rows = []
    total_log = NEG_INF
    for log_mass in grouped.values():
        total_log = log2_add(total_log, log_mass)
    for key, log_mass in grouped.items():
        ell_sum, full_count, active_count = key
        full_frac = full_count / active_count if active_count else 0.0
        avg_ell = ell_sum / active_count if active_count else 0.0
        rows.append((log_mass, ell_sum, full_count, active_count, avg_ell, full_frac))
    rows.sort(reverse=True)

    print(
        "arity,blocks,p_total,a_total,require_p_for_a,total_log2_profiles,"
        "ell_sum,full_cover_a_blocks,a_active_blocks,avg_ell_on_a,full_cover_fraction,"
        "group_log2_mass,group_log2_fraction,best_local_shape_counts"
    )
    for log_mass, ell_sum, full_count, active_count, avg_ell, full_frac in rows[: args.top]:
        key = (ell_sum, full_count, active_count)
        print(
            f"{args.arity},{args.blocks},{args.p_total},{args.a_total},"
            f"{int(args.require_p_for_a)},{total_log:.6f},{ell_sum},{full_count},{active_count},"
            f"{avg_ell:.6f},{full_frac:.6f},{log_mass:.6f},"
            f"{log_mass - total_log:.6f},{format_counts(traces[key].counts)}"
        )


if __name__ == "__main__":
    main()
