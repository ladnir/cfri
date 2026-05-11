#!/usr/bin/env python3
"""Compare old RFC distance certificates against the original-MDS target."""

from __future__ import annotations

import argparse
import csv
import math
from pathlib import Path


def queries_for_bits(delta: float, bits: int) -> int:
    if not 0.0 < delta < 1.0:
        raise ValueError("delta must be in (0,1)")
    return math.ceil(bits * math.log(2) / -math.log(1.0 - delta))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("summary_csv")
    parser.add_argument("--security-bits", type=int, default=80)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    with Path(args.summary_csv).open(newline="") as input_handle, Path(args.out).open("w", newline="") as output_handle:
        reader = csv.DictReader(input_handle)
        fieldnames = [
            "depth",
            "k",
            "total_n",
            "old_non_systematic_relative",
            "new_mds_relative",
            "systematic_threshold_relative",
            "absolute_gain_vs_old_non_systematic",
            "relative_gain_vs_old_non_systematic",
            "old_non_systematic_queries_80",
            "new_mds_queries_80",
            "query_reduction",
        ]
        writer = csv.DictWriter(output_handle, fieldnames=fieldnames)
        writer.writeheader()
        for row in reader:
            depth = int(row["depth"].strip('"'))
            k = int(row["k"].strip('"'))
            total_n = int(row["total_n"].strip('"'))
            old_delta = float(row["non_systematic_relative"].strip('"'))
            systematic_delta = float(row["systematic_relative"].strip('"'))
            mds_delta = (total_n - k + 1) / total_n
            old_queries = queries_for_bits(old_delta, args.security_bits)
            mds_queries = queries_for_bits(mds_delta, args.security_bits)
            writer.writerow(
                {
                    "depth": depth,
                    "k": k,
                    "total_n": total_n,
                    "old_non_systematic_relative": f"{old_delta:.8f}",
                    "new_mds_relative": f"{mds_delta:.8f}",
                    "systematic_threshold_relative": f"{systematic_delta:.8f}",
                    "absolute_gain_vs_old_non_systematic": f"{mds_delta - old_delta:.8f}",
                    "relative_gain_vs_old_non_systematic": f"{(mds_delta / old_delta) - 1.0:.8f}",
                    "old_non_systematic_queries_80": old_queries,
                    "new_mds_queries_80": mds_queries,
                    "query_reduction": old_queries - mds_queries,
                }
            )

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
