#!/usr/bin/env python3
"""Toy support-stratified distance calculator for systematic RFCs.

This implements the first recurrence in docs/systematic_rfc_distance_analysis.md.
It is intended for small research sanity checks, not large production parameters.
All probabilities are accumulated in log2 space.
"""

from __future__ import annotations

import argparse
import csv
import math
import sys


NEG_INF = float("-inf")


def log2_add(lhs: float, rhs: float) -> float:
    if lhs == NEG_INF:
        return rhs
    if rhs == NEG_INF:
        return lhs
    if rhs > lhs:
        lhs, rhs = rhs, lhs
    return lhs + math.log2(1.0 + 2.0 ** (rhs - lhs))


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (
        math.lgamma(n + 1)
        - math.lgamma(k + 1)
        - math.lgamma(n - k + 1)
    ) / math.log(2.0)


def log2_combinations(n: int) -> list[float]:
    return [log2_comb(n, k) for k in range(n + 1)]


def log2_factorials(n: int) -> list[float]:
    out = [0.0] * (n + 1)
    for i in range(2, n + 1):
        out[i] = out[i - 1] + math.log2(i)
    return out


def log2_comb_from_factorials(log2_fact: list[float], n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return log2_fact[n] - log2_fact[k] - log2_fact[n - k]


def log2_bad_root_prob(field_bits: float) -> float:
    return math.log2(2.002) - field_bits


def log2_outside_tail(
    n: int,
    common_zeros: int,
    needed_zeros: int,
    log_bad_prob: float,
    log2_fact_n: list[float],
) -> float:
    available = n - common_zeros
    if needed_zeros <= 0:
        return 0.0
    if needed_zeros > available:
        return NEG_INF
    return (
        log2_comb_from_factorials(log2_fact_n, available, needed_zeros)
        + needed_zeros * log_bad_prob
    )


def log2_one_minus_q_inv(field_bits: float) -> float:
    if field_bits > 64.0:
        return 0.0
    return math.log2(1.0 - 2.0 ** (-field_bits))


def log2_k_bound(k: int, b_prev: list[int], field_bits: float, u: int, a: int) -> float:
    if u == 0:
        return 0.0
    if a >= b_prev[u]:
        return NEG_INF
    support_choices = log2_comb(k, u)
    kernel_bound = support_choices + (b_prev[u] - a) * field_bits
    exact_support_universe = support_choices + u * (
        field_bits + log2_one_minus_q_inv(field_bits)
    )
    return min(kernel_bound, exact_support_universe)


def log2_k_bound_with_combs(
    log2_comb_k: list[float],
    b_prev: list[int],
    field_bits: float,
    u: int,
    a: int,
) -> float:
    if u == 0:
        return 0.0
    if a >= b_prev[u]:
        return NEG_INF
    support_choices = log2_comb_k[u]
    kernel_bound = support_choices + (b_prev[u] - a) * field_bits
    exact_support_universe = support_choices + u * (
        field_bits + log2_one_minus_q_inv(field_bits)
    )
    return min(kernel_bound, exact_support_universe)


def log2_failure(
    k: int,
    n: int,
    b_prev: list[int],
    field_bits: float,
    s: int,
    b: int,
) -> float:
    log_total = NEG_INF
    log2_comb_k = log2_combinations(k)
    log2_comb_n = log2_combinations(n)
    log2_fact_n = log2_factorials(n)
    log_bad_prob = log2_bad_root_prob(field_bits)
    for u in range(max(0, s - k), min(k, s) + 1):
        v = s - u
        for a in range(n + 1):
            log_k_l = log2_k_bound_with_combs(log2_comb_k, b_prev, field_bits, u, a)
            if log_k_l == NEG_INF:
                continue
            log_k_r = log2_k_bound_with_combs(log2_comb_k, b_prev, field_bits, v, a)
            if log_k_r == NEG_INF:
                continue
            log_term = log2_comb_n[a] + log_k_l + log_k_r
            if b > 2 * a:
                log_tail = log2_outside_tail(n, a, b - 2 * a, log_bad_prob, log2_fact_n)
                if log_tail == NEG_INF:
                    continue
                log_term += log_tail
            log_total = log2_add(log_total, log_term)
    return log_total


def log2_failure_precomputed(
    k: int,
    n: int,
    b_prev: list[int],
    field_bits: float,
    s: int,
    b: int,
    log2_comb_k: list[float],
    log2_comb_n: list[float],
) -> float:
    log_total = NEG_INF
    log2_fact_n = log2_factorials(n)
    log_bad_prob = log2_bad_root_prob(field_bits)
    for u in range(max(0, s - k), min(k, s) + 1):
        v = s - u
        for a in range(n + 1):
            log_k_l = log2_k_bound_with_combs(log2_comb_k, b_prev, field_bits, u, a)
            if log_k_l == NEG_INF:
                continue
            log_k_r = log2_k_bound_with_combs(log2_comb_k, b_prev, field_bits, v, a)
            if log_k_r == NEG_INF:
                continue
            log_term = log2_comb_n[a] + log_k_l + log_k_r
            if b > 2 * a:
                log_tail = log2_outside_tail(n, a, b - 2 * a, log_bad_prob, log2_fact_n)
                if log_tail == NEG_INF:
                    continue
                log_term += log_tail
            log_total = log2_add(log_total, log_term)
    return log_total


def build_split_terms(
    k: int,
    n: int,
    b_prev: list[int],
    field_bits: float,
    log2_comb_k: list[float],
    log2_comb_n: list[float],
) -> list[list[float]]:
    parent_k = 2 * k
    split_terms = [[NEG_INF] * (n + 1) for _ in range(parent_k + 1)]
    for a in range(n + 1):
        active_bounds: list[tuple[int, float]] = []
        for u in range(k + 1):
            log_bound = log2_k_bound_with_combs(
                log2_comb_k,
                b_prev,
                field_bits,
                u,
                a,
            )
            if log_bound != NEG_INF:
                active_bounds.append((u, log_bound))
        for u, log_u in active_bounds:
            for v, log_v in active_bounds:
                support = u + v
                split_terms[support][a] = log2_add(
                    split_terms[support][a],
                    log2_comb_n[a] + log_u + log_v,
                )
    return split_terms


def log2_failure_from_split_terms(
    n: int,
    field_bits: float,
    split_terms: list[list[float]],
    s: int,
    b: int,
    log2_fact_n: list[float],
) -> float:
    log_total = NEG_INF
    log_bad_prob = log2_bad_root_prob(field_bits)
    for a, log_term in enumerate(split_terms[s]):
        if log_term == NEG_INF:
            continue
        if b > 2 * a:
            log_tail = log2_outside_tail(n, a, b - 2 * a, log_bad_prob, log2_fact_n)
            if log_tail == NEG_INF:
                continue
            log_term += log_tail
        log_total = log2_add(log_total, log_term)
    return log_total


def failure_terms(
    k: int,
    n: int,
    b_prev: list[int],
    field_bits: float,
    s: int,
    b: int,
) -> list[tuple[float, int, int, int]]:
    terms: list[tuple[float, int, int, int]] = []
    log2_comb_k = log2_combinations(k)
    log2_comb_n = log2_combinations(n)
    log2_fact_n = log2_factorials(n)
    log_bad_prob = log2_bad_root_prob(field_bits)
    for u in range(max(0, s - k), min(k, s) + 1):
        v = s - u
        for a in range(n + 1):
            log_k_l = log2_k_bound_with_combs(log2_comb_k, b_prev, field_bits, u, a)
            if log_k_l == NEG_INF:
                continue
            log_k_r = log2_k_bound_with_combs(log2_comb_k, b_prev, field_bits, v, a)
            if log_k_r == NEG_INF:
                continue
            log_term = log2_comb_n[a] + log_k_l + log_k_r
            if b > 2 * a:
                log_tail = log2_outside_tail(n, a, b - 2 * a, log_bad_prob, log2_fact_n)
                if log_tail == NEG_INF:
                    continue
                log_term += log_tail
            terms.append((log_term, u, v, a))
    terms.sort(reverse=True)
    return terms


def print_failure_explanation(
    k: int,
    n: int,
    b_prev: list[int],
    field_bits: float,
    s: int,
    b: int,
    top_terms: int,
) -> None:
    terms = failure_terms(k, n, b_prev, field_bits, s, b)
    aggregate: dict[tuple[int, int], float] = {}
    for log_term, u, v, _ in terms:
        key = (u, v)
        aggregate[key] = log2_add(aggregate.get(key, NEG_INF), log_term)
    print("dominant_terms_log2_probability,u,v,common_zero_size")
    for log_term, u, v, a in terms[:top_terms]:
        print(f"{log_term:.3f},{u},{v},{a}")
    print("dominant_splits_log2_probability,u,v")
    for (u, v), log_value in sorted(aggregate.items(), key=lambda item: item[1], reverse=True)[
        :top_terms
    ]:
        print(f"{log_value:.3f},{u},{v}")


def next_thresholds(
    k: int,
    parity_expansion: int,
    b_prev: list[int],
    field_bits: float,
    security_bits: float,
    monotone_thresholds: bool,
) -> list[int]:
    if monotone_thresholds:
        b_prev = prefix_max_thresholds(b_prev)
    n = parity_expansion * k
    parent_n = 2 * n
    parent_k = 2 * k
    per_support_budget = -security_bits - math.log2(parent_k)
    log2_comb_k = log2_combinations(k)
    log2_comb_n = log2_combinations(n)
    log2_fact_n = log2_factorials(n)
    split_terms = build_split_terms(
        k,
        n,
        b_prev,
        field_bits,
        log2_comb_k,
        log2_comb_n,
    )
    b_next = [parent_n + 1] * (parent_k + 1)
    b_next[0] = parent_n + 1

    for s in range(1, parent_k + 1):
        lower = 1
        upper = parent_n
        if lower > upper:
            raise ValueError(
                f"threshold lower bound {lower} exceeds admissible range "
                f"1..{parent_n} for parent support {s}"
            )
        while lower < upper:
            mid = (lower + upper) >> 1
            if (
                log2_failure_from_split_terms(
                    n,
                    field_bits,
                    split_terms,
                    s,
                    mid,
                    log2_fact_n,
                )
                <= per_support_budget
            ):
                upper = mid
            else:
                lower = mid + 1
        if (
            log2_failure_from_split_terms(
                n,
                field_bits,
                split_terms,
                s,
                lower,
                log2_fact_n,
            )
            > per_support_budget
        ):
            raise ValueError(
                f"no admissible threshold for parent support {s}; "
                f"best candidate {lower} exceeds the failure budget"
            )
        b_next[s] = lower
    return b_next


def prefix_max_thresholds(thresholds: list[int]) -> list[int]:
    out = thresholds[:]
    if len(out) <= 2:
        return out
    current = out[1]
    for i in range(2, len(out)):
        current = max(current, out[i])
        out[i] = current
    return out


def basefold_global_next_threshold(
    n: int,
    t_prev: int,
    field_bits: float,
    security_bits: float,
) -> int:
    denominator = field_bits - 1.001
    if denominator <= 0.0:
        raise ValueError("field_bits must be greater than 1.001")
    ell = (
        2.0 * math.log2(n)
        + security_bits
        + 2.002 * t_prev
        + 0.6 * n
    ) / denominator
    return 2 * t_prev + math.ceil(ell)


def basefold_global_distance(
    depth: int,
    expansion: int,
    field_bits: float,
    security_bits: float,
    k0: int,
) -> tuple[int, int, float]:
    k = k0
    n = expansion * k
    t = k0
    for _ in range(1, depth + 1):
        t = basefold_global_next_threshold(n, t, field_bits, security_bits)
        k *= 2
        n *= 2
    distance = n - (t - 1)
    return distance, n, distance / n


def systematic_distance(k: int, n: int, b: list[int]) -> tuple[int, int]:
    best_distance = k + n + 1
    best_support = 0
    for s in range(1, k + 1):
        parity_nonzeros = n - (b[s] - 1)
        distance = s + parity_nonzeros
        if distance < best_distance:
            best_distance = distance
            best_support = s
    return best_distance, best_support


def theorem_condition_report(
    k: int,
    n: int,
    b_prev: list[int],
    b_next: list[int],
    field_bits: float,
    security_bits: float,
) -> tuple[float, int]:
    parent_k = 2 * k
    per_support_budget = -security_bits - math.log2(parent_k)
    log2_comb_k = log2_combinations(k)
    log2_comb_n = log2_combinations(n)
    log2_fact_n = log2_factorials(n)
    split_terms = build_split_terms(
        k,
        n,
        b_prev,
        field_bits,
        log2_comb_k,
        log2_comb_n,
    )
    worst_log_failure = NEG_INF
    worst_support = 0
    for support in range(1, parent_k + 1):
        log_failure = log2_failure_from_split_terms(
            n,
            field_bits,
            split_terms,
            support,
            b_next[support],
            log2_fact_n,
        )
        if log_failure > worst_log_failure:
            worst_log_failure = log_failure
            worst_support = support
    return worst_log_failure - per_support_budget, worst_support


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--k0", type=int, default=1)
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--parity-expansion", type=int, required=True)
    parser.add_argument("--field-bits", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument(
        "--compare",
        action="store_true",
        help="also print non-systematic RFC global recurrence and dense-message heuristic",
    )
    parser.add_argument(
        "--dump-final-profile",
        action="store_true",
        help="print support, zero threshold, and distance profile for the final round",
    )
    parser.add_argument(
        "--monotone-thresholds",
        action="store_true",
        help="enforce prefix-max B_i(s) thresholds required by the inside-support proof",
    )
    parser.add_argument(
        "--explain-support",
        type=int,
        default=None,
        help="explain dominant final-round failure terms for this support size",
    )
    parser.add_argument(
        "--emit-certificate",
        action="store_true",
        help="emit CSV threshold rows",
    )
    parser.add_argument(
        "--certificate-path",
        default=None,
        help="write emitted certificate CSV to this path instead of stdout",
    )
    parser.add_argument(
        "--full-thresholds-path",
        default=None,
        help="write all per-round per-support thresholds to this CSV path",
    )
    parser.add_argument("--top-terms", type=int, default=12)
    args = parser.parse_args()

    if args.k0 != 1:
        raise SystemExit("this toy calculator currently assumes the k0=1 repetition base code")
    if args.depth < 0:
        raise SystemExit("depth must be non-negative")
    if args.parity_expansion <= 0:
        raise SystemExit("parity expansion must be positive")

    k = args.k0
    n = args.parity_expansion * k
    b = [n + 1] * (k + 1)
    b[0] = n + 1
    b[1] = 1
    transition: tuple[int, int, list[int]] | None = None
    certificate_rows: list[dict[str, int | float]] = []
    threshold_rows: list[dict[str, int | float]] = []

    if args.compare:
        print(
            "round,k,parity_n,total_n,systematic_distance,best_support,"
            "systematic_relative,non_systematic_relative,dense_heuristic_relative"
        )
    else:
        print("round,k,n,min_distance,best_support,relative_distance")
    distance, support = systematic_distance(k, n, b)
    certificate_rows.append(
        {
            "round": 0,
            "k": k,
            "parity_n": n,
            "support": support,
            "threshold": b[support],
            "distance": distance,
            "relative_distance": distance / (k + n),
            "worst_log2_slack": 0.0,
            "worst_support": support,
        }
    )
    for support_index in range(1, k + 1):
        distance_at_support = support_index + n - (b[support_index] - 1)
        threshold_rows.append(
            {
                "round": 0,
                "k": k,
                "parity_n": n,
                "support": support_index,
                "threshold": b[support_index],
                "distance_at_support": distance_at_support,
                "relative_distance_at_support": distance_at_support / (k + n),
            }
        )
    if args.compare:
        total_expansion = args.parity_expansion + 1
        _, _, old_delta = basefold_global_distance(
            0, total_expansion, args.field_bits, args.security_bits, args.k0
        )
        _, _, parity_delta = basefold_global_distance(
            0, args.parity_expansion, args.field_bits, args.security_bits, args.k0
        )
        dense_delta = (1.0 + args.parity_expansion * parity_delta) / total_expansion
        print(
            f"0,{k},{n},{k+n},{distance},{support},{distance / (k + n):.8f},"
            f"{old_delta:.8f},{dense_delta:.8f}"
        )
    else:
        print(f"0,{k},{n},{distance},{support},{distance / (k + n):.8f}")

    for round_index in range(1, args.depth + 1):
        transition = (k, n, b)
        previous_b = prefix_max_thresholds(b) if args.monotone_thresholds else b
        b = next_thresholds(
            k,
            args.parity_expansion,
            b,
            args.field_bits,
            args.security_bits,
            args.monotone_thresholds,
        )
        if args.monotone_thresholds:
            b = prefix_max_thresholds(b)
        slack, worst_failure_support = theorem_condition_report(
            k,
            n,
            previous_b,
            b,
            args.field_bits,
            args.security_bits,
        )
        k *= 2
        n *= 2
        distance, support = systematic_distance(k, n, b)
        certificate_rows.append(
            {
                "round": round_index,
                "k": k,
                "parity_n": n,
                "support": support,
                "threshold": b[support],
                "distance": distance,
                "relative_distance": distance / (k + n),
                "worst_log2_slack": slack,
                "worst_support": worst_failure_support,
            }
        )
        for support_index in range(1, k + 1):
            distance_at_support = support_index + n - (b[support_index] - 1)
            threshold_rows.append(
                {
                    "round": round_index,
                    "k": k,
                    "parity_n": n,
                    "support": support_index,
                    "threshold": b[support_index],
                    "distance_at_support": distance_at_support,
                    "relative_distance_at_support": distance_at_support / (k + n),
                }
            )
        if args.compare:
            total_expansion = args.parity_expansion + 1
            _, _, old_delta = basefold_global_distance(
                round_index,
                total_expansion,
                args.field_bits,
                args.security_bits,
                args.k0,
            )
            _, _, parity_delta = basefold_global_distance(
                round_index,
                args.parity_expansion,
                args.field_bits,
                args.security_bits,
                args.k0,
            )
            dense_delta = (1.0 + args.parity_expansion * parity_delta) / total_expansion
            print(
                f"{round_index},{k},{n},{k+n},{distance},{support},"
                f"{distance / (k + n):.8f},{old_delta:.8f},{dense_delta:.8f}"
            )
        else:
            print(f"{round_index},{k},{n},{distance},{support},{distance / (k + n):.8f}")

    if args.dump_final_profile:
        print("support,zero_threshold,systematic_distance,relative_distance")
        for support in range(1, k + 1):
            distance_at_support = support + n - (b[support] - 1)
            print(
                f"{support},{b[support]},{distance_at_support},"
                f"{distance_at_support / (k + n):.8f}"
            )

    if args.explain_support is not None:
        if transition is None:
            raise SystemExit("--explain-support requires depth >= 1")
        prev_k, prev_n, prev_b = transition
        support = args.explain_support
        if support <= 0 or support >= len(b):
            raise SystemExit("support is outside the final message length")
        print_failure_explanation(
            prev_k,
            prev_n,
            prev_b,
            args.field_bits,
            support,
            b[support],
            args.top_terms,
        )

    if args.emit_certificate:
        fieldnames = [
            "round",
            "k",
            "parity_n",
            "support",
            "threshold",
            "distance",
            "relative_distance",
            "worst_log2_slack",
            "worst_support",
        ]
        if args.certificate_path:
            with open(args.certificate_path, "w", newline="") as handle:
                writer = csv.DictWriter(handle, fieldnames=fieldnames)
                writer.writeheader()
                writer.writerows(certificate_rows)
        else:
            print("certificate")
            writer = csv.DictWriter(sys.stdout, fieldnames=fieldnames)
            writer.writeheader()
            writer.writerows(certificate_rows)

    if args.full_thresholds_path:
        with open(args.full_thresholds_path, "w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "round",
                    "k",
                    "parity_n",
                    "support",
                    "threshold",
                    "distance_at_support",
                    "relative_distance_at_support",
                ],
            )
            writer.writeheader()
            writer.writerows(threshold_rows)


if __name__ == "__main__":
    main()
