#!/usr/bin/env python3
"""Analyze sampled one-step RFC category spectra.

This postprocessor reads the category CSV emitted by the C++ `--sample-rfc-one-step` mode and
summarizes the low-tail contribution under several compressed state projections. It is a design
tool for the first-moment certificate; it is not itself a certificate.
"""

from __future__ import annotations

import argparse
import csv
import math
from collections import defaultdict
from pathlib import Path
from typing import Callable


def binom_pmf(n: int, p: float) -> list[float]:
    return [math.comb(n, k) * (p**k) * ((1.0 - p) ** (n - k)) for k in range(n + 1)]


def low_tail_probability(
    equal_nonzero: int,
    single_root: int,
    double_root: int,
    parity_cutoff: int,
    prime: int,
    cache: dict[tuple[int, int, int], float],
) -> float:
    max_weight = 2 * (equal_nonzero + single_root + double_root)
    min_weight = max_weight - single_root - double_root
    if parity_cutoff < min_weight:
        return 0.0
    if parity_cutoff >= max_weight:
        return 1.0

    key = (single_root, double_root, parity_cutoff - min_weight)
    if key in cache:
        return cache[key]

    single_pmf = binom_pmf(single_root, 1.0 / (prime - 1))
    double_pmf = binom_pmf(double_root, 2.0 / (prime - 1))
    total = 0.0
    for single_hits, single_prob in enumerate(single_pmf):
        for double_hits, double_prob in enumerate(double_pmf):
            if max_weight - single_hits - double_hits <= parity_cutoff:
                total += single_prob * double_prob
    cache[key] = total
    return total


def projection_key(row: dict[str, int | float | str], mode: str) -> tuple[int, ...]:
    support = int(row["parent_support"])
    equal_nonzero = int(row["equal_nonzero"])
    single_root = int(row["single_root"])
    double_root = int(row["double_root"])
    root_capable = single_root + double_root
    active = equal_nonzero + root_capable

    if mode == "support":
        return (support,)
    if mode == "support_active":
        return (support, active)
    if mode == "support_equal_root":
        return (support, equal_nonzero, root_capable)
    if mode == "support_equal_single_double":
        return (support, equal_nonzero, single_root, double_root)
    raise ValueError(f"unknown projection mode {mode}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("category_csv")
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--old-cutoff", type=int, required=True)
    parser.add_argument("--systematic-cutoff", type=int, required=True)
    parser.add_argument("--out", default=None)
    args = parser.parse_args()

    cache: dict[tuple[int, int, int], float] = {}
    modes = [
        "support",
        "support_active",
        "support_equal_root",
        "support_equal_single_double",
    ]
    totals: dict[str, dict[str, float]] = {mode: defaultdict(float) for mode in modes}
    pair_counts: dict[str, dict[str, float]] = {mode: defaultdict(float) for mode in modes}
    exact_total = defaultdict(float)

    with Path(args.category_csv).open(newline="") as handle:
        for raw in csv.DictReader(handle):
            ensemble = raw["ensemble"]
            cutoff = args.old_cutoff if ensemble == "original" else args.systematic_cutoff
            support = int(raw["parent_support"])
            equal_nonzero = int(raw["equal_nonzero"])
            single_root = int(raw["single_root"])
            double_root = int(raw["double_root"])
            pair_count = float(raw["expected_pair_count"])
            parity_cutoff = cutoff - (support if ensemble == "systematic" else 0)
            probability = low_tail_probability(
                equal_nonzero,
                single_root,
                double_root,
                parity_cutoff,
                args.prime,
                cache,
            )
            contribution = pair_count * probability
            exact_total[ensemble] += contribution

            typed_row: dict[str, int | float | str] = {
                "parent_support": support,
                "equal_nonzero": equal_nonzero,
                "single_root": single_root,
                "double_root": double_root,
            }
            for mode in modes:
                key = (ensemble, *projection_key(typed_row, mode))
                totals[mode][key] += contribution
                pair_counts[mode][key] += pair_count

    rows: list[dict[str, str | int | float]] = []
    for mode in modes:
        for key, contribution in sorted(totals[mode].items(), key=lambda item: item[1], reverse=True):
            ensemble = key[0]
            projected = key[1:]
            rows.append(
                {
                    "mode": mode,
                    "ensemble": ensemble,
                    "projected_state": ":".join(str(x) for x in projected),
                    "expected_pair_count": pair_counts[mode][key],
                    "expected_low_tail_contribution": contribution,
                    "fraction_of_low_tail": contribution / exact_total[ensemble]
                    if exact_total[ensemble]
                    else 0.0,
                }
            )

    print("ensemble,exact_low_tail")
    for ensemble, total in sorted(exact_total.items()):
        print(f"{ensemble},{total:.12g}")
    print()
    for mode in modes:
        print(f"mode={mode}")
        shown = 0
        for row in rows:
            if row["mode"] != mode:
                continue
            print(
                f"{row['ensemble']},{row['projected_state']},"
                f"{row['expected_low_tail_contribution']:.12g},"
                f"{row['fraction_of_low_tail']:.6f}"
            )
            shown += 1
            if shown >= 12:
                break
        print()

    if args.out is not None:
        with Path(args.out).open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "mode",
                    "ensemble",
                    "projected_state",
                    "expected_pair_count",
                    "expected_low_tail_contribution",
                    "fraction_of_low_tail",
                ],
            )
            writer.writeheader()
            writer.writerows(rows)


if __name__ == "__main__":
    main()
