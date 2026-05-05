#!/usr/bin/env python3
"""Compare systematic threshold certificates against the collapse-family ceiling."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("threshold_summary_csv")
    parser.add_argument("collapse_ceiling_csv")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    ceiling_by_depth = {}
    with Path(args.collapse_ceiling_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            ceiling_by_depth[int(row["depth"])] = row

    with Path(args.threshold_summary_csv).open(newline="") as input_handle, Path(args.out).open("w", newline="") as output_handle:
        reader = csv.DictReader(input_handle)
        writer = csv.writer(output_handle)
        writer.writerow(
            [
                "depth",
                "k",
                "total_n",
                "systematic_threshold_relative",
                "collapse_relative_upper_bound",
                "headroom_absolute",
                "headroom_relative",
                "collapse_zero_count",
                "collapse_best_live_rows",
            ]
        )
        for row in reader:
            depth = int(row["depth"].strip('"'))
            ceiling = ceiling_by_depth[depth]
            threshold = float(row["systematic_relative"].strip('"'))
            ceiling_relative = float(ceiling["relative_distance_upper_bound"])
            writer.writerow(
                [
                    depth,
                    int(row["k"].strip('"')),
                    int(row["total_n"].strip('"')),
                    f"{threshold:.8f}",
                    f"{ceiling_relative:.8f}",
                    f"{ceiling_relative - threshold:.8f}",
                    f"{(ceiling_relative / threshold) - 1.0:.8f}",
                    int(ceiling["zero_count"]),
                    int(ceiling["best_live_rows"]),
                ]
            )

    print(f"out={args.out}")


if __name__ == "__main__":
    main()
