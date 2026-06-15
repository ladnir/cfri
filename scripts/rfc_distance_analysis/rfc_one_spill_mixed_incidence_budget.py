#!/usr/bin/env python3
"""Mixed lower-triple/spill-singleton incidence budget for the one-spill row.

The dominant one-spill row has compressed profile:

    p0=3, s0=1, D0=14, h0=27.

At the spill level, the three paired lower columns must have rank two.  The
single spill coordinate then imposes one linear equation on K_P plus K_P unless
its lower column is already in the span of the rank-two triple.  For rank-one
tensor columns, that span is the Segre line containing the triple.

This diagnostic counts how often a fourth RFC column is forced onto that same
Segre line by the same codimension-minimal triple incidence equations.  If the
answer is small, mixed root-line incidence does not supply a q-dimensional
charge: the rank-one spill equation is generic/open after the triple event.
"""

from __future__ import annotations

import argparse
import math
from collections import Counter

from rfc_tensor_triple_segre_classifier import INF, column_factors, equality_codim, triple_codim


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return float("-inf")
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


def line_codim_for_free_level(depth: int, expansion: int, columns: tuple[int, ...], free_level: int) -> int:
    per_column = [column_factors(depth, expansion, col) for col in columns]
    total = 0
    for level in range(depth):
        if level == free_level:
            continue
        cost = equality_codim([per_column[col_idx][level] for col_idx in range(len(columns))])
        if cost >= INF:
            return INF
        total += cost
    return total


def achieving_free_levels(depth: int, expansion: int, triple: tuple[int, int, int], codim: int) -> list[int]:
    out: list[int] = []
    for free_level in range(depth):
        if line_codim_for_free_level(depth, expansion, triple, free_level) == codim:
            out.append(free_level)
    return out


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--depth", type=int, default=4)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--target-codim", type=int, default=3)
    parser.add_argument("--top", type=int, default=12)
    args = parser.parse_args()

    n = args.expansion * (1 << args.depth)
    triples = []
    for c0 in range(n):
        for c1 in range(c0 + 1, n):
            for c2 in range(c1 + 1, n):
                codim, _free_level, _level_costs = triple_codim(args.depth, args.expansion, (c0, c1, c2))
                if codim == args.target_codim:
                    triples.append((c0, c1, c2))

    forced_closure_hist: Counter[int] = Counter()
    extra_codim_hist: Counter[int | str] = Counter()
    total_singleton_choices = 0
    forced_closure_choices = 0
    examples: dict[int, tuple[tuple[int, int, int], int, list[int]]] = {}

    for triple in triples:
        free_levels = achieving_free_levels(args.depth, args.expansion, triple, args.target_codim)
        forced_for_triple = 0
        triple_set = set(triple)
        for c in range(n):
            if c in triple_set:
                continue
            total_singleton_choices += 1
            best_extra = INF
            for free_level in free_levels:
                four_codim = line_codim_for_free_level(args.depth, args.expansion, (*triple, c), free_level)
                if four_codim < INF:
                    best_extra = min(best_extra, four_codim - args.target_codim)
            key: int | str = "inf" if best_extra >= INF else best_extra
            extra_codim_hist[key] += 1
            if best_extra == 0:
                forced_for_triple += 1
        forced_closure_hist[forced_for_triple] += 1
        forced_closure_choices += forced_for_triple
        examples.setdefault(forced_for_triple, (triple, forced_for_triple, free_levels))

    side_choices = 2
    current_log2 = math.log2(len(triples)) + log2_comb(n - 3, 1) + math.log2(side_choices)
    corrected_pairs = (total_singleton_choices - forced_closure_choices) * side_choices
    corrected_log2 = math.log2(corrected_pairs) if corrected_pairs > 0 else float("-inf")
    saving_bits = current_log2 - corrected_log2

    print("summary,depth,expansion,target_codim,n,triples,total_singletons,forced_closure_singletons,current_log2,corrected_log2,saving_bits,saving_qdims_128")
    print(
        f"summary,{args.depth},{args.expansion},{args.target_codim},{n},{len(triples)},"
        f"{total_singleton_choices},{forced_closure_choices},"
        f"{current_log2:.6f},{corrected_log2:.6f},{saving_bits:.6f},{saving_bits / 128.0:.6f}"
    )

    print("forced_closure_per_triple,triple_count,example,free_levels")
    for forced, count in sorted(forced_closure_hist.items())[: args.top]:
        triple, _forced, free_levels = examples[forced]
        print(f"{forced},{count},{':'.join(str(c) for c in triple)},{':'.join(str(level) for level in free_levels)}")

    print("extra_codim_for_fourth_on_same_line,pair_count")
    def sort_key(item: tuple[int | str, int]) -> tuple[int, int]:
        key, _count = item
        return (1, 0) if key == "inf" else (0, int(key))
    for key, count in sorted(extra_codim_hist.items(), key=sort_key)[: args.top]:
        print(f"{key},{count}")


if __name__ == "__main__":
    main()
