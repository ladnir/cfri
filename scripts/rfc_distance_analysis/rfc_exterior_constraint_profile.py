#!/usr/bin/env python3
"""Exact counts for r=2 exterior constraints on selected child coordinates.

For selected coordinates S and child image space U <= F^S, count:

    {(x,y,z,w) in U^4 : x_j w_j - y_j z_j = 0 for all j in S}.

This is the finite-field version of the r=2 rank-one compatibility variety.  The count is computed
by enumerating (x,y) in U^2 and, for each pair, taking the rank of the linear system imposed on
(z,w) in U^2.  This avoids enumerating all message quadruples.
"""

from __future__ import annotations

import argparse
import csv
import itertools
import math
import random
from pathlib import Path

from sample_rfc_rank_failure import rank_selected_columns, rfc_generator_prime


def modinv(value: int, prime: int) -> int:
    return pow(value % prime, prime - 2, prime)


def rref_nonzero_rows(rows: list[list[int]], prime: int) -> list[list[int]]:
    if not rows:
        return []
    rows = [[value % prime for value in row] for row in rows]
    rank = 0
    width = len(rows[0])
    for col in range(width):
        pivot = rank
        while pivot < len(rows) and rows[pivot][col] == 0:
            pivot += 1
        if pivot == len(rows):
            continue
        rows[rank], rows[pivot] = rows[pivot], rows[rank]
        inv = modinv(rows[rank][col], prime)
        rows[rank] = [(value * inv) % prime for value in rows[rank]]
        for r, row in enumerate(rows):
            if r == rank or row[col] == 0:
                continue
            factor = row[col]
            rows[r] = [(value - factor * pivot_value) % prime for value, pivot_value in zip(row, rows[rank])]
        rank += 1
        if rank == len(rows):
            break
    return [row for row in rows[:rank] if any(value != 0 for value in row)]


def restricted_image_basis(
    generator: list[list[int]],
    columns: tuple[int, ...],
    prime: int,
) -> list[list[int]]:
    restricted_rows = [[row[col] % prime for col in columns] for row in generator]
    return rref_nonzero_rows(restricted_rows, prime)


def all_space_vectors(basis: list[list[int]], prime: int) -> list[tuple[int, ...]]:
    if not basis:
        return [()]
    dimension = len(basis)
    width = len(basis[0])
    vectors: list[tuple[int, ...]] = []
    for coeffs in itertools.product(range(prime), repeat=dimension):
        out = [0] * width
        for coeff, row in zip(coeffs, basis):
            if coeff == 0:
                continue
            for j, value in enumerate(row):
                out[j] = (out[j] + coeff * value) % prime
        vectors.append(tuple(out))
    return vectors


def matrix_rank(rows: list[list[int]], prime: int) -> int:
    return len(rref_nonzero_rows(rows, prime))


def exterior_image_count(basis: list[list[int]], prime: int) -> int:
    rho = len(basis)
    if rho == 0:
        return 1
    width = len(basis[0])
    vectors = all_space_vectors(basis, prime)
    total = 0
    for x in vectors:
        for y in vectors:
            equations: list[list[int]] = []
            for j in range(width):
                row = [(-y[j] * basis_i[j]) % prime for basis_i in basis]
                row += [(x[j] * basis_i[j]) % prime for basis_i in basis]
                if any(value != 0 for value in row):
                    equations.append(row)
            rank = matrix_rank(equations, prime)
            total += prime ** (2 * rho - rank)
    return total


class DisjointSet:
    def __init__(self, size: int) -> None:
        self.parent = list(range(size))

    def find(self, value: int) -> int:
        while self.parent[value] != value:
            self.parent[value] = self.parent[self.parent[value]]
            value = self.parent[value]
        return value

    def union(self, left: int, right: int) -> None:
        left_root = self.find(left)
        right_root = self.find(right)
        if left_root != right_root:
            self.parent[right_root] = left_root

    def count(self) -> int:
        return len({self.find(index) for index in range(len(self.parent))})


def matroid_component_count(generator: list[list[int]], columns: tuple[int, ...], prime: int) -> int:
    """Count connected components of the represented matroid restricted to columns.

    This brute-force circuit scan is for small diagnostic subsets. Coloops become singleton
    components because they are not in any circuit.
    """

    size = len(columns)
    if size == 0:
        return 0
    dsu = DisjointSet(size)
    rank_cache: dict[int, int] = {0: 0}

    def rank_mask(mask: int) -> int:
        if mask not in rank_cache:
            subset = [columns[i] for i in range(size) if (mask >> i) & 1]
            rank_cache[mask] = rank_selected_columns(generator, subset, prime)
        return rank_cache[mask]

    for mask in range(1, 1 << size):
        bit_count = mask.bit_count()
        if bit_count < 2 or rank_mask(mask) == bit_count:
            continue
        minimal = True
        for i in range(size):
            if (mask >> i) & 1 and rank_mask(mask ^ (1 << i)) < bit_count - 1:
                minimal = False
                break
        if not minimal:
            continue
        first = (mask & -mask).bit_length() - 1
        for i in range(first + 1, size):
            if (mask >> i) & 1:
                dsu.union(first, i)
    return dsu.count()


def iter_subsets(n: int, size: int, samples: int, rng: random.Random):
    if samples <= 0:
        yield from itertools.combinations(range(n), size)
        return
    for _ in range(samples):
        yield tuple(sorted(rng.sample(range(n), size)))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--child-depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--size", type=int, required=True)
    parser.add_argument("--samples", type=int, default=0)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--print-subsets", action="store_true")
    parser.add_argument("--summary-csv", default=None)
    parser.add_argument("--subset-csv", default=None)
    args = parser.parse_args()

    if args.prime <= 2:
        raise SystemExit("--prime must be > 2")
    rng = random.Random(args.seed)
    generator = rfc_generator_prime(args.child_depth, args.expansion, args.prime, rng)
    n = len(generator[0])
    k = len(generator)
    if args.size < 0 or args.size > n:
        raise SystemExit("--size outside coordinate range")

    grouped: dict[tuple[int, int, int], list[float]] = {}
    subset_rows: list[dict[str, int | float | str]] = []
    checked = 0
    for columns in iter_subsets(n, args.size, args.samples, rng):
        basis = restricted_image_basis(generator, columns, args.prime)
        rho = len(basis)
        count = exterior_image_count(basis, args.prime)
        logq_count = math.log(count, args.prime)
        codim = 4 * rho - logq_count
        components = matroid_component_count(generator, columns, args.prime)
        predicted = max(0, 2 * rho - components)
        grouped.setdefault((rho, components, predicted), []).append(codim)
        checked += 1
        if args.print_subsets:
            column_text = ":".join(str(column) for column in columns)
            print(
                f"subset,{column_text},rho={rho},count={count},"
                f"logq_count={logq_count:.10f},codim={codim:.10f},"
                f"components={components},predicted={predicted}"
            )
        subset_rows.append(
            {
                "columns": ":".join(str(column) for column in columns),
                "rho": rho,
                "components": components,
                "predicted_codim": predicted,
                "count": count,
                "logq_count": logq_count,
                "codim": codim,
            }
        )

    print(
        f"prime={args.prime} child_depth={args.child_depth} k={k} n={n} "
        f"expansion={args.expansion} size={args.size} checked={checked}"
    )
    print("rho,components,predicted_codim,shapes,min_codim,avg_codim,max_codim")
    summary_rows: list[dict[str, int | float]] = []
    for (rho, components, predicted), values in sorted(grouped.items()):
        row = {
            "rho": rho,
            "components": components,
            "predicted_codim": predicted,
            "shapes": len(values),
            "min_codim": min(values),
            "avg_codim": sum(values) / len(values),
            "max_codim": max(values),
        }
        summary_rows.append(row)
        print(
            f"{rho},{components},{predicted},{row['shapes']},"
            f"{row['min_codim']:.10f},{row['avg_codim']:.10f},{row['max_codim']:.10f}"
        )
    if args.summary_csv is not None:
        with Path(args.summary_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "rho",
                    "components",
                    "predicted_codim",
                    "shapes",
                    "min_codim",
                    "avg_codim",
                    "max_codim",
                ],
            )
            writer.writeheader()
            writer.writerows(summary_rows)
    if args.subset_csv is not None:
        with Path(args.subset_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "columns",
                    "rho",
                    "components",
                    "predicted_codim",
                    "count",
                    "logq_count",
                    "codim",
                ],
            )
            writer.writeheader()
            writer.writerows(subset_rows)


if __name__ == "__main__":
    main()
