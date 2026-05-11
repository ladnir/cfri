#!/usr/bin/env python3
"""Exhaust small RFC samples and check the one-copy support uncertainty law."""

from __future__ import annotations

import argparse
import csv
import random
from pathlib import Path

from sample_rfc_rank_failure import rfc_generator_prime


def increment_message(message: list[int], prime: int) -> bool:
    for index, value in enumerate(message):
        value += 1
        if value < prime:
            message[index] = value
            return True
        message[index] = 0
    return False


def support_and_output_weight(message: list[int], generator: list[list[int]], prime: int) -> tuple[int, int]:
    k = len(generator)
    n = len(generator[0])
    scratch = [0] * n
    support = 0
    for row in range(k):
        value = message[row]
        if value == 0:
            continue
        support += 1
        gen_row = generator[row]
        for col in range(n):
            scratch[col] = (scratch[col] + value * gen_row[col]) % prime
    return support, sum(1 for value in scratch if value != 0)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--samples", type=int, default=1)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    if args.prime <= 2:
        raise SystemExit("--prime must be odd for this diagnostic")
    if args.samples <= 0:
        raise SystemExit("--samples must be positive")

    rng = random.Random(args.seed)
    k = 1 << args.depth

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "sample",
                "depth",
                "prime",
                "k",
                "message_support",
                "message_count",
                "min_output_weight",
                "min_support_product",
                "equality_count",
                "violation_count",
            ]
        )

        for sample in range(args.samples):
            generator = rfc_generator_prime(args.depth, 1, args.prime, rng)
            counts = [0] * (k + 1)
            min_output = [k + 1] * (k + 1)
            min_product = [10**18] * (k + 1)
            equality = [0] * (k + 1)
            violations = [0] * (k + 1)

            message = [0] * k
            while increment_message(message, args.prime):
                support, output_weight = support_and_output_weight(message, generator, args.prime)
                product = support * output_weight
                counts[support] += 1
                min_output[support] = min(min_output[support], output_weight)
                min_product[support] = min(min_product[support], product)
                if product == k:
                    equality[support] += 1
                if product < k:
                    violations[support] += 1

            for support in range(1, k + 1):
                writer.writerow(
                    [
                        sample,
                        args.depth,
                        args.prime,
                        k,
                        support,
                        counts[support],
                        min_output[support],
                        min_product[support],
                        equality[support],
                        violations[support],
                    ]
                )

    print(f"depth={args.depth} prime={args.prime} samples={args.samples} out={args.out}")


if __name__ == "__main__":
    main()
