#!/usr/bin/env python3
"""Compute compressed one-step first-moment crossings from category samples.

Given the category CSV from `systematic_rfc_cert --sample-rfc-one-step`, this script compares the
exact one-step first-moment crossing to certificate-style compressed projections. For a projection
bucket, it upper-bounds the cumulative low-tail contribution at cutoff D by:

    total_pair_count(bucket) * max_{exact category in bucket} Pr[wt <= D].

This is the natural worst-case loss if a future recurrence tracks only the projected state.
"""

from __future__ import annotations

import argparse
import csv
import math
from collections import defaultdict
from pathlib import Path


def binom_pmf(n: int, p: float) -> list[float]:
    return [math.comb(n, k) * (p**k) * ((1.0 - p) ** (n - k)) for k in range(n + 1)]


class TailComputer:
    def __init__(self, prime: int) -> None:
        self.prime = prime
        self._pmf_cache: dict[tuple[int, int], tuple[list[float], list[float]]] = {}
        self._tail_cache: dict[tuple[int, int, int, int], float] = {}

    def tail_probability(
        self,
        equal_nonzero: int,
        single_root: int,
        double_root: int,
        parity_cutoff: int,
    ) -> float:
        max_weight = 2 * (equal_nonzero + single_root + double_root)
        min_weight = max_weight - single_root - double_root
        if parity_cutoff < min_weight:
            return 0.0
        if parity_cutoff >= max_weight:
            return 1.0

        key = (equal_nonzero, single_root, double_root, parity_cutoff)
        if key in self._tail_cache:
            return self._tail_cache[key]

        pmf_key = (single_root, double_root)
        if pmf_key not in self._pmf_cache:
            self._pmf_cache[pmf_key] = (
                binom_pmf(single_root, 1.0 / (self.prime - 1)),
                binom_pmf(double_root, 2.0 / (self.prime - 1)),
            )
        single_pmf, double_pmf = self._pmf_cache[pmf_key]
        total = 0.0
        for single_hits, single_prob in enumerate(single_pmf):
            for double_hits, double_prob in enumerate(double_pmf):
                if max_weight - single_hits - double_hits <= parity_cutoff:
                    total += single_prob * double_prob
        self._tail_cache[key] = total
        return total


def projection(row: dict[str, int | float | str], mode: str) -> tuple[int, ...]:
    support = int(row["parent_support"])
    equal_nonzero = int(row["equal_nonzero"])
    single_root = int(row["single_root"])
    double_root = int(row["double_root"])
    active = equal_nonzero + single_root + double_root

    if mode == "support":
        return (support,)
    if mode == "support_active":
        return (support, active)
    if mode == "support_equal_root":
        return (support, equal_nonzero, single_root + double_root)
    if mode == "support_single_double":
        return (support, single_root, double_root)
    if mode == "full":
        return (support, equal_nonzero, single_root, double_root)
    raise ValueError(f"unknown mode {mode}")


def crossing(values: list[float]) -> int | None:
    for index, value in enumerate(values):
        if index > 0 and value >= 1.0:
            return index
    return None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("category_csv")
    parser.add_argument("--prime", type=int, default=5)
    parser.add_argument("--total-n", type=int, required=True)
    parser.add_argument("--out", default=None)
    args = parser.parse_args()

    modes = [
        "support",
        "support_active",
        "support_equal_root",
        "support_single_double",
        "full",
    ]
    rows_by_ensemble: dict[str, list[dict[str, int | float | str]]] = defaultdict(list)
    with Path(args.category_csv).open(newline="") as handle:
        for raw in csv.DictReader(handle):
            row: dict[str, int | float | str] = {
                "ensemble": raw["ensemble"],
                "parent_support": int(raw["parent_support"]),
                "equal_nonzero": int(raw["equal_nonzero"]),
                "single_root": int(raw["single_root"]),
                "double_root": int(raw["double_root"]),
                "expected_pair_count": float(raw["expected_pair_count"]),
            }
            rows_by_ensemble[raw["ensemble"]].append(row)

    tails = TailComputer(args.prime)
    summary_rows: list[dict[str, int | float | str | None]] = []
    curve_rows: list[dict[str, int | float | str]] = []

    for ensemble, rows in sorted(rows_by_ensemble.items()):
        exact_curve = [0.0] * (args.total_n + 1)
        for cutoff in range(args.total_n + 1):
            total = 0.0
            for row in rows:
                support = int(row["parent_support"])
                parity_cutoff = cutoff - (support if ensemble == "systematic" else 0)
                total += float(row["expected_pair_count"]) * tails.tail_probability(
                    int(row["equal_nonzero"]),
                    int(row["single_root"]),
                    int(row["double_root"]),
                    parity_cutoff,
                )
            exact_curve[cutoff] = total

        exact_crossing = crossing(exact_curve)
        summary_rows.append(
            {
                "ensemble": ensemble,
                "mode": "exact",
                "crossing": exact_crossing,
                "value_at_crossing": exact_curve[exact_crossing] if exact_crossing is not None else "",
            }
        )
        for cutoff, value in enumerate(exact_curve):
            curve_rows.append(
                {
                    "ensemble": ensemble,
                    "mode": "exact",
                    "cutoff": cutoff,
                    "cumulative_expected_count": value,
                }
            )

        for mode in modes:
            buckets: dict[tuple[int, ...], list[dict[str, int | float | str]]] = defaultdict(list)
            pair_counts: dict[tuple[int, ...], float] = defaultdict(float)
            for row in rows:
                key = projection(row, mode)
                buckets[key].append(row)
                pair_counts[key] += float(row["expected_pair_count"])

            curve = [0.0] * (args.total_n + 1)
            for cutoff in range(args.total_n + 1):
                total = 0.0
                for key, bucket_rows in buckets.items():
                    max_probability = 0.0
                    for row in bucket_rows:
                        support = int(row["parent_support"])
                        parity_cutoff = cutoff - (support if ensemble == "systematic" else 0)
                        probability = tails.tail_probability(
                            int(row["equal_nonzero"]),
                            int(row["single_root"]),
                            int(row["double_root"]),
                            parity_cutoff,
                        )
                        if probability > max_probability:
                            max_probability = probability
                    total += pair_counts[key] * max_probability
                curve[cutoff] = total

            projected_crossing = crossing(curve)
            summary_rows.append(
                {
                    "ensemble": ensemble,
                    "mode": mode,
                    "crossing": projected_crossing,
                    "value_at_crossing": curve[projected_crossing]
                    if projected_crossing is not None
                    else "",
                    "crossing_loss": (
                        projected_crossing - exact_crossing
                        if projected_crossing is not None and exact_crossing is not None
                        else ""
                    ),
                }
            )
            for cutoff, value in enumerate(curve):
                curve_rows.append(
                    {
                        "ensemble": ensemble,
                        "mode": mode,
                        "cutoff": cutoff,
                        "cumulative_expected_count": value,
                    }
                )

    print("ensemble,mode,crossing,value_at_crossing,crossing_loss")
    for row in summary_rows:
        print(
            f"{row['ensemble']},{row['mode']},{row['crossing']},"
            f"{row['value_at_crossing']},{row.get('crossing_loss', '')}"
        )

    if args.out is not None:
        out = Path(args.out)
        summary_path = out.with_suffix(".summary.csv")
        curves_path = out.with_suffix(".curves.csv")
        with summary_path.open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "ensemble",
                    "mode",
                    "crossing",
                    "value_at_crossing",
                    "crossing_loss",
                ],
            )
            writer.writeheader()
            writer.writerows(summary_rows)
        with curves_path.open("w", newline="") as handle:
            writer = csv.DictWriter(
                handle,
                fieldnames=[
                    "ensemble",
                    "mode",
                    "cutoff",
                    "cumulative_expected_count",
                ],
            )
            writer.writeheader()
            writer.writerows(curve_rows)


if __name__ == "__main__":
    main()
