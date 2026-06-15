#!/usr/bin/env python3
"""Prototype recursive codimension profiles for RFC zero-set shapes."""

from __future__ import annotations

import argparse
import itertools
import random
from functools import lru_cache

from rfc_matroid_lift_check import local_lift_codimension, matroid_union_singleton_rank
from sample_rfc_rank_failure import rank_selected_columns


INF = 10**9


def split_shape(columns: tuple[int, ...], child_n: int) -> tuple[tuple[int, ...], tuple[int, ...]]:
    groups: dict[int, int] = {}
    for col in columns:
        side = 1 if col >= child_n else 0
        low = col - child_n if side else col
        groups[low] = groups.get(low, 0) | (1 << side)
    paired = tuple(sorted(low for low, mask in groups.items() if mask == 3))
    singleton = tuple(sorted(low for low, mask in groups.items() if mask != 3))
    return paired, singleton


class AlphaProfiler:
    def __init__(self, depth: int, expansion: int, prime: int, seed: int) -> None:
        self.depth = depth
        self.expansion = expansion
        self.prime = prime
        self.generators = self._generator_tower(depth, expansion, prime, random.Random(seed))

    @staticmethod
    def _generator_tower(
        depth: int,
        expansion: int,
        prime: int,
        rng: random.Random,
    ) -> list[list[list[int]]]:
        """Build one nested RFC tower, retaining the actual child of every parent."""

        generator = [[1 for _ in range(expansion)]]
        generators = [generator]
        k = 1
        n = expansion
        for _round in range(depth):
            next_n = 2 * n
            next_generator = [[0 for _ in range(next_n)] for _ in range(2 * k)]
            diagonal = [rng.randrange(1, prime) for _ in range(n)]
            for row in range(k):
                for j, entry in enumerate(generator[row]):
                    t = diagonal[j]
                    next_generator[row][j] = ((1 - t) * entry) % prime
                    next_generator[row][n + j] = (-t * entry) % prime
                    next_generator[row + k][j] = (t * entry) % prime
                    next_generator[row + k][n + j] = ((t + 1) * entry) % prime
            generator = next_generator
            generators.append(generator)
            k *= 2
            n = next_n
        return generators

    def rank(self, depth: int, columns: tuple[int, ...]) -> int:
        if not columns:
            return 0
        return rank_selected_columns(self.generators[depth], list(columns), self.prime)

    @lru_cache(maxsize=None)
    def beta(self, depth: int, columns: tuple[int, ...]) -> int:
        """Codimension lower bound for rank dropping below generic rank of this shape."""

        columns = tuple(sorted(columns))
        generic_rank = self.rank(depth, columns)
        if generic_rank == 0:
            return INF
        if depth == 0:
            return INF

        child_n = self.expansion * (1 << (depth - 1))
        paired, singleton = split_shape(columns, child_n)
        child = self.generators[depth - 1]

        child_pair_rank = self.rank(depth - 1, paired)
        singleton_rank = matroid_union_singleton_rank(
            child,
            list(paired),
            list(singleton),
            self.prime,
        )
        local_generic = 2 * child_pair_rank + singleton_rank
        if local_generic != generic_rank:
            raise RuntimeError(
                f"local generic rank mismatch depth={depth} columns={columns} "
                f"rank={generic_rank} local={local_generic}"
            )

        candidates: list[int] = []
        root_codim = local_lift_codimension(child, list(paired), list(singleton), self.prime)
        # The current local codimension lemma applies to rank drop below full rank for the
        # quotient two-copy lift. For structurally rank-limited shapes, drop below generic rank is
        # a different determinantal problem; do not charge it with this full-rank formula.
        if root_codim is not None and local_generic == 2 * len(child):
            candidates.append(root_codim)

        # Conservative child-specialization terms: if a child subset used by the local rank formula
        # loses rank below generic, the parent rank may also drop.
        s_count = len(singleton)
        for mask in range(1 << s_count):
            active = tuple(singleton[i] for i in range(s_count) if (mask >> i) & 1)
            child_columns = tuple(sorted(set(paired).union(active)))
            child_beta = self.beta(depth - 1, child_columns)
            if child_beta != INF:
                candidates.append(child_beta)

        return min(candidates) if candidates else INF

    def alpha_full(self, depth: int, columns: tuple[int, ...]) -> int:
        k = 1 << depth
        if self.rank(depth, columns) < k:
            return 0
        return self.beta(depth, tuple(sorted(columns)))


def iter_shapes(n: int, z: int, samples: int, rng: random.Random):
    if samples <= 0:
        yield from itertools.combinations(range(n), z)
        return
    for _ in range(samples):
        yield tuple(sorted(rng.sample(range(n), z)))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--z", type=int, required=True)
    parser.add_argument("--samples", type=int, default=0)
    parser.add_argument("--seed", type=int, default=1)
    args = parser.parse_args()

    n = args.expansion * (1 << args.depth)
    profiler = AlphaProfiler(args.depth, args.expansion, args.prime, args.seed)
    rng = random.Random(args.seed)
    counts: dict[int, int] = {}
    checked = 0
    for shape in iter_shapes(n, args.z, args.samples, rng):
        alpha = profiler.alpha_full(args.depth, tuple(shape))
        counts[alpha] = counts.get(alpha, 0) + 1
        checked += 1

    print(f"depth={args.depth} expansion={args.expansion} n={n} z={args.z} checked={checked}")
    print("alpha,count")
    for alpha, count in sorted(counts.items()):
        label = "inf" if alpha >= INF else str(alpha)
        print(f"{label},{count}")


if __name__ == "__main__":
    main()
