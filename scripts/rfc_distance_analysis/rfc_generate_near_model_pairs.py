#!/usr/bin/env python3
"""Generate matched-core plus extra output support pairs."""

from __future__ import annotations

import argparse
import csv
import itertools
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--live-rows", type=int, required=True)
    parser.add_argument("--extra-outputs", type=int, required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    k = 1 << args.depth
    m = args.live_rows
    if m <= 0 or k % m != 0:
        raise SystemExit("--live-rows must divide k")
    if args.extra_outputs < 0 or args.extra_outputs > k - k // m:
        raise SystemExit("--extra-outputs outside valid range")

    count = 0
    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "rows",
                "outputs",
                "rows_subcube",
                "outputs_subcube",
                "matched_block_stride",
                "contains_matched_core",
            ]
        )
        for block_start in range(0, k, m):
            rows = tuple(range(block_start, block_start + m))
            for residue in range(m):
                core = tuple(range(residue, k, m))
                core_set = set(core)
                extras_universe = [value for value in range(k) if value not in core_set]
                for extras in itertools.combinations(extras_universe, args.extra_outputs):
                    outputs = tuple(sorted(core + extras))
                    writer.writerow(
                        [
                            ":".join(str(value) for value in rows),
                            ":".join(str(value) for value in outputs),
                            1,
                            int(args.extra_outputs == 0),
                            int(args.extra_outputs == 0),
                            1,
                        ]
                    )
                    count += 1

    print(f"pairs={count} out={args.out}")


if __name__ == "__main__":
    main()
