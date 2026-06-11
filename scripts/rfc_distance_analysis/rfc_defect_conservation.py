#!/usr/bin/env python3
"""Check fixed-witness rank-defect conservation for one RFC fold.

This is a deterministic proof diagnostic.  It does not sample codes.  For a
fixed parent zero witness with

    z = 2p + s,

it enumerates one-step split/profile exponents and checks whether

    local_charge + child_defect - lift_exponent >= parent_defect.

The inequality is the fixed-witness version of the exposed-flag induction
target in docs/rfc_distance_analysis/rfc_multilayer_flag_transition_theorem.md.
Negative slack marks a profile that needs a sharper local/incidence theorem or
an enlarged child flag state.
"""

from __future__ import annotations

import argparse
import csv
import math
import sys
from dataclasses import asdict, dataclass
from pathlib import Path

from rfc_flag_span_moment import INF, local_visible_trace_profile


def ceil_div(num: int, den: int) -> int:
    return (num + den - 1) // den


def flag_dimension(ambient_k: int, dims: tuple[int, ...]) -> int:
    """Grassmann dimension of a nested flag with the given dimensions."""

    total = 0
    lower = 0
    for dim in reversed(dims):
        total += (dim - lower) * (ambient_k - dim)
        lower = dim
    return total


def flag_zero_equations(dims: tuple[int, ...], zeros: tuple[int, ...]) -> int:
    """Zero-equation count for nested witnesses with increasing zero budgets."""

    if len(dims) != len(zeros):
        raise ValueError("dims and zeros must have the same length")
    total = zeros[0] * dims[0]
    for index in range(1, len(dims)):
        total += (zeros[index] - zeros[index - 1]) * dims[index]
    return total


def flag_defect(ambient_k: int, dims: tuple[int, ...], zeros: tuple[int, ...]) -> int:
    """Generic fixed-witness defect: zero equations minus flag dimension."""

    return flag_zero_equations(dims, zeros) - flag_dimension(ambient_k, dims)


@dataclass(frozen=True)
class Profile:
    child_k: int
    expansion: int
    parent_dim: int
    parent_zeros: int
    paired: int
    singletons: int
    visible_support: int
    visible_tau: int
    outer_dim: int
    inner_dim: int
    outer_zeros: int
    inner_zeros: int
    local_charge: int
    local_delta: int
    local_components: int
    local_g: int
    local_theta: int
    local_h: int
    local_gamma: int
    parent_defect: int
    child_defect: int
    lift_exponent: int
    slack: int


def profile_slack(
    *,
    child_k: int,
    expansion: int,
    parent_dim: int,
    parent_zeros: int,
    paired: int,
    singletons: int,
    visible_support: int,
    visible_tau: int,
    outer_dim: int,
    inner_dim: int,
    singleton_charge_mode: str,
) -> Profile | None:
    parent_k = 2 * child_k
    child_n = expansion * child_k
    kernel_dim = parent_dim - visible_tau
    outer_zeros = paired + singletons - visible_support
    inner_zeros = paired + singletons
    if not (0 <= outer_zeros <= inner_zeros <= child_n):
        return None
    if inner_dim > outer_dim:
        return None
    if visible_tau == 0 and inner_dim != outer_dim:
        return None

    quotient_delta_floor = 0
    if singleton_charge_mode == "endpoint-tau2-layer-incidence" and visible_tau == 2:
        quotient_delta_floor = max(0, outer_dim - inner_dim)

    (
        local_charge,
        local_delta,
        local_components,
        local_g,
        local_theta,
        local_h,
        local_gamma,
    ) = local_visible_trace_profile(
        visible_tau=visible_tau,
        child_k=child_k,
        outer_zero_count=outer_zeros,
        visible_support_size=visible_support,
        singleton_charge_mode=singleton_charge_mode,
        quotient_delta_floor=quotient_delta_floor,
    )
    if local_charge >= INF:
        return None

    parent_def = flag_defect(parent_k, (parent_dim,), (parent_zeros,))
    if visible_tau == 0:
        child_def = flag_defect(child_k, (outer_dim,), (outer_zeros,))
        lift = parent_dim * (2 * outer_dim - parent_dim)
    else:
        child_def = flag_defect(child_k, (outer_dim, inner_dim), (outer_zeros, inner_zeros))
        lift = kernel_dim * (2 * inner_dim - kernel_dim)
        lift += visible_tau * (2 * outer_dim - parent_dim)
    slack = local_charge + child_def - lift - parent_def

    return Profile(
        child_k=child_k,
        expansion=expansion,
        parent_dim=parent_dim,
        parent_zeros=parent_zeros,
        paired=paired,
        singletons=singletons,
        visible_support=visible_support,
        visible_tau=visible_tau,
        outer_dim=outer_dim,
        inner_dim=inner_dim,
        outer_zeros=outer_zeros,
        inner_zeros=inner_zeros,
        local_charge=local_charge,
        local_delta=local_delta,
        local_components=local_components,
        local_g=local_g,
        local_theta=local_theta,
        local_h=local_h,
        local_gamma=local_gamma,
        parent_defect=parent_def,
        child_defect=child_def,
        lift_exponent=lift,
        slack=slack,
    )


def projection_dims(total_dim: int, child_k: int) -> range:
    """Feasible exact projection dimensions for a subspace of F^k + F^k."""

    if total_dim == 0:
        return range(0, 1)
    lo = ceil_div(total_dim, 2)
    hi = min(total_dim, child_k)
    return range(lo, hi + 1)


def iter_profiles(
    *,
    child_k: int,
    expansion: int,
    parent_dim: int,
    parent_zeros: int,
    max_visible_tau: int,
    singleton_charge_mode: str,
) -> list[Profile]:
    child_n = expansion * child_k
    profiles: list[Profile] = []
    for paired in range(parent_zeros // 2 + 1):
        singletons = parent_zeros - 2 * paired
        if paired + singletons > child_n:
            continue
        for visible_support in range(0, singletons + 1):
            outer_zeros = paired + singletons - visible_support
            if outer_zeros > child_n:
                continue
            max_tau = min(parent_dim, max_visible_tau, visible_support)
            for visible_tau in range(max_tau + 1):
                if visible_tau == 0 and visible_support != 0:
                    continue
                if visible_tau > 0 and visible_support == 0:
                    continue
                kernel_dim = parent_dim - visible_tau
                for inner_dim in projection_dims(kernel_dim, child_k):
                    for outer_dim in projection_dims(parent_dim, child_k):
                        if inner_dim > outer_dim:
                            continue
                        if visible_tau == 0 and inner_dim != outer_dim:
                            continue
                        if visible_tau > 0 and visible_tau > 2 * outer_dim - kernel_dim:
                            continue
                        profile = profile_slack(
                            child_k=child_k,
                            expansion=expansion,
                            parent_dim=parent_dim,
                            parent_zeros=parent_zeros,
                            paired=paired,
                            singletons=singletons,
                            visible_support=visible_support,
                            visible_tau=visible_tau,
                            outer_dim=outer_dim,
                            inner_dim=inner_dim,
                            singleton_charge_mode=singleton_charge_mode,
                        )
                        if profile is not None:
                            profiles.append(profile)
    profiles.sort(key=lambda row: row.slack)
    return profiles


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--child-k", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--parent-dim", type=int, required=True)
    parser.add_argument("--parent-zeros", type=int, required=True)
    parser.add_argument("--max-visible-tau", type=int, default=2)
    parser.add_argument(
        "--singleton-charge-mode",
        choices=("endpoint-tau2", "endpoint-tau2-layer", "endpoint-tau2-layer-incidence"),
        default="endpoint-tau2-layer-incidence",
    )
    parser.add_argument("--limit", type=int, default=20)
    parser.add_argument("--output-csv", type=Path)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    profiles = iter_profiles(
        child_k=args.child_k,
        expansion=args.expansion,
        parent_dim=args.parent_dim,
        parent_zeros=args.parent_zeros,
        max_visible_tau=args.max_visible_tau,
        singleton_charge_mode=args.singleton_charge_mode,
    )
    if not profiles:
        raise SystemExit("no feasible profiles")

    fields = list(asdict(profiles[0]).keys())
    if args.output_csv:
        with args.output_csv.open("w", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=fields)
            writer.writeheader()
            for row in profiles:
                writer.writerow(asdict(row))

    print(
        "child_k,parent_dim,parent_zeros,profiles,worst_slack,nonnegative",
        file=sys.stderr,
    )
    print(
        f"{args.child_k},{args.parent_dim},{args.parent_zeros},"
        f"{len(profiles)},{profiles[0].slack},{int(profiles[0].slack >= 0)}",
        file=sys.stderr,
    )
    for row in profiles[: args.limit]:
        print(
            "slack={slack} p={paired} s={singletons} a={visible_support} "
            "tau={visible_tau} rV={outer_dim} rK={inner_dim} "
            "zV={outer_zeros} zK={inner_zeros} charge={local_charge} "
            "parent_def={parent_defect} child_def={child_defect} lift={lift_exponent}".format(
                **asdict(row)
            )
        )


if __name__ == "__main__":
    main()
