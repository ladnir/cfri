#!/usr/bin/env python3
"""Closed-form paired-spine lift diagnostic for complete-stride seed flags.

This diagnostic starts from the 48 ordered depth-4 complete-stride flags and
counts one paired lift using the proof-contract Lift term.  It deliberately
does not enumerate Gaussian subspaces over the field; for large primes the
canonical lift count is computed with Gaussian-binomial formulas.
"""

from __future__ import annotations

import argparse
import itertools
import math
from typing import Iterable


def fmt_float(value: float | str) -> str:
    if isinstance(value, str):
        return value
    if math.isinf(value):
        return "inf" if value > 0 else "-inf"
    return f"{value:.10f}"


def logq(value: int, prime: int) -> float:
    if value <= 0:
        return -math.inf
    return math.log(value) / math.log(prime)


def gaussian_binomial(ambient_dim: int, sub_dim: int, prime: int) -> int:
    if sub_dim < 0 or sub_dim > ambient_dim:
        return 0
    sub_dim = min(sub_dim, ambient_dim - sub_dim)
    out = 1
    for i in range(sub_dim):
        out *= prime ** (ambient_dim - i) - 1
        out //= prime ** (sub_dim - i) - 1
    return out


def columns_text(columns: Iterable[int]) -> str:
    return ":".join(str(column) for column in columns)


def stride_class(stride_index: int, k: int, live_rows: int) -> tuple[int, ...]:
    return tuple(range(stride_index, k, live_rows))


def complete_stride_seed_rows(depth: int, live_rows: int) -> list[dict[str, int | str]]:
    if depth != 4 or live_rows != 4:
        raise SystemExit("paired-spine cascade currently uses the depth-4, m=4 seed family")
    k = 1 << depth
    rows: list[dict[str, int | str]] = []
    for block_index in range(k // live_rows):
        block = tuple(range(block_index * live_rows, (block_index + 1) * live_rows))
        for left_stride, right_stride in itertools.combinations(range(live_rows), 2):
            left_class = stride_class(left_stride, k, live_rows)
            right_class = stride_class(right_stride, k, live_rows)
            omega = tuple(sorted(left_class + right_class))
            outer_zero = tuple(column for column in range(k) if column not in set(omega))
            for kernel_stride, visible_stride, kernel_class, visible_class in [
                (left_stride, right_stride, left_class, right_class),
                (right_stride, left_stride, right_class, left_class),
            ]:
                inner_zero = tuple(column for column in range(k) if column not in set(kernel_class))
                rows.append(
                    {
                        "child_h": depth,
                        "row_block": columns_text(block),
                        "kernel_stride": kernel_stride,
                        "visible_stride": visible_stride,
                        "support_key": f"B={columns_text(block)}|Omega={columns_text(omega)}",
                        "outer_zero_key": columns_text(outer_zero),
                        "inner_zero_key": columns_text(inner_zero),
                        "p": len(outer_zero),
                        "s": len(visible_class),
                        "a": len(visible_class),
                        "r1": 2,
                        "r0": 1,
                        "z_V": len(outer_zero),
                        "z_L": len(inner_zero),
                        "rank_S": 1,
                        "rank_S_minus_A": 0,
                        "delta": 1,
                        "comp": 1,
                        "g": 0,
                    }
                )
    return rows


def complete_stride_tau2_aggregate_seed_rows(depth: int, live_rows: int) -> list[dict[str, int | str]]:
    if depth != 4 or live_rows != 4:
        raise SystemExit("tau=2 aggregate seed currently uses the depth-4, m=4 complete-stride family")
    k = 1 << depth
    rows: list[dict[str, int | str]] = []
    for block_index in range(k // live_rows):
        block = tuple(range(block_index * live_rows, (block_index + 1) * live_rows))
        for left_stride, right_stride in itertools.combinations(range(live_rows), 2):
            left_class = stride_class(left_stride, k, live_rows)
            right_class = stride_class(right_stride, k, live_rows)
            omega = tuple(sorted(left_class + right_class))
            outer_zero = tuple(column for column in range(k) if column not in set(omega))
            rows.append(
                {
                    "child_h": depth,
                    "row_block": columns_text(block),
                    "kernel_stride": "NA",
                    "visible_stride": f"{left_stride}:{right_stride}",
                    "support_key": f"B={columns_text(block)}|Omega={columns_text(omega)}",
                    "outer_zero_key": columns_text(outer_zero),
                    "inner_zero_key": "NA",
                    "p": len(outer_zero),
                    "s": len(omega),
                    "a": len(omega),
                    "r1": 2,
                    "r0": 0,
                    "z_V": len(outer_zero),
                    "z_L": len(outer_zero) + len(omega),
                    "rank_S": 2,
                    "rank_S_minus_A": 0,
                    "delta": 2,
                    "comp": 2,
                    "g": 0,
                }
            )
    return rows


def complete_stride_rows_for_tau(depth: int, live_rows: int, tau: int, seed_mode: str) -> list[dict[str, int | str]]:
    resolved_mode = seed_mode
    if seed_mode == "auto":
        resolved_mode = "aggregate_tau2" if tau == 2 else "ordered_tau1"
    if resolved_mode == "ordered_tau1":
        return complete_stride_seed_rows(depth, live_rows)
    if resolved_mode == "aggregate_tau2":
        return complete_stride_tau2_aggregate_seed_rows(depth, live_rows)
    raise SystemExit(f"unknown seed mode {seed_mode}")


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


def parse_t_values(text: str) -> list[int]:
    values = [int(part) for part in text.split(":") if part]
    if not values:
        raise SystemExit("--t-values must not be empty")
    return values


def lift_exponent(*, t: int, tau: int, kappa: int, r1: int, r0: int) -> int:
    return kappa * (2 * r0 - kappa) + tau * (2 * r1 - t)


def product_rank1_tau2_endpoint_count(seed: dict[str, int | str], prime: int) -> int | None:
    if (
        int(seed["a"]) == 8
        and int(seed["delta"]) == 2
        and int(seed["comp"]) == 2
        and int(seed["g"]) == 0
        and int(seed["rank_S"]) == 2
        and int(seed["rank_S_minus_A"]) == 0
    ):
        return (prime + 1) ** 2
    return None


def profile_rows(
    *,
    prime: int,
    depth: int,
    live_rows: int,
    chain_length: int,
    t_values: list[int],
    tau: int,
    tracked_kappa: int,
    kernel_mode: str,
    downstream_constraint_key: str,
    seed_mode: str,
) -> list[dict[str, int | float | str]]:
    seeds = complete_stride_rows_for_tau(depth, live_rows, tau, seed_mode)
    seed_keys = {
        (
            row["support_key"],
            row["kernel_stride"],
            row["visible_stride"],
            row["outer_zero_key"],
            row["inner_zero_key"],
        )
        for row in seeds
    }
    seed_duplicate_count = len(seeds) - len(seed_keys)
    seed = seeds[0]
    rows: list[dict[str, int | float | str]] = []
    for t in t_values:
        h = depth + chain_length - 1
        z = 2 * int(seed["p"]) + int(seed["s"])
        full_kernel_dim = t - tau
        if full_kernel_dim < 0:
            raise SystemExit("t must be >= tau")
        if kernel_mode != "full_kernel":
            raise SystemExit("only --kernel-mode full_kernel is currently implemented")
        if tracked_kappa > full_kernel_dim:
            raise SystemExit("--tracked-kappa must be <= full_kernel_dim for every requested t")
        chain_key = (
            f"h={h}|t={t}|kernel_mode=full_kernel|tau={tau}|full_kernel_dim={full_kernel_dim}|"
            f"tracked_kappa={tracked_kappa}|r1={seed['r1']}|r0={seed['r0']}|"
            f"zV={seed['z_V']}|zL={seed['z_L']}|seed_flags={len(seeds)}"
        )
        hidden_line_fiber_count = (
            gaussian_binomial(full_kernel_dim, tracked_kappa, prime)
            if 0 < tracked_kappa < full_kernel_dim
            else 1
        )
        hidden_line_fiber_logq = logq(hidden_line_fiber_count, prime)
        hidden_line_fiber_charged = int(bool(downstream_constraint_key))
        endpoint_component_logq: int | str = "NA"
        endpoint_generic_logq: int | str = "NA"
        endpoint_bound_logq: int | str = "NA"
        canonical_endpoint_event_count: int | str = "NA"
        exact_support_event_count: int | str = "NA"
        observed_endpoint_logq: float | str = "NA"
        endpoint_status = "not_applicable"
        endpoint_excess_logq: float | str = "NA"
        if tau == 2:
            endpoint_component_logq = int(seed["comp"]) + 2 * int(seed["delta"]) - 4 - int(seed["a"])
            endpoint_generic_logq = 2 * int(seed["g"]) - 4
            endpoint_bound_logq = max(endpoint_component_logq, endpoint_generic_logq)
            endpoint_count = product_rank1_tau2_endpoint_count(seed, prime)
            if endpoint_count is None:
                endpoint_status = "not_computed_requires_exact_support_endpoint_count"
            else:
                canonical_endpoint_event_count = endpoint_count
                exact_support_event_count = endpoint_count
                observed_endpoint_logq = logq(endpoint_count, prime) - int(seed["a"])
                endpoint_excess_logq = observed_endpoint_logq - endpoint_bound_logq
                endpoint_status = "closed_form_product_rank1_components"
        base_row: dict[str, int | float | str] = {
            "h": h,
            "t": t,
            "z": z,
            "p": seed["p"],
            "s": seed["s"],
            "a": seed["a"],
            "tau": tau,
            "kappa": full_kernel_dim,
            "full_kernel_dim": full_kernel_dim,
            "tracked_kappa": tracked_kappa,
            "kernel_mode": "full_kernel",
            "r1": seed["r1"],
            "r0": seed["r0"],
            "z_V": seed["z_V"],
            "z_L": seed["z_L"],
            "rank_S": seed["rank_S"],
            "rank_S_minus_A": seed["rank_S_minus_A"],
            "delta": seed["delta"],
            "comp": seed["comp"],
            "g": seed["g"],
            "seed_ordered_flags": len(seeds),
            "seed_duplicate_certificate_count": seed_duplicate_count,
            "chain_length": chain_length,
            "chain_key": chain_key,
            "seed_mode": seed_mode,
            "downstream_constraint_key": downstream_constraint_key,
            "hidden_line_fiber_count": hidden_line_fiber_count,
            "hidden_line_fiber_logq": hidden_line_fiber_logq,
            "hidden_line_fiber_charged": hidden_line_fiber_charged,
            "endpoint_component_logq": endpoint_component_logq,
            "endpoint_generic_logq": endpoint_generic_logq,
            "endpoint_bound_logq": endpoint_bound_logq,
            "canonical_endpoint_event_count": canonical_endpoint_event_count,
            "exact_support_event_count": exact_support_event_count,
            "observed_endpoint_logq": observed_endpoint_logq,
            "endpoint_excess_logq": endpoint_excess_logq,
            "endpoint_status": endpoint_status,
        }

        if tau == 2 and int(seed["delta"]) < 2:
            base_row.update(
                {
                    "k_subspaces_per_seed": "NA",
                    "w_extensions_per_k": "NA",
                    "chain_count": 0,
                    "canonical_chain_count": 0,
                    "duplicate_certificate_count": seed_duplicate_count,
                    "expected_lift_logq": "NA",
                    "observed_chain_logq": "NA",
                    "chain_excess_logq": "NA",
                    "status": "impossible_tau2_delta_lt2",
                    "blocker": "tau=2 requires delta>=2",
                }
            )
            rows.append(base_row)
            continue

        r1 = int(seed["r1"])
        r0 = int(seed["r0"])
        quotient_dim = t - full_kernel_dim
        k_subspaces = gaussian_binomial(2 * r0, full_kernel_dim, prime)
        w_extensions = gaussian_binomial(2 * r1 - full_kernel_dim, quotient_dim, prime)
        chains_per_seed = k_subspaces * w_extensions
        canonical_chains = len(seeds) * chains_per_seed
        duplicate_certificates = seed_duplicate_count
        if not hidden_line_fiber_charged:
            duplicate_certificates += canonical_chains * (hidden_line_fiber_count - 1)
        lift_bound = lift_exponent(t=t, tau=tau, kappa=full_kernel_dim, r1=r1, r0=r0)
        observed = logq(chains_per_seed, prime)
        base_row.update(
            {
                "k_subspaces_per_seed": k_subspaces,
                "w_extensions_per_k": w_extensions,
                "chain_count": canonical_chains,
                "canonical_chain_count": canonical_chains,
                "duplicate_certificate_count": duplicate_certificates,
                "expected_lift_logq": lift_bound,
                "observed_chain_logq": observed,
                "chain_excess_logq": observed - lift_bound,
                "status": "ok"
                if tracked_kappa == full_kernel_dim or downstream_constraint_key
                else "ok_tracked_line_metadata_uncharged",
                "blocker": ""
                if tracked_kappa == full_kernel_dim or downstream_constraint_key
                else "tracked line is metadata only; not counted without downstream_constraint_key",
            }
        )
        rows.append(base_row)
    return rows


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--depth", type=int, default=4)
    parser.add_argument("--near-live-rows", type=int, default=4)
    parser.add_argument("--chain-length", type=int, default=2)
    parser.add_argument("--t-values", default="2:3")
    parser.add_argument("--tau", type=int, default=1)
    parser.add_argument("--tracked-kappa", type=int, default=1)
    parser.add_argument("--kernel-mode", choices=["full_kernel"], default="full_kernel")
    parser.add_argument("--downstream-constraint-key", default="")
    parser.add_argument("--seed-mode", choices=["auto", "ordered_tau1", "aggregate_tau2"], default="auto")
    parser.add_argument("--positive-epsilon", type=float, default=1e-12)
    args = parser.parse_args()

    if args.prime <= 2:
        raise SystemExit("--prime must be > 2")
    if args.chain_length != 2:
        raise SystemExit("paired-spine cascade scaffold currently supports --chain-length 2 only")
    if args.tau < 0 or args.tracked_kappa < 0:
        raise SystemExit("--tau and --tracked-kappa must be nonnegative")

    rows = profile_rows(
        prime=args.prime,
        depth=args.depth,
        live_rows=args.near_live_rows,
        chain_length=args.chain_length,
        t_values=parse_t_values(args.t_values),
        tau=args.tau,
        tracked_kappa=args.tracked_kappa,
        kernel_mode=args.kernel_mode,
        downstream_constraint_key=args.downstream_constraint_key,
        seed_mode=args.seed_mode,
    )
    completed = [row for row in rows if str(row["status"]).startswith("ok")]
    positive = [
        row
        for row in completed
        if isinstance(row["chain_excess_logq"], float)
        and row["chain_excess_logq"] > args.positive_epsilon
    ]
    q_dimension_positive = [
        row
        for row in completed
        if isinstance(row["chain_excess_logq"], float) and row["chain_excess_logq"] >= 1.0
    ]
    summary = [
        {
            "prime": args.prime,
            "depth": args.depth,
            "near_live_rows": args.near_live_rows,
            "chain_length": args.chain_length,
            "tau": args.tau,
            "t_values": args.t_values,
            "seed_mode": args.seed_mode,
            "profile_rows": len(rows),
            "completed_rows": len(completed),
            "skipped_rows": len(rows) - len(completed),
            "chain_count": sum(int(row["chain_count"]) for row in completed),
            "canonical_chain_count": sum(int(row["canonical_chain_count"]) for row in completed),
            "duplicate_certificate_count": sum(int(row["duplicate_certificate_count"]) for row in rows),
            "max_expected_lift_logq": max(
                (float(row["expected_lift_logq"]) for row in completed),
                default=-math.inf,
            ),
            "max_observed_chain_logq": max(
                (float(row["observed_chain_logq"]) for row in completed),
                default=-math.inf,
            ),
            "max_chain_excess_logq": max(
                (float(row["chain_excess_logq"]) for row in completed),
                default=-math.inf,
            ),
            "max_hidden_line_fiber_logq": max(
                (float(row["hidden_line_fiber_logq"]) for row in completed),
                default=-math.inf,
            ),
            "max_endpoint_component_logq": max(
                (
                    int(row["endpoint_component_logq"])
                    for row in completed
                    if isinstance(row["endpoint_component_logq"], int)
                ),
                default="NA",
            ),
            "max_endpoint_generic_logq": max(
                (
                    int(row["endpoint_generic_logq"])
                    for row in completed
                    if isinstance(row["endpoint_generic_logq"], int)
                ),
                default="NA",
            ),
            "max_endpoint_bound_logq": max(
                (
                    int(row["endpoint_bound_logq"])
                    for row in completed
                    if isinstance(row["endpoint_bound_logq"], int)
                ),
                default="NA",
            ),
            "max_observed_endpoint_logq": max(
                (
                    float(row["observed_endpoint_logq"])
                    for row in completed
                    if isinstance(row["observed_endpoint_logq"], float)
                ),
                default="NA",
            ),
            "max_endpoint_excess_logq": max(
                (
                    float(row["endpoint_excess_logq"])
                    for row in completed
                    if isinstance(row["endpoint_excess_logq"], float)
                ),
                default="NA",
            ),
            "positive_excess_witness_rows": len(positive),
            "q_dimension_excess_witness_rows": len(q_dimension_positive),
            "blockers": "|".join(
                sorted(
                    {
                        str(row["endpoint_status"])
                        for row in completed
                        if row["endpoint_status"] != "not_applicable"
                    }
                )
            )
            or "none",
            "status": "blocked_shape" if not completed else "ok_with_skips" if len(completed) < len(rows) else "ok",
        }
    ]
    print_rows("paired_spine_cascade_summary", summary)
    print_rows("paired_spine_cascade_profiles", rows)
    print_rows("paired_spine_positive_excess_witnesses", positive)


if __name__ == "__main__":
    main()
