#!/usr/bin/env python3
"""Certificate checks for original non-systematic RFC column subsets."""

from __future__ import annotations

import argparse
import csv
import itertools
import random
from collections import Counter
from pathlib import Path

from rfc_split_profile import certificate_full, certified_rank, live_rows


def decode_original(columns: list[int], expansion: int) -> list[tuple[int, int]]:
    return [(column % expansion, column // expansion) for column in columns]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--subset-size", type=int, default=None)
    parser.add_argument("--samples", type=int, default=0)
    parser.add_argument("--seed", type=int, default=1)
    parser.add_argument("--full-only", action="store_true")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    n = args.expansion * k
    subset_size = k if args.subset_size is None else args.subset_size
    rng = random.Random(args.seed)

    if args.samples > 0:
        subset_iter = (tuple(sorted(rng.sample(range(n), subset_size))) for _ in range(args.samples))
        checked_target = args.samples
    else:
        subset_iter = itertools.combinations(range(n), subset_size)
        checked_target = None

    by_defect: Counter[int] = Counter()
    first_defective = ""
    checked = 0
    for columns in subset_iter:
        parity = decode_original(list(columns), args.expansion)
        if args.full_only:
            defect = 0 if certificate_full(set(), parity, args.depth) else 1
        else:
            defect = live_rows(set(), args.depth) - certified_rank(set(), parity, args.depth)
        by_defect[defect] += 1
        checked += 1
        if defect > 0 and first_defective == "":
            first_defective = ":".join(str(column) for column in columns)

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "expansion",
                "k",
                "n",
                "subset_size",
                "mode",
                "checked_subsets",
                "cert_defect",
                "subset_count",
                "first_defective",
            ]
        )
        mode = f"sample:{checked_target}" if args.samples > 0 else "exact"
        if args.full_only:
            mode += ":full_only"
        for defect, count in sorted(by_defect.items()):
            writer.writerow(
                [
                    args.depth,
                    args.expansion,
                    k,
                    n,
                    subset_size,
                    mode,
                    checked,
                    defect,
                    count,
                    first_defective if defect > 0 else "",
                ]
            )

    defective = sum(count for defect, count in by_defect.items() if defect > 0)
    print(
        f"depth={args.depth} expansion={args.expansion} subset_size={subset_size} "
        f"checked={checked} cert_defective={defective} out={args.out}"
    )


if __name__ == "__main__":
    main()
