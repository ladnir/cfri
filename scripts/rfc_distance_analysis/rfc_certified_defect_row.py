#!/usr/bin/env python3
"""Exact row counts for the recursive RFC restricted-rank certificate."""

from __future__ import annotations

import argparse
import csv
import itertools
from collections import Counter
from pathlib import Path

from rfc_split_profile import certified_rank, decode_shape, live_rows


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--s-identity", type=int, required=True)
    parser.add_argument("--z-parity", type=int, required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    parity_expansion = args.total_expansion - 1
    parity_n = parity_expansion * k

    by_defect: Counter[int] = Counter()
    first_defective = ""
    total = 0
    for identity_columns in itertools.combinations(range(k), args.s_identity):
        for parity_columns in itertools.combinations(range(parity_n), args.z_parity):
            columns = list(identity_columns) + [k + column for column in parity_columns]
            systematic, parity = decode_shape(columns, args.depth, parity_expansion)
            defect = live_rows(systematic, args.depth) - certified_rank(systematic, parity, args.depth)
            by_defect[defect] += 1
            total += 1
            if defect > 0 and first_defective == "":
                first_defective = ":".join(str(column) for column in columns)

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "total_expansion",
                "s_identity",
                "z_parity",
                "checked_shapes",
                "cert_defect",
                "shape_count",
                "first_defective",
            ]
        )
        for defect, count in sorted(by_defect.items()):
            writer.writerow(
                [
                    args.depth,
                    args.total_expansion,
                    args.s_identity,
                    args.z_parity,
                    total,
                    defect,
                    count,
                    first_defective if defect > 0 else "",
                ]
            )

    defective = sum(count for defect, count in by_defect.items() if defect > 0)
    print(
        f"depth={args.depth} s_identity={args.s_identity} z_parity={args.z_parity} "
        f"checked={total} cert_defective={defective} out={args.out}"
    )


if __name__ == "__main__":
    main()
