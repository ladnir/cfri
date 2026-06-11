#!/usr/bin/env python3
"""Check the matroid-union rank formula for one RFC lift.

For a parent zero-set shape, root sibling pairs contribute two deterministic child copies.
Root singletons contribute columns of the form (h_j, lambda_j h_j) after quotienting by the
paired child span. The generic rank of those singleton lifts should be the two-copy matroid union
rank:

    min_A |S \\ A| + 2 rank_{M/P}(A).

This script samples small shapes and compares that predicted generic rank with actual parent ranks.
"""

from __future__ import annotations

import argparse
import itertools
import random
from collections import defaultdict

from sample_rfc_rank_failure import rank_selected_columns, rfc_generator_prime


def submatrix_rank(generator: list[list[int]], columns: list[int], prime: int) -> int:
    if not columns:
        return 0
    return rank_selected_columns(generator, columns, prime)


def matroid_union_singleton_rank(
    child_generator: list[list[int]],
    paired: list[int],
    singleton: list[int],
    prime: int,
) -> int:
    paired_rank = submatrix_rank(child_generator, paired, prime)
    best = len(singleton) + 2 * (submatrix_rank(child_generator, paired, prime) - paired_rank)
    singleton_count = len(singleton)
    for mask in range(1 << singleton_count):
        active = [singleton[i] for i in range(singleton_count) if (mask >> i) & 1]
        quotient_rank = submatrix_rank(child_generator, paired + active, prime) - paired_rank
        term = (singleton_count - len(active)) + 2 * quotient_rank
        best = min(best, term)
    return best


def local_lift_codimension(
    child_generator: list[list[int]],
    paired: list[int],
    singleton: list[int],
    prime: int,
) -> int | None:
    """Predicted codimension of finite-field rank drop for a structurally full lift.

    Dual witness formula: choose C inside singleton where both dual witnesses vanish. If the
    quotient rank remaining after C is u, then the |S|-|C| outside coordinates impose ratios from
    a 2u-dimensional witness family, giving codimension |S|-|C|-(2u-1).
    """

    child_k = len(child_generator)
    paired_rank = submatrix_rank(child_generator, paired, prime)
    quotient_rank = child_k - paired_rank
    if quotient_rank <= 0:
        return None

    best: int | None = None
    singleton_count = len(singleton)
    for mask in range(1 << singleton_count):
        common_zero = [singleton[i] for i in range(singleton_count) if (mask >> i) & 1]
        common_rank = submatrix_rank(child_generator, paired + common_zero, prime) - paired_rank
        witness_dimension = quotient_rank - common_rank
        if witness_dimension <= 0:
            continue
        outside = singleton_count - len(common_zero)
        codim = outside - (2 * witness_dimension - 1)
        if codim < 0:
            codim = 0
        best = codim if best is None else min(best, codim)
    return best


def parent_rank_for_shape(
    child_generator: list[list[int]],
    paired: list[int],
    singleton: list[int],
    prime: int,
    rng: random.Random,
) -> int:
    child_k = len(child_generator)
    child_n = len(child_generator[0])
    parent = [[0 for _ in range(2 * child_n)] for _ in range(2 * child_k)]
    diagonal = [rng.randrange(1, prime) for _ in range(child_n)]
    for row in range(child_k):
        for j, entry in enumerate(child_generator[row]):
            t = diagonal[j]
            parent[row][j] = ((1 - t) * entry) % prime
            parent[row][child_n + j] = (-t * entry) % prime
            parent[row + child_k][j] = (t * entry) % prime
            parent[row + child_k][child_n + j] = ((t + 1) * entry) % prime
    columns = paired + [child_n + j for j in paired] + singleton
    return rank_selected_columns(parent, columns, prime)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--child-depth", type=int, default=2)
    parser.add_argument("--expansion", type=int, default=4)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--shape-samples", type=int, default=1000)
    parser.add_argument("--rank-samples", type=int, default=20)
    parser.add_argument("--failure-profile", action="store_true")
    parser.add_argument("--failure-trials", type=int, default=500)
    parser.add_argument("--max-paired", type=int, default=5)
    parser.add_argument("--max-singleton", type=int, default=8)
    parser.add_argument("--seed", type=int, default=1)
    args = parser.parse_args()

    rng = random.Random(args.seed)
    child_generator = rfc_generator_prime(args.child_depth, args.expansion, args.prime, rng)
    child_n = len(child_generator[0])

    failures = 0
    checked = 0
    first_failure = ""
    profiles: dict[tuple[int, int], list[int]] = defaultdict(lambda: [0, 0, 0])
    for _ in range(args.shape_samples):
        paired_count = rng.randrange(0, min(args.max_paired, child_n) + 1)
        remaining = child_n - paired_count
        singleton_count = rng.randrange(0, min(args.max_singleton, remaining) + 1)
        selected = rng.sample(range(child_n), paired_count + singleton_count)
        paired = sorted(selected[:paired_count])
        singleton = sorted(selected[paired_count:])

        paired_rank = submatrix_rank(child_generator, paired, args.prime)
        predicted = 2 * paired_rank + matroid_union_singleton_rank(
            child_generator,
            paired,
            singleton,
            args.prime,
        )
        actual_max = 0
        for _rank_sample in range(args.rank_samples):
            actual_max = max(
                actual_max,
                parent_rank_for_shape(child_generator, paired, singleton, args.prime, rng),
            )
        checked += 1
        if actual_max != predicted:
            failures += 1
            if not first_failure:
                first_failure = (
                    f"paired={':'.join(map(str, paired))} "
                    f"singleton={':'.join(map(str, singleton))} "
                    f"predicted={predicted} actual_max={actual_max}"
                )
        if args.failure_profile:
            child_k = len(child_generator)
            quotient_rank = child_k - paired_rank
            structurally_full = predicted == 2 * child_k
            if structurally_full:
                excess = len(singleton) - 2 * quotient_rank
                codim = local_lift_codimension(child_generator, paired, singleton, args.prime)
                if codim is None:
                    codim = -1
                drops = 0
                columns = paired + [child_n + j for j in paired] + singleton
                for _trial in range(args.failure_trials):
                    diagonal = [rng.randrange(1, args.prime) for _ in range(child_n)]
                    parent = [[0 for _ in range(2 * child_n)] for _ in range(2 * child_k)]
                    for row in range(child_k):
                        for j, entry in enumerate(child_generator[row]):
                            t = diagonal[j]
                            parent[row][j] = ((1 - t) * entry) % args.prime
                            parent[row][child_n + j] = (-t * entry) % args.prime
                            parent[row + child_k][j] = (t * entry) % args.prime
                            parent[row + child_k][child_n + j] = ((t + 1) * entry) % args.prime
                    if rank_selected_columns(parent, columns, args.prime) < predicted:
                        drops += 1
                profiles[(excess, codim)][0] += 1
                profiles[(excess, codim)][1] += drops
                profiles[(excess, codim)][2] += args.failure_trials

    print(
        f"child_depth={args.child_depth} expansion={args.expansion} prime={args.prime} "
        f"checked={checked} failures={failures} first_failure={first_failure}"
    )
    if args.failure_profile:
        print("excess,predicted_codim,shapes,drops,trials,drop_rate")
        for (excess, codim), (shapes, drops, trials) in sorted(profiles.items()):
            rate = 0.0 if trials == 0 else drops / trials
            print(f"{excess},{codim},{shapes},{drops},{trials},{rate:.12g}")


if __name__ == "__main__":
    main()
