#!/usr/bin/env python3
"""Profile the r=2 root-compatibility condition for RFC singleton requests.

For two parent messages, one singleton coordinate must satisfy the same root challenge in both
replicas.  If the child value vectors are X=(L0,L1) and Y=(R0,R1), this requires
dim span(X,Y) <= 1.  This script compares the exact singleton-request generating function with the
loose bound that treats every non-common-zero coordinate as root-capable.

For a tuple of two parent messages, define

    w_j = Pr[left singleton at j succeeds] + Pr[right singleton at j succeeds].

Then the sum over all oriented singleton zero sets of size s is e_s(w_1,...,w_n).  The loose bound
uses w_j=2 for common-zero coordinates and w_j=2/(q-1) for every other coordinate.  The exact value
sets w_j=0 on rank-2 and rank-1/no-root coordinates.
"""

from __future__ import annotations

import argparse
import csv
import itertools
import math
import random
from pathlib import Path

from sample_rfc_rank_failure import rfc_generator_prime


def encode(message: tuple[int, ...], generator: list[list[int]], prime: int) -> tuple[int, ...]:
    out = [0] * len(generator[0])
    for value, row in zip(message, generator):
        if value == 0:
            continue
        for j, entry in enumerate(row):
            out[j] = (out[j] + value * entry) % prime
    return tuple(out)


def all_messages(k: int, prime: int) -> list[tuple[int, ...]]:
    return list(itertools.product(range(prime), repeat=k))


def elementary_symmetric(weights: list[float], max_degree: int) -> list[float]:
    out = [0.0] * (max_degree + 1)
    out[0] = 1.0
    for weight in weights:
        if weight == 0.0:
            continue
        stop = min(max_degree, len(weights))
        for degree in range(stop, 0, -1):
            out[degree] += out[degree - 1] * weight
    return out


def root_hit_counts(x0: int, x1: int, y0: int, y1: int, prime: int) -> tuple[int, int]:
    left = 0
    right = 0
    for t in range(1, prime):
        if (x0 + t * (y0 - x0)) % prime == 0 and (x1 + t * (y1 - x1)) % prime == 0:
            left = 1
        if (y0 + t * (y0 - x0)) % prime == 0 and (y1 + t * (y1 - x1)) % prime == 0:
            right = 1
    return left, right


def classify_coordinate(x0: int, x1: int, y0: int, y1: int, prime: int) -> tuple[str, float, float]:
    common = x0 == 0 and x1 == 0 and y0 == 0 and y1 == 0
    if common:
        return "rank0", 2.0, 2.0

    determinant = (x0 * y1 - x1 * y0) % prime
    loose = 2.0 / (prime - 1)
    if determinant != 0:
        return "rank2", 0.0, loose

    left, right = root_hit_counts(x0, x1, y0, y1, prime)
    hits = left + right
    if hits == 0:
        return "rank1_no_root", 0.0, loose
    if hits == 1:
        return "rank1_one_root", 1.0 / (prime - 1), loose
    return "rank1_two_roots", 2.0 / (prime - 1), loose


def iter_tuple_indices(
    message_count: int,
    samples: int,
    rng: random.Random,
):
    if samples <= 0:
        yield from itertools.product(range(message_count), repeat=4)
        return
    for _ in range(samples):
        yield (
            rng.randrange(message_count),
            rng.randrange(message_count),
            rng.randrange(message_count),
            rng.randrange(message_count),
        )


def log2_or_neg(value: float) -> float:
    if value <= 0.0:
        return -math.inf
    return math.log2(value)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--child-depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--tuple-samples", type=int, default=0)
    parser.add_argument("--max-singletons", type=int, default=-1)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--category-csv", default=None)
    parser.add_argument("--singleton-csv", default=None)
    args = parser.parse_args()

    if args.prime <= 2:
        raise SystemExit("--prime must be > 2")
    rng = random.Random(args.seed)
    generator = rfc_generator_prime(args.child_depth, args.expansion, args.prime, rng)
    child_k = len(generator)
    child_n = len(generator[0])
    max_singletons = child_n if args.max_singletons < 0 else min(args.max_singletons, child_n)

    messages = all_messages(child_k, args.prime)
    encodings = [encode(message, generator, args.prime) for message in messages]
    exact_sum = [0.0] * (max_singletons + 1)
    loose_sum = [0.0] * (max_singletons + 1)
    category_counts = {
        "rank0": 0,
        "rank1_no_root": 0,
        "rank1_one_root": 0,
        "rank1_two_roots": 0,
        "rank2": 0,
    }
    checked = 0
    skipped_all_zero_pair = 0

    for left0_i, right0_i, left1_i, right1_i in iter_tuple_indices(len(messages), args.tuple_samples, rng):
        if left0_i == 0 and right0_i == 0 and left1_i == 0 and right1_i == 0:
            skipped_all_zero_pair += 1
            continue
        left0 = encodings[left0_i]
        right0 = encodings[right0_i]
        left1 = encodings[left1_i]
        right1 = encodings[right1_i]
        exact_weights: list[float] = []
        loose_weights: list[float] = []
        for j in range(child_n):
            category, exact_weight, loose_weight = classify_coordinate(
                left0[j],
                left1[j],
                right0[j],
                right1[j],
                args.prime,
            )
            category_counts[category] += 1
            exact_weights.append(exact_weight)
            loose_weights.append(loose_weight)

        exact_poly = elementary_symmetric(exact_weights, max_singletons)
        loose_poly = elementary_symmetric(loose_weights, max_singletons)
        for degree in range(max_singletons + 1):
            exact_sum[degree] += exact_poly[degree]
            loose_sum[degree] += loose_poly[degree]
        checked += 1

    mode = "sampled" if args.tuple_samples > 0 else "exact"
    print(
        f"mode={mode} prime={args.prime} child_depth={args.child_depth} "
        f"child_k={child_k} child_n={child_n} expansion={args.expansion} "
        f"checked_tuples={checked} skipped_all_zero_pair={skipped_all_zero_pair}"
    )
    print("category,total,per_tuple_coordinate_rate")
    denominator = checked * child_n if checked else 1
    for category, count in category_counts.items():
        print(f"{category},{count},{count / denominator:.12g}")
    if args.category_csv is not None:
        with Path(args.category_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "mode",
                    "prime",
                    "child_depth",
                    "child_k",
                    "child_n",
                    "expansion",
                    "checked_tuples",
                    "category",
                    "total",
                    "per_tuple_coordinate_rate",
                ],
            )
            writer.writeheader()
            for category, count in category_counts.items():
                writer.writerow(
                    {
                        "mode": mode,
                        "prime": args.prime,
                        "child_depth": args.child_depth,
                        "child_k": child_k,
                        "child_n": child_n,
                        "expansion": args.expansion,
                        "checked_tuples": checked,
                        "category": category,
                        "total": count,
                        "per_tuple_coordinate_rate": count / denominator,
                    }
                )

    print("singletons,log2_exact,log2_loose,log2_slack,exact_over_loose")
    singleton_rows = []
    for degree in range(max_singletons + 1):
        exact_log = log2_or_neg(exact_sum[degree])
        loose_log = log2_or_neg(loose_sum[degree])
        slack_log = loose_log - exact_log if math.isfinite(exact_log) and math.isfinite(loose_log) else math.inf
        ratio = exact_sum[degree] / loose_sum[degree] if loose_sum[degree] else 0.0
        exact_label = "-inf" if not math.isfinite(exact_log) else f"{exact_log:.8f}"
        loose_label = "-inf" if not math.isfinite(loose_log) else f"{loose_log:.8f}"
        slack_label = "inf" if not math.isfinite(slack_log) else f"{slack_log:.8f}"
        print(f"{degree},{exact_label},{loose_label},{slack_label},{ratio:.12g}")
        singleton_rows.append(
            {
                "mode": mode,
                "prime": args.prime,
                "child_depth": args.child_depth,
                "child_k": child_k,
                "child_n": child_n,
                "expansion": args.expansion,
                "checked_tuples": checked,
                "singletons": degree,
                "log2_exact": exact_log,
                "log2_loose": loose_log,
                "log2_slack": slack_log,
                "exact_over_loose": ratio,
            }
        )
    if args.singleton_csv is not None:
        with Path(args.singleton_csv).open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "mode",
                    "prime",
                    "child_depth",
                    "child_k",
                    "child_n",
                    "expansion",
                    "checked_tuples",
                    "singletons",
                    "log2_exact",
                    "log2_loose",
                    "log2_slack",
                    "exact_over_loose",
                ],
            )
            writer.writeheader()
            writer.writerows(singleton_rows)


if __name__ == "__main__":
    main()
