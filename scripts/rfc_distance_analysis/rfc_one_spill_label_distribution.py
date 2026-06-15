#!/usr/bin/env python3
"""Profile P/C label distributions inside the dominant one-spill lifted blocks.

The dominant one-spill row has seven lifted blocks of size 32 and 87 C-labels
inside Z=P union C.  The coarse ledger pays log2 binom(224,87).  This script
stratifies that label entropy by the number of genuinely mixed blocks and tests
simple q-penalty models per mixed block.
"""

from __future__ import annotations

import argparse
import math
from collections import defaultdict


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


def distribution_profile(blocks: int, block_size: int, c_total: int) -> dict[tuple[int, int], float]:
    dp: dict[tuple[int, int, int], float] = {(0, 0, 0): 0.0}
    for _ in range(blocks):
        nxt: dict[tuple[int, int, int], float] = defaultdict(lambda: NEG_INF)
        for (used, mixed, minority), log_count in dp.items():
            for c in range(block_size + 1):
                next_used = used + c
                if next_used > c_total:
                    continue
                is_mixed = 0 < c < block_size
                next_mixed = mixed + (1 if is_mixed else 0)
                next_minority = minority + (min(c, block_size - c) if is_mixed else 0)
                term = log_count + log2_comb(block_size, c)
                key = (next_used, next_mixed, next_minority)
                nxt[key] = log2_add(nxt[key], term)
        dp = dict(nxt)
    out: dict[tuple[int, int], float] = {}
    for (used, mixed, minority), log_count in dp.items():
        if used == c_total:
            key = (mixed, minority)
            out[key] = log2_add(out.get(key, NEG_INF), log_count)
    return out


def penalized_logsum(
    profile: dict[tuple[int, int], float],
    q_log2: float,
    mixed_penalty_q: float,
    minority_penalty_q: float,
) -> float:
    total = NEG_INF
    for (mixed, minority), log_count in profile.items():
        penalty = mixed_penalty_q * mixed + minority_penalty_q * minority
        total = log2_add(total, log_count - q_log2 * penalty)
    return total


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--blocks", type=int, default=7)
    parser.add_argument("--block-size", type=int, default=32)
    parser.add_argument("--c-total", type=int, default=87)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--needed-q", type=float, default=30.196314)
    parser.add_argument("--max-penalty", type=float, default=40.0)
    parser.add_argument("--penalty-step", type=float, default=0.5)
    parser.add_argument("--minority-mode", action="store_true")
    args = parser.parse_args()

    profile = distribution_profile(args.blocks, args.block_size, args.c_total)
    total = NEG_INF
    for log_count in profile.values():
        total = log2_add(total, log_count)

    by_mixed: dict[int, float] = {}
    by_minority: dict[int, float] = {}
    for (mixed, minority), log_count in profile.items():
        by_mixed[mixed] = log2_add(by_mixed.get(mixed, NEG_INF), log_count)
        by_minority[minority] = log2_add(by_minority.get(minority, NEG_INF), log_count)

    print("mixed_blocks,log2_count,share_log2")
    for mixed in sorted(by_mixed):
        print(f"{mixed},{by_mixed[mixed]:.6f},{by_mixed[mixed] - total:.6f}")

    print("minority_labels,log2_count,share_log2")
    for minority in sorted(by_minority):
        print(f"{minority},{by_minority[minority]:.6f},{by_minority[minority] - total:.6f}")

    print("summary,total_log2,min_mixed,max_mixed")
    min_mixed = min(mixed for mixed, _minority in profile)
    max_mixed = max(mixed for mixed, _minority in profile)
    print(f"summary,{total:.6f},{min_mixed},{max_mixed}")

    target_log2 = total - args.needed_q * args.q_log2
    feature_name = "minority" if args.minority_mode else "mixed"
    print(f"penalty_feature,{feature_name}_penalty_q,penalized_log2,gain_q,dominant_mixed,dominant_minority")
    penalty = 0.0
    while penalty <= args.max_penalty + 1e-12:
        if args.minority_mode:
            value = penalized_logsum(profile, args.q_log2, 0.0, penalty)
            best = max(
                profile,
                key=lambda key: profile[key] - args.q_log2 * penalty * key[1],
            )
        else:
            value = penalized_logsum(profile, args.q_log2, penalty, 0.0)
            best = max(
                profile,
                key=lambda key: profile[key] - args.q_log2 * penalty * key[0],
            )
        gain_q = (total - value) / args.q_log2
        print(f"penalty,{penalty:.6f},{value:.6f},{gain_q:.6f},{best[0]},{best[1]}")
        if value <= target_log2:
            break
        penalty += args.penalty_step


if __name__ == "__main__":
    main()
