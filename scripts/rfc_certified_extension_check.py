#!/usr/bin/env python3
"""Check whether certified-defective cores remain defective after parity extensions."""

from __future__ import annotations

import argparse
import csv
import itertools
from pathlib import Path

from rfc_split_profile import certified_rank, decode_shape, live_rows, parse_shape


def load_defective_cores(path: str, depth: int, parity_expansion: int) -> list[tuple[int, ...]]:
    cores: list[tuple[int, ...]] = []
    with Path(path).open(newline="") as handle:
        for row in csv.DictReader(handle):
            columns = tuple(parse_shape(row["columns"]))
            systematic, parity = decode_shape(list(columns), depth, parity_expansion)
            defect = live_rows(systematic, depth) - certified_rank(systematic, parity, depth)
            if defect > 0:
                cores.append(columns)
    return cores


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--cores-csv", required=True)
    parser.add_argument("--target-z-parity", type=int, required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--defective-cores-out", default=None)
    parser.add_argument("--max-records", type=int, default=20)
    args = parser.parse_args()

    k = 1 << args.depth
    parity_expansion = args.total_expansion - 1
    parity_n = parity_expansion * k
    cores = load_defective_cores(args.cores_csv, args.depth, parity_expansion)

    total_extensions = 0
    defective_extensions = 0
    unique_defective: set[tuple[int, ...]] = set()
    first_defective = ""
    recorded: list[tuple[int, str, int]] = []
    for core_index, core in enumerate(cores):
        systematic_columns = [column for column in core if column < k]
        core_parity = sorted(column - k for column in core if column >= k)
        add_count = args.target_z_parity - len(core_parity)
        if add_count < 0:
            raise SystemExit("target parity count is smaller than a loaded core")

        core_set = set(core_parity)
        available = [column for column in range(parity_n) if column not in core_set]
        for extra in itertools.combinations(available, add_count):
            columns = systematic_columns + [k + column for column in sorted(core_parity + list(extra))]
            systematic, parity = decode_shape(columns, args.depth, parity_expansion)
            defect = live_rows(systematic, args.depth) - certified_rank(systematic, parity, args.depth)
            total_extensions += 1
            if defect > 0:
                defective_extensions += 1
                unique_defective.add(tuple(columns))
                text = ":".join(str(column) for column in columns)
                if first_defective == "":
                    first_defective = text
                if len(recorded) < args.max_records:
                    recorded.append((core_index, text, defect))

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "total_expansion",
                "cores",
                "target_z_parity",
                "checked_extensions",
                "defective_extensions",
                "unique_defective_extensions",
                "first_defective",
            ]
        )
        writer.writerow(
            [
                args.depth,
                args.total_expansion,
                len(cores),
                args.target_z_parity,
                total_extensions,
                defective_extensions,
                len(unique_defective),
                first_defective,
            ]
        )
        writer.writerow([])
        writer.writerow(["core_index", "columns", "cert_defect"])
        for core_index, columns, defect in recorded:
            writer.writerow([core_index, columns, defect])

    if args.defective_cores_out is not None:
        with Path(args.defective_cores_out).open("w", newline="") as handle:
            writer = csv.writer(handle)
            writer.writerow(["index", "columns"])
            for index, columns in enumerate(sorted(unique_defective)):
                writer.writerow([index, ":".join(str(column) for column in columns)])

    print(
        f"depth={args.depth} cores={len(cores)} target_z_parity={args.target_z_parity} "
        f"checked={total_extensions} defective={defective_extensions} "
        f"unique_defective={len(unique_defective)} out={args.out}"
    )


if __name__ == "__main__":
    main()
