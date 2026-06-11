#!/usr/bin/env python3
"""Profile visible singleton-span subspaces for RFC rank-pattern induction.

For a selected child coordinate set S, let U <= F^S be the restricted child image.  This script
enumerates tau-dimensional subspaces R <= U + U and tests the local singleton compatibility
condition:

    for every j in S, dim projection_j(R) <= 1 in F^2.

Compatible coordinates with projection rank 1 require one root value; projection rank 0 is a
common-zero/invisible coordinate for this local block.
"""

from __future__ import annotations

import argparse
import csv
import itertools
import math
import random
from pathlib import Path

from rfc_exterior_constraint_profile import (
    matroid_component_count,
    restricted_image_basis,
)
from sample_rfc_rank_failure import rfc_generator_prime


def rref_subspaces(ambient_dim: int, tau: int, prime: int):
    if tau < 0 or tau > ambient_dim:
        return
    for pivots in itertools.combinations(range(ambient_dim), tau):
        pivot_set = set(pivots)
        free_positions: list[tuple[int, int]] = []
        for row, pivot in enumerate(pivots):
            for col in range(pivot + 1, ambient_dim):
                if col not in pivot_set:
                    free_positions.append((row, col))
        for values in itertools.product(range(prime), repeat=len(free_positions)):
            rows = [[0] * ambient_dim for _ in range(tau)]
            for row, pivot in enumerate(pivots):
                rows[row][pivot] = 1
            for (row, col), value in zip(free_positions, values):
                rows[row][col] = value
            yield rows


def coordinate_projection_rank(
    subspace_rows: list[list[int]],
    image_basis: list[list[int]],
    coordinate: int,
    prime: int,
) -> int:
    rho = len(image_basis)
    seen_nonzero = False
    first_pair: tuple[int, int] | None = None
    for row in subspace_rows:
        left = 0
        right = 0
        for i, basis_row in enumerate(image_basis):
            left = (left + row[i] * basis_row[coordinate]) % prime
            right = (right + row[rho + i] * basis_row[coordinate]) % prime
        pair = (left, right)
        if pair == (0, 0):
            continue
        if not seen_nonzero:
            seen_nonzero = True
            first_pair = pair
            continue
        assert first_pair is not None
        if (first_pair[0] * pair[1] - first_pair[1] * pair[0]) % prime != 0:
            return 2
    return 1 if seen_nonzero else 0


def iter_subsets(n: int, size: int, samples: int, rng: random.Random):
    if samples <= 0:
        yield from itertools.combinations(range(n), size)
        return
    for _ in range(samples):
        yield tuple(sorted(rng.sample(range(n), size)))


def logq(value: int, prime: int) -> float:
    if value <= 0:
        return -math.inf
    return math.log(value, prime)


def gaussian_binomial(ambient_dim: int, tau: int, prime: int) -> int:
    if tau < 0 or tau > ambient_dim:
        return 0
    tau = min(tau, ambient_dim - tau)
    out = 1
    for i in range(tau):
        out *= prime ** (ambient_dim - i) - 1
        out //= prime ** (tau - i) - 1
    return out


def support_columns_from_mask(columns: tuple[int, ...], support_mask: int) -> tuple[int, ...]:
    return tuple(columns[index] for index in range(len(columns)) if (support_mask >> index) & 1)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--child-depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--size", type=int, required=True)
    parser.add_argument("--tau", type=int, required=True)
    parser.add_argument("--samples", type=int, default=0)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--max-subspaces", type=int, default=2_000_000)
    parser.add_argument("--summary-csv", default=None)
    parser.add_argument("--support-summary-csv", default=None)
    args = parser.parse_args()

    if args.prime <= 2:
        raise SystemExit("--prime must be > 2")
    rng = random.Random(args.seed)
    generator = rfc_generator_prime(args.child_depth, args.expansion, args.prime, rng)
    n = len(generator[0])
    if args.size < 0 or args.size > n:
        raise SystemExit("--size outside coordinate range")

    grouped: dict[tuple[int, int, int, int], list[tuple[int, int]]] = {}
    support_grouped: dict[tuple[int, int, int, int, int, int, int, int], list[tuple[int, int]]] = {}
    checked = 0
    skipped = 0
    for columns in iter_subsets(n, args.size, args.samples, rng):
        image_basis = restricted_image_basis(generator, columns, args.prime)
        rho = len(image_basis)
        if args.tau > 2 * rho:
            continue
        total_subspaces = gaussian_binomial(2 * rho, args.tau, args.prime)
        checked += 1
        if total_subspaces > args.max_subspaces:
            skipped += 1
            continue
        components = matroid_component_count(generator, columns, args.prime)
        compatible = 0
        total = 0
        by_root_count: dict[int, int] = {}
        by_support_mask: dict[int, int] = {}
        for subspace in rref_subspaces(2 * rho, args.tau, args.prime):
            total += 1
            if total > args.max_subspaces:
                skipped += 1
                compatible = -1
                break
            root_count = 0
            support_mask = 0
            ok = True
            for coordinate in range(args.size):
                rank = coordinate_projection_rank(subspace, image_basis, coordinate, args.prime)
                if rank == 2:
                    ok = False
                    break
                if rank == 1:
                    root_count += 1
                    support_mask |= 1 << coordinate
            if ok:
                compatible += 1
                by_root_count[root_count] = by_root_count.get(root_count, 0) + 1
                by_support_mask[support_mask] = by_support_mask.get(support_mask, 0) + 1
        if compatible < 0:
            continue
        for root_count, count in by_root_count.items():
            grouped.setdefault((rho, components, args.tau, root_count), []).append((count, total))
        for support_mask, count in by_support_mask.items():
            support_columns = support_columns_from_mask(columns, support_mask)
            complement_columns = tuple(
                column for index, column in enumerate(columns) if ((support_mask >> index) & 1) == 0
            )
            support_rank = len(restricted_image_basis(generator, support_columns, args.prime))
            support_components = matroid_component_count(generator, support_columns, args.prime)
            complement_rank = len(restricted_image_basis(generator, complement_columns, args.prime))
            support_kernel_dim = rho - complement_rank
            root_count = support_mask.bit_count()
            support_grouped.setdefault(
                (
                    rho,
                    components,
                    args.tau,
                    root_count,
                    support_rank,
                    support_components,
                    support_kernel_dim,
                    complement_rank,
                ),
                [],
            ).append((count, total))

    print(
        f"prime={args.prime} child_depth={args.child_depth} expansion={args.expansion} "
        f"size={args.size} tau={args.tau} checked={checked} skipped={skipped}"
    )
    print("rho,components,tau,root_count,shapes,min_logq_count,avg_logq_count,max_logq_count,min_fraction")
    rows: list[dict[str, int | float]] = []
    for key, values in sorted(grouped.items()):
        logs = [logq(count, args.prime) for count, _total in values]
        fractions = [count / total for count, total in values if total]
        rho, components, tau, root_count = key
        row = {
            "rho": rho,
            "components": components,
            "tau": tau,
            "root_count": root_count,
            "shapes": len(values),
            "min_logq_count": min(logs),
            "avg_logq_count": sum(logs) / len(logs),
            "max_logq_count": max(logs),
            "min_fraction": min(fractions),
        }
        rows.append(row)
        print(
            f"{rho},{components},{tau},{root_count},{row['shapes']},"
            f"{row['min_logq_count']:.10f},{row['avg_logq_count']:.10f},"
            f"{row['max_logq_count']:.10f},{row['min_fraction']:.12g}"
        )
    if args.summary_csv is not None:
        with Path(args.summary_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "rho",
                    "components",
                    "tau",
                    "root_count",
                    "shapes",
                    "min_logq_count",
                    "avg_logq_count",
                    "max_logq_count",
                    "min_fraction",
                ],
            )
            writer.writeheader()
            writer.writerows(rows)

    print()
    print(
        "rho,components,tau,root_count,support_rank,support_components,"
        "support_kernel_dim,complement_rank,shapes,support_gaussian_logq_upper,"
        "min_logq_count,avg_logq_count,max_logq_count,min_fraction"
    )
    support_rows: list[dict[str, int | float]] = []
    for key, values in sorted(support_grouped.items()):
        logs = [logq(count, args.prime) for count, _total in values]
        fractions = [count / total for count, total in values if total]
        (
            rho,
            components,
            tau,
            root_count,
            support_rank,
            support_components,
            support_kernel_dim,
            complement_rank,
        ) = key
        row = {
            "rho": rho,
            "components": components,
            "tau": tau,
            "root_count": root_count,
            "support_rank": support_rank,
            "support_components": support_components,
            "support_kernel_dim": support_kernel_dim,
            "complement_rank": complement_rank,
            "support_gaussian_logq_upper": logq(
                gaussian_binomial(2 * support_kernel_dim, tau, args.prime),
                args.prime,
            ),
            "shapes": len(values),
            "min_logq_count": min(logs),
            "avg_logq_count": sum(logs) / len(logs),
            "max_logq_count": max(logs),
            "min_fraction": min(fractions),
        }
        support_rows.append(row)
        print(
            f"{rho},{components},{tau},{root_count},{support_rank},{support_components},"
            f"{support_kernel_dim},{complement_rank},{row['shapes']},"
            f"{row['support_gaussian_logq_upper']:.10f},{row['min_logq_count']:.10f},"
            f"{row['avg_logq_count']:.10f},{row['max_logq_count']:.10f},"
            f"{row['min_fraction']:.12g}"
        )
    if args.support_summary_csv is not None:
        with Path(args.support_summary_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "rho",
                    "components",
                    "tau",
                    "root_count",
                    "support_rank",
                    "support_components",
                    "support_kernel_dim",
                    "complement_rank",
                    "shapes",
                    "support_gaussian_logq_upper",
                    "min_logq_count",
                    "avg_logq_count",
                    "max_logq_count",
                    "min_fraction",
                ],
            )
            writer.writeheader()
            writer.writerows(support_rows)


if __name__ == "__main__":
    main()
