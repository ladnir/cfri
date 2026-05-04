#!/usr/bin/env python3
"""Test a simple tree-collision rule against exact rank feature rows."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def predicted_structural_bad(row: dict[str, str]) -> bool:
    s_identity = int(row["s_identity"])
    z_parity = int(row["z_parity"])
    i1 = int(row["identity_full_level_1"])
    i2 = int(row["identity_full_level_2"])
    p1 = int(row["parity_collision_level_1"])
    p2 = int(row["parity_collision_level_2"])

    # Empirical depth-3 rule for the exact rows we have enumerated. It is intentionally narrow:
    # adding an extra parity column can repair a lower-level collision, so z_parity is part of the
    # state. This rule is only a regression target for the future recursive theorem.
    if s_identity == 6 and z_parity == 2:
        return (i1 >= 3 and p1 >= 1) or (i1 >= 2 and i2 >= 1 and p2 >= 1)
    if s_identity == 6 and z_parity == 3:
        return i1 >= 3 and i2 >= 1 and p1 >= 2 and p2 >= 1
    if s_identity == 5 and z_parity == 3:
        return i1 >= 2 and p1 >= 1 and p2 >= 1 and (i2 >= 1 or p1 >= 2)
    return False


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("feature_csv", nargs="+")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    rows: list[dict[str, str | int | float | bool]] = []
    for path in args.feature_csv:
        with Path(path).open(newline="") as handle:
            for row in csv.DictReader(handle):
                predicted = predicted_structural_bad(row)
                bad_count = int(row["bad_count"])
                total_count = int(row["total_count"])
                rows.append(
                    {
                        "source": path,
                        "feature": ":".join(
                            row[key]
                            for key in [
                                "s_identity",
                                "z_parity",
                                "identity_full_level_1",
                                "identity_full_level_2",
                                "identity_full_level_3",
                                "parity_collision_level_1",
                                "parity_collision_level_2",
                                "parity_collision_level_3",
                            ]
                        ),
                        "total_count": total_count,
                        "bad_count": bad_count,
                        "bad_fraction": float(row["bad_fraction"]),
                        "predicted_structural_bad": predicted,
                        "false_positive_count": total_count if predicted and bad_count == 0 else 0,
                        "false_negative_count": bad_count if (not predicted and bad_count > 0) else 0,
                    }
                )

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=[
                "source",
                "feature",
                "total_count",
                "bad_count",
                "bad_fraction",
                "predicted_structural_bad",
                "false_positive_count",
                "false_negative_count",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)

    false_positive = sum(int(row["false_positive_count"]) for row in rows)
    false_negative = sum(int(row["false_negative_count"]) for row in rows)
    print(f"rows={len(rows)} false_positive_count={false_positive} false_negative_count={false_negative} out={args.out}")


if __name__ == "__main__":
    main()
