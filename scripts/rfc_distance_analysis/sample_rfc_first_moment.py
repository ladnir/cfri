#!/usr/bin/env python3
"""Sample first-moment spectra for tiny affine RFC ensembles.

This is an empirical calibrator, not a certificate. For each sampled construction it exhausts all
messages over GF(p), accumulates the low-weight enumerator, and reports the average number of
nonzero codewords of each weight. It compares:

  * original non-systematic affine RFC at total expansion c;
  * systematic affine RFC with parity expansion c-1.

The sampling rule is T uniform in GF(p)^*, with the affine pair T,T+1.
"""

from __future__ import annotations

import argparse
import csv
import itertools
import random
from pathlib import Path


def rfc_generator(k0: int, depth: int, expansion: int, p: int, rng: random.Random) -> list[list[int]]:
    if k0 != 1:
        raise ValueError("only k0=1 repetition base is implemented")
    g = [[1 for _ in range(expansion)]]
    n = expansion
    for _ in range(depth):
        t = [rng.randrange(1, p) for _ in range(n)]
        top: list[list[int]] = []
        bottom: list[list[int]] = []
        for row in g:
            top.append(
                [((1 - t[j]) * row[j]) % p for j in range(n)]
                + [((1 - (t[j] + 1)) * row[j]) % p for j in range(n)]
            )
            bottom.append(
                [(t[j] * row[j]) % p for j in range(n)]
                + [((t[j] + 1) * row[j]) % p for j in range(n)]
            )
        g = top + bottom
        n *= 2
    return g


def encode(message: tuple[int, ...], generator: list[list[int]], p: int) -> list[int]:
    out = [0] * len(generator[0])
    for i, value in enumerate(message):
        if value == 0:
            continue
        row = generator[i]
        for j, entry in enumerate(row):
            out[j] = (out[j] + value * entry) % p
    return out


def weight(values: list[int] | tuple[int, ...]) -> int:
    return sum(1 for value in values if value != 0)


def accumulate_spectrum(
    generator: list[list[int]],
    p: int,
    systematic: bool,
    total_n: int,
) -> tuple[list[int], list[list[int]], int]:
    k = len(generator)
    spectrum = [0] * (total_n + 1)
    by_support = [[0] * (total_n + 1) for _ in range(k + 1)]
    min_distance = total_n + 1
    for message in itertools.product(range(p), repeat=k):
        support = weight(message)
        if support == 0:
            continue
        code_weight = weight(encode(message, generator, p))
        total_weight = code_weight + (support if systematic else 0)
        spectrum[total_weight] += 1
        by_support[support][total_weight] += 1
        min_distance = min(min_distance, total_weight)
    return spectrum, by_support, min_distance


def first_cumulative_at_least(spectrum: list[float], target: float) -> int | None:
    total = 0.0
    for index, value in enumerate(spectrum):
        total += value
        if index > 0 and total >= target:
            return index
    return None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--p", type=int, default=5)
    parser.add_argument("--depth", type=int, default=3)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--samples", type=int, default=100)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--csv-out", default=None)
    parser.add_argument("--support-csv-out", default=None)
    parser.add_argument("--max-print-weight", type=int, default=32)
    args = parser.parse_args()

    if args.p <= 2:
        raise SystemExit("use an odd prime p > 2 so GF(p)^* has nontrivial T samples")
    if args.total_expansion <= 1:
        raise SystemExit("total expansion must be greater than one")

    rng = random.Random(args.seed)
    k = 1 << args.depth
    total_n = args.total_expansion * k
    old_acc = [0.0] * (total_n + 1)
    sys_acc = [0.0] * (total_n + 1)
    old_support_acc = [[0.0] * (total_n + 1) for _ in range(k + 1)]
    sys_support_acc = [[0.0] * (total_n + 1) for _ in range(k + 1)]
    old_min_sum = 0.0
    sys_min_sum = 0.0

    for sample in range(args.samples):
        old = rfc_generator(1, args.depth, args.total_expansion, args.p, rng)
        sys_parity = rfc_generator(1, args.depth, args.total_expansion - 1, args.p, rng)

        old_spec, old_by_support, old_min = accumulate_spectrum(old, args.p, False, total_n)
        sys_spec, sys_by_support, sys_min = accumulate_spectrum(sys_parity, args.p, True, total_n)
        old_min_sum += old_min
        sys_min_sum += sys_min
        for h in range(total_n + 1):
            old_acc[h] += old_spec[h]
            sys_acc[h] += sys_spec[h]
        for support in range(1, k + 1):
            for h in range(total_n + 1):
                old_support_acc[support][h] += old_by_support[support][h]
                sys_support_acc[support][h] += sys_by_support[support][h]

        if (sample + 1) % max(1, args.samples // 10) == 0:
            print(f"sample={sample + 1}/{args.samples}")

    old_avg = [value / args.samples for value in old_acc]
    sys_avg = [value / args.samples for value in sys_acc]

    print(
        f"p={args.p} depth={args.depth} k={k} total_n={total_n} "
        f"samples={args.samples} total_expansion={args.total_expansion}"
    )
    print(
        f"old_average_min_distance={old_min_sum / args.samples:.6f} "
        f"relative={old_min_sum / args.samples / total_n:.8f} "
        f"first_moment_crossing={first_cumulative_at_least(old_avg, 1.0)}"
    )
    print(
        f"systematic_average_min_distance={sys_min_sum / args.samples:.6f} "
        f"relative={sys_min_sum / args.samples / total_n:.8f} "
        f"first_moment_crossing={first_cumulative_at_least(sys_avg, 1.0)}"
    )
    print("weight,old_expected_count,systematic_expected_count")
    for h in range(min(total_n, args.max_print_weight) + 1):
        if old_avg[h] or sys_avg[h]:
            print(f"{h},{old_avg[h]:.12g},{sys_avg[h]:.12g}")

    if args.csv_out is not None:
        path = Path(args.csv_out)
        with path.open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=["weight", "old_expected_count", "systematic_expected_count"],
            )
            writer.writeheader()
            for h in range(total_n + 1):
                writer.writerow(
                    {
                        "weight": h,
                        "old_expected_count": old_avg[h],
                        "systematic_expected_count": sys_avg[h],
                    }
                )

    if args.support_csv_out is not None:
        path = Path(args.support_csv_out)
        with path.open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "support",
                    "weight",
                    "old_expected_count",
                    "systematic_expected_count",
                ],
            )
            writer.writeheader()
            for support in range(1, k + 1):
                for h in range(total_n + 1):
                    writer.writerow(
                        {
                            "support": support,
                            "weight": h,
                            "old_expected_count": old_support_acc[support][h] / args.samples,
                            "systematic_expected_count": sys_support_acc[support][h]
                            / args.samples,
                        }
                    )


if __name__ == "__main__":
    main()
