#!/usr/bin/env python3
"""Verify systematic RFC threshold certificates.

This checker reads the full-threshold CSV emitted by systematic_rfc_bound.py and
independently checks the theorem conditions used by docs/systematic_rfc_distance_proof.md:

  * positive-support thresholds are monotone,
  * positive-support thresholds are in the admissible range 1..parity_n,
  * affine RFC random failure bound is below the allocated per-support budget.

It intentionally reuses only small log-domain helpers, so the emitted CSV is not trusted.
"""

from __future__ import annotations

import argparse
import csv
import math


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


def log2_k_bound(
    log2_comb_k: list[float],
    thresholds: list[int],
    field_bits: float,
    support: int,
    zero_count: int,
) -> float:
    if support == 0:
        return 0.0
    if zero_count >= thresholds[support]:
        return NEG_INF
    support_choices = log2_comb_k[support]
    kernel_bound = support_choices + (thresholds[support] - zero_count) * field_bits
    exact_support_universe = support_choices + support * (
        field_bits + log2_one_minus_q_inv(field_bits)
    )
    return min(kernel_bound, exact_support_universe)


def log2_failure(
    k: int,
    n: int,
    prev_thresholds: list[int],
    field_bits: float,
    support: int,
    threshold: int,
) -> float:
    log_total = NEG_INF
    log2_comb_k = log2_combinations(k)
    log2_comb_n = log2_combinations(n)
    log2_fact_n = log2_factorials(n)
    log_bad_prob = log2_bad_root_prob(field_bits)

    for left_support in range(max(0, support - k), min(k, support) + 1):
        right_support = support - left_support
        for common_zeros in range(n + 1):
            log_left = log2_k_bound(
                log2_comb_k,
                prev_thresholds,
                field_bits,
                left_support,
                common_zeros,
            )
            if log_left == NEG_INF:
                continue
            log_right = log2_k_bound(
                log2_comb_k,
                prev_thresholds,
                field_bits,
                right_support,
                common_zeros,
            )
            if log_right == NEG_INF:
                continue

            log_term = log2_comb_n[common_zeros] + log_left + log_right
            if threshold > 2 * common_zeros:
                log_tail = log2_outside_tail(
                    n,
                    common_zeros,
                    threshold - 2 * common_zeros,
                    log_bad_prob,
                    log2_fact_n,
                )
                if log_tail == NEG_INF:
                    continue
                log_term += log_tail
            log_total = log2_add(log_total, log_term)
    return log_total


def build_split_terms(
    k: int,
    n: int,
    thresholds: list[int],
    field_bits: float,
    log2_comb_k: list[float],
    log2_comb_n: list[float],
) -> list[list[float]]:
    parent_k = 2 * k
    split_terms = [[NEG_INF] * (n + 1) for _ in range(parent_k + 1)]
    for common_zeros in range(n + 1):
        active_bounds: list[tuple[int, float]] = []
        for support in range(k + 1):
            log_bound = log2_k_bound(
                log2_comb_k,
                thresholds,
                field_bits,
                support,
                common_zeros,
            )
            if log_bound != NEG_INF:
                active_bounds.append((support, log_bound))
        for left_support, log_left in active_bounds:
            for right_support, log_right in active_bounds:
                support = left_support + right_support
                split_terms[support][common_zeros] = log2_add(
                    split_terms[support][common_zeros],
                    log2_comb_n[common_zeros] + log_left + log_right,
                )
    return split_terms


def log2_failure_from_split_terms(
    n: int,
    field_bits: float,
    split_terms: list[list[float]],
    support: int,
    threshold: int,
    log2_fact_n: list[float],
) -> float:
    log_total = NEG_INF
    log_bad_prob = log2_bad_root_prob(field_bits)
    for common_zeros, log_term in enumerate(split_terms[support]):
        if log_term == NEG_INF:
            continue
        if threshold > 2 * common_zeros:
            log_tail = log2_outside_tail(
                n,
                common_zeros,
                threshold - 2 * common_zeros,
                log_bad_prob,
                log2_fact_n,
            )
            if log_tail == NEG_INF:
                continue
            log_term += log_tail
        log_total = log2_add(log_total, log_term)
    return log_total


def read_thresholds(path: str) -> dict[int, tuple[int, int, list[int]]]:
    by_round: dict[int, list[tuple[int, int, int, int]]] = {}
    with open(path, newline="") as handle:
        for row in csv.DictReader(handle):
            round_index = int(row["round"])
            by_round.setdefault(round_index, []).append(
                (
                    int(row["k"]),
                    int(row["support"]),
                    int(row["threshold"]),
                    int(row["parity_n"]),
                )
            )

    result: dict[int, tuple[int, int, list[int]]] = {}
    for round_index, rows in by_round.items():
        rows.sort(key=lambda row: row[1])
        k = len(rows)
        declared_ks = {declared_k for declared_k, _, _, _ in rows}
        if declared_ks != {k}:
            raise ValueError(
                f"round {round_index}: declared k values {sorted(declared_ks)} "
                f"do not match support count {k}"
            )
        parity_ns = {parity_n for _, _, _, parity_n in rows}
        if len(parity_ns) != 1:
            raise ValueError(
                f"round {round_index}: inconsistent parity_n values {sorted(parity_ns)}"
            )
        parity_n = next(iter(parity_ns))
        if [support for _, support, _, _ in rows] != list(range(1, k + 1)):
            raise ValueError(f"round {round_index} supports are not contiguous 1..k")
        thresholds = [parity_n + 1] + [threshold for _, _, threshold, _ in rows]
        result[round_index] = (k, parity_n, thresholds)
    return result


def check_monotone(round_index: int, thresholds: list[int]) -> list[str]:
    errors: list[str] = []
    for support in range(2, len(thresholds)):
        if thresholds[support] < thresholds[support - 1]:
            errors.append(
                f"round {round_index}: threshold decreases at support {support}: "
                f"{thresholds[support - 1]} -> {thresholds[support]}"
            )
    return errors


def check_admissible(round_index: int, n: int, thresholds: list[int]) -> list[str]:
    errors: list[str] = []
    for support in range(1, len(thresholds)):
        if thresholds[support] < 1 or thresholds[support] > n:
            errors.append(
                f"round {round_index} support {support}: threshold "
                f"{thresholds[support]} is outside positive-support range 1..{n}"
            )
    return errors


def systematic_distance(k: int, n: int, thresholds: list[int]) -> tuple[int, int]:
    best_distance = k + n + 1
    best_support = 0
    for support in range(1, k + 1):
        distance = support + n - (thresholds[support] - 1)
        if distance < best_distance:
            best_distance = distance
            best_support = support
    return best_distance, best_support


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("thresholds_csv")
    parser.add_argument("--field-bits", type=float, required=True)
    parser.add_argument("--security-bits", type=float, required=True)
    args = parser.parse_args()

    if args.field_bits < 10.0:
        raise SystemExit("the theorem verifier requires field_bits >= 10")

    rounds = read_thresholds(args.thresholds_csv)
    errors: list[str] = []
    worst_slack = NEG_INF
    worst_round = 0
    worst_support = 0
    round_indices = sorted(rounds)
    if round_indices != list(range(0, max(round_indices) + 1)):
        errors.append(f"rounds are not contiguous from 0: {round_indices}")
    first_k, first_n, _ = rounds[round_indices[0]]
    if first_n % first_k != 0:
        errors.append(f"round 0 parity_n {first_n} is not divisible by k {first_k}")
        parity_expansion = 0
    else:
        parity_expansion = first_n // first_k

    for round_index in round_indices:
        k, n, thresholds = rounds[round_index]
        if parity_expansion and n != parity_expansion * k:
            errors.append(
                f"round {round_index}: parity_n {n} does not match "
                f"parity expansion {parity_expansion} * k {k}"
            )
        errors.extend(check_monotone(round_index, thresholds))
        errors.extend(check_admissible(round_index, n, thresholds))
        if round_index == 0:
            continue
        if round_index - 1 not in rounds:
            errors.append(f"missing previous round {round_index - 1}")
            continue

        prev_k, prev_n, prev_thresholds = rounds[round_index - 1]
        if k != 2 * prev_k or n != 2 * prev_n:
            errors.append(
                f"round {round_index}: expected k,n = {2 * prev_k},{2 * prev_n}, got {k},{n}"
            )
            continue

        per_support_budget = -args.security_bits - math.log2(k)
        log2_fact_prev_n = log2_factorials(prev_n)
        split_terms = build_split_terms(
            prev_k,
            prev_n,
            prev_thresholds,
            args.field_bits,
            log2_combinations(prev_k),
            log2_combinations(prev_n),
        )
        for support in range(1, k + 1):
            log_failure = log2_failure_from_split_terms(
                prev_n,
                args.field_bits,
                split_terms,
                support,
                thresholds[support],
                log2_fact_prev_n,
            )
            slack = log_failure - per_support_budget
            if slack > worst_slack:
                worst_slack = slack
                worst_round = round_index
                worst_support = support
            if slack > 0.0:
                errors.append(
                    f"round {round_index} support {support}: log2 failure exceeds budget "
                    f"by {slack:.6f}"
                )

    final_round = max(rounds)
    final_k, final_n, final_thresholds = rounds[final_round]
    distance, support = systematic_distance(final_k, final_n, final_thresholds)

    print(f"ok={str(not errors).lower()}")
    print(f"rounds={len(rounds)}")
    print(f"final_round={final_round}")
    print(f"final_k={final_k}")
    print(f"final_parity_n={final_n}")
    print(f"parity_expansion={parity_expansion}")
    print(f"final_distance={distance}")
    print(f"final_relative_distance={distance / (final_k + final_n):.12f}")
    print(f"final_worst_distance_support={support}")
    print(f"worst_log2_slack={worst_slack:.12f}")
    print(f"worst_slack_round={worst_round}")
    print(f"worst_slack_support={worst_support}")

    if errors:
        print("errors:")
        for error in errors:
            print(f"- {error}")
        raise SystemExit(1)


if __name__ == "__main__":
    main()
