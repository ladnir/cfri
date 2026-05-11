#!/usr/bin/env python3
"""Summarize persistent structural rank-defect counts from resampling classifications."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def read_row_count(row_csv: str) -> tuple[int, int, int, int, int]:
    with Path(row_csv).open(newline="") as handle:
        rows = list(csv.DictReader(handle))
    if len(rows) != 1:
        raise ValueError(f"expected one data row in {row_csv}")
    row = rows[0]
    return (
        int(row["s_identity"]),
        int(row["z_parity"]),
        int(row["checked_shapes"]),
        int(row["rank_deficient_shapes"]),
        int(row["min_rank"]),
    )


def read_persistent_count(classification_csv: str, samples: int) -> int:
    persistent = 0
    with Path(classification_csv).open(newline="") as handle:
        for row in csv.DictReader(handle):
            if int(row["deficient_samples"]) == samples:
                persistent += int(row["shape_count"])
    return persistent


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--samples", type=int, default=20)
    parser.add_argument("--row", action="append", required=True, help="row_csv=classification_csv")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "s_identity",
                "z_parity",
                "total_z",
                "checked_shapes",
                "rank_deficient_shapes",
                "persistent_structural_shapes",
                "accidental_or_unstable_shapes",
                "structural_fraction",
                "min_rank",
            ]
        )
        for item in args.row:
            row_path, classification_path = item.split("=", 1)
            s_identity, z_parity, checked, deficient, min_rank = read_row_count(row_path)
            persistent = read_persistent_count(classification_path, args.samples)
            writer.writerow(
                [
                    s_identity,
                    z_parity,
                    s_identity + z_parity,
                    checked,
                    deficient,
                    persistent,
                    deficient - persistent,
                    persistent / checked if checked else 0.0,
                    min_rank,
                ]
            )
    print(f"out={args.out}")


if __name__ == "__main__":
    main()
