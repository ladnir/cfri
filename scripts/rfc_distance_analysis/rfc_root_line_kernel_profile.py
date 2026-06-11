#!/usr/bin/env python3
r"""Profile root-line kernel dimensions for local visible-span counting.

For U <= F^S and support A subset S, choose a projective line ell_j in P^1(F) for each j in A.
Let K(A, ell) be the linear subspace of U_A + U_A satisfying the line relation at every visible
coordinate:

    (x_j, y_j) in ell_j  for all j in A.

Every locally compatible visible subspace with support contained in A is contained in one of these
kernels.  Therefore:

    sum_ell [dim K(A, ell) choose tau]_q

is a root-line-aware upper bound for tau-dimensional compatible visible subspaces supported inside
A.  Unlike the coarse support bound [2 delta(A) choose tau]_q, this sees the exterior/component
savings: full-visible-rank kernels occur only on low-dimensional diagonal-multiplier families.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import itertools
import math
import random
from pathlib import Path

from rfc_exterior_constraint_profile import matroid_component_count, restricted_image_basis
from rfc_support_profile_bound import gaussian_binomial, iter_subsets, logq
from sample_rfc_rank_failure import rank_selected_columns, rfc_generator_prime


def modinv(value: int, prime: int) -> int:
    return pow(value % prime, prime - 2, prime)


def rref_with_pivots(rows: list[list[int]], width: int, prime: int) -> tuple[list[list[int]], list[int]]:
    if not rows:
        return [], []
    work = [[value % prime for value in row] for row in rows if any(value % prime for value in row)]
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


def matrix_rank(rows: list[list[int]], width: int, prime: int) -> int:
    return len(rref_with_pivots(rows, width, prime)[1])


def nullspace_basis(rows: list[list[int]], width: int, prime: int) -> list[list[int]]:
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
    return basis


def columns_from_mask(columns: tuple[int, ...], mask: int) -> tuple[int, ...]:
    return tuple(columns[index] for index in range(len(columns)) if (mask >> index) & 1)


def parse_column_list(value: str, name: str) -> tuple[int, ...]:
    if value.strip() == "":
        raise SystemExit(f"{name} must not be empty")
    columns = tuple(int(part) for part in value.replace(",", ":").split(":") if part != "")
    if any(column < 0 for column in columns):
        raise SystemExit(f"{name} must contain nonnegative columns")
    if len(set(columns)) != len(columns):
        raise SystemExit(f"{name} must not contain duplicate columns")
    return columns


def support_subcode_basis(
    image_basis: list[list[int]],
    support_mask: int,
    prime: int,
) -> list[list[int]]:
    rho = len(image_basis)
    if rho == 0:
        return []
    size = len(image_basis[0])
    support_indices = [index for index in range(size) if (support_mask >> index) & 1]
    complement_indices = [index for index in range(size) if ((support_mask >> index) & 1) == 0]
    equations = [
        [image_basis[basis_index][coordinate] for basis_index in range(rho)]
        for coordinate in complement_indices
    ]
    coefficient_basis = nullspace_basis(equations, rho, prime)
    out: list[list[int]] = []
    for coeffs in coefficient_basis:
        row = [0] * len(support_indices)
        for coeff, basis_row in zip(coeffs, image_basis):
            if coeff == 0:
                continue
            for out_index, coordinate in enumerate(support_indices):
                row[out_index] = (row[out_index] + coeff * basis_row[coordinate]) % prime
        out.append(row)
    return out


def projective_lines(prime: int) -> list[tuple[int, int]]:
    # Finite slope lambda: y = lambda x, encoded as -lambda*x + y = 0.
    lines = [((-slope) % prime, 1) for slope in range(prime)]
    # Infinity: x = 0.
    lines.append((1, 0))
    return lines


def kernel_dimension(
    support_basis: list[list[int]],
    line_assignment: tuple[tuple[int, int], ...],
    prime: int,
) -> int:
    delta = len(support_basis)
    if delta == 0:
        return 0
    equations: list[list[int]] = []
    for coordinate, (alpha, beta) in enumerate(line_assignment):
        row = [alpha * basis_row[coordinate] % prime for basis_row in support_basis]
        row += [beta * basis_row[coordinate] % prime for basis_row in support_basis]
        if any(value != 0 for value in row):
            equations.append(row)
    return 2 * delta - matrix_rank(equations, 2 * delta, prime)


def projected_rank(support_basis: list[list[int]], mask: int, prime: int) -> int:
    if not support_basis or mask == 0:
        return 0
    width = len(support_basis[0])
    indices = [index for index in range(width) if (mask >> index) & 1]
    projected_rows = [[row[index] for index in indices] for row in support_basis]
    return matrix_rank(projected_rows, len(indices), prime)


def two_copy_generic_kernel_dim(support_basis: list[list[int]], prime: int) -> int:
    delta = len(support_basis)
    if delta == 0:
        return 0
    width = len(support_basis[0])
    full_mask = (1 << width) - 1
    generic_rank = 2 * delta
    for mask in range(full_mask + 1):
        rank = projected_rank(support_basis, mask, prime)
        candidate = (width - mask.bit_count()) + 2 * rank
        generic_rank = min(generic_rank, candidate)
    return 2 * delta - generic_rank


def two_copy_generic_kernel_profile(
    support_basis: list[list[int]],
    support_columns: tuple[int, ...],
    prime: int,
) -> tuple[int, str]:
    delta = len(support_basis)
    if delta == 0:
        return 0, "empty"
    width = len(support_basis[0])
    full_mask = (1 << width) - 1
    generic_rank = 2 * delta
    minimizers: list[int] = []
    for mask in range(full_mask + 1):
        rank = projected_rank(support_basis, mask, prime)
        candidate = (width - mask.bit_count()) + 2 * rank
        if candidate < generic_rank:
            generic_rank = candidate
            minimizers = [mask]
        elif candidate == generic_rank:
            minimizers.append(mask)

    def mask_key(mask: int) -> str:
        selected = [str(support_columns[index]) for index in range(width) if (mask >> index) & 1]
        return ":".join(selected) if selected else "empty"

    return 2 * delta - generic_rank, "|".join(mask_key(mask) for mask in minimizers)


def local_endpoint_logq(
    tau: int,
    root_count: int,
    delta: int,
    support_components: int,
    generic_kernel_dim: int,
) -> tuple[float, float, float]:
    generic_endpoint = -math.inf
    if generic_kernel_dim >= tau:
        generic_endpoint = root_count + tau * (generic_kernel_dim - tau)
    component_endpoint = -math.inf
    if delta >= tau:
        component_endpoint = support_components + tau * (delta - tau)
    return generic_endpoint, component_endpoint, max(generic_endpoint, component_endpoint)


def projective_line_factor_logq(root_count: int, prime: int) -> float:
    return root_count * (math.log(prime + 1) / math.log(prime) - 1.0)


def nonzero_root_normalization_logq(root_count: int, prime: int) -> float:
    return root_count * (1.0 - math.log(prime - 1) / math.log(prime))


def exceptional_kappa_count(kernel_profile: str, kappa: int = 3) -> int | str:
    if kernel_profile in ("", "NA"):
        return "NA"
    prefix = f"{kappa}:"
    for part in kernel_profile.split(";"):
        if part.startswith(prefix):
            return int(part[len(prefix) :])
    return 0


def parse_kernel_profile(kernel_profile: str) -> dict[int, int]:
    if kernel_profile in ("", "NA"):
        return {}
    counts: dict[int, int] = {}
    for part in kernel_profile.split(";"):
        if not part:
            continue
        kappa_text, count_text = part.split(":", 1)
        counts[int(kappa_text)] = int(count_text)
    return counts


def kernel_layer_diagnostics(
    kernel_profile: str,
    tau: int,
    root_count: int,
    prime: int,
) -> dict[str, int | float | str]:
    counts = parse_kernel_profile(kernel_profile)
    if not counts:
        return {
            "dominant_contained_layer_h": "NA",
            "dominant_contained_layer_root_weight_logq": "NA",
            "dominant_contained_layer_codim_logq": "NA",
            "empirical_theta_tau_logq": "NA",
        }

    dominant_h = "NA"
    dominant_root_weight = -math.inf
    for kappa, count in counts.items():
        if kappa < tau or count == 0:
            continue
        contribution = count * gaussian_binomial(kappa, tau, prime)
        contribution_root_weight = logq(contribution, prime) - root_count
        if contribution_root_weight > dominant_root_weight:
            dominant_root_weight = contribution_root_weight
            dominant_h = kappa

    empirical_theta = -math.inf
    dominant_codim: float | str = "NA"
    for h in range(tau, max(counts) + 1):
        layer_count = sum(count for kappa, count in counts.items() if kappa >= h)
        if layer_count == 0:
            continue
        codim = root_count - logq(layer_count, prime)
        theta = tau * (h - tau) - codim
        if theta > empirical_theta:
            empirical_theta = theta
        if h == dominant_h:
            dominant_codim = codim

    return {
        "dominant_contained_layer_h": dominant_h,
        "dominant_contained_layer_root_weight_logq": dominant_root_weight,
        "dominant_contained_layer_codim_logq": dominant_codim,
        "empirical_theta_tau_logq": empirical_theta,
    }


def support_filter_active(args: argparse.Namespace) -> bool:
    return (
        args.require_root_count is not None
        or args.require_delta is not None
        or args.require_support_components is not None
        or args.require_generic_kernel_dim is not None
    )


def support_matches_requirements(
    args: argparse.Namespace,
    root_count: int,
    delta: int,
    support_components: int,
    generic_kernel_dim: int,
) -> bool:
    if args.require_root_count is not None and root_count != args.require_root_count:
        return False
    if args.require_delta is not None and delta != args.require_delta:
        return False
    if (
        args.require_support_components is not None
        and support_components != args.require_support_components
    ):
        return False
    if (
        args.require_generic_kernel_dim is not None
        and generic_kernel_dim != args.require_generic_kernel_dim
    ):
        return False
    return True


def support_certificate_hash(
    prime: int,
    columns: tuple[int, ...],
    support_columns: tuple[int, ...],
    rank_s: int,
    rank_s_minus_a: int,
    delta: int,
    support_components: int,
    generic_kernel_dim: int,
) -> str:
    payload = (
        f"prime={prime};S={':'.join(str(column) for column in columns)};"
        f"A={':'.join(str(column) for column in support_columns)};"
        f"rank_S={rank_s};rank_S_minus_A={rank_s_minus_a};"
        f"delta={delta};comp={support_components};g={generic_kernel_dim}"
    )
    return hashlib.sha256(payload.encode("ascii")).hexdigest()


def rank_subset_table_hash(generator: list[list[int]], columns: tuple[int, ...], prime: int) -> str:
    entries = []
    for mask in range(1 << len(columns)):
        subset = columns_from_mask(columns, mask)
        rank = rank_selected_columns(generator, list(subset), prime)
        entries.append(f"{mask}:{rank}")
    payload = f"prime={prime};columns={':'.join(str(column) for column in columns)};" + ";".join(entries)
    return hashlib.sha256(payload.encode("ascii")).hexdigest()


def component_partition_and_circuits(
    generator: list[list[int]],
    columns: tuple[int, ...],
    prime: int,
) -> tuple[str, str]:
    size = len(columns)
    if size == 0:
        return "empty", "NA"
    parent = list(range(size))
    rank_cache: dict[int, int] = {0: 0}

    def find(value: int) -> int:
        while parent[value] != value:
            parent[value] = parent[parent[value]]
            value = parent[value]
        return value

    def union(left: int, right: int) -> None:
        left_root = find(left)
        right_root = find(right)
        if left_root != right_root:
            parent[right_root] = left_root

    def rank_mask(mask: int) -> int:
        if mask not in rank_cache:
            subset = [columns[index] for index in range(size) if (mask >> index) & 1]
            rank_cache[mask] = rank_selected_columns(generator, subset, prime)
        return rank_cache[mask]

    witnesses: list[str] = []
    for mask in range(1, 1 << size):
        bit_count = mask.bit_count()
        if bit_count < 2 or rank_mask(mask) == bit_count:
            continue
        minimal = True
        for index in range(size):
            if (mask >> index) & 1 and rank_mask(mask ^ (1 << index)) < bit_count - 1:
                minimal = False
                break
        if not minimal:
            continue
        first = (mask & -mask).bit_length() - 1
        circuit_columns = [str(columns[index]) for index in range(size) if (mask >> index) & 1]
        witnesses.append(":".join(circuit_columns))
        for index in range(first + 1, size):
            if (mask >> index) & 1:
                union(first, index)

    groups: dict[int, list[int]] = {}
    for index, column in enumerate(columns):
        groups.setdefault(find(index), []).append(column)
    partition = "|".join(
        ":".join(str(column) for column in sorted(group))
        for group in sorted(groups.values(), key=lambda group: group[0])
    )
    return partition, "|".join(witnesses) if witnesses else "NA"


def exact_support_profile_row(
    generator: list[list[int]],
    columns: tuple[int, ...],
    support_columns: tuple[int, ...],
    tau: int,
    prime: int,
    max_line_assignments: int,
) -> dict[str, int | float | str]:
    column_set = set(columns)
    if any(column not in column_set for column in support_columns):
        raise ValueError("support_columns must be contained in columns")
    column_positions = {column: index for index, column in enumerate(columns)}
    target_mask = 0
    for column in support_columns:
        target_mask |= 1 << column_positions[column]
    full_mask = (1 << len(columns)) - 1
    image_basis = restricted_image_basis(generator, columns, prime)
    rank_s = len(image_basis)
    rank_s_minus_a = rank_selected_columns(
        generator,
        list(columns_from_mask(columns, full_mask ^ target_mask)),
        prime,
    )
    support_rank = rank_selected_columns(generator, list(support_columns), prime)
    support_basis = support_subcode_basis(image_basis, target_mask, prime)
    root_count = target_mask.bit_count()
    delta = len(support_basis)
    support_components = matroid_component_count(generator, support_columns, prime)
    generic_kernel_dim, g_minimizer_keys = two_copy_generic_kernel_profile(
        support_basis,
        support_columns,
        prime,
    )
    component_partition_key, circuit_witnesses = component_partition_and_circuits(
        generator,
        support_columns,
        prime,
    )
    generic_endpoint, component_endpoint, endpoint_bound = local_endpoint_logq(
        tau,
        root_count,
        delta,
        support_components,
        generic_kernel_dim,
    )
    endpoint_root_weight = endpoint_bound - root_count
    projective_factor = projective_line_factor_logq(root_count, prime)
    nonzero_normalization = nonzero_root_normalization_logq(root_count, prime)
    lines = projective_lines(prime)
    target_line_assignments = len(lines) ** root_count
    row: dict[str, int | float | str] = {
        "mode": "exact_support",
        "status": "ok",
        "prime": prime,
        "columns": ":".join(str(column) for column in columns),
        "support_columns": ":".join(str(column) for column in support_columns),
        "tau": tau,
        "root_count": root_count,
        "rank_S": rank_s,
        "rank_S_minus_A": rank_s_minus_a,
        "support_rank": support_rank,
        "delta": delta,
        "comp": support_components,
        "g": generic_kernel_dim,
        "generic_endpoint_logq": generic_endpoint,
        "component_endpoint_logq": component_endpoint,
        "endpoint_bound_logq": endpoint_bound,
        "endpoint_root_weight_logq": endpoint_root_weight,
        "target_line_assignments": target_line_assignments,
        "projective_line_factor_logq": projective_factor,
        "nonzero_root_normalization_logq": nonzero_normalization,
        "exceptional_kappa3_count": "NA",
        "dominant_contained_layer_h": "NA",
        "dominant_contained_layer_root_weight_logq": "NA",
        "dominant_contained_layer_codim_logq": "NA",
        "empirical_theta_tau_logq": "NA",
        "kernel_profile": "NA",
        "contained_root_line_count": "NA",
        "contained_root_line_logq": "NA",
        "exact_root_line_count": "NA",
        "exact_root_line_logq": "NA",
        "asymptotic_root_weight_logq": "NA",
        "endpoint_excess_logq": "NA",
        "residual_endpoint_excess_logq": "NA",
        "support_certificate_hash": support_certificate_hash(
            prime,
            columns,
            support_columns,
            rank_s,
            rank_s_minus_a,
            delta,
            support_components,
            generic_kernel_dim,
        ),
        "rank_subset_table_hash": rank_subset_table_hash(generator, columns, prime),
        "component_partition_key": component_partition_key,
        "circuit_witnesses": circuit_witnesses,
        "g_minimizer_keys": g_minimizer_keys,
        "blocker": "",
    }
    if tau > 2 * delta:
        row["status"] = "zero_tau_exceeds_support_dimension"
        row["blocker"] = "tau exceeds 2*delta for the targeted support"
        return row
    if target_line_assignments > max_line_assignments:
        row["status"] = "line_assignment_cap"
        row["blocker"] = (
            f"(p+1)^a={target_line_assignments} exceeds "
            f"--max-line-assignments={max_line_assignments}"
        )
        return row

    contained_counts: dict[int, int] = {}
    kernel_profiles: dict[int, str] = {}
    for mask in sorted(
        (mask for mask in range(1, target_mask + 1) if (mask & target_mask) == mask),
        key=int.bit_count,
    ):
        sub_root_count = mask.bit_count()
        line_assignment_count = len(lines) ** sub_root_count
        if line_assignment_count > max_line_assignments:
            row["status"] = "line_assignment_cap"
            row["blocker"] = (
                f"submask line assignments {line_assignment_count} exceed "
                f"--max-line-assignments={max_line_assignments}"
            )
            return row
        sub_support_basis = support_subcode_basis(image_basis, mask, prime)
        if tau > 2 * len(sub_support_basis):
            contained_counts[mask] = 0
            kernel_profiles[mask] = ""
            continue
        by_kappa: dict[int, int] = {}
        contribution = 0
        for assignment in itertools.product(lines, repeat=sub_root_count):
            kappa = kernel_dimension(sub_support_basis, assignment, prime)
            by_kappa[kappa] = by_kappa.get(kappa, 0) + 1
            contribution += gaussian_binomial(kappa, tau, prime)
        contained_counts[mask] = contribution
        kernel_profiles[mask] = ";".join(
            f"{kappa}:{count}" for kappa, count in sorted(by_kappa.items())
        )

    exact_counts: dict[int, int] = {}
    for mask in sorted(contained_counts, key=int.bit_count):
        count = contained_counts[mask]
        submask = (mask - 1) & mask
        while submask:
            if submask in exact_counts:
                count -= exact_counts[submask] * (
                    len(lines) ** (mask.bit_count() - submask.bit_count())
                )
            submask = (submask - 1) & mask
        if count < 0:
            raise RuntimeError(f"negative exact root-line count for mask {mask}: {count}")
        exact_counts[mask] = count

    exact_count = exact_counts[target_mask]
    exact_log = logq(exact_count, prime) if exact_count > 0 else -math.inf
    observed_endpoint_logq = exact_log - root_count
    endpoint_excess = observed_endpoint_logq - endpoint_root_weight
    kernel_profile = kernel_profiles[target_mask]
    layer_diagnostics = kernel_layer_diagnostics(kernel_profile, tau, root_count, prime)
    row.update(
        {
            "kernel_profile": kernel_profile,
            "contained_root_line_count": contained_counts[target_mask],
            "contained_root_line_logq": logq(contained_counts[target_mask], prime),
            "exact_root_line_count": exact_count,
            "exact_root_line_logq": exact_log,
            "asymptotic_root_weight_logq": observed_endpoint_logq,
            "endpoint_excess_logq": endpoint_excess,
            "residual_endpoint_excess_logq": (
                endpoint_excess - projective_factor - nonzero_normalization
            ),
            "exceptional_kappa3_count": exceptional_kappa_count(kernel_profile),
            **layer_diagnostics,
        }
    )
    return row


def run_connected_support_discovery(
    args: argparse.Namespace,
    generator: list[list[int]],
    rng: random.Random,
) -> None:
    n = len(generator[0])
    root_count = args.require_root_count if args.require_root_count is not None else 4
    parent_size = args.size if args.size is not None else root_count + 1
    required_components = (
        args.require_support_components if args.require_support_components is not None else 1
    )
    if parent_size <= root_count:
        raise SystemExit("--size must be larger than discovery root count")
    if root_count > n or parent_size > n:
        raise SystemExit("discovery support size outside coordinate range")
    if args.discovery_attempts < 1:
        raise SystemExit("--discovery-attempts must be positive")
    if args.discovery_extra_per_support < 1:
        raise SystemExit("--discovery-extra-per-support must be positive")
    if args.discovery_max_hits < 1:
        raise SystemExit("--discovery-max-hits must be positive")

    rows: list[dict[str, int | float | str]] = []
    seen_proposals: set[tuple[tuple[int, ...], tuple[int, ...]]] = set()
    component_rejects = 0
    profile_rejects = 0
    connected_accepts = 0
    proposals = 0
    profile_accepts = 0

    for attempt_index in range(args.discovery_attempts):
        support_columns = tuple(sorted(rng.sample(range(n), root_count)))
        support_components = matroid_component_count(generator, support_columns, args.prime)
        if support_components != required_components:
            component_rejects += 1
            continue
        connected_accepts += 1
        extra_pool = [column for column in range(n) if column not in set(support_columns)]
        extra_count = min(args.discovery_extra_per_support, len(extra_pool))
        for extra_column in rng.sample(extra_pool, extra_count):
            columns = tuple(sorted(support_columns + (extra_column,)))
            if len(columns) != parent_size:
                continue
            proposal_key = (columns, support_columns)
            if proposal_key in seen_proposals:
                continue
            seen_proposals.add(proposal_key)
            proposals += 1

            column_positions = {column: index for index, column in enumerate(columns)}
            support_mask = 0
            for column in support_columns:
                support_mask |= 1 << column_positions[column]
            full_mask = (1 << len(columns)) - 1
            image_basis = restricted_image_basis(generator, columns, args.prime)
            support_basis = support_subcode_basis(image_basis, support_mask, args.prime)
            delta = len(support_basis)
            if args.tau > 2 * delta:
                profile_rejects += 1
                continue
            generic_kernel_dim = two_copy_generic_kernel_dim(support_basis, args.prime)
            if not support_matches_requirements(
                args,
                root_count,
                delta,
                support_components,
                generic_kernel_dim,
            ):
                profile_rejects += 1
                continue
            profile_accepts += 1
            row = exact_support_profile_row(
                generator,
                columns,
                support_columns,
                args.tau,
                args.prime,
                args.max_line_assignments,
            )
            row.update(
                {
                    "generator_mode": "discovery",
                    "sampler_mode": "discovery",
                    "proposal_index": proposals,
                    "attempt_index": attempt_index,
                    "extra_column": extra_column,
                    "attempts": attempt_index + 1,
                    "connected_accepts": connected_accepts,
                    "component_rejects": component_rejects,
                    "profile_rejects": profile_rejects,
                    "profile_accepts": profile_accepts,
                    "rank_S_minus_A": rank_selected_columns(
                        generator,
                        list(columns_from_mask(columns, full_mask ^ support_mask)),
                        args.prime,
                    ),
                }
            )
            rows.append(row)
            if len(rows) >= args.discovery_max_hits:
                break
        if len(rows) >= args.discovery_max_hits:
            break

    residuals = [
        float(row["residual_endpoint_excess_logq"])
        for row in rows
        if row["residual_endpoint_excess_logq"] != "NA"
    ]
    worst_residual = max(residuals) if residuals else "NA"
    summary_row: dict[str, int | float | str] = {
        "generator_mode": "discovery",
        "sampler_mode": "discovery",
        "prime": args.prime,
        "child_depth": args.child_depth,
        "expansion": args.expansion,
        "parent_size": parent_size,
        "tau": args.tau,
        "root_count": root_count,
        "attempts": args.discovery_attempts,
        "connected_accepts": connected_accepts,
        "component_rejects": component_rejects,
        "proposals": proposals,
        "profile_rejects": profile_rejects,
        "profile_accepts": profile_accepts,
        "target_rows": len(rows),
        "worst_residual_endpoint_excess_logq": worst_residual,
    }
    print(",".join(summary_row.keys()))
    print(",".join(str(value) for value in summary_row.values()))

    if args.summary_csv is not None:
        with Path(args.summary_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=list(summary_row.keys()))
            writer.writeheader()
            writer.writerow(summary_row)
    if args.support_csv is not None:
        fieldnames = [
            "generator_mode",
            "sampler_mode",
            "proposal_index",
            "attempt_index",
            "extra_column",
            "attempts",
            "connected_accepts",
            "component_rejects",
            "profile_rejects",
            "profile_accepts",
            "mode",
            "status",
            "prime",
            "columns",
            "support_columns",
            "tau",
            "root_count",
            "rank_S",
            "rank_S_minus_A",
            "support_rank",
            "delta",
            "comp",
            "g",
            "generic_endpoint_logq",
            "component_endpoint_logq",
            "endpoint_bound_logq",
            "endpoint_root_weight_logq",
            "target_line_assignments",
            "projective_line_factor_logq",
            "nonzero_root_normalization_logq",
            "exceptional_kappa3_count",
            "dominant_contained_layer_h",
            "dominant_contained_layer_root_weight_logq",
            "dominant_contained_layer_codim_logq",
            "empirical_theta_tau_logq",
            "kernel_profile",
            "contained_root_line_count",
            "contained_root_line_logq",
            "exact_root_line_count",
            "exact_root_line_logq",
            "asymptotic_root_weight_logq",
            "endpoint_excess_logq",
            "residual_endpoint_excess_logq",
            "support_certificate_hash",
            "rank_subset_table_hash",
            "component_partition_key",
            "circuit_witnesses",
            "g_minimizer_keys",
            "blocker",
        ]
        with Path(args.support_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=fieldnames, extrasaction="ignore")
            writer.writeheader()
            writer.writerows(rows)


def run_targeted_support_replay(
    generator: list[list[int]],
    columns: tuple[int, ...],
    support_columns: tuple[int, ...],
    tau: int,
    prime: int,
    max_line_assignments: int,
) -> None:
    n = len(generator[0])
    if any(column >= n for column in columns):
        raise SystemExit("--columns contains a coordinate outside the generator")
    if any(column not in set(columns) for column in support_columns):
        raise SystemExit("--support-columns must be contained in --columns")

    column_positions = {column: index for index, column in enumerate(columns)}
    target_mask = 0
    for column in support_columns:
        target_mask |= 1 << column_positions[column]
    if target_mask == 0:
        raise SystemExit("--support-columns must not be empty")

    full_mask = (1 << len(columns)) - 1
    image_basis = restricted_image_basis(generator, columns, prime)
    rank_s = len(image_basis)
    ranks = {
        mask: rank_selected_columns(generator, list(columns_from_mask(columns, mask)), prime)
        for mask in [target_mask, full_mask ^ target_mask]
    }
    support_basis = support_subcode_basis(image_basis, target_mask, prime)
    root_count = target_mask.bit_count()
    delta = len(support_basis)
    comp = matroid_component_count(generator, support_columns, prime)
    g = two_copy_generic_kernel_dim(support_basis, prime)
    generic_endpoint, component_endpoint, endpoint_bound = local_endpoint_logq(
        tau,
        root_count,
        delta,
        comp,
        g,
    )
    endpoint_root_weight = endpoint_bound - root_count
    projective_factor = projective_line_factor_logq(root_count, prime)
    nonzero_normalization = nonzero_root_normalization_logq(root_count, prime)
    lines = projective_lines(prime)
    target_line_assignments = len(lines) ** root_count

    row: dict[str, int | float | str] = {
        "mode": "targeted_support",
        "status": "ok",
        "prime": prime,
        "columns": ":".join(str(column) for column in columns),
        "support_columns": ":".join(str(column) for column in support_columns),
        "tau": tau,
        "root_count": root_count,
        "rank_S": rank_s,
        "rank_S_minus_A": ranks[full_mask ^ target_mask],
        "support_rank": ranks[target_mask],
        "delta": delta,
        "comp": comp,
        "g": g,
        "generic_endpoint_logq": generic_endpoint,
        "component_endpoint_logq": component_endpoint,
        "endpoint_bound_logq": endpoint_bound,
        "endpoint_root_weight_logq": endpoint_root_weight,
        "target_line_assignments": target_line_assignments,
        "projective_line_factor_logq": projective_factor,
        "nonzero_root_normalization_logq": nonzero_normalization,
        "exceptional_kappa3_count": "NA",
        "dominant_contained_layer_h": "NA",
        "dominant_contained_layer_root_weight_logq": "NA",
        "dominant_contained_layer_codim_logq": "NA",
        "empirical_theta_tau_logq": "NA",
        "kernel_profile": "NA",
        "contained_root_line_count": "NA",
        "contained_root_line_logq": "NA",
        "exact_root_line_count": "NA",
        "exact_root_line_logq": "NA",
        "asymptotic_root_weight_logq": "NA",
        "endpoint_excess_logq": "NA",
        "residual_endpoint_excess_logq": "NA",
        "blocker": "",
    }

    if tau > 2 * delta:
        row["status"] = "zero_tau_exceeds_support_dimension"
        row["blocker"] = "tau exceeds 2*delta for the targeted support"
    elif target_line_assignments > max_line_assignments:
        row["status"] = "line_assignment_cap"
        row["blocker"] = (
            f"(p+1)^a={target_line_assignments} exceeds "
            f"--max-line-assignments={max_line_assignments}"
        )
    else:
        contained_counts: dict[int, int] = {}
        kernel_profiles: dict[int, str] = {}
        for mask in sorted(
            (mask for mask in range(1, target_mask + 1) if (mask & target_mask) == mask),
            key=int.bit_count,
        ):
            sub_root_count = mask.bit_count()
            line_assignment_count = len(lines) ** sub_root_count
            if line_assignment_count > max_line_assignments:
                row["status"] = "line_assignment_cap"
                row["blocker"] = (
                    f"submask line assignments {line_assignment_count} exceed "
                    f"--max-line-assignments={max_line_assignments}"
                )
                break
            sub_support_basis = support_subcode_basis(image_basis, mask, prime)
            if tau > 2 * len(sub_support_basis):
                contained_counts[mask] = 0
                kernel_profiles[mask] = ""
                continue
            by_kappa: dict[int, int] = {}
            contribution = 0
            for assignment in itertools.product(lines, repeat=sub_root_count):
                kappa = kernel_dimension(sub_support_basis, assignment, prime)
                by_kappa[kappa] = by_kappa.get(kappa, 0) + 1
                contribution += gaussian_binomial(kappa, tau, prime)
            contained_counts[mask] = contribution
            kernel_profiles[mask] = ";".join(
                f"{kappa}:{count}" for kappa, count in sorted(by_kappa.items())
            )

        if row["status"] == "ok":
            exact_counts: dict[int, int] = {}
            for mask in sorted(contained_counts, key=int.bit_count):
                count = contained_counts[mask]
                submask = (mask - 1) & mask
                while submask:
                    if submask in exact_counts:
                        count -= exact_counts[submask] * (
                            len(lines) ** (mask.bit_count() - submask.bit_count())
                        )
                    submask = (submask - 1) & mask
                if count < 0:
                    raise RuntimeError(f"negative exact root-line count for mask {mask}: {count}")
                exact_counts[mask] = count

            exact_count = exact_counts[target_mask]
            exact_log = logq(exact_count, prime) if exact_count > 0 else -math.inf
            observed_endpoint_logq = exact_log - root_count
            endpoint_excess = observed_endpoint_logq - endpoint_root_weight
            kernel_profile = kernel_profiles[target_mask]
            layer_diagnostics = kernel_layer_diagnostics(kernel_profile, tau, root_count, prime)
            row.update(
                {
                    "kernel_profile": kernel_profile,
                    "contained_root_line_count": contained_counts[target_mask],
                    "contained_root_line_logq": logq(contained_counts[target_mask], prime),
                    "exact_root_line_count": exact_count,
                    "exact_root_line_logq": exact_log,
                    "asymptotic_root_weight_logq": observed_endpoint_logq,
                    "endpoint_excess_logq": endpoint_excess,
                    "residual_endpoint_excess_logq": (
                        endpoint_excess - projective_factor - nonzero_normalization
                    ),
                    "exceptional_kappa3_count": exceptional_kappa_count(kernel_profile),
                    **layer_diagnostics,
                }
            )

    fieldnames = list(row.keys())
    print(",".join(fieldnames))
    print(",".join(str(row[field]) for field in fieldnames))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--child-depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--size", type=int, default=None)
    parser.add_argument("--tau", type=int, required=True)
    parser.add_argument("--samples", type=int, default=0)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--max-size", type=int, default=10)
    parser.add_argument("--max-line-assignments", type=int, default=2_000_000)
    parser.add_argument("--require-root-count", type=int, default=None)
    parser.add_argument("--require-delta", type=int, default=None)
    parser.add_argument("--require-support-components", type=int, default=None)
    parser.add_argument("--require-generic-kernel-dim", type=int, default=None)
    parser.add_argument("--connected-support-discovery", action="store_true")
    parser.add_argument("--discovery-attempts", type=int, default=100)
    parser.add_argument("--discovery-extra-per-support", type=int, default=4)
    parser.add_argument("--discovery-max-hits", type=int, default=1)
    parser.add_argument("--columns", default=None, help="targeted parent support S, e.g. 0:4:7:11:14")
    parser.add_argument(
        "--support-columns",
        default=None,
        help="targeted exact support A contained in --columns, e.g. 4:7:11:14",
    )
    parser.add_argument("--summary-csv", default=None)
    parser.add_argument(
        "--support-csv",
        default=None,
        help=(
            "write per-support detail rows, including sampled S columns, A/support columns, "
            "endpoint columns, and finite-constant decomposition diagnostics"
        ),
    )
    args = parser.parse_args()

    if args.prime <= 2:
        raise SystemExit("--prime must be > 2")
    for name in (
        "require_root_count",
        "require_delta",
        "require_support_components",
        "require_generic_kernel_dim",
    ):
        value = getattr(args, name)
        if value is not None and value < 0:
            raise SystemExit(f"--{name.replace('_', '-')} must be nonnegative")

    rng = random.Random(args.seed)
    generator = rfc_generator_prime(args.child_depth, args.expansion, args.prime, rng)
    n = len(generator[0])

    if (args.columns is None) != (args.support_columns is None):
        raise SystemExit("--columns and --support-columns must be supplied together")
    if args.connected_support_discovery and args.columns is not None:
        raise SystemExit("--connected-support-discovery cannot be combined with --columns")
    if args.connected_support_discovery:
        run_connected_support_discovery(args, generator, rng)
        return
    if args.columns is not None:
        columns = parse_column_list(args.columns, "--columns")
        support_columns = parse_column_list(args.support_columns, "--support-columns")
        run_targeted_support_replay(
            generator,
            columns,
            support_columns,
            args.tau,
            args.prime,
            args.max_line_assignments,
        )
        return

    if args.size is None:
        raise SystemExit("--size is required unless --columns/--support-columns are supplied")
    if args.size < 0:
        raise SystemExit("--size must be nonnegative")
    if args.size > args.max_size:
        raise SystemExit("--size exceeds --max-size; root-line enumeration is exponential")
    if args.size > n:
        raise SystemExit("--size outside coordinate range")

    lines = projective_lines(args.prime)
    grouped: dict[tuple[int, int, int, int, int, int, int, int, int], list[tuple[int, int, int, str]]] = {}
    detail_rows: list[dict[str, int | float | str]] = []
    checked = 0
    skipped_supports = 0
    filtered_supports = 0
    filter_active = support_filter_active(args)
    for subset_index, columns in enumerate(iter_subsets(n, args.size, args.samples, rng)):
        full_mask = (1 << args.size) - 1
        image_basis = restricted_image_basis(generator, columns, args.prime)
        rho = len(image_basis)
        if args.tau > 2 * rho:
            continue
        ranks = [
            rank_selected_columns(generator, list(columns_from_mask(columns, mask)), args.prime)
            for mask in range(full_mask + 1)
        ]
        components = matroid_component_count(generator, columns, args.prime)
        checked += 1
        contained_counts = [0] * (full_mask + 1)
        line_assignment_counts = [0] * (full_mask + 1)
        kernel_profiles = [""] * (full_mask + 1)
        support_data: dict[int, tuple[tuple[int, ...], int, int, int, int, int]] = {}
        candidate_masks: set[int] = set()
        required_masks: set[int] = set()
        if filter_active:
            for mask in range(1, full_mask + 1):
                support_columns = columns_from_mask(columns, mask)
                support_basis = support_subcode_basis(image_basis, mask, args.prime)
                delta = len(support_basis)
                if args.tau > 2 * delta:
                    filtered_supports += 1
                    continue
                generic_kernel_dim = two_copy_generic_kernel_dim(support_basis, args.prime)
                support_rank = ranks[mask]
                complement_rank = ranks[full_mask ^ mask]
                support_components = matroid_component_count(generator, support_columns, args.prime)
                support_data[mask] = (
                    support_columns,
                    support_rank,
                    support_components,
                    delta,
                    generic_kernel_dim,
                    complement_rank,
                )
                if support_matches_requirements(
                    args,
                    mask.bit_count(),
                    delta,
                    support_components,
                    generic_kernel_dim,
                ):
                    candidate_masks.add(mask)
                    submask = mask
                    while submask:
                        required_masks.add(submask)
                        submask = (submask - 1) & mask
                else:
                    filtered_supports += 1
            masks_to_enumerate = sorted(required_masks, key=int.bit_count)
            masks_to_report = candidate_masks
        else:
            masks_to_enumerate = list(range(1, full_mask + 1))
            masks_to_report = set(range(1, full_mask + 1))
        for mask in masks_to_enumerate:
            root_count = mask.bit_count()
            line_assignment_count = len(lines) ** root_count
            if line_assignment_count > args.max_line_assignments:
                skipped_supports += 1
                continue
            if filter_active:
                if mask not in support_data:
                    continue
                (
                    support_columns,
                    support_rank,
                    support_components,
                    delta,
                    generic_kernel_dim,
                    complement_rank,
                ) = support_data[mask]
                support_basis = support_subcode_basis(image_basis, mask, args.prime)
            else:
                support_columns = columns_from_mask(columns, mask)
                support_basis = support_subcode_basis(image_basis, mask, args.prime)
                delta = len(support_basis)
                if args.tau > 2 * delta:
                    continue
                generic_kernel_dim = two_copy_generic_kernel_dim(support_basis, args.prime)
                support_rank = ranks[mask]
                complement_rank = ranks[full_mask ^ mask]
                support_components = matroid_component_count(generator, support_columns, args.prime)
                support_data[mask] = (
                    support_columns,
                    support_rank,
                    support_components,
                    delta,
                    generic_kernel_dim,
                    complement_rank,
                )
            generic_endpoint, component_endpoint, endpoint_bound = local_endpoint_logq(
                args.tau,
                root_count,
                delta,
                support_components,
                generic_kernel_dim,
            )
            by_kappa: dict[int, int] = {}
            contribution = 0
            for assignment in itertools.product(lines, repeat=root_count):
                kappa = kernel_dimension(support_basis, assignment, args.prime)
                by_kappa[kappa] = by_kappa.get(kappa, 0) + 1
                contribution += gaussian_binomial(kappa, args.tau, args.prime)
            if contribution == 0:
                continue
            kappa_profile = ";".join(f"{kappa}:{count}" for kappa, count in sorted(by_kappa.items()))
            contained_counts[mask] = contribution
            line_assignment_counts[mask] = line_assignment_count
            kernel_profiles[mask] = kappa_profile

        exact_counts = [0] * (full_mask + 1)
        for mask in masks_to_enumerate:
            if mask not in support_data:
                continue
            count = contained_counts[mask]
            submask = (mask - 1) & mask
            while submask:
                count -= exact_counts[submask] * (len(lines) ** (mask.bit_count() - submask.bit_count()))
                submask = (submask - 1) & mask
            if count < 0:
                raise RuntimeError(f"negative exact root-line count for mask {mask}: {count}")
            exact_counts[mask] = count
            if count == 0:
                continue
            if mask not in masks_to_report:
                continue
            (
                support_columns,
                support_rank,
                support_components,
                delta,
                generic_kernel_dim,
                complement_rank,
            ) = support_data[mask]
            root_count = mask.bit_count()
            generic_endpoint, component_endpoint, endpoint_bound = local_endpoint_logq(
                args.tau,
                root_count,
                delta,
                support_components,
                generic_kernel_dim,
            )
            endpoint_root_weight = endpoint_bound - root_count
            kappa_profile = kernel_profiles[mask]
            exact_root_line_log = logq(count, args.prime)
            observed_endpoint_logq = exact_root_line_log - root_count
            endpoint_excess_logq = observed_endpoint_logq - endpoint_root_weight
            projective_factor = projective_line_factor_logq(root_count, args.prime)
            nonzero_normalization = nonzero_root_normalization_logq(root_count, args.prime)
            layer_diagnostics = kernel_layer_diagnostics(
                kappa_profile,
                args.tau,
                root_count,
                args.prime,
            )
            key = (
                rho,
                components,
                args.tau,
                root_count,
                support_rank,
                support_components,
                delta,
                generic_kernel_dim,
                complement_rank,
            )
            grouped.setdefault(key, []).append(
                (count, contained_counts[mask], line_assignment_counts[mask], kappa_profile)
            )
            detail_rows.append(
                {
                    "subset_index": subset_index,
                    "columns": ":".join(str(column) for column in columns),
                    "support_mask": mask,
                    "support_columns": ":".join(str(column) for column in support_columns),
                    "rho": rho,
                    "components": components,
                    "tau": args.tau,
                    "root_count": root_count,
                    "support_rank": support_rank,
                    "support_components": support_components,
                    "delta": delta,
                    "generic_kernel_dim": generic_kernel_dim,
                    "complement_rank": complement_rank,
                    "generic_endpoint_logq": generic_endpoint,
                    "component_endpoint_logq": component_endpoint,
                    "endpoint_bound_logq": endpoint_bound,
                    "endpoint_root_weight_logq": endpoint_root_weight,
                    "line_assignments": line_assignment_counts[mask],
                    "kernel_profile": kappa_profile,
                    "contained_root_line_count": contained_counts[mask],
                    "contained_root_line_logq": logq(contained_counts[mask], args.prime),
                    "exact_root_line_count": count,
                    "exact_root_line_logq": exact_root_line_log,
                    "asymptotic_root_weight_logq": observed_endpoint_logq,
                    "endpoint_excess_logq": endpoint_excess_logq,
                    "projective_line_factor_logq": projective_factor,
                    "nonzero_root_normalization_logq": nonzero_normalization,
                    "exceptional_kappa3_count": exceptional_kappa_count(kappa_profile),
                    **layer_diagnostics,
                    "residual_endpoint_excess_logq": (
                        endpoint_excess_logq - projective_factor - nonzero_normalization
                    ),
                }
            )

    print(
        f"prime={args.prime} child_depth={args.child_depth} expansion={args.expansion} "
        f"size={args.size} tau={args.tau} checked={checked} skipped_supports={skipped_supports} "
        f"filtered_supports={filtered_supports}"
    )
    print(
        "rho,components,tau,root_count,support_rank,support_components,delta,complement_rank,"
        "generic_kernel_dim,generic_endpoint_logq,component_endpoint_logq,endpoint_bound_logq,"
        "endpoint_root_weight_logq,shapes,min_exact_root_line_logq,avg_exact_root_line_logq,"
        "max_exact_root_line_logq,"
        "min_contained_root_line_logq,max_contained_root_line_logq,"
        "min_asymptotic_root_weight_logq,max_asymptotic_root_weight_logq,kernel_profiles"
    )
    summary_rows: list[dict[str, int | float | str]] = []
    for key, values in sorted(grouped.items()):
        exact_contributions = [exact for exact, _contained, _line_count, _profile in values]
        contained_contributions = [contained for _exact, contained, _line_count, _profile in values]
        exact_logs = [logq(contribution, args.prime) for contribution in exact_contributions]
        contained_logs = [logq(contribution, args.prime) for contribution in contained_contributions]
        (
            rho,
            components,
            tau,
            root_count,
            support_rank,
            support_components,
            delta,
            generic_kernel_dim,
            complement_rank,
        ) = key
        profiles = sorted({profile for _exact, _contained, _line_count, profile in values})
        generic_endpoint, component_endpoint, endpoint_bound = local_endpoint_logq(
            tau,
            root_count,
            delta,
            support_components,
            generic_kernel_dim,
        )
        row = {
            "rho": rho,
            "components": components,
            "tau": tau,
            "root_count": root_count,
            "support_rank": support_rank,
            "support_components": support_components,
            "delta": delta,
            "generic_kernel_dim": generic_kernel_dim,
            "complement_rank": complement_rank,
            "generic_endpoint_logq": generic_endpoint,
            "component_endpoint_logq": component_endpoint,
            "endpoint_bound_logq": endpoint_bound,
            "endpoint_root_weight_logq": endpoint_bound - root_count,
            "shapes": len(values),
            "min_exact_root_line_logq": min(exact_logs),
            "avg_exact_root_line_logq": sum(exact_logs) / len(exact_logs),
            "max_exact_root_line_logq": max(exact_logs),
            "min_contained_root_line_logq": min(contained_logs),
            "max_contained_root_line_logq": max(contained_logs),
            "min_asymptotic_root_weight_logq": min(value - root_count for value in exact_logs),
            "max_asymptotic_root_weight_logq": max(value - root_count for value in exact_logs),
            "kernel_profiles": "|".join(profiles),
        }
        summary_rows.append(row)
        print(
            f"{rho},{components},{tau},{root_count},{support_rank},{support_components},"
            f"{delta},{complement_rank},{generic_kernel_dim},"
            f"{row['generic_endpoint_logq']:.10f},"
            f"{row['component_endpoint_logq']:.10f},"
            f"{row['endpoint_bound_logq']:.10f},"
            f"{row['endpoint_root_weight_logq']:.10f},{row['shapes']},"
            f"{row['min_exact_root_line_logq']:.10f},"
            f"{row['avg_exact_root_line_logq']:.10f},"
            f"{row['max_exact_root_line_logq']:.10f},"
            f"{row['min_contained_root_line_logq']:.10f},"
            f"{row['max_contained_root_line_logq']:.10f},"
            f"{row['min_asymptotic_root_weight_logq']:.10f},"
            f"{row['max_asymptotic_root_weight_logq']:.10f},{row['kernel_profiles']}"
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
                    "support_rank",
                    "support_components",
                    "delta",
                    "complement_rank",
                    "generic_kernel_dim",
                    "generic_endpoint_logq",
                    "component_endpoint_logq",
                    "endpoint_bound_logq",
                    "endpoint_root_weight_logq",
                    "shapes",
                    "min_exact_root_line_logq",
                    "avg_exact_root_line_logq",
                    "max_exact_root_line_logq",
                    "min_contained_root_line_logq",
                    "max_contained_root_line_logq",
                    "min_asymptotic_root_weight_logq",
                    "max_asymptotic_root_weight_logq",
                    "kernel_profiles",
                ],
            )
            writer.writeheader()
            writer.writerows(summary_rows)

    if args.support_csv is not None:
        with Path(args.support_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "subset_index",
                    "columns",
                    "support_mask",
                    "support_columns",
                    "rho",
                    "components",
                    "tau",
                    "root_count",
                    "support_rank",
                    "support_components",
                    "delta",
                    "generic_kernel_dim",
                    "complement_rank",
                    "generic_endpoint_logq",
                    "component_endpoint_logq",
                    "endpoint_bound_logq",
                    "endpoint_root_weight_logq",
                    "line_assignments",
                    "kernel_profile",
                    "contained_root_line_count",
                    "contained_root_line_logq",
                    "exact_root_line_count",
                    "exact_root_line_logq",
                    "asymptotic_root_weight_logq",
                    "endpoint_excess_logq",
                    "projective_line_factor_logq",
                    "nonzero_root_normalization_logq",
                    "exceptional_kappa3_count",
                    "dominant_contained_layer_h",
                    "dominant_contained_layer_root_weight_logq",
                    "dominant_contained_layer_codim_logq",
                    "empirical_theta_tau_logq",
                    "residual_endpoint_excess_logq",
                ],
            )
            writer.writeheader()
            writer.writerows(detail_rows)


if __name__ == "__main__":
    main()
