#!/usr/bin/env python3
r"""Exact small-depth child-flag intersections for original RFC copies.

This is a bounded falsification diagnostic for the original non-systematic RFC
distance project.  It enumerates zero-induced child flags

    L = ker(G_{P union S}) <= V = ker(G_{P union (S \ A)})

for independently sampled RFC copies over a small prime.  The canonical RREF
bases of L and V are used as the identity of the flag, so multiple paired/core
certificates for the same child flag are counted once.  The intersection pass
then compares exact common L and V dimensions across copies with the generic
linear-subspace intersection dimension.

The script is deliberately size guarded.  It is for depth-4-scale exact checks
and tiny smoke runs, not for production-depth benchmarking.
"""

from __future__ import annotations

import argparse
import csv
import itertools
import math
import random
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Iterator

from sample_rfc_rank_failure import rfc_generator_prime
from rfc_exterior_constraint_profile import matroid_component_count


NEG_INF = -math.inf
BasisKey = tuple[tuple[int, ...], ...]


def modinv(value: int, prime: int) -> int:
    return pow(value % prime, prime - 2, prime)


def rref_with_pivots(
    rows: Iterable[Iterable[int]],
    width: int,
    prime: int,
) -> tuple[list[list[int]], list[int]]:
    work = [
        [value % prime for value in row]
        for row in rows
        if any((value % prime) != 0 for value in row)
    ]
    rank = 0
    pivots: list[int] = []
    for col in range(width):
        pivot = rank
        while pivot < len(work) and work[pivot][col] == 0:
            pivot += 1
        if pivot == len(work):
            continue
        work[rank], work[pivot] = work[pivot], work[rank]
        inv = modinv(work[rank][col], prime)
        work[rank] = [(value * inv) % prime for value in work[rank]]
        for row_index, row in enumerate(work):
            if row_index == rank or row[col] == 0:
                continue
            factor = row[col]
            work[row_index] = [
                (value - factor * pivot_value) % prime
                for value, pivot_value in zip(row, work[rank])
            ]
        pivots.append(col)
        rank += 1
        if rank == len(work):
            break
    return work[:rank], pivots


def canonical_basis(rows: Iterable[Iterable[int]], width: int, prime: int) -> list[list[int]]:
    return rref_with_pivots(rows, width, prime)[0]


def basis_key(rows: Iterable[Iterable[int]], width: int, prime: int) -> BasisKey:
    return tuple(tuple(row) for row in canonical_basis(rows, width, prime))


def nullspace_basis(rows: Iterable[Iterable[int]], width: int, prime: int) -> list[list[int]]:
    rref, pivots = rref_with_pivots(rows, width, prime)
    pivot_set = set(pivots)
    free_cols = [col for col in range(width) if col not in pivot_set]
    basis: list[list[int]] = []
    for free_col in free_cols:
        vector = [0] * width
        vector[free_col] = 1
        for row_index, pivot_col in enumerate(pivots):
            vector[pivot_col] = (-rref[row_index][free_col]) % prime
        basis.append(vector)
    return canonical_basis(basis, width, prime)


def kernel_basis_for_zero_columns(
    generator: list[list[int]],
    columns: tuple[int, ...],
    prime: int,
) -> list[list[int]]:
    width = len(generator)
    equations = [
        [generator[row][column] % prime for row in range(width)]
        for column in columns
    ]
    return nullspace_basis(equations, width, prime)


def rank_columns(generator: list[list[int]], columns: tuple[int, ...], prime: int) -> int:
    if not columns:
        return 0
    rows = [[row[column] % prime for column in columns] for row in generator]
    return len(rref_with_pivots(rows, len(columns), prime)[1])


def restricted_image_basis(
    generator: list[list[int]],
    columns: tuple[int, ...],
    prime: int,
) -> list[list[int]]:
    if not columns:
        return []
    rows = [[row[column] % prime for column in columns] for row in generator]
    return canonical_basis(rows, len(columns), prime)


def support_subcode_basis(
    image_basis: list[list[int]],
    support_positions: tuple[int, ...],
    prime: int,
) -> list[list[int]]:
    rho = len(image_basis)
    if rho == 0 or not support_positions:
        return []
    support_set = set(support_positions)
    width = len(image_basis[0])
    complement_positions = [index for index in range(width) if index not in support_set]
    equations = [
        [image_basis[basis_index][coordinate] for basis_index in range(rho)]
        for coordinate in complement_positions
    ]
    coefficient_basis = nullspace_basis(equations, rho, prime)
    out: list[list[int]] = []
    for coeffs in coefficient_basis:
        row = [0] * len(support_positions)
        for coeff, basis_row in zip(coeffs, image_basis):
            if coeff == 0:
                continue
            for out_index, coordinate in enumerate(support_positions):
                row[out_index] = (row[out_index] + coeff * basis_row[coordinate]) % prime
        out.append(row)
    return canonical_basis(out, len(support_positions), prime)


def projected_rank(subspace_basis: list[list[int]], mask: int, prime: int) -> int:
    if not subspace_basis or mask == 0:
        return 0
    width = len(subspace_basis[0])
    positions = [index for index in range(width) if (mask >> index) & 1]
    rows = [[row[index] for index in positions] for row in subspace_basis]
    return len(rref_with_pivots(rows, len(positions), prime)[1])


def two_copy_generic_kernel_dim(subspace_basis: list[list[int]], prime: int) -> int:
    delta = len(subspace_basis)
    if delta == 0:
        return 0
    width = len(subspace_basis[0])
    full_mask = (1 << width) - 1
    generic_rank = 2 * delta
    for mask in range(full_mask + 1):
        rank = projected_rank(subspace_basis, mask, prime)
        candidate = (width - mask.bit_count()) + 2 * rank
        generic_rank = min(generic_rank, candidate)
    return 2 * delta - generic_rank


def gaussian_binomial(ambient_dim: int, sub_dim: int, prime: int) -> int:
    if sub_dim < 0 or sub_dim > ambient_dim:
        return 0
    sub_dim = min(sub_dim, ambient_dim - sub_dim)
    out = 1
    for i in range(sub_dim):
        out *= prime ** (ambient_dim - i) - 1
        out //= prime ** (sub_dim - i) - 1
    return out


def rref_subspaces(ambient_dim: int, sub_dim: int, prime: int) -> Iterator[list[list[int]]]:
    if sub_dim < 0 or sub_dim > ambient_dim:
        return
    for pivots in itertools.combinations(range(ambient_dim), sub_dim):
        pivot_set = set(pivots)
        free_positions: list[tuple[int, int]] = []
        for row, pivot in enumerate(pivots):
            for col in range(pivot + 1, ambient_dim):
                if col not in pivot_set:
                    free_positions.append((row, col))
        for values in itertools.product(range(prime), repeat=len(free_positions)):
            rows = [[0] * ambient_dim for _ in range(sub_dim)]
            for row, pivot in enumerate(pivots):
                rows[row][pivot] = 1
            for (row, col), value in zip(free_positions, values):
                rows[row][col] = value
            yield rows


def coordinate_subspace_to_basis(
    coordinate_rows: list[list[int]],
    ambient_basis: BasisKey,
    width: int,
    prime: int,
) -> BasisKey:
    vectors: list[list[int]] = []
    for coordinate_row in coordinate_rows:
        vector = [0] * width
        for coeff, basis_row in zip(coordinate_row, ambient_basis):
            if coeff == 0:
                continue
            for col, value in enumerate(basis_row):
                vector[col] = (vector[col] + coeff * value) % prime
        vectors.append(vector)
    return basis_key(vectors, width, prime)


def contained_in(subspace: BasisKey, container: BasisKey, width: int, prime: int) -> bool:
    return len(intersection_basis(subspace, container, width, prime)) == len(subspace)


def iter_subflags(
    outer_key: BasisKey,
    inner_key: BasisKey,
    r1: int,
    r0: int,
    width: int,
    prime: int,
) -> Iterator[tuple[BasisKey, BasisKey]]:
    for l_coordinates in rref_subspaces(len(inner_key), r0, prime):
        l_key = coordinate_subspace_to_basis(l_coordinates, inner_key, width, prime)
        for v_coordinates in rref_subspaces(len(outer_key), r1, prime):
            v_key = coordinate_subspace_to_basis(v_coordinates, outer_key, width, prime)
            if contained_in(l_key, v_key, width, prime):
                yield l_key, v_key


def local_support_profile(
    generator: list[list[int]],
    singleton: tuple[int, ...],
    visible: tuple[int, ...],
    prime: int,
) -> tuple[int, int, int]:
    if not visible:
        return 0, 0, 0
    visible_set = set(visible)
    support_positions = tuple(
        index for index, column in enumerate(singleton) if column in visible_set
    )
    singleton_rank = rank_columns(generator, singleton, prime)
    hidden_singleton = tuple(column for column in singleton if column not in visible_set)
    hidden_rank = rank_columns(generator, hidden_singleton, prime)
    delta = singleton_rank - hidden_rank
    comp = matroid_component_count(generator, visible, prime)
    image_basis = restricted_image_basis(generator, singleton, prime)
    support_basis = support_subcode_basis(image_basis, support_positions, prime)
    g = two_copy_generic_kernel_dim(support_basis, prime)
    return delta, comp, g


def intersection_basis(
    left: BasisKey,
    right: BasisKey,
    width: int,
    prime: int,
) -> BasisKey:
    if not left or not right:
        return ()
    left_dim = len(left)
    right_dim = len(right)
    equations: list[list[int]] = []
    for col in range(width):
        row = [left[i][col] for i in range(left_dim)]
        row += [(-right[i][col]) % prime for i in range(right_dim)]
        equations.append(row)
    coeff_basis = nullspace_basis(equations, left_dim + right_dim, prime)
    vectors: list[list[int]] = []
    for coeffs in coeff_basis:
        vector = [0] * width
        for coeff, basis_row in zip(coeffs[:left_dim], left):
            if coeff == 0:
                continue
            for col, value in enumerate(basis_row):
                vector[col] = (vector[col] + coeff * value) % prime
        vectors.append(vector)
    return basis_key(vectors, width, prime)


def intersection_many(bases: list[BasisKey], width: int, prime: int) -> BasisKey:
    if not bases:
        return ()
    out = bases[0]
    for basis in bases[1:]:
        out = intersection_basis(out, basis, width, prime)
        if not out:
            break
    return out


def generic_intersection_dim(dimensions: Iterable[int], ambient_dim: int) -> int:
    dims = list(dimensions)
    if not dims:
        return 0
    return max(0, sum(dims) - (len(dims) - 1) * ambient_dim)


def logq(value: int, prime: int) -> float:
    if value <= 0:
        return NEG_INF
    return math.log(value, prime)


def projective_logq_from_linear_dim(dim: int) -> float:
    if dim <= 0:
        return NEG_INF
    return float(dim - 1)


def projective_gain(observed: float, generic: float) -> float:
    if observed == NEG_INF and generic == NEG_INF:
        return 0.0
    if generic == NEG_INF:
        return math.inf if observed > NEG_INF else 0.0
    if observed == NEG_INF:
        return NEG_INF
    return observed - generic


def fmt_float(value: float) -> str:
    if value == NEG_INF:
        return "-inf"
    if value == math.inf:
        return "inf"
    return f"{value:.10f}"


def parse_int_set(text: str, low: int, high: int) -> list[int]:
    if text == "all":
        return list(range(low, high + 1))
    values: list[int] = []
    for part in text.split(","):
        part = part.strip()
        if not part:
            continue
        if ".." in part:
            start_text, end_text = part.split("..", 1)
            start = int(start_text)
            end = int(end_text)
            step = 1 if end >= start else -1
            values.extend(range(start, end + step, step))
        else:
            values.append(int(part))
    out = sorted(set(values))
    for value in out:
        if value < low or value > high:
            raise ValueError(f"value {value} outside {low}..{high}")
    return out


def parse_columns(text: str) -> tuple[int, ...]:
    if not text:
        return ()
    columns = tuple(sorted(int(value) for value in text.split(":") if value != ""))
    if len(columns) != len(set(columns)):
        raise ValueError("--inner-columns contains duplicates")
    return columns


def columns_text(columns: tuple[int, ...]) -> str:
    return ":".join(str(column) for column in columns)


def basis_text(key: BasisKey) -> str:
    return "/".join(":".join(str(value) for value in row) for row in key)


def choose_by_positions(columns: tuple[int, ...], positions: Iterable[int]) -> tuple[int, ...]:
    return tuple(columns[position] for position in sorted(positions))


def subtract_columns(left: tuple[int, ...], right: tuple[int, ...]) -> tuple[int, ...]:
    right_set = set(right)
    return tuple(column for column in left if column not in right_set)


def path_indices(columns: tuple[int, ...], expansion: int) -> set[int]:
    return {column // expansion for column in columns}


def full_sibling_pair_count(columns: tuple[int, ...], expansion: int) -> int:
    paths = path_indices(columns, expansion)
    return sum(1 for path in range(0, max(paths, default=-1) + 1, 2) if path in paths and path + 1 in paths)


def complete_path_stride_count(
    columns: tuple[int, ...],
    child_k: int,
    expansion: int,
    stride_modulus: int,
) -> int:
    if stride_modulus <= 0:
        return 0
    if child_k % stride_modulus != 0:
        return 0
    paths = path_indices(columns, expansion)
    class_size = child_k // stride_modulus
    complete = 0
    for residue in range(stride_modulus):
        if all(path in paths for path in range(residue, child_k, stride_modulus)):
            complete += 1
    return complete if class_size > 0 else 0


def complete_column_stride_count(
    columns: tuple[int, ...],
    child_k: int,
    expansion: int,
    stride_modulus: int,
) -> int:
    if stride_modulus <= 0:
        return 0
    if child_k % stride_modulus != 0:
        return 0
    column_set = set(columns)
    complete = 0
    for residue in range(stride_modulus):
        ok = True
        for path in range(residue, child_k, stride_modulus):
            for copy in range(expansion):
                if path * expansion + copy not in column_set:
                    ok = False
                    break
            if not ok:
                break
        if ok:
            complete += 1
    return complete


def encode_vector(
    message: tuple[int, ...],
    generator: list[list[int]],
    prime: int,
) -> tuple[int, ...]:
    out = [0] * len(generator[0])
    for coeff, row in zip(message, generator):
        if coeff == 0:
            continue
        for col, value in enumerate(row):
            out[col] = (out[col] + coeff * value) % prime
    return tuple(out)


def zero_supported_subspace(
    generator: list[list[int]],
    live_rows: tuple[int, ...],
    zero_columns: tuple[int, ...],
    prime: int,
) -> BasisKey:
    equations = [
        [generator[row][column] % prime for row in live_rows]
        for column in zero_columns
    ]
    coeff_basis = nullspace_basis(equations, len(live_rows), prime)
    width = len(generator)
    vectors: list[list[int]] = []
    for coeffs in coeff_basis:
        vector = [0] * width
        for coeff, row in zip(coeffs, live_rows):
            vector[row] = coeff
        vectors.append(vector)
    return basis_key(vectors, width, prime)


def zero_supported_dim(
    generator: list[list[int]],
    live_rows: tuple[int, ...],
    zero_columns: tuple[int, ...],
    prime: int,
) -> int:
    equations = [
        [generator[row][column] % prime for row in live_rows]
        for column in zero_columns
    ]
    return len(live_rows) - len(rref_with_pivots(equations, len(live_rows), prime)[1])


def project_parent_subspace(
    parent_basis: BasisKey,
    coefficient_basis: list[list[int]] | None,
    child_k: int,
    prime: int,
) -> BasisKey:
    """Return span of left/right child projections of a subspace of parent_basis."""

    if coefficient_basis is None:
        coefficient_basis = [
            [1 if i == j else 0 for i in range(len(parent_basis))]
            for j in range(len(parent_basis))
        ]
    projected_rows: list[list[int]] = []
    for coeffs in coefficient_basis:
        parent_vector = [0] * (2 * child_k)
        for coeff, basis_row in zip(coeffs, parent_basis):
            if coeff == 0:
                continue
            for col, value in enumerate(basis_row):
                parent_vector[col] = (parent_vector[col] + coeff * value) % prime
        left = parent_vector[:child_k]
        right = parent_vector[child_k:]
        if any(left):
            projected_rows.append(left)
        if any(right):
            projected_rows.append(right)
    return basis_key(projected_rows, child_k, prime)


def rank_functionals(functionals: list[tuple[int, ...]], width: int, prime: int) -> int:
    return len(rref_with_pivots(functionals, width, prime)[1])


def is_zero_functional(functional: tuple[int, ...]) -> bool:
    return all(value == 0 for value in functional)


@dataclass
class FlagRecord:
    copy_index: int
    seed: int
    h: int
    t: int
    z: int
    p: int
    s: int
    a: int
    tau: int
    kappa: int
    r1: int
    r0: int
    z_v: int
    z_l: int
    delta: int
    comp: int
    g: int
    l_key: BasisKey
    v_key: BasisKey
    dim_l: int
    dim_v: int
    occurrences: int
    first_inner: tuple[int, ...]
    first_outer: tuple[int, ...]
    first_paired: tuple[int, ...]
    first_visible: tuple[int, ...]
    min_paired_size: int
    max_paired_size: int
    min_visible_size: int
    max_visible_size: int
    max_inner_complete_path_strides: int
    max_visible_complete_path_strides: int
    max_inner_complete_column_strides: int
    max_visible_complete_column_strides: int
    max_inner_full_sibling_pairs: int
    max_visible_full_sibling_pairs: int

    @property
    def quotient_dim(self) -> int:
        return self.dim_v - self.dim_l


@dataclass
class CopyEnumeration:
    copy_index: int
    seed: int
    raw_profiles: int
    skipped_subflag_profiles: int
    exact_flags: list[FlagRecord]


def update_flag_record(
    record: FlagRecord,
    *,
    paired: tuple[int, ...],
    visible: tuple[int, ...],
    inner_complete_path_strides: int,
    visible_complete_path_strides: int,
    inner_complete_column_strides: int,
    visible_complete_column_strides: int,
    inner_full_sibling_pairs: int,
    visible_full_sibling_pairs: int,
) -> None:
    record.occurrences += 1
    paired_size = len(paired)
    visible_size = len(visible)
    record.min_paired_size = min(record.min_paired_size, paired_size)
    record.max_paired_size = max(record.max_paired_size, paired_size)
    record.min_visible_size = min(record.min_visible_size, visible_size)
    record.max_visible_size = max(record.max_visible_size, visible_size)
    record.max_inner_complete_path_strides = max(
        record.max_inner_complete_path_strides,
        inner_complete_path_strides,
    )
    record.max_visible_complete_path_strides = max(
        record.max_visible_complete_path_strides,
        visible_complete_path_strides,
    )
    record.max_inner_complete_column_strides = max(
        record.max_inner_complete_column_strides,
        inner_complete_column_strides,
    )
    record.max_visible_complete_column_strides = max(
        record.max_visible_complete_column_strides,
        visible_complete_column_strides,
    )
    record.max_inner_full_sibling_pairs = max(
        record.max_inner_full_sibling_pairs,
        inner_full_sibling_pairs,
    )
    record.max_visible_full_sibling_pairs = max(
        record.max_visible_full_sibling_pairs,
        visible_full_sibling_pairs,
    )


def iter_column_sets(
    n: int,
    size: int,
    samples: int,
    rng: random.Random,
) -> Iterator[tuple[int, ...]]:
    if samples <= 0:
        yield from itertools.combinations(range(n), size)
        return
    seen: set[tuple[int, ...]] = set()
    attempts = 0
    max_attempts = max(100, 20 * samples)
    while len(seen) < samples and attempts < max_attempts:
        attempts += 1
        columns = tuple(sorted(rng.sample(range(n), size)))
        if columns in seen:
            continue
        seen.add(columns)
        yield columns


def enumerate_copy_flags(
    *,
    copy_index: int,
    seed: int,
    prime: int,
    child_depth: int,
    h: int,
    t: int,
    tau: int,
    z: int | None,
    expansion: int,
    inner_size: int,
    inner_columns: tuple[int, ...] | None,
    paired_sizes: list[int],
    visible_sizes: list[int],
    inner_samples: int,
    max_inner_sets: int,
    max_raw_profiles: int,
    max_flags_per_copy: int,
    min_l_dim: int,
    min_v_dim: int,
    min_quotient_dim: int,
    max_quotient_dim: int | None,
    exact_r0: int | None,
    exact_r1: int | None,
    enumerate_subflags: bool,
    max_subflags_per_profile: int,
    stride_modulus: int,
) -> CopyEnumeration:
    rng = random.Random(seed)
    generator = rfc_generator_prime(child_depth, expansion, prime, rng)
    child_k = 1 << child_depth
    child_n = expansion * child_k

    if inner_columns is not None:
        if len(inner_columns) != inner_size:
            raise SystemExit("--inner-size must match --inner-columns length")
        if any(column < 0 or column >= child_n for column in inner_columns):
            raise SystemExit("--inner-columns outside child coordinate range")
    if inner_size > child_n:
        raise SystemExit("--inner-size exceeds child code length")
    exact_inner_count = math.comb(child_n, inner_size)
    if inner_columns is None and inner_samples <= 0 and exact_inner_count > max_inner_sets:
        raise SystemExit(
            f"copy {copy_index}: exact inner-set count {exact_inner_count} exceeds "
            f"--max-inner-sets {max_inner_sets}; use --inner-samples or raise the cap"
        )

    kernel_cache: dict[tuple[int, ...], BasisKey] = {}

    def cached_kernel(columns: tuple[int, ...]) -> BasisKey:
        if columns not in kernel_cache:
            kernel_cache[columns] = basis_key(
                kernel_basis_for_zero_columns(generator, columns, prime),
                child_k,
                prime,
            )
        return kernel_cache[columns]

    if tau < 0 or tau > t:
        raise SystemExit("--tau must be in 0..t")
    if enumerate_subflags and (exact_r0 is None or exact_r1 is None):
        raise SystemExit("--enumerate-subflags requires --exact-r0 and --exact-r1")
    kappa = t - tau

    flags: dict[tuple[tuple[int, ...], BasisKey, BasisKey], FlagRecord] = {}
    raw_profiles = 0
    skipped_subflag_profiles = 0
    inner_seen = 0

    inner_iterable = (
        [inner_columns]
        if inner_columns is not None
        else iter_column_sets(child_n, inner_size, inner_samples, rng)
    )
    for inner in inner_iterable:
        inner_seen += 1
        if inner_seen > max_inner_sets:
            raise SystemExit(f"copy {copy_index}: exceeded --max-inner-sets")
        inner_complete_path_strides = complete_path_stride_count(
            inner,
            child_k,
            expansion,
            stride_modulus,
        )
        inner_complete_column_strides = complete_column_stride_count(
            inner,
            child_k,
            expansion,
            stride_modulus,
        )
        inner_full_sibling_pairs = full_sibling_pair_count(inner, expansion)

        for paired_size in paired_sizes:
            if paired_size > inner_size:
                continue
            for paired_positions in itertools.combinations(range(inner_size), paired_size):
                paired = choose_by_positions(inner, paired_positions)
                paired_position_set = set(paired_positions)
                singleton_positions = [
                    index for index in range(inner_size) if index not in paired_position_set
                ]
                singleton_size = len(singleton_positions)
                singleton = choose_by_positions(inner, singleton_positions)
                parent_z = 2 * paired_size + singleton_size
                if z is not None and parent_z != z:
                    continue
                for visible_size in visible_sizes:
                    if visible_size > singleton_size:
                        continue
                    for visible_positions_local in itertools.combinations(
                        range(singleton_size),
                        visible_size,
                    ):
                        visible_positions = [
                            singleton_positions[index] for index in visible_positions_local
                        ]
                        visible = choose_by_positions(inner, visible_positions)
                        outer = subtract_columns(inner, visible)
                        z_v = paired_size + singleton_size - visible_size
                        z_l = paired_size + singleton_size
                        raw_profiles += 1
                        if raw_profiles > max_raw_profiles:
                            raise SystemExit(f"copy {copy_index}: exceeded --max-raw-profiles")

                        inner_kernel = cached_kernel(inner)
                        outer_kernel = cached_kernel(outer)
                        delta, comp, g = local_support_profile(
                            generator,
                            singleton,
                            visible,
                            prime,
                        )
                        if tau > delta:
                            continue
                        if enumerate_subflags:
                            assert exact_r0 is not None and exact_r1 is not None
                            subflag_budget = gaussian_binomial(
                                len(inner_kernel),
                                exact_r0,
                                prime,
                            ) * gaussian_binomial(len(outer_kernel), exact_r1, prime)
                            if subflag_budget > max_subflags_per_profile:
                                skipped_subflag_profiles += 1
                                continue
                            flag_candidates = list(
                                iter_subflags(
                                    outer_kernel,
                                    inner_kernel,
                                    exact_r1,
                                    exact_r0,
                                    child_k,
                                    prime,
                                )
                            )
                        else:
                            dim_l = len(inner_kernel)
                            dim_v = len(outer_kernel)
                            if exact_r0 is not None and dim_l != exact_r0:
                                continue
                            if exact_r1 is not None and dim_v != exact_r1:
                                continue
                            flag_candidates = [(inner_kernel, outer_kernel)]

                        visible_complete_path_strides = complete_path_stride_count(
                            visible,
                            child_k,
                            expansion,
                            stride_modulus,
                        )
                        visible_complete_column_strides = complete_column_stride_count(
                            visible,
                            child_k,
                            expansion,
                            stride_modulus,
                        )
                        visible_full_sibling_pairs = full_sibling_pair_count(visible, expansion)
                        for l_key, v_key in flag_candidates:
                            dim_l = len(l_key)
                            dim_v = len(v_key)
                            quotient_dim = dim_v - dim_l
                            if dim_l < min_l_dim or dim_v < min_v_dim:
                                continue
                            if quotient_dim < min_quotient_dim:
                                continue
                            if max_quotient_dim is not None and quotient_dim > max_quotient_dim:
                                continue
                            shape_key = (
                                h,
                                t,
                                parent_z,
                                paired_size,
                                singleton_size,
                                visible_size,
                                tau,
                                kappa,
                                dim_v,
                                dim_l,
                                z_v,
                                z_l,
                                delta,
                                comp,
                                g,
                            )
                            key = (shape_key, l_key, v_key)
                            if key in flags:
                                update_flag_record(
                                    flags[key],
                                    paired=paired,
                                    visible=visible,
                                    inner_complete_path_strides=inner_complete_path_strides,
                                    visible_complete_path_strides=visible_complete_path_strides,
                                    inner_complete_column_strides=inner_complete_column_strides,
                                    visible_complete_column_strides=visible_complete_column_strides,
                                    inner_full_sibling_pairs=inner_full_sibling_pairs,
                                    visible_full_sibling_pairs=visible_full_sibling_pairs,
                                )
                                continue
                            if len(flags) >= max_flags_per_copy:
                                raise SystemExit(f"copy {copy_index}: exceeded --max-flags-per-copy")
                            flags[key] = FlagRecord(
                                copy_index=copy_index,
                                seed=seed,
                                h=h,
                                t=t,
                                z=parent_z,
                                p=paired_size,
                                s=singleton_size,
                                a=visible_size,
                                tau=tau,
                                kappa=kappa,
                                r1=dim_v,
                                r0=dim_l,
                                z_v=z_v,
                                z_l=z_l,
                                delta=delta,
                                comp=comp,
                                g=g,
                                l_key=l_key,
                                v_key=v_key,
                                dim_l=dim_l,
                                dim_v=dim_v,
                                occurrences=1,
                                first_inner=inner,
                                first_outer=outer,
                                first_paired=paired,
                                first_visible=visible,
                                min_paired_size=len(paired),
                                max_paired_size=len(paired),
                                min_visible_size=len(visible),
                                max_visible_size=len(visible),
                                max_inner_complete_path_strides=inner_complete_path_strides,
                                max_visible_complete_path_strides=visible_complete_path_strides,
                                max_inner_complete_column_strides=inner_complete_column_strides,
                                max_visible_complete_column_strides=visible_complete_column_strides,
                                max_inner_full_sibling_pairs=inner_full_sibling_pairs,
                                max_visible_full_sibling_pairs=visible_full_sibling_pairs,
                            )

    return CopyEnumeration(
        copy_index=copy_index,
        seed=seed,
        raw_profiles=raw_profiles,
        skipped_subflag_profiles=skipped_subflag_profiles,
        exact_flags=list(flags.values()),
    )


def enumerate_copy_near_stride_w_flags(
    *,
    copy_index: int,
    seed: int,
    prime: int,
    parent_depth: int,
    h: int,
    t: int,
    tau: int,
    z: int | None,
    near_live_rows: int,
    paired_sizes: list[int],
    visible_sizes: list[int],
    max_w_spaces: int,
    max_raw_profiles: int,
    max_flags_per_copy: int,
    min_l_dim: int,
    min_v_dim: int,
    min_quotient_dim: int,
    max_quotient_dim: int | None,
    exact_r0: int | None,
    exact_r1: int | None,
    stride_modulus: int,
) -> CopyEnumeration:
    if parent_depth < 1:
        raise SystemExit("targeted near-stride mode requires parent depth >= 1")
    if near_live_rows <= 0:
        raise SystemExit("--near-live-rows must be positive")

    parent_k = 1 << parent_depth
    child_depth = parent_depth - 1
    child_k = 1 << child_depth
    if parent_k % near_live_rows != 0:
        raise SystemExit("--near-live-rows must divide the parent dimension")
    if near_live_rows != 4:
        raise SystemExit("targeted near-stride mode currently implements the m=4 class only")

    parent_generator = rfc_generator_prime(parent_depth, 1, prime, random.Random(seed))
    child_generator = rfc_generator_prime(child_depth, 1, prime, random.Random(seed))

    kappa = t - tau
    flags: dict[tuple[tuple[int, ...], BasisKey, BasisKey], FlagRecord] = {}
    raw_profiles = 0
    checked_w_spaces = 0

    for block_start in range(0, parent_k, near_live_rows):
        live_rows = tuple(range(block_start, block_start + near_live_rows))
        for core_residue in range(near_live_rows):
            for extra_residue in range(near_live_rows):
                if extra_residue == core_residue:
                    continue
                support_residues = {core_residue, extra_residue}
                support_columns = tuple(
                    column for column in range(parent_k) if column % near_live_rows in support_residues
                )
                support_set = set(support_columns)
                zero_columns = tuple(column for column in range(parent_k) if column not in support_set)
                w_key = zero_supported_subspace(
                    parent_generator,
                    live_rows,
                    zero_columns,
                    prime,
                )
                if len(w_key) != t:
                    continue
                checked_w_spaces += 1
                if checked_w_spaces > max_w_spaces:
                    raise SystemExit("targeted near-stride mode exceeded --near-max-w-spaces")

                encoded_rows = [
                    encode_vector(tuple(row), parent_generator, prime)
                    for row in w_key
                ]
                paired_candidates: list[int] = []
                visible_candidates: list[tuple[int, tuple[int, ...]]] = []
                for child_col in range(child_k):
                    left = tuple(row[child_col] for row in encoded_rows)
                    right = tuple(row[child_k + child_col] for row in encoded_rows)
                    left_zero = is_zero_functional(left)
                    right_zero = is_zero_functional(right)
                    if left_zero and right_zero:
                        paired_candidates.append(child_col)
                    elif left_zero:
                        visible_candidates.append((child_col, right))
                    elif right_zero:
                        visible_candidates.append((child_col, left))

                v_key = project_parent_subspace(w_key, None, child_k, prime)
                for paired_size in paired_sizes:
                    if paired_size > len(paired_candidates):
                        continue
                    for paired in itertools.combinations(paired_candidates, paired_size):
                        paired_tuple = tuple(sorted(paired))
                        for visible_size in visible_sizes:
                            if visible_size > len(visible_candidates):
                                continue
                            for visible_indices in itertools.combinations(
                                range(len(visible_candidates)),
                                visible_size,
                            ):
                                selected_visible = [visible_candidates[index] for index in visible_indices]
                                visible_tuple = tuple(sorted(column for column, _func in selected_visible))
                                visible_funcs = [func for _column, func in selected_visible]
                                actual_tau = rank_functionals(visible_funcs, t, prime)
                                if actual_tau != tau:
                                    continue
                                k_coeff_basis = nullspace_basis(visible_funcs, t, prime)
                                if len(k_coeff_basis) != kappa:
                                    continue

                                parent_z = 2 * paired_size + visible_size
                                if z is not None and parent_z != z:
                                    continue
                                raw_profiles += 1
                                if raw_profiles > max_raw_profiles:
                                    raise SystemExit("targeted near-stride mode exceeded --max-raw-profiles")

                                l_key = project_parent_subspace(
                                    w_key,
                                    k_coeff_basis,
                                    child_k,
                                    prime,
                                )
                                dim_l = len(l_key)
                                dim_v = len(v_key)
                                if exact_r0 is not None and dim_l != exact_r0:
                                    continue
                                if exact_r1 is not None and dim_v != exact_r1:
                                    continue
                                quotient_dim = dim_v - dim_l
                                if dim_l < min_l_dim or dim_v < min_v_dim:
                                    continue
                                if quotient_dim < min_quotient_dim:
                                    continue
                                if max_quotient_dim is not None and quotient_dim > max_quotient_dim:
                                    continue

                                delta, comp, g = local_support_profile(
                                    child_generator,
                                    visible_tuple,
                                    visible_tuple,
                                    prime,
                                )
                                if tau > delta:
                                    continue

                                z_v = paired_size
                                z_l = paired_size + visible_size
                                inner = tuple(sorted(paired_tuple + visible_tuple))
                                outer = paired_tuple
                                inner_complete_path_strides = complete_path_stride_count(
                                    inner,
                                    child_k,
                                    1,
                                    stride_modulus,
                                )
                                visible_complete_path_strides = complete_path_stride_count(
                                    visible_tuple,
                                    child_k,
                                    1,
                                    stride_modulus,
                                )
                                inner_complete_column_strides = complete_column_stride_count(
                                    inner,
                                    child_k,
                                    1,
                                    stride_modulus,
                                )
                                visible_complete_column_strides = complete_column_stride_count(
                                    visible_tuple,
                                    child_k,
                                    1,
                                    stride_modulus,
                                )
                                inner_full_sibling_pairs = full_sibling_pair_count(inner, 1)
                                visible_full_sibling_pairs = full_sibling_pair_count(visible_tuple, 1)
                                shape_key = (
                                    h,
                                    t,
                                    parent_z,
                                    paired_size,
                                    visible_size,
                                    visible_size,
                                    tau,
                                    kappa,
                                    dim_v,
                                    dim_l,
                                    z_v,
                                    z_l,
                                    delta,
                                    comp,
                                    g,
                                )
                                key = (shape_key, l_key, v_key)
                                if key in flags:
                                    update_flag_record(
                                        flags[key],
                                        paired=paired_tuple,
                                        visible=visible_tuple,
                                        inner_complete_path_strides=inner_complete_path_strides,
                                        visible_complete_path_strides=visible_complete_path_strides,
                                        inner_complete_column_strides=inner_complete_column_strides,
                                        visible_complete_column_strides=visible_complete_column_strides,
                                        inner_full_sibling_pairs=inner_full_sibling_pairs,
                                        visible_full_sibling_pairs=visible_full_sibling_pairs,
                                    )
                                    continue
                                if len(flags) >= max_flags_per_copy:
                                    raise SystemExit("targeted near-stride mode exceeded --max-flags-per-copy")
                                flags[key] = FlagRecord(
                                    copy_index=copy_index,
                                    seed=seed,
                                    h=h,
                                    t=t,
                                    z=parent_z,
                                    p=paired_size,
                                    s=visible_size,
                                    a=visible_size,
                                    tau=tau,
                                    kappa=kappa,
                                    r1=dim_v,
                                    r0=dim_l,
                                    z_v=z_v,
                                    z_l=z_l,
                                    delta=delta,
                                    comp=comp,
                                    g=g,
                                    l_key=l_key,
                                    v_key=v_key,
                                    dim_l=dim_l,
                                    dim_v=dim_v,
                                    occurrences=1,
                                    first_inner=inner,
                                    first_outer=outer,
                                    first_paired=paired_tuple,
                                    first_visible=visible_tuple,
                                    min_paired_size=len(paired_tuple),
                                    max_paired_size=len(paired_tuple),
                                    min_visible_size=len(visible_tuple),
                                    max_visible_size=len(visible_tuple),
                                    max_inner_complete_path_strides=inner_complete_path_strides,
                                    max_visible_complete_path_strides=visible_complete_path_strides,
                                    max_inner_complete_column_strides=inner_complete_column_strides,
                                    max_visible_complete_column_strides=visible_complete_column_strides,
                                    max_inner_full_sibling_pairs=inner_full_sibling_pairs,
                                    max_visible_full_sibling_pairs=visible_full_sibling_pairs,
                                )

    return CopyEnumeration(
        copy_index=copy_index,
        seed=seed,
        raw_profiles=raw_profiles,
        skipped_subflag_profiles=0,
        exact_flags=list(flags.values()),
    )


def row_block(block_index: int, live_rows: int) -> tuple[int, ...]:
    start = block_index * live_rows
    return tuple(range(start, start + live_rows))


def stride_class(stride_index: int, k: int, live_rows: int) -> tuple[int, ...]:
    return tuple(range(stride_index, k, live_rows))


def complete_stride_combinatorial_rows(depth: int, live_rows: int) -> list[dict[str, int | float | str]]:
    if live_rows != 4:
        raise SystemExit("complete-stride combinatorial gate currently implements m=4 only")
    k = 1 << depth
    if k != 16:
        raise SystemExit("complete-stride combinatorial gate currently targets depth 4 / k=16")

    rows: list[dict[str, int | float | str]] = []
    lift_bound = 3.0
    for block_index in range(k // live_rows):
        block = row_block(block_index, live_rows)
        for left_stride, right_stride in itertools.combinations(range(live_rows), 2):
            left_class = stride_class(left_stride, k, live_rows)
            right_class = stride_class(right_stride, k, live_rows)
            omega = tuple(sorted(left_class + right_class))
            outer_zero = tuple(column for column in range(k) if column not in set(omega))
            support_key = f"B={columns_text(block)}|Omega={columns_text(omega)}"
            for kernel_stride, visible_stride, kernel_class, visible_class in [
                (left_stride, right_stride, left_class, right_class),
                (right_stride, left_stride, right_class, left_class),
            ]:
                inner_zero = tuple(column for column in range(k) if column not in set(kernel_class))
                rows.append(
                    {
                        "field_prime": "NA",
                        "copy_id": "NA",
                        "level_h": depth,
                        "h": depth,
                        "t": 2,
                        "z": len(outer_zero),
                        "p": len(outer_zero),
                        "s": len(visible_class),
                        "a": len(visible_class),
                        "tau": 1,
                        "kappa": 1,
                        "r1": 2,
                        "r0": 1,
                        "z_V": len(outer_zero),
                        "z_L": len(inner_zero),
                        "delta": 1,
                        "comp": 1,
                        "g": 0,
                        "row_block": columns_text(block),
                        "kernel_stride": kernel_stride,
                        "visible_stride": visible_stride,
                        "exact_support_key": support_key,
                        "outer_zero_key": columns_text(outer_zero),
                        "inner_zero_key": columns_text(inner_zero),
                        "canonical_flag_count": 1,
                        "canonical_parent_W_count": 1,
                        "duplicate_certificate_count": 0,
                        "max_certificates_per_flag": 1,
                        "complete_extra_stride_count": 1,
                        "observed_parent_lift_logq": 0.0,
                        "lift_bound_logq": lift_bound,
                        "parent_lift_excess_logq": -lift_bound,
                        "common_v_dim": "NA",
                        "common_l_dim": "NA",
                        "generic_v_dim": "NA",
                        "generic_l_dim": "NA",
                        "v_excess": "NA",
                        "l_excess": "NA",
                        "status": "ok",
                    }
                )
    return rows


def print_complete_stride_combinatorial_gate(depth: int, live_rows: int) -> list[dict[str, int | float | str]]:
    rows = complete_stride_combinatorial_rows(depth, live_rows)
    support_keys = {str(row["exact_support_key"]) for row in rows}
    ordered_flags = len(rows)
    support_count = len(support_keys)
    status = "ok" if support_count == 24 and ordered_flags == 48 else "bad-count"
    print("targeted_combinatorial_counts")
    print("depth,k,live_rows,unmarked_support_pairs,ordered_flags,expected_support_pairs,expected_ordered_flags,status")
    print(f"{depth},{1 << depth},{live_rows},{support_count},{ordered_flags},24,48,{status}")
    print_rows("targeted_combinatorial_ordered_flags", rows)
    if status != "ok":
        raise SystemExit("complete-stride combinatorial gate failed expected 24/48 counts")
    return rows


def complete_stride_finite_field_rows(
    depth: int,
    live_rows: int,
    prime: int,
    seed: int,
) -> list[dict[str, int | float | str]]:
    if live_rows != 4:
        raise SystemExit("complete-stride finite-field gate currently implements m=4 only")
    k = 1 << depth
    if k != 16:
        raise SystemExit("complete-stride finite-field gate currently targets depth 4 / k=16")

    generator = rfc_generator_prime(depth, 1, prime, random.Random(seed))
    rows: list[dict[str, int | float | str]] = []
    lift_bound = 3.0
    for block_index in range(k // live_rows):
        block = row_block(block_index, live_rows)
        for left_stride, right_stride in itertools.combinations(range(live_rows), 2):
            left_class = stride_class(left_stride, k, live_rows)
            right_class = stride_class(right_stride, k, live_rows)
            omega = tuple(sorted(left_class + right_class))
            omega_set = set(omega)
            outer_zero = tuple(column for column in range(k) if column not in omega_set)
            k_omega = zero_supported_subspace(generator, block, outer_zero, prime)

            left_inner_zero = tuple(column for column in range(k) if column not in set(left_class))
            right_inner_zero = tuple(column for column in range(k) if column not in set(right_class))
            ell_left = zero_supported_subspace(generator, block, left_inner_zero, prime)
            ell_right = zero_supported_subspace(generator, block, right_inner_zero, prime)
            span_lines = basis_key(list(ell_left) + list(ell_right), k, prime)
            line_intersection_dim = len(intersection_basis(ell_left, ell_right, k, prime))
            direct_sum_ok = (
                len(k_omega) == 2
                and len(ell_left) == 1
                and len(ell_right) == 1
                and line_intersection_dim == 0
                and span_lines == k_omega
            )
            support_status = "ok" if direct_sum_ok else "bad-direct-sum"
            support_key = f"B={columns_text(block)}|Omega={columns_text(omega)}"

            for kernel_stride, visible_stride, kernel_class, visible_class, inner_zero, l_key in [
                (left_stride, right_stride, left_class, right_class, left_inner_zero, ell_left),
                (right_stride, left_stride, right_class, left_class, right_inner_zero, ell_right),
            ]:
                canonical_flag_key = f"L={basis_text(l_key)}|V={basis_text(k_omega)}"
                rows.append(
                    {
                        "field_prime": prime,
                        "copy_id": 0,
                        "level_h": depth,
                        "h": depth,
                        "t": 2,
                        "z": len(outer_zero),
                        "p": len(outer_zero),
                        "s": len(visible_class),
                        "a": len(visible_class),
                        "tau": 1,
                        "kappa": 1,
                        "r1": len(k_omega),
                        "r0": len(l_key),
                        "z_V": len(outer_zero),
                        "z_L": len(inner_zero),
                        "delta": 1,
                        "comp": 1,
                        "g": 0,
                        "row_block": columns_text(block),
                        "kernel_stride": kernel_stride,
                        "visible_stride": visible_stride,
                        "exact_support_key": support_key,
                        "outer_zero_key": columns_text(outer_zero),
                        "inner_zero_key": columns_text(inner_zero),
                        "dim_K_Omega": len(k_omega),
                        "dim_ell_i": len(ell_left),
                        "dim_ell_j": len(ell_right),
                        "line_intersection_dim": line_intersection_dim,
                        "direct_sum_ok": int(direct_sum_ok),
                        "canonical_flag_key": canonical_flag_key,
                        "canonical_flag_count": 1,
                        "canonical_parent_W_count": 1,
                        "duplicate_certificate_count": 0,
                        "max_certificates_per_flag": 1,
                        "complete_extra_stride_count": 1,
                        "observed_parent_lift_logq": 0.0,
                        "lift_bound_logq": lift_bound,
                        "parent_lift_excess_logq": -lift_bound,
                        "common_v_dim": "NA",
                        "common_l_dim": "NA",
                        "generic_v_dim": "NA",
                        "generic_l_dim": "NA",
                        "v_excess": "NA",
                        "l_excess": "NA",
                        "status": support_status,
                    }
                )
    return rows


def print_complete_stride_finite_field_gate(
    depth: int,
    live_rows: int,
    prime: int,
    seed: int,
) -> list[dict[str, int | float | str]]:
    rows = complete_stride_finite_field_rows(depth, live_rows, prime, seed)
    support_keys = {str(row["exact_support_key"]) for row in rows}
    canonical_flags = {str(row["canonical_flag_key"]) for row in rows}
    support_count = len(support_keys)
    ordered_flags = len(rows)
    canonical_ordered_flags = len(canonical_flags)
    dim_k_ok = sum(
        1
        for support_key in support_keys
        if any(row["exact_support_key"] == support_key and row["dim_K_Omega"] == 2 for row in rows)
    )
    ell_i_ok = sum(1 for row in rows if row["r0"] == 1)
    ell_j_ok = sum(1 for row in rows if row["dim_ell_j"] == 1)
    direct_sum_ok = sum(
        1
        for support_key in support_keys
        if all(row["direct_sum_ok"] == 1 for row in rows if row["exact_support_key"] == support_key)
    )
    status = (
        "ok"
        if support_count == 24
        and ordered_flags == 48
        and canonical_ordered_flags == 48
        and dim_k_ok == 24
        and ell_i_ok == 48
        and ell_j_ok == 48
        and direct_sum_ok == 24
        else "bad-finite-field-gate"
    )
    print("targeted_finite_field_counts")
    print(
        "depth,k,prime,seed,live_rows,unmarked_support_pairs,ordered_flags,"
        "canonical_ordered_flags,dim_K_Omega_2_supports,dim_ell_i_1_flags,"
        "dim_ell_j_1_flags,direct_sum_ok_supports,status"
    )
    print(
        f"{depth},{1 << depth},{prime},{seed},{live_rows},{support_count},"
        f"{ordered_flags},{canonical_ordered_flags},{dim_k_ok},{ell_i_ok},"
        f"{ell_j_ok},{direct_sum_ok},{status}"
    )
    print_rows("targeted_finite_field_ordered_flags", rows)
    if status != "ok":
        raise SystemExit("complete-stride finite-field gate failed expected dimensions/counts")
    return rows


def is_complete_stride_omega(omega: tuple[int, ...], k: int, live_rows: int) -> bool:
    if len(omega) != 2 * (k // live_rows):
        return False
    return complete_path_stride_count(omega, k, 1, live_rows) == 2


def columns_mask(columns: tuple[int, ...]) -> int:
    mask = 0
    for column in columns:
        mask |= 1 << column
    return mask


def support_mask(vector: tuple[int, ...]) -> int:
    mask = 0
    for index, value in enumerate(vector):
        if value != 0:
            mask |= 1 << index
    return mask


def canonical_projective_coefficients(width: int, prime: int) -> Iterator[tuple[int, ...]]:
    for coeffs in itertools.product(range(prime), repeat=width):
        if all(value == 0 for value in coeffs):
            continue
        first = next(index for index, value in enumerate(coeffs) if value != 0)
        if coeffs[first] != 1:
            continue
        yield coeffs


def projective_line_support_masks(
    generator: list[list[int]],
    block: tuple[int, ...],
    prime: int,
) -> list[int]:
    masks: list[int] = []
    for coeffs in canonical_projective_coefficients(len(block), prime):
        message = [0] * len(generator)
        for coeff, row in zip(coeffs, block):
            message[row] = coeff
        masks.append(support_mask(encode_vector(tuple(message), generator, prime)))
    return masks


def print_size8_omega_growth_scan(
    *,
    depth: int,
    live_rows: int,
    prime: int,
    seed: int,
    max_omegas: int,
    max_failure_rows: int,
    fast_rank_only: bool,
) -> None:
    if live_rows != 4:
        raise SystemExit("size-8 Omega scan currently implements m=4 only")
    k = 1 << depth
    if k != 16:
        raise SystemExit("size-8 Omega scan currently targets depth 4 / k=16")
    omega_size = 2 * (k // live_rows)
    total_profiles = (k // live_rows) * math.comb(k, omega_size)
    if total_profiles > max_omegas:
        raise SystemExit(
            f"size-8 Omega scan has {total_profiles} profiles; "
            f"raise --omega-scan-max-omegas above {max_omegas} to run"
        )

    generator = rfc_generator_prime(depth, 1, prime, random.Random(seed))
    complete_total = 0
    complete_dim_ge2 = 0
    nonstride_total = 0
    nonstride_dim_ge2 = 0
    max_complete_dim = 0
    max_nonstride_dim = 0
    failures: list[dict[str, int | str]] = []
    grouped: dict[tuple[int, ...], int] = {}
    omega_profiles: list[tuple[tuple[int, ...], int, bool]] = []
    for omega in itertools.combinations(range(k), omega_size):
        omega_tuple = tuple(omega)
        omega_profiles.append(
            (
                omega_tuple,
                columns_mask(omega_tuple),
                is_complete_stride_omega(omega_tuple, k, live_rows),
            )
        )

    for block_index in range(k // live_rows):
        block = row_block(block_index, live_rows)
        block_mask = columns_mask(block)
        line_support_masks = (
            [] if fast_rank_only else projective_line_support_masks(generator, block, prime)
        )
        stride_line_masks: list[int] = []
        for stride_index in range(live_rows):
            stride = stride_class(stride_index, k, live_rows)
            stride_mask = columns_mask(stride)
            stride_zero = tuple(
                column for column in range(k) if ((stride_mask >> column) & 1) == 0
            )
            if zero_supported_dim(generator, block, stride_zero, prime) == 1:
                stride_line_masks.append(stride_mask)
        for omega_tuple, omega_mask, complete in omega_profiles:
            zero_columns = tuple(
                column for column in range(k) if ((omega_mask >> column) & 1) == 0
            )
            dim = zero_supported_dim(generator, block, zero_columns, prime)
            contained_stride_lines = sum(
                1 for stride_mask in stride_line_masks if (stride_mask & ~omega_mask) == 0
            )
            row_block_hits = (block_mask & omega_mask).bit_count()
            contains_row_block = int((block_mask & ~omega_mask) == 0)
            if fast_rank_only:
                group_key = (
                    dim,
                    int(complete),
                    contained_stride_lines,
                    row_block_hits,
                    contains_row_block,
                )
            else:
                contained_kernel_lines = sum(
                    1 for line_mask in line_support_masks if (line_mask & ~omega_mask) == 0
                )
                group_key = (
                    dim,
                    int(complete),
                    contained_stride_lines,
                    contained_kernel_lines,
                    row_block_hits,
                    contains_row_block,
                )
            grouped[group_key] = grouped.get(group_key, 0) + 1
            if complete:
                complete_total += 1
                max_complete_dim = max(max_complete_dim, dim)
                if dim >= 2:
                    complete_dim_ge2 += 1
                elif len(failures) < max_failure_rows:
                    failures.append(
                        {
                            "kind": "complete_stride_dim_lt2",
                            "row_block": columns_text(block),
                            "omega": columns_text(omega_tuple),
                            "dim_K_Omega": dim,
                        }
                    )
            else:
                nonstride_total += 1
                max_nonstride_dim = max(max_nonstride_dim, dim)
                if dim >= 2:
                    nonstride_dim_ge2 += 1
                    if len(failures) < max_failure_rows:
                        failure_row: dict[str, int | str] = {
                            "kind": "nonstride_dim_ge2",
                            "row_block": columns_text(block),
                            "omega": columns_text(omega_tuple),
                            "dim_K_Omega": dim,
                            "contained_stride_line_count": contained_stride_lines,
                        }
                        if not fast_rank_only:
                            failure_row["contained_kernel_line_count"] = contained_kernel_lines
                        failure_row["row_block_hits"] = row_block_hits
                        failures.append(failure_row)

    baseline_ok = complete_total == 24 and complete_dim_ge2 == 24
    if not baseline_ok:
        status = "bad-complete-stride-baseline"
    elif nonstride_dim_ge2 == 0:
        status = "ok"
    else:
        status = "red-flag-classified"
    print("targeted_size8_omega_scan_summary")
    print(
        "depth,k,prime,seed,live_rows,omega_size,total_profiles,"
        "complete_stride_omegas,complete_stride_dim_ge2,nonstride_omegas,"
        "nonstride_dim_ge2,max_complete_dim,max_nonstride_dim,status"
    )
    print(
        f"{depth},{k},{prime},{seed},{live_rows},{omega_size},{total_profiles},"
        f"{complete_total},{complete_dim_ge2},{nonstride_total},{nonstride_dim_ge2},"
        f"{max_complete_dim},{max_nonstride_dim},{status}"
    )
    if failures:
        print_rows("targeted_size8_omega_scan_failures", failures)
    group_rows: list[dict[str, int]] = []
    if fast_rank_only:
        for (
            dim,
            complete,
            contained_stride_lines,
            row_block_hits,
            contains_row_block,
        ), count in sorted(grouped.items()):
            group_rows.append(
                {
                    "dim_K_Omega": dim,
                    "complete_stride_union": complete,
                    "contained_stride_line_count": contained_stride_lines,
                    "row_block_hits": row_block_hits,
                    "contains_row_block_outputs": contains_row_block,
                    "profiles": count,
                }
            )
    else:
        for (
            dim,
            complete,
            contained_stride_lines,
            contained_kernel_lines,
            row_block_hits,
            contains_row_block,
        ), count in sorted(grouped.items()):
            group_rows.append(
                {
                    "dim_K_Omega": dim,
                    "complete_stride_union": complete,
                    "contained_stride_line_count": contained_stride_lines,
                    "contained_kernel_line_count": contained_kernel_lines,
                    "row_block_hits": row_block_hits,
                    "contains_row_block_outputs": contains_row_block,
                    "profiles": count,
                }
            )
    print_rows("targeted_size8_omega_scan_groups", group_rows)
    if not baseline_ok:
        raise SystemExit("size-8 Omega scan failed the complete-stride baseline")


def enumerate_copy_complete_stride_direct_flags(
    *,
    copy_index: int,
    seed: int,
    prime: int,
    depth: int,
    h: int,
    t: int,
    tau: int,
    live_rows: int,
    max_raw_profiles: int,
    max_flags_per_copy: int,
    min_l_dim: int,
    min_v_dim: int,
    min_quotient_dim: int,
    max_quotient_dim: int | None,
    exact_r0: int | None,
    exact_r1: int | None,
    stride_modulus: int,
) -> CopyEnumeration:
    if depth != 4 or live_rows != 4:
        raise SystemExit("complete-stride direct mode currently targets depth 4, m=4")
    if t != 2 or tau != 1:
        raise SystemExit("complete-stride direct mode currently targets t=2, tau=1")

    k = 1 << depth
    generator = rfc_generator_prime(depth, 1, prime, random.Random(seed))
    kappa = t - tau
    flags: dict[tuple[tuple[int, ...], BasisKey, BasisKey], FlagRecord] = {}
    raw_profiles = 0

    for block_index in range(k // live_rows):
        block = row_block(block_index, live_rows)
        for left_stride, right_stride in itertools.combinations(range(live_rows), 2):
            left_class = stride_class(left_stride, k, live_rows)
            right_class = stride_class(right_stride, k, live_rows)
            omega = tuple(sorted(left_class + right_class))
            outer_zero = tuple(column for column in range(k) if column not in set(omega))
            v_key = zero_supported_subspace(generator, block, outer_zero, prime)
            if len(v_key) != 2:
                continue

            for kernel_stride, visible_stride, kernel_class, visible_class in [
                (left_stride, right_stride, left_class, right_class),
                (right_stride, left_stride, right_class, left_class),
            ]:
                inner_zero = tuple(column for column in range(k) if column not in set(kernel_class))
                l_key = zero_supported_subspace(generator, block, inner_zero, prime)
                raw_profiles += 1
                if raw_profiles > max_raw_profiles:
                    raise SystemExit("complete-stride direct mode exceeded --max-raw-profiles")
                dim_l = len(l_key)
                dim_v = len(v_key)
                if dim_l != 1:
                    continue
                if exact_r0 is not None and dim_l != exact_r0:
                    continue
                if exact_r1 is not None and dim_v != exact_r1:
                    continue
                quotient_dim = dim_v - dim_l
                if dim_l < min_l_dim or dim_v < min_v_dim:
                    continue
                if quotient_dim < min_quotient_dim:
                    continue
                if max_quotient_dim is not None and quotient_dim > max_quotient_dim:
                    continue

                z_v = len(outer_zero)
                z_l = len(inner_zero)
                visible_tuple = visible_class
                shape_key = (
                    h,
                    t,
                    z_v,
                    z_v,
                    len(visible_tuple),
                    len(visible_tuple),
                    tau,
                    kappa,
                    dim_v,
                    dim_l,
                    z_v,
                    z_l,
                    1,
                    1,
                    0,
                )
                key = (shape_key, l_key, v_key)
                inner = tuple(sorted(outer_zero + visible_tuple))
                inner_complete_path_strides = complete_path_stride_count(inner, k, 1, stride_modulus)
                visible_complete_path_strides = complete_path_stride_count(visible_tuple, k, 1, stride_modulus)
                inner_complete_column_strides = complete_column_stride_count(inner, k, 1, stride_modulus)
                visible_complete_column_strides = complete_column_stride_count(visible_tuple, k, 1, stride_modulus)
                inner_full_sibling_pairs = full_sibling_pair_count(inner, 1)
                visible_full_sibling_pairs = full_sibling_pair_count(visible_tuple, 1)

                if key in flags:
                    update_flag_record(
                        flags[key],
                        paired=outer_zero,
                        visible=visible_tuple,
                        inner_complete_path_strides=inner_complete_path_strides,
                        visible_complete_path_strides=visible_complete_path_strides,
                        inner_complete_column_strides=inner_complete_column_strides,
                        visible_complete_column_strides=visible_complete_column_strides,
                        inner_full_sibling_pairs=inner_full_sibling_pairs,
                        visible_full_sibling_pairs=visible_full_sibling_pairs,
                    )
                    continue
                if len(flags) >= max_flags_per_copy:
                    raise SystemExit("complete-stride direct mode exceeded --max-flags-per-copy")
                flags[key] = FlagRecord(
                    copy_index=copy_index,
                    seed=seed,
                    h=h,
                    t=t,
                    z=z_v,
                    p=z_v,
                    s=len(visible_tuple),
                    a=len(visible_tuple),
                    tau=tau,
                    kappa=kappa,
                    r1=dim_v,
                    r0=dim_l,
                    z_v=z_v,
                    z_l=z_l,
                    delta=1,
                    comp=1,
                    g=0,
                    l_key=l_key,
                    v_key=v_key,
                    dim_l=dim_l,
                    dim_v=dim_v,
                    occurrences=1,
                    first_inner=inner,
                    first_outer=outer_zero,
                    first_paired=outer_zero,
                    first_visible=visible_tuple,
                    min_paired_size=len(outer_zero),
                    max_paired_size=len(outer_zero),
                    min_visible_size=len(visible_tuple),
                    max_visible_size=len(visible_tuple),
                    max_inner_complete_path_strides=inner_complete_path_strides,
                    max_visible_complete_path_strides=visible_complete_path_strides,
                    max_inner_complete_column_strides=inner_complete_column_strides,
                    max_visible_complete_column_strides=visible_complete_column_strides,
                    max_inner_full_sibling_pairs=inner_full_sibling_pairs,
                    max_visible_full_sibling_pairs=visible_full_sibling_pairs,
                )

    return CopyEnumeration(
        copy_index=copy_index,
        seed=seed,
        raw_profiles=raw_profiles,
        skipped_subflag_profiles=0,
        exact_flags=list(flags.values()),
    )


def summarize_flag_profiles(records: list[FlagRecord]) -> list[dict[str, int]]:
    grouped: dict[tuple[int, ...], list[FlagRecord]] = {}
    for record in records:
        key = (
            record.h,
            record.t,
            record.z,
            record.p,
            record.s,
            record.a,
            record.tau,
            record.kappa,
            record.r1,
            record.r0,
            record.z_v,
            record.z_l,
            record.delta,
            record.comp,
            record.g,
            record.max_visible_complete_path_strides,
            record.max_inner_complete_path_strides,
            record.max_visible_full_sibling_pairs,
            record.max_inner_full_sibling_pairs,
        )
        grouped.setdefault(key, []).append(record)

    rows: list[dict[str, int]] = []
    for key, values in sorted(grouped.items()):
        (
            h,
            t,
            z,
            p,
            s,
            a,
            tau,
            kappa,
            r1,
            r0,
            z_v,
            z_l,
            delta,
            comp,
            g,
            visible_path_strides,
            inner_path_strides,
            visible_sibling_pairs,
            inner_sibling_pairs,
        ) = key
        rows.append(
            {
                "h": h,
                "t": t,
                "z": z,
                "p": p,
                "s": s,
                "a": a,
                "tau": tau,
                "kappa": kappa,
                "r1": r1,
                "r0": r0,
                "z_V": z_v,
                "z_L": z_l,
                "delta": delta,
                "comp": comp,
                "g": g,
                "max_visible_complete_path_strides": visible_path_strides,
                "max_inner_complete_path_strides": inner_path_strides,
                "max_visible_full_sibling_pairs": visible_sibling_pairs,
                "max_inner_full_sibling_pairs": inner_sibling_pairs,
                "exact_flags": len(values),
                "raw_profiles": sum(record.occurrences for record in values),
                "max_profiles_per_flag": max(record.occurrences for record in values),
            }
        )
    return rows


def iter_record_tuples(
    enumerations: list[CopyEnumeration],
    samples: int,
    rng: random.Random,
) -> Iterator[tuple[FlagRecord, ...]]:
    lists = [enum.exact_flags for enum in enumerations]
    if samples <= 0:
        yield from itertools.product(*lists)
        return
    for _ in range(samples):
        yield tuple(rng.choice(records) for records in lists)


def intersection_rows(
    *,
    enumerations: list[CopyEnumeration],
    ambient_dim: int,
    prime: int,
    max_intersections: int,
    intersection_samples: int,
    seed: int,
    top_rows: int,
) -> tuple[list[dict[str, int | float | str]], int]:
    tuple_count = math.prod(len(enum.exact_flags) for enum in enumerations)
    if intersection_samples <= 0 and tuple_count > max_intersections:
        raise SystemExit(
            f"exact flag tuple count {tuple_count} exceeds --max-intersections "
            f"{max_intersections}; use --intersection-samples or raise the cap"
        )
    rng = random.Random(seed)
    grouped: dict[tuple[str, ...], int] = {}
    examples: dict[tuple[str, ...], tuple[FlagRecord, ...]] = {}
    checked = 0
    for records in iter_record_tuples(enumerations, intersection_samples, rng):
        checked += 1
        if checked > max_intersections and intersection_samples <= 0:
            raise SystemExit("exceeded --max-intersections")
        dim_v_tuple = tuple(record.dim_v for record in records)
        dim_l_tuple = tuple(record.dim_l for record in records)
        common_v = intersection_many([record.v_key for record in records], ambient_dim, prime)
        common_l = intersection_many([record.l_key for record in records], ambient_dim, prime)
        common_v_dim = len(common_v)
        common_l_dim = len(common_l)
        generic_v_dim = generic_intersection_dim(dim_v_tuple, ambient_dim)
        generic_l_dim = generic_intersection_dim(dim_l_tuple, ambient_dim)
        v_excess = common_v_dim - generic_v_dim
        l_excess = common_l_dim - generic_l_dim
        key = (
            ":".join(str(record.h) for record in records),
            ":".join(str(record.t) for record in records),
            ":".join(str(record.z) for record in records),
            ":".join(str(record.p) for record in records),
            ":".join(str(record.s) for record in records),
            ":".join(str(record.a) for record in records),
            ":".join(str(record.tau) for record in records),
            ":".join(str(record.kappa) for record in records),
            ":".join(str(record.r1) for record in records),
            ":".join(str(record.r0) for record in records),
            ":".join(str(record.z_v) for record in records),
            ":".join(str(record.z_l) for record in records),
            ":".join(str(record.delta) for record in records),
            ":".join(str(record.comp) for record in records),
            ":".join(str(record.g) for record in records),
            ":".join(str(value) for value in dim_v_tuple),
            ":".join(str(value) for value in dim_l_tuple),
            str(common_v_dim),
            str(common_l_dim),
            str(generic_v_dim),
            str(generic_l_dim),
            str(v_excess),
            str(l_excess),
            str(common_v_dim - common_l_dim),
            str(sum(record.max_visible_complete_path_strides for record in records)),
            str(sum(record.max_inner_complete_path_strides for record in records)),
        )
        grouped[key] = grouped.get(key, 0) + 1
        examples.setdefault(key, records)

    rows: list[dict[str, int | float | str]] = []
    for key, count in grouped.items():
        (
            h,
            t,
            z,
            p,
            s,
            a,
            tau,
            kappa,
            r1,
            r0,
            z_v,
            z_l,
            delta,
            comp,
            g,
            dim_v_text,
            dim_l_text,
            common_v_dim,
            common_l_dim,
            generic_v_dim,
            generic_l_dim,
            v_excess,
            l_excess,
            common_quotient_dim,
            visible_complete_path_stride_sum,
            inner_complete_path_stride_sum,
        ) = key
        common_v_dim_i = int(common_v_dim)
        common_l_dim_i = int(common_l_dim)
        generic_v_dim_i = int(generic_v_dim)
        generic_l_dim_i = int(generic_l_dim)
        records = examples[key]
        pair_logq = logq(count, prime)
        observed_l_line = pair_logq + projective_logq_from_linear_dim(common_l_dim_i)
        generic_l_line = pair_logq + projective_logq_from_linear_dim(generic_l_dim_i)
        l_gain = projective_gain(observed_l_line, generic_l_line)
        observed_v_line = pair_logq + projective_logq_from_linear_dim(common_v_dim_i)
        generic_v_line = pair_logq + projective_logq_from_linear_dim(generic_v_dim_i)
        v_gain = projective_gain(observed_v_line, generic_v_line)
        rows.append(
            {
                "copy_count": len(enumerations),
                "h": h,
                "t": t,
                "z": z,
                "p": p,
                "s": s,
                "a": a,
                "tau": tau,
                "kappa": kappa,
                "r1": r1,
                "r0": r0,
                "z_V": z_v,
                "z_L": z_l,
                "delta": delta,
                "comp": comp,
                "g": g,
                "dim_v_tuple": dim_v_text,
                "dim_l_tuple": dim_l_text,
                "common_v_dim": common_v_dim_i,
                "common_l_dim": common_l_dim_i,
                "common_quotient_dim": int(common_quotient_dim),
                "generic_v_dim": generic_v_dim_i,
                "generic_l_dim": generic_l_dim_i,
                "v_excess": int(v_excess),
                "l_excess": int(l_excess),
                "exact_flag_tuples": count,
                "logq_exact_flag_tuples": pair_logq,
                "observed_l_line_logq": observed_l_line,
                "generic_l_line_logq": generic_l_line,
                "l_line_gain_logq": l_gain,
                "observed_v_line_logq": observed_v_line,
                "generic_v_line_logq": generic_v_line,
                "v_line_gain_logq": v_gain,
                "visible_complete_path_stride_sum": int(visible_complete_path_stride_sum),
                "inner_complete_path_stride_sum": int(inner_complete_path_stride_sum),
                "profile_occurrence_sum_example": sum(record.occurrences for record in records),
                "example_copy0_inner": columns_text(records[0].first_inner),
                "example_copy0_visible": columns_text(records[0].first_visible),
            }
        )

    rows.sort(
        key=lambda row: (
            row["l_line_gain_logq"] == math.inf,
            row["v_line_gain_logq"] == math.inf,
            row["l_excess"],
            row["v_excess"],
            row["observed_l_line_logq"],
        ),
        reverse=True,
    )
    return rows[:top_rows], checked


def write_csv(path: str | None, rows: list[dict[str, int | float | str]]) -> None:
    if path is None or not rows:
        return
    with Path(path).open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)


def print_rows(title: str, rows: list[dict[str, int | float | str]]) -> None:
    print(title)
    if not rows:
        print("(none)")
        return
    fieldnames = list(rows[0].keys())
    print(",".join(fieldnames))
    for row in rows:
        values: list[str] = []
        for field in fieldnames:
            value = row[field]
            if isinstance(value, float):
                values.append(fmt_float(value))
            else:
                values.append(str(value))
        print(",".join(values))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--child-depth", type=int, required=True)
    parser.add_argument("--h", type=int, default=None, help="parent transition level; defaults to child-depth+1")
    parser.add_argument("--t", type=int, default=2, help="parent span dimension dim(W)")
    parser.add_argument("--tau", type=int, default=1, help="visible singleton quotient dimension")
    parser.add_argument("--z", type=int, default=None, help="optional parent zero request z=2p+s filter")
    parser.add_argument("--expansion", type=int, default=1)
    parser.add_argument("--copies", type=int, default=2)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--seed-stride", type=int, default=1009)
    parser.add_argument(
        "--targeted-near-stride-w",
        action="store_true",
        help=(
            "use the depth-h m=4, extra=4 complete-stride parent-W generator; "
            "in this mode --child-depth is the parent depth h"
        ),
    )
    parser.add_argument(
        "--targeted-combinatorial-only",
        action="store_true",
        help="for the complete-stride target, verify the 24-support/48-ordered-flag combinatorial gate and exit",
    )
    parser.add_argument(
        "--targeted-direct-check-only",
        action="store_true",
        help="for the complete-stride target, verify the direct finite-field dimensions/counts and exit",
    )
    parser.add_argument(
        "--targeted-size8-omega-scan-only",
        action="store_true",
        help="scan all size-8 Omega supports for non-stride dimension growth and exit",
    )
    parser.add_argument("--near-live-rows", type=int, default=4)
    parser.add_argument("--near-max-w-spaces", type=int, default=48)
    parser.add_argument("--omega-scan-max-omegas", type=int, default=60_000)
    parser.add_argument("--omega-scan-max-failures", type=int, default=20)
    parser.add_argument(
        "--omega-scan-fast-rank-only",
        action="store_true",
        help=(
            "for the size-8 Omega scan, skip exact projective kernel-line enumeration; "
            "keeps only rank dimensions, canonical C_j stride-line counts, and row-block metadata"
        ),
    )
    parser.add_argument("--inner-size", type=int, default=None)
    parser.add_argument(
        "--inner-columns",
        default=None,
        help="optional colon-separated child columns for one explicit inner zero set",
    )
    parser.add_argument("--paired-sizes", default="all")
    parser.add_argument("--visible-sizes", default="all")
    parser.add_argument("--inner-samples", type=int, default=0)
    parser.add_argument("--intersection-samples", type=int, default=0)
    parser.add_argument("--max-inner-sets", type=int, default=20_000)
    parser.add_argument("--max-raw-profiles", type=int, default=500_000)
    parser.add_argument("--max-flags-per-copy", type=int, default=50_000)
    parser.add_argument("--max-intersections", type=int, default=200_000)
    parser.add_argument("--min-l-dim", type=int, default=0)
    parser.add_argument("--min-v-dim", type=int, default=0)
    parser.add_argument("--min-quotient-dim", type=int, default=0)
    parser.add_argument("--max-quotient-dim", type=int, default=None)
    parser.add_argument("--exact-r0", type=int, default=None, help="optional exact child inner dimension")
    parser.add_argument("--exact-r1", type=int, default=None, help="optional exact child outer dimension")
    parser.add_argument(
        "--enumerate-subflags",
        action="store_true",
        help="enumerate r0/r1-dimensional flags inside the zero kernels; requires --exact-r0/--exact-r1",
    )
    parser.add_argument(
        "--max-subflags-per-profile",
        type=int,
        default=100_000,
        help="skip a zero profile when the naive Gaussian subflag budget exceeds this cap",
    )
    parser.add_argument(
        "--stride-modulus",
        type=int,
        default=0,
        help="optional live-row modulus for complete path/column stride metadata",
    )
    parser.add_argument("--top-intersections", type=int, default=20)
    parser.add_argument("--flag-summary-csv", default=None)
    parser.add_argument("--intersection-csv", default=None)
    args = parser.parse_args()

    if args.prime <= 2:
        raise SystemExit("--prime must be > 2")
    if args.child_depth < 0:
        raise SystemExit("--child-depth must be nonnegative")
    if args.expansion < 1:
        raise SystemExit("--expansion must be positive")
    if args.copies < 2 or args.copies > 3:
        raise SystemExit("--copies must be 2 or 3 for this exact diagnostic")
    if args.t < 0:
        raise SystemExit("--t must be nonnegative")
    if args.tau < 0 or args.tau > args.t:
        raise SystemExit("--tau must be in 0..t")
    if args.max_quotient_dim is not None and args.max_quotient_dim < args.min_quotient_dim:
        raise SystemExit("--max-quotient-dim must be >= --min-quotient-dim")

    inner_columns = parse_columns(args.inner_columns) if args.inner_columns is not None else None
    if args.targeted_near_stride_w:
        if args.expansion != 1:
            raise SystemExit("--targeted-near-stride-w currently supports --expansion 1 only")
        if args.inner_size is not None or inner_columns is not None:
            raise SystemExit("--targeted-near-stride-w does not use --inner-size/--inner-columns")
        if args.enumerate_subflags:
            raise SystemExit("--targeted-near-stride-w already enumerates parent-produced flags")
        parent_depth = args.child_depth
        if parent_depth < 1:
            raise SystemExit("--targeted-near-stride-w requires --child-depth >= 1")
        h = parent_depth if args.h is None else args.h
        child_k = 1 << parent_depth
        child_n = child_k
        inner_size = -1
        paired_sizes = parse_int_set(args.paired_sizes, 0, child_n)
        visible_sizes = parse_int_set(args.visible_sizes, 0, child_n)
        if args.targeted_combinatorial_only:
            print_complete_stride_combinatorial_gate(parent_depth, args.near_live_rows)
            return
        if args.targeted_direct_check_only:
            print_complete_stride_finite_field_gate(
                parent_depth,
                args.near_live_rows,
                args.prime,
                args.seed,
            )
            return
        if args.targeted_size8_omega_scan_only:
            print_size8_omega_growth_scan(
                depth=parent_depth,
                live_rows=args.near_live_rows,
                prime=args.prime,
                seed=args.seed,
                max_omegas=args.omega_scan_max_omegas,
                max_failure_rows=args.omega_scan_max_failures,
                fast_rank_only=args.omega_scan_fast_rank_only,
            )
            return
    else:
        if args.targeted_combinatorial_only:
            raise SystemExit("--targeted-combinatorial-only requires --targeted-near-stride-w")
        if args.targeted_direct_check_only:
            raise SystemExit("--targeted-direct-check-only requires --targeted-near-stride-w")
        if args.targeted_size8_omega_scan_only:
            raise SystemExit("--targeted-size8-omega-scan-only requires --targeted-near-stride-w")
        if args.inner_size is None:
            if inner_columns is None:
                raise SystemExit("provide --inner-size or --inner-columns")
            inner_size = len(inner_columns)
        else:
            inner_size = args.inner_size
        if inner_size < 0:
            raise SystemExit("--inner-size must be nonnegative")
        if inner_columns is not None and len(inner_columns) != inner_size:
            raise SystemExit("--inner-size must match --inner-columns length")
        paired_sizes = parse_int_set(args.paired_sizes, 0, inner_size)
        visible_sizes = parse_int_set(args.visible_sizes, 0, inner_size)
        child_k = 1 << args.child_depth
        child_n = args.expansion * child_k
        if inner_size > child_n:
            raise SystemExit("--inner-size exceeds child length")
        h = args.child_depth + 1 if args.h is None else args.h

    enumerations: list[CopyEnumeration] = []
    for copy_index in range(args.copies):
        seed = args.seed + copy_index * args.seed_stride
        if args.targeted_near_stride_w:
            enumerations.append(
                enumerate_copy_complete_stride_direct_flags(
                    copy_index=copy_index,
                    seed=seed,
                    prime=args.prime,
                    depth=args.child_depth,
                    h=h,
                    t=args.t,
                    tau=args.tau,
                    live_rows=args.near_live_rows,
                    max_raw_profiles=args.max_raw_profiles,
                    max_flags_per_copy=args.max_flags_per_copy,
                    min_l_dim=args.min_l_dim,
                    min_v_dim=args.min_v_dim,
                    min_quotient_dim=args.min_quotient_dim,
                    max_quotient_dim=args.max_quotient_dim,
                    exact_r0=args.exact_r0,
                    exact_r1=args.exact_r1,
                    stride_modulus=args.stride_modulus,
                )
            )
        else:
            enumerations.append(
                enumerate_copy_flags(
                    copy_index=copy_index,
                    seed=seed,
                    prime=args.prime,
                    child_depth=args.child_depth,
                    h=h,
                    t=args.t,
                    tau=args.tau,
                    z=args.z,
                    expansion=args.expansion,
                    inner_size=inner_size,
                    inner_columns=inner_columns,
                    paired_sizes=paired_sizes,
                    visible_sizes=visible_sizes,
                    inner_samples=args.inner_samples,
                    max_inner_sets=args.max_inner_sets,
                    max_raw_profiles=args.max_raw_profiles,
                    max_flags_per_copy=args.max_flags_per_copy,
                    min_l_dim=args.min_l_dim,
                    min_v_dim=args.min_v_dim,
                    min_quotient_dim=args.min_quotient_dim,
                    max_quotient_dim=args.max_quotient_dim,
                    exact_r0=args.exact_r0,
                    exact_r1=args.exact_r1,
                    enumerate_subflags=args.enumerate_subflags,
                    max_subflags_per_profile=args.max_subflags_per_profile,
                    stride_modulus=args.stride_modulus,
                )
            )

    mode = "targeted-near-stride-w" if args.targeted_near_stride_w else ("sampled" if args.inner_samples > 0 else "exact")
    print(
        "mode,prime,child_depth,child_k,child_n,expansion,copies,"
        "h,t,tau,kappa,z_filter,inner_size,paired_sizes,visible_sizes,"
        "enumerate_subflags,max_subflags_per_profile,near_live_rows,near_max_w_spaces"
    )
    print(
        f"{mode},{args.prime},{args.child_depth},{child_k},{child_n},{args.expansion},"
        f"{args.copies},{h},{args.t},{args.tau},{args.t - args.tau},"
        f"{'' if args.z is None else args.z},{inner_size},{':'.join(map(str, paired_sizes))},"
        f"{':'.join(map(str, visible_sizes))},{int(args.enumerate_subflags)},"
        f"{args.max_subflags_per_profile},{args.near_live_rows},{args.near_max_w_spaces}"
    )

    print("copy,seed,raw_profiles,skipped_subflag_profiles,exact_flags,max_profiles_per_flag")
    flag_summary_rows: list[dict[str, int]] = []
    for enum in enumerations:
        max_profiles = max((record.occurrences for record in enum.exact_flags), default=0)
        print(
            f"{enum.copy_index},{enum.seed},{enum.raw_profiles},"
            f"{enum.skipped_subflag_profiles},{len(enum.exact_flags)},{max_profiles}"
        )
        for row in summarize_flag_profiles(enum.exact_flags):
            out = {"copy": enum.copy_index, "seed": enum.seed}
            out.update(row)
            flag_summary_rows.append(out)

    print_rows("flag_profile_summary", flag_summary_rows)
    write_csv(args.flag_summary_csv, flag_summary_rows)

    rows, checked = intersection_rows(
        enumerations=enumerations,
        ambient_dim=child_k,
        prime=args.prime,
        max_intersections=args.max_intersections,
        intersection_samples=args.intersection_samples,
        seed=args.seed + 17,
        top_rows=args.top_intersections,
    )
    print(f"intersections_checked={checked}")
    print_rows("top_intersection_profiles", rows)
    write_csv(args.intersection_csv, rows)


if __name__ == "__main__":
    main()
