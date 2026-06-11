#!/usr/bin/env python3
r"""Support-profile upper bound for local visible-span states.

For a selected child coordinate set S with image U <= F^S and support A subset S, define:

    delta(A) = dim{u in U : supp(u) subset A}
             = rank(S) - rank(S \ A).

Any tau-dimensional visible subspace R <= U + U whose visible support is contained in A must lie in
U_A + U_A, so the number of such R is at most [2 delta(A) choose tau]_q.  Subtracting smaller
supports gives an exact-support upper-bound profile depending only on the represented matroid rank
function on S.

For tau=1 this is the exact compatible line-support profile, since every line is locally
rank-one-compatible.  For tau>1 this is an ambient support bound; exterior/rank-one equations can
and should give additional savings.
"""

from __future__ import annotations

import argparse
import csv
import itertools
import math
import random
from pathlib import Path

from rfc_exterior_constraint_profile import matroid_component_count
from sample_rfc_rank_failure import rank_selected_columns, rfc_generator_prime


def iter_subsets(n: int, size: int, samples: int, rng: random.Random):
    if samples <= 0:
        yield from itertools.combinations(range(n), size)
        return
    for _ in range(samples):
        yield tuple(sorted(rng.sample(range(n), size)))


def columns_from_mask(columns: tuple[int, ...], mask: int) -> tuple[int, ...]:
    return tuple(columns[index] for index in range(len(columns)) if (mask >> index) & 1)


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


def rank_profile(
    generator: list[list[int]],
    columns: tuple[int, ...],
    prime: int,
) -> list[int]:
    ranks = [0] * (1 << len(columns))
    for mask in range(1, 1 << len(columns)):
        ranks[mask] = rank_selected_columns(generator, list(columns_from_mask(columns, mask)), prime)
    return ranks


def exact_support_counts(
    ranks: list[int],
    full_mask: int,
    tau: int,
    prime: int,
) -> tuple[list[int], list[int], list[int]]:
    rho = ranks[full_mask]
    contained = [0] * (full_mask + 1)
    delta = [0] * (full_mask + 1)
    for mask in range(full_mask + 1):
        complement = full_mask ^ mask
        delta[mask] = rho - ranks[complement]
        contained[mask] = gaussian_binomial(2 * delta[mask], tau, prime)

    exact = [0] * (full_mask + 1)
    for mask in sorted(range(full_mask + 1), key=int.bit_count):
        count = contained[mask]
        submask = (mask - 1) & mask
        while submask:
            count -= exact[submask]
            submask = (submask - 1) & mask
        if mask != 0:
            count -= exact[0]
        if count < 0:
            raise RuntimeError(f"negative exact-support count for mask {mask}: {count}")
        exact[mask] = count
    return contained, exact, delta


def summarize_values(values: list[int], prime: int) -> tuple[float, float, float]:
    logs = [logq(value, prime) for value in values]
    return min(logs), sum(logs) / len(logs), max(logs)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--child-depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--size", type=int, required=True)
    parser.add_argument("--tau", type=int, required=True)
    parser.add_argument("--samples", type=int, default=0)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--max-size", type=int, default=16)
    parser.add_argument("--summary-csv", default=None)
    parser.add_argument("--support-csv", default=None)
    args = parser.parse_args()

    if args.prime <= 2:
        raise SystemExit("--prime must be > 2")
    if args.size < 0:
        raise SystemExit("--size must be nonnegative")
    if args.size > args.max_size:
        raise SystemExit("--size exceeds --max-size; the support lattice is exponential")

    rng = random.Random(args.seed)
    generator = rfc_generator_prime(args.child_depth, args.expansion, args.prime, rng)
    n = len(generator[0])
    if args.size > n:
        raise SystemExit("--size outside coordinate range")

    grouped: dict[tuple[int, int, int, int, int, int, int, int], list[tuple[int, int]]] = {}
    detail_rows: list[dict[str, int | float | str]] = []
    checked = 0
    for subset_index, columns in enumerate(iter_subsets(n, args.size, args.samples, rng)):
        full_mask = (1 << args.size) - 1
        ranks = rank_profile(generator, columns, args.prime)
        rho = ranks[full_mask]
        if args.tau > 2 * rho:
            continue
        components = matroid_component_count(generator, columns, args.prime)
        contained, exact, delta = exact_support_counts(ranks, full_mask, args.tau, args.prime)
        checked += 1
        for mask, exact_count in enumerate(exact):
            if exact_count == 0:
                continue
            support_columns = columns_from_mask(columns, mask)
            complement_mask = full_mask ^ mask
            support_rank = ranks[mask]
            support_components = matroid_component_count(generator, support_columns, args.prime)
            complement_rank = ranks[complement_mask]
            root_count = mask.bit_count()
            key = (
                rho,
                components,
                args.tau,
                root_count,
                support_rank,
                support_components,
                delta[mask],
                complement_rank,
            )
            grouped.setdefault(key, []).append((exact_count, contained[mask]))
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
                    "delta": delta[mask],
                    "complement_rank": complement_rank,
                    "contained_count": contained[mask],
                    "exact_support_count": exact_count,
                    "exact_logq": logq(exact_count, args.prime),
                    "asymptotic_root_weight_logq": logq(exact_count, args.prime) - root_count,
                }
            )

    print(
        f"prime={args.prime} child_depth={args.child_depth} expansion={args.expansion} "
        f"size={args.size} tau={args.tau} checked={checked}"
    )
    print(
        "rho,components,tau,root_count,support_rank,support_components,delta,complement_rank,"
        "shapes,min_exact_logq,avg_exact_logq,max_exact_logq,"
        "min_contained_logq,avg_contained_logq,max_contained_logq,"
        "min_asymptotic_root_weight_logq,max_asymptotic_root_weight_logq"
    )
    summary_rows: list[dict[str, int | float]] = []
    for key, values in sorted(grouped.items()):
        exact_values = [exact_count for exact_count, _contained_count in values]
        contained_values = [contained_count for _exact_count, contained_count in values]
        min_exact, avg_exact, max_exact = summarize_values(exact_values, args.prime)
        min_contained, avg_contained, max_contained = summarize_values(contained_values, args.prime)
        rho, components, tau, root_count, support_rank, support_components, delta_value, complement_rank = key
        weight_logs = [logq(value, args.prime) - root_count for value in exact_values]
        row = {
            "rho": rho,
            "components": components,
            "tau": tau,
            "root_count": root_count,
            "support_rank": support_rank,
            "support_components": support_components,
            "delta": delta_value,
            "complement_rank": complement_rank,
            "shapes": len(values),
            "min_exact_logq": min_exact,
            "avg_exact_logq": avg_exact,
            "max_exact_logq": max_exact,
            "min_contained_logq": min_contained,
            "avg_contained_logq": avg_contained,
            "max_contained_logq": max_contained,
            "min_asymptotic_root_weight_logq": min(weight_logs),
            "max_asymptotic_root_weight_logq": max(weight_logs),
        }
        summary_rows.append(row)
        print(
            f"{rho},{components},{tau},{root_count},{support_rank},{support_components},"
            f"{delta_value},{complement_rank},{row['shapes']},"
            f"{min_exact:.10f},{avg_exact:.10f},{max_exact:.10f},"
            f"{min_contained:.10f},{avg_contained:.10f},{max_contained:.10f},"
            f"{row['min_asymptotic_root_weight_logq']:.10f},"
            f"{row['max_asymptotic_root_weight_logq']:.10f}"
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
                    "shapes",
                    "min_exact_logq",
                    "avg_exact_logq",
                    "max_exact_logq",
                    "min_contained_logq",
                    "avg_contained_logq",
                    "max_contained_logq",
                    "min_asymptotic_root_weight_logq",
                    "max_asymptotic_root_weight_logq",
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
                    "complement_rank",
                    "contained_count",
                    "exact_support_count",
                    "exact_logq",
                    "asymptotic_root_weight_logq",
                ],
            )
            writer.writeheader()
            writer.writerows(detail_rows)


if __name__ == "__main__":
    main()
