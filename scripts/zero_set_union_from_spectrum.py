#!/usr/bin/env python3
"""Compute the zero-set union-bound moment from a codeword spectrum."""

from __future__ import annotations

import argparse
import csv
import math
from pathlib import Path


def comb_float(n: int, k: int) -> float:
    if k < 0 or k > n:
        return 0.0
    return math.exp(math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("spectrum_csv")
    parser.add_argument("--total-n", type=int, required=True)
    parser.add_argument("--zero-count", type=int, required=True)
    args = parser.parse_args()

    totals = {"original": 0.0, "systematic": 0.0}
    low_words = {"original": 0.0, "systematic": 0.0}
    with Path(args.spectrum_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            weight = int(row["weight"])
            zeroes = args.total_n - weight
            multiplier = comb_float(zeroes, args.zero_count)
            old_count = float(row["old_expected_count"])
            sys_count = float(row["systematic_expected_count"])
            totals["original"] += old_count * multiplier
            totals["systematic"] += sys_count * multiplier
            if zeroes >= args.zero_count:
                low_words["original"] += old_count
                low_words["systematic"] += sys_count

    print("ensemble,zero_count,low_word_first_moment,zero_set_union_moment,log2_zero_set_union_moment")
    for ensemble in ("original", "systematic"):
        value = totals[ensemble]
        log_value = -math.inf if value == 0.0 else math.log2(value)
        print(f"{ensemble},{args.zero_count},{low_words[ensemble]:.12g},{value:.12g},{log_value:.12g}")


if __name__ == "__main__":
    main()
