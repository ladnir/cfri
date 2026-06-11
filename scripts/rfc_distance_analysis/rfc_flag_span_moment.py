#!/usr/bin/env python3
r"""Two-layer flag checkpoint for original RFC zero-set moments.

This is a small diagnostic, not a certificate.  It tests the proposed
kernel-zero propagation state

    pi(W) zero on P union (S \ A)
    pi(K) zero on P union S

where K is the kernel of the singleton visible quotient of W.  The child flag
count is upper-bounded from existing one-layer child counts by taking the best
of several safe coarse bounds.  This is deliberately more structured than the
scalar span recurrence, but it is still only a checkpoint for small depths.

The endpoint-tau2-layer-incidence mode is an experimental calibration for the quotient-framing
lemma: after a child flag is chosen, tau-two local layers are measured in the quotient `V/L`
rather than only in the full child code after outer zeros.  It also applies the exact-support
Grassmann cap `local_charge >= |A|` from the incidence note.

The cover-lift modes are diagnostics for existence-style container counting. They are not
certificates. They test whether dominant pessimism is coming from counting every parent lift inside
one child container, instead of counting the child container once.
"""

from __future__ import annotations

import argparse
import math


NEG_INF = -1.0e300
INF = 10**9
DEFAULT_MAX_N = 512


def log2_add(left: float, right: float) -> float:
    if left <= NEG_INF / 2:
        return right
    if right <= NEG_INF / 2:
        return left
    if right > left:
        left, right = right, left
    return left + math.log2(1.0 + 2.0 ** (right - left))


def log2_comb_table(n: int) -> list[list[float]]:
    table: list[list[float]] = []
    for row_n in range(n + 1):
        row = [NEG_INF] * (row_n + 1)
        row[0] = 0.0
        row[row_n] = 0.0
        for k in range(1, row_n):
            row[k] = log2_add(table[row_n - 1][k - 1], table[row_n - 1][k])
        table.append(row)
    return table


def uniform_component_count(size: int, rank: int) -> int:
    if size == 0 or rank == 0:
        return 0
    if rank == size:
        return size
    return 1


def endpoint_tau2_charge(size: int, delta: int, components: int) -> int:
    generic_kernel_dim = max(0, 2 * delta - size)
    if delta < 2 and generic_kernel_dim < 2:
        return INF
    component_charge = INF
    if delta >= 2:
        component_charge = size + 2 * delta - components
    generic_charge = INF
    if generic_kernel_dim >= 2:
        generic_charge = 4 * delta - 2 * generic_kernel_dim
    return min(component_charge, generic_charge)


def endpoint_tau2_layer_profile(size: int, delta: int, components: int) -> tuple[int, int, int, int, int]:
    if delta < 2:
        return INF, -INF, -1, -1, max(0, 2 * delta - size)
    generic_kernel_dim = max(0, 2 * delta - size)
    full_component_codim = max(0, size - components)
    theta = -INF
    dominant_h = -1
    dominant_gamma = -1
    for h in range(2, delta + 1):
        if h == delta:
            gamma_h = full_component_codim
        else:
            gamma_h = max(0, h - generic_kernel_dim) ** 2
        layer_theta = 2 * h - 4 - gamma_h
        if layer_theta > theta:
            theta = layer_theta
            dominant_h = h
            dominant_gamma = gamma_h
    return int(4 * delta - 4 - theta), int(theta), dominant_h, dominant_gamma, generic_kernel_dim


def endpoint_tau2_layer_charge(size: int, delta: int, components: int) -> int:
    charge, _theta, _dominant_h, _dominant_gamma, _g = endpoint_tau2_layer_profile(
        size,
        delta,
        components,
    )
    return charge


def local_visible_charge(
    *,
    visible_tau: int,
    child_k: int,
    outer_zero_count: int,
    visible_support_size: int,
    singleton_charge_mode: str,
) -> int:
    if visible_support_size == 0:
        return 0 if visible_tau == 0 else INF
    if visible_tau == 0:
        return INF

    outer_rank = min(outer_zero_count, child_k)
    quotient_rank = max(0, child_k - outer_rank)
    delta = min(visible_support_size, quotient_rank)
    if delta < visible_tau:
        return INF

    components = uniform_component_count(visible_support_size, delta)
    if singleton_charge_mode == "endpoint-tau2" and visible_tau == 2:
        return endpoint_tau2_charge(visible_support_size, delta, components)
    if singleton_charge_mode == "endpoint-tau2-layer" and visible_tau == 2:
        return endpoint_tau2_layer_charge(visible_support_size, delta, components)
    return visible_support_size + visible_tau * delta - components


def local_visible_trace_profile(
    *,
    visible_tau: int,
    child_k: int,
    outer_zero_count: int,
    visible_support_size: int,
    singleton_charge_mode: str,
    quotient_delta_floor: int = 0,
) -> tuple[int, int, int, int, int, int, int]:
    if visible_support_size == 0:
        return (0 if visible_tau == 0 else INF, 0, 0, 0, -1, -1, 0)
    outer_rank = min(outer_zero_count, child_k)
    quotient_rank = max(0, child_k - outer_rank)
    delta = min(visible_support_size, quotient_rank)
    if quotient_delta_floor > 0:
        delta = max(delta, min(visible_support_size, quotient_delta_floor))
    components = uniform_component_count(visible_support_size, delta)
    generic_kernel_dim = max(0, 2 * delta - visible_support_size)
    if delta < visible_tau:
        return (INF, delta, components, generic_kernel_dim, -1, -1, -1)
    if singleton_charge_mode in ("endpoint-tau2-layer", "endpoint-tau2-layer-incidence") and visible_tau == 2:
        charge, theta, h, gamma, generic_kernel_dim = endpoint_tau2_layer_profile(
            visible_support_size,
            delta,
            components,
        )
        return charge, delta, components, generic_kernel_dim, theta, h, gamma
    charge = local_visible_charge(
        visible_tau=visible_tau,
        child_k=child_k,
        outer_zero_count=outer_zero_count,
        visible_support_size=visible_support_size,
        singleton_charge_mode=singleton_charge_mode,
    )
    return charge, delta, components, generic_kernel_dim, -1, -1, -1


def get_child_value(
    child_by_span: dict[int, list[float]],
    child_n: int,
    span: int,
    zero_count: int,
) -> float:
    if zero_count < 0 or zero_count > child_n:
        return NEG_INF
    if span == 0:
        return 0.0
    values = child_by_span.get(span)
    if values is None:
        return NEG_INF
    return values[zero_count]


def flag_child_bound(
    *,
    child_by_span: dict[int, list[float]],
    child_k: int,
    child_n: int,
    outer_span: int,
    inner_span: int,
    outer_zeros: int,
    inner_zeros: int,
    q_log2: float,
    mode: str,
) -> float:
    outer = get_child_value(child_by_span, child_n, outer_span, outer_zeros)
    inner = get_child_value(child_by_span, child_n, inner_span, inner_zeros)
    if outer <= NEG_INF / 2 or inner <= NEG_INF / 2:
        return NEG_INF
    if inner_span > outer_span:
        return NEG_INF
    if mode == "product":
        return outer + inner
    if mode == "outer-only":
        return outer + inner_span * (outer_span - inner_span) * q_log2
    if mode != "best":
        raise ValueError(f"unknown flag bound mode {mode!r}")

    outer_first = outer + inner_span * (outer_span - inner_span) * q_log2
    inner_first = inner + (outer_span - inner_span) * (child_k - outer_span) * q_log2
    return min(outer_first, inner_first)


def marked_line_child_bound(
    *,
    child_by_span: dict[int, list[float]],
    child_k: int,
    child_n: int,
    outer_span: int,
    outer_zeros: int,
    q_log2: float,
) -> float:
    """Safe bound for one marked line inside a child span.

    The state is a joint flag L <= V with dim(V)=outer_span and dim(L)=1,
    where V has outer_zeros common zeros and L has one additional marked zero.
    """

    if outer_span < 1:
        return NEG_INF
    inner_zeros = outer_zeros + 1
    if inner_zeros > child_n:
        return NEG_INF
    return flag_child_bound(
        child_by_span=child_by_span,
        child_k=child_k,
        child_n=child_n,
        outer_span=outer_span,
        inner_span=1,
        outer_zeros=outer_zeros,
        inner_zeros=inner_zeros,
        q_log2=q_log2,
        mode="best",
    )


def lift_flag_span_moment(
    child_by_span: dict[int, list[float]],
    comb: list[list[float]],
    q_log2: float,
    child_k: int,
    singleton_charge_mode: str,
    flag_bound_mode: str,
    max_visible_tau: int,
    cover_lift_mode: str,
    max_parent_span: int | None = None,
) -> tuple[dict[int, list[float]], dict[tuple[int, int], tuple[int, ...] | None]]:
    child_n = len(next(iter(child_by_span.values()))) - 1
    parent_n = 2 * child_n
    parent_k = 2 * child_k
    if max_parent_span is None:
        max_parent_span = parent_k
    max_parent_span = max(0, min(max_parent_span, parent_k))
    parent_by_span: dict[int, list[float]] = {
        span: [NEG_INF] * (parent_n + 1) for span in range(1, max_parent_span + 1)
    }
    choices: dict[tuple[int, int], tuple[int, ...] | None] = {}
    local_profile_cache: dict[tuple[int, int, int, str, int], tuple[int, int, int, int, int, int, int]] = {}

    def cached_local_profile(
        visible_tau: int,
        outer_zero_count: int,
        visible_support_size: int,
        quotient_delta_floor: int = 0,
    ) -> tuple[int, int, int, int, int, int, int]:
        key = (
            visible_tau,
            outer_zero_count,
            visible_support_size,
            singleton_charge_mode,
            quotient_delta_floor,
        )
        if key not in local_profile_cache:
            local_profile_cache[key] = local_visible_trace_profile(
                visible_tau=visible_tau,
                child_k=child_k,
                outer_zero_count=outer_zero_count,
                visible_support_size=visible_support_size,
                singleton_charge_mode=singleton_charge_mode,
                quotient_delta_floor=quotient_delta_floor,
            )
        return local_profile_cache[key]

    child_spans = sorted(child_by_span)
    for parent_span in range(1, max_parent_span + 1):
        for z in range(parent_n + 1):
            total = NEG_INF
            best = NEG_INF
            best_choice: tuple[int, ...] | None = None

            for p in range(z // 2 + 1):
                singleton_count = z - 2 * p
                if p + singleton_count > child_n:
                    continue
                orientation_log = float(singleton_count)

                max_visible_support = min(singleton_count, child_n - p)
                for visible_support_size in range(max_visible_support + 1):
                    outer_zeros = p + singleton_count - visible_support_size
                    inner_zeros = p + singleton_count
                    if outer_zeros > child_n:
                        continue
                    if visible_support_size > child_n - outer_zeros:
                        continue

                    shape_log = (
                        orientation_log
                        + comb[outer_zeros][p]
                        + comb[child_n - outer_zeros][visible_support_size]
                    )

                    max_tau = min(parent_span, max_visible_tau)
                    for visible_tau in range(max_tau + 1):
                        kernel_dim = parent_span - visible_tau
                        if visible_tau == 0 and visible_support_size != 0:
                            continue
                        if visible_tau > 0 and visible_support_size == 0:
                            continue

                        (
                            charge,
                            local_delta,
                            local_components,
                            local_g,
                            local_theta,
                            local_h,
                            local_gamma,
                        ) = cached_local_profile(
                            visible_tau,
                            outer_zeros,
                            visible_support_size,
                        )
                        incidence_mode = singleton_charge_mode == "endpoint-tau2-layer-incidence"
                        if charge >= INF and not incidence_mode:
                            continue

                        if kernel_dim == 0:
                            inner_span_candidates = [0]
                        else:
                            inner_span_candidates = [
                                r0
                                for r0 in child_spans
                                if kernel_dim <= 2 * r0 and r0 <= 2 * kernel_dim
                            ]

                        for inner_span in inner_span_candidates:
                            if visible_tau == 0:
                                outer_span_candidates = [inner_span]
                            else:
                                outer_span_candidates = [
                                    r1
                                    for r1 in child_spans
                                    if inner_span <= r1
                                    and parent_span <= 2 * r1
                                    and r1 <= 2 * parent_span
                                ]

                            for outer_span in outer_span_candidates:
                                if outer_span == 0:
                                    continue
                                if incidence_mode:
                                    (
                                        charge,
                                        local_delta,
                                        local_components,
                                        local_g,
                                        local_theta,
                                        local_h,
                                        local_gamma,
                                    ) = cached_local_profile(
                                        visible_tau,
                                        outer_zeros,
                                        visible_support_size,
                                        max(0, outer_span - inner_span),
                                    )
                                    if charge >= INF:
                                        continue
                                exact_support_grassmann_cap = (
                                    incidence_mode and visible_tau == 2
                                )
                                if exact_support_grassmann_cap:
                                    grassmann_charge = visible_support_size
                                    if grassmann_charge > charge:
                                        charge = grassmann_charge
                                        local_theta = 4 * local_delta - 4 - charge
                                        local_h = -2
                                        local_gamma = -2
                                charge_log = -charge * q_log2
                                if visible_tau == 0:
                                    lift_log = parent_span * (2 * outer_span - parent_span) * q_log2
                                    if cover_lift_mode in ("tau0", "all"):
                                        lift_log = 0.0
                                    child_log = get_child_value(
                                        child_by_span,
                                        child_n,
                                        outer_span,
                                        outer_zeros,
                                    )
                                else:
                                    kernel_lift_log = (
                                        kernel_dim * (2 * inner_span - kernel_dim) * q_log2
                                    )
                                    quotient_lift_log = (
                                        visible_tau * (2 * outer_span - parent_span) * q_log2
                                    )
                                    lift_log = kernel_lift_log + quotient_lift_log
                                    if cover_lift_mode == "all":
                                        lift_log = 0.0
                                    child_log = flag_child_bound(
                                        child_by_span=child_by_span,
                                        child_k=child_k,
                                        child_n=child_n,
                                        outer_span=outer_span,
                                        inner_span=inner_span,
                                        outer_zeros=outer_zeros,
                                        inner_zeros=inner_zeros,
                                        q_log2=q_log2,
                                        mode=flag_bound_mode,
                                    )

                                if child_log <= NEG_INF / 2:
                                    continue
                                marked_line_used = False
                                if (
                                    incidence_mode
                                    and parent_span == 2
                                    and visible_tau == 2
                                    and kernel_dim == 0
                                    and outer_span == 2
                                    and inner_span == 0
                                    and visible_support_size == 2
                                    and local_delta == 2
                                    and local_components == 2
                                ):
                                    marked_child_log = marked_line_child_bound(
                                        child_by_span=child_by_span,
                                        child_k=child_k,
                                        child_n=child_n,
                                        outer_span=outer_span,
                                        outer_zeros=outer_zeros,
                                        q_log2=q_log2,
                                    )
                                    if marked_child_log > NEG_INF / 2:
                                        marked_child_log += q_log2
                                        if marked_child_log < child_log:
                                            child_log = marked_child_log
                                            marked_line_used = True
                                term = shape_log + charge_log + lift_log + child_log
                                total = log2_add(total, term)
                                if term > best:
                                    best = term
                                    best_choice = (
                                        p,
                                        singleton_count,
                                        visible_support_size,
                                        visible_tau,
                                        outer_span,
                                        inner_span,
                                        outer_zeros,
                                        charge,
                                        local_delta,
                                        local_components,
                                        local_g,
                                        local_theta,
                                        -4 if marked_line_used else local_h,
                                        -4 if marked_line_used else local_gamma,
                                        int(round(lift_log / q_log2)),
                                    )

            parent_by_span[parent_span][z] = total
            choices[(parent_span, z)] = best_choice
    return parent_by_span, choices


def first_crossing(values: list[float], security_bits: float) -> int | None:
    target = -security_bits
    for z, value in enumerate(values):
        if value <= target:
            return z
    return None


def is_tau2_theta(choice: tuple[int, ...] | None, theta: int) -> bool:
    return choice is not None and choice[3] == 2 and choice[11] == theta


def choice_children(choice: tuple[int, ...]) -> list[tuple[str, int, int]]:
    p = choice[0]
    singleton_count = choice[1]
    outer_span = choice[4]
    inner_span = choice[5]
    outer_zeros = choice[6]
    inner_zeros = p + singleton_count
    children = [("outer", outer_span, outer_zeros)]
    if inner_span > 0:
        children.append(("inner", inner_span, inner_zeros))
    return children


def theta_chain_reports(
    trace: list[dict[tuple[int, int], tuple[int, ...] | None]],
    values_by_level: list[dict[int, list[float]]],
    theta: int,
    limit: int,
) -> list[tuple[int, int, int, int, float, str]]:
    memo: dict[tuple[int, int, int], tuple[int, str]] = {}

    def best_chain(level: int, span: int, z: int) -> tuple[int, str]:
        key = (level, span, z)
        if key in memo:
            return memo[key]
        if level <= 0:
            memo[key] = (0, "")
            return memo[key]
        choice = trace[level - 1].get((span, z))
        if not is_tau2_theta(choice, theta):
            memo[key] = (0, "")
            return memo[key]
        best_child_len = 0
        best_child_path = ""
        for edge_name, child_span, child_z in choice_children(choice):
            child_len, child_path = best_chain(level - 1, child_span, child_z)
            if child_len > best_child_len:
                best_child_len = child_len
                best_child_path = f"{edge_name}->{child_path}" if child_path else edge_name
        result = (1 + best_child_len, best_child_path)
        memo[key] = result
        return result

    reports: list[tuple[int, int, int, int, float, str]] = []
    for level in range(1, len(trace) + 1):
        for (span, z), choice in trace[level - 1].items():
            if not is_tau2_theta(choice, theta):
                continue
            chain_len, path = best_chain(level, span, z)
            values = values_by_level[level].get(span)
            if values is None or z < 0 or z >= len(values):
                continue
            value = values[z]
            if value <= NEG_INF / 2:
                continue
            reports.append((chain_len, level, span, z, value, path))
    reports.sort(key=lambda row: (row[0], row[4]), reverse=True)
    return reports[:limit]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--print-window", type=int, default=5)
    parser.add_argument("--trace-z", type=int, default=-1)
    parser.add_argument("--trace-span", type=int, default=1)
    parser.add_argument("--max-n", type=int, default=DEFAULT_MAX_N)
    parser.add_argument("--allow-large", action="store_true")
    parser.add_argument(
        "--singleton-charge",
        choices=[
            "component-uniform",
            "endpoint-tau2",
            "endpoint-tau2-layer",
            "endpoint-tau2-layer-incidence",
        ],
        default="endpoint-tau2",
    )
    parser.add_argument(
        "--flag-bound",
        choices=["best", "product", "outer-only"],
        default="best",
    )
    parser.add_argument("--max-visible-tau", type=int, default=2)
    parser.add_argument(
        "--cover-lift-mode",
        choices=["none", "tau0", "all"],
        default="none",
        help=(
            "Diagnostic only: remove selected parent-lift multiplicities to test container-style "
            "existence counting. Not a certificate mode."
        ),
    )
    parser.add_argument(
        "--prune-to-final-span",
        type=int,
        default=0,
        help=(
            "If positive, only compute spans that can feed this final span by doubling backward "
            "each remaining level. Use 1 for the distance first moment."
        ),
    )
    parser.add_argument(
        "--report-local-theta",
        type=int,
        default=None,
        help="Report best-transition states whose local tau-two theta equals this value.",
    )
    parser.add_argument(
        "--report-theta-chains",
        type=int,
        default=None,
        help="Report longest best-transition chains containing this tau-two theta value.",
    )
    parser.add_argument("--report-limit", type=int, default=20)
    args = parser.parse_args()

    if args.depth < 0:
        raise SystemExit("--depth must be nonnegative")
    if args.expansion < 1:
        raise SystemExit("--expansion must be positive")
    if args.trace_span < 1:
        raise SystemExit("--trace-span must be positive")
    if args.max_visible_tau < 0:
        raise SystemExit("--max-visible-tau must be nonnegative")
    if args.prune_to_final_span < 0:
        raise SystemExit("--prune-to-final-span must be nonnegative")
    if args.report_limit < 0:
        raise SystemExit("--report-limit must be nonnegative")

    total_n = args.expansion * (1 << args.depth)
    if total_n > args.max_n and not args.allow_large:
        raise SystemExit(
            f"refusing total length {total_n}; pass --allow-large or raise --max-n"
        )

    comb = log2_comb_table(total_n)
    values_by_span: dict[int, list[float]] = {1: [NEG_INF] * (args.expansion + 1)}
    values_by_span[1][0] = 0.0
    trace: list[dict[tuple[int, int], tuple[int, ...] | None]] = []
    values_by_level: list[dict[int, list[float]]] = [{span: values[:] for span, values in values_by_span.items()}]
    theta_reports: list[tuple[float, int, int, int, tuple[int, ...]]] = []

    print("level,k,n,span_count,min_log2,max_log2", flush=True)
    print(f"0,1,{args.expansion},1,0.00000000,0.00000000", flush=True)
    for level in range(1, args.depth + 1):
        max_parent_span = None
        if args.prune_to_final_span > 0:
            max_parent_span = args.prune_to_final_span * (1 << (args.depth - level))
        values_by_span, choices = lift_flag_span_moment(
            values_by_span,
            comb,
            args.q_log2,
            1 << (level - 1),
            args.singleton_charge,
            args.flag_bound,
            args.max_visible_tau,
            args.cover_lift_mode,
            max_parent_span,
        )
        trace.append(choices)
        values_by_level.append({span: values[:] for span, values in values_by_span.items()})
        if args.report_local_theta is not None:
            for (span, z), choice in choices.items():
                if choice is None:
                    continue
                if choice[3] != 2:
                    continue
                if choice[11] != args.report_local_theta:
                    continue
                value = values_by_span[span][z]
                if value <= NEG_INF / 2:
                    continue
                theta_reports.append((value, level, span, z, choice))
        finite = [
            value
            for values in values_by_span.values()
            for value in values
            if value > NEG_INF / 2
        ]
        print(
            f"{level},{1 << level},{args.expansion * (1 << level)},"
            f"{len(values_by_span)},{min(finite):.8f},{max(finite):.8f}",
            flush=True,
        )

    final_lines = values_by_span[1]
    final_vectors = [
        value + args.q_log2 if value > NEG_INF / 2 else NEG_INF
        for value in final_lines
    ]
    crossing = first_crossing(final_vectors, args.security_bits)
    k = 1 << args.depth
    center = k if crossing is None else crossing
    start = max(0, center - args.print_window)
    stop = min(total_n, center + args.print_window)
    print("z,excess,log2_vector_moment")
    for z in range(start, stop + 1):
        label = "-inf" if final_vectors[z] <= NEG_INF / 2 else f"{final_vectors[z]:.8f}"
        print(f"{z},{z-k},{label}")
    print(
        f"crossing_z={crossing} crossing_excess="
        f"{'' if crossing is None else crossing-k}"
    )

    if args.report_local_theta is not None:
        theta_reports.sort(reverse=True, key=lambda row: row[0])
        print(
            "theta_report_level,span,z,log2_state,p,s,a,tau,outer_span,inner_span,"
            "outer_zeros,inner_zeros,local_charge,delta,comp,g,theta,dominant_h,gamma,lift_qdim"
            ",outer_next_tau,outer_next_theta,inner_next_tau,inner_next_theta"
        )
        for value, level, span, z, choice in theta_reports[: args.report_limit]:
            (
                p,
                singleton_count,
                visible_support_size,
                visible_tau,
                outer_span,
                inner_span,
                outer_zeros,
                local_charge,
                local_delta,
                local_components,
                local_g,
                local_theta,
                local_h,
                local_gamma,
                lift_qdim,
            ) = choice
            inner_zeros = p + singleton_count
            outer_next_tau = ""
            outer_next_theta = ""
            inner_next_tau = ""
            inner_next_theta = ""
            if level > 1:
                child_choices = trace[level - 2]
                outer_next = child_choices.get((outer_span, outer_zeros))
                if outer_next is not None:
                    outer_next_tau = str(outer_next[3])
                    if outer_next[3] == 2:
                        outer_next_theta = str(outer_next[11])
                if inner_span > 0:
                    inner_next = child_choices.get((inner_span, inner_zeros))
                    if inner_next is not None:
                        inner_next_tau = str(inner_next[3])
                        if inner_next[3] == 2:
                            inner_next_theta = str(inner_next[11])
            print(
                f"{level},{span},{z},{value:.8f},{p},{singleton_count},"
                f"{visible_support_size},{visible_tau},{outer_span},{inner_span},"
                f"{outer_zeros},{inner_zeros},{local_charge},{local_delta},"
                f"{local_components},{local_g},{local_theta},{local_h},"
                f"{local_gamma},{lift_qdim},{outer_next_tau},{outer_next_theta},"
                f"{inner_next_tau},{inner_next_theta}"
            )

    if args.report_theta_chains is not None:
        print("theta_chain_len,start_level,span,z,log2_state,path")
        for chain_len, level, span, z, value, path in theta_chain_reports(
            trace,
            values_by_level,
            args.report_theta_chains,
            args.report_limit,
        ):
            print(f"{chain_len},{level},{span},{z},{value:.8f},{path}")

    if args.trace_z >= 0:
        print(
            "trace_level,span,z,p,s,a,tau,outer_span,inner_span,outer_zeros,inner_zeros,"
            "local_charge,delta,comp,g,theta,dominant_h,gamma,lift_qdim"
        )
        span = args.trace_span
        z = args.trace_z
        for level in range(args.depth, 0, -1):
            choice = trace[level - 1].get((span, z))
            if choice is None:
                print(f"{level},{span},{z},,,,,,,,")
                break
            (
                p,
                singleton_count,
                visible_support_size,
                visible_tau,
                outer_span,
                inner_span,
                outer_zeros,
                local_charge,
                local_delta,
                local_components,
                local_g,
                local_theta,
                local_h,
                local_gamma,
                lift_qdim,
            ) = choice
            inner_zeros = p + singleton_count
            print(
                f"{level},{span},{z},{p},{singleton_count},{visible_support_size},"
                f"{visible_tau},{outer_span},{inner_span},{outer_zeros},{inner_zeros},"
                f"{local_charge},{local_delta},{local_components},{local_g},{local_theta},"
                f"{local_h},{local_gamma},{lift_qdim}"
            )
            span = outer_span
            z = outer_zeros


if __name__ == "__main__":
    main()
