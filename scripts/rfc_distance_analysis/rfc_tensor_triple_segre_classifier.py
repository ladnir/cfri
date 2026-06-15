#!/usr/bin/env python3
"""Classify RFC tensor triples by Segre-line dependence codimension.

For three distinct rank-one tensors, linear dependence can occur only when the
three projective local factors agree in all but one tensor level.  RFC local
factors are

    u_0(T) = (1-T, T),     u_1(T) = (-T, T+1),

and their projective class is determined by theta = T + bit.  This script counts
the number of independent theta-equality equations needed for a fixed triple to
lie on such a Segre line.
"""

from __future__ import annotations

import argparse
import itertools
from collections import Counter
from dataclasses import dataclass


INF = 10**9


@dataclass(frozen=True)
class Factor:
    variable: int
    bit: int


def parse_columns(text: str) -> tuple[int, ...]:
    out = tuple(int(part) for part in text.split(",") if part)
    if len(out) != 3:
        raise argparse.ArgumentTypeError("expected exactly three comma-separated columns")
    return out


def column_factors(depth: int, expansion: int, col: int) -> list[Factor]:
    copy = col % expansion
    path = col // expansion
    idx = copy
    factors: list[Factor] = []
    for level in range(depth):
        bit = (path >> level) & 1
        # Variables at different levels are independent; pack level into the id.
        variable = (level << 32) | idx
        factors.append(Factor(variable=variable, bit=bit))
        idx += bit * (expansion << level)
    return factors


def equality_codim(factors: list[Factor]) -> int:
    """Codimension for theta_0=theta_1=theta_2 at one tensor level.

    Each theta is T_variable + bit. Equal variables with different bits are
    inconsistent. Otherwise the codimension is (# distinct affine theta groups - 1).
    """
    by_var: dict[int, int] = {}
    for factor in factors:
        old = by_var.get(factor.variable)
        if old is not None and old != factor.bit:
            return INF
        by_var[factor.variable] = factor.bit
    return max(0, len(by_var) - 1)


def triple_codim(depth: int, expansion: int, columns: tuple[int, int, int]) -> tuple[int, int | None, list[int]]:
    per_column = [column_factors(depth, expansion, col) for col in columns]
    level_costs = [
        equality_codim([per_column[col_idx][level] for col_idx in range(3)])
        for level in range(depth)
    ]
    best = INF
    best_free: int | None = None
    for free_level in range(depth):
        total = 0
        ok = True
        for level, cost in enumerate(level_costs):
            if level == free_level:
                continue
            if cost >= INF:
                ok = False
                break
            total += cost
        if ok and total < best:
            best = total
            best_free = free_level
    return best, best_free, level_costs


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--depth", type=int, default=4)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--columns", type=parse_columns, action="append")
    parser.add_argument("--profile-all", action="store_true")
    parser.add_argument("--top", type=int, default=12)
    args = parser.parse_args()

    n = args.expansion * (1 << args.depth)

    if args.columns:
        print("columns,codim,free_level,level_costs")
        for columns in args.columns:
            codim, free_level, level_costs = triple_codim(args.depth, args.expansion, columns)
            codim_text = "inf" if codim >= INF else str(codim)
            free_text = "" if free_level is None else str(free_level)
            print(f"{':'.join(str(col) for col in columns)},{codim_text},{free_text},{':'.join(str(c) if c < INF else 'inf' for c in level_costs)}")

    if args.profile_all:
        counts: Counter[int | str] = Counter()
        examples: dict[int | str, tuple[int, int, int]] = {}
        for columns in itertools.combinations(range(n), 3):
            codim, _free_level, _level_costs = triple_codim(args.depth, args.expansion, columns)
            key: int | str = "inf" if codim >= INF else codim
            counts[key] += 1
            examples.setdefault(key, columns)
        print("codim,count,example")
        def sort_key(item: tuple[int | str, int]) -> tuple[int, int | str]:
            key, _count = item
            return (1, key) if key == "inf" else (0, key)
        for key, count in sorted(counts.items(), key=sort_key)[: args.top]:
            example = examples[key]
            print(f"{key},{count},{':'.join(str(col) for col in example)}")


if __name__ == "__main__":
    main()
