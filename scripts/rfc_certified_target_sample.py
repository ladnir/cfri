#!/usr/bin/env python3
"""Sample recursive-certificate defects at a fixed total zero-set size."""

from __future__ import annotations

import argparse
import csv
import random
from pathlib import Path

from rfc_split_profile import certificate_full, certified_rank, decode_shape, live_rows


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--zero-count", type=int, required=True)
    parser.add_argument("--samples", type=int, default=20000)
    parser.add_argument("--seed", type=int, default=17)
    parser.add_argument("--full-only", action="store_true")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    rng = random.Random(args.seed)
    k = 1 << args.depth
    parity_expansion = args.total_expansion - 1
    parity_n = parity_expansion * k

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "depth",
                "total_expansion",
                "zero_count",
                "s_identity",
                "z_parity",
                "samples",
                "cert_defective",
                "first_defective",
            ]
        )
        for s_identity in range(min(k, args.zero_count) + 1):
            z_parity = args.zero_count - s_identity
            if z_parity < 0 or z_parity > parity_n:
                continue
            bad = 0
            first_defective = ""
            for _ in range(args.samples):
                identities = sorted(rng.sample(range(k), s_identity))
                parities = sorted(rng.sample(range(parity_n), z_parity))
                columns = identities + [k + column for column in parities]
                systematic, parity = decode_shape(columns, args.depth, parity_expansion)
                if args.full_only:
                    defect = 0 if certificate_full(systematic, parity, args.depth) else 1
                else:
                    defect = live_rows(systematic, args.depth) - certified_rank(systematic, parity, args.depth)
                if defect > 0:
                    bad += 1
                    if first_defective == "":
                        first_defective = ":".join(str(column) for column in columns)
            writer.writerow(
                [
                    args.depth,
                    args.total_expansion,
                    args.zero_count,
                    s_identity,
                    z_parity,
                    args.samples,
                    bad,
                    first_defective,
                ]
            )

    print(f"depth={args.depth} zero_count={args.zero_count} samples={args.samples} out={args.out}")


if __name__ == "__main__":
    main()
