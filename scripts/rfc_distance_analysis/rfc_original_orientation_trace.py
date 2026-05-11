#!/usr/bin/env python3
"""Trace the inductive singleton orientation theorem for original RFC subsets."""

from __future__ import annotations

import argparse
import random
from collections import defaultdict

from rfc_original_certified_defect import decode_original


def orientation_ok(parity: list[tuple[int, int]], depth: int) -> bool:
    if depth == 0:
        return len(set(parity)) == 1

    child_size = 1 << (depth - 1)
    child_bit = child_size
    low_mask = child_size - 1

    groups: dict[tuple[int, int], set[int]] = defaultdict(set)
    for copy, path in parity:
        groups[(copy, path & low_mask)].add(1 if (path & child_bit) else 0)

    paired: list[tuple[int, int]] = []
    singletons: list[tuple[int, int]] = []
    for key, sides in groups.items():
        if len(sides) == 2:
            paired.append(key)
        else:
            singletons.append(key)

    need_left = child_size - len(paired)
    if need_left < 0 or need_left > len(singletons):
        return False
    if len(singletons) - need_left != child_size - len(paired):
        return False

    left = paired + singletons[:need_left]
    right = paired + singletons[need_left:]
    if len(set(left)) != child_size or len(set(right)) != child_size:
        return False
    return orientation_ok(left, depth - 1) and orientation_ok(right, depth - 1)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--samples", type=int, default=1000)
    parser.add_argument("--seed", type=int, default=1)
    args = parser.parse_args()

    rng = random.Random(args.seed)
    k = 1 << args.depth
    n = args.expansion * k
    failures = 0
    first_failure = ""
    for _ in range(args.samples):
        columns = sorted(rng.sample(range(n), k))
        if not orientation_ok(decode_original(columns, args.expansion), args.depth):
            failures += 1
            if first_failure == "":
                first_failure = ":".join(str(column) for column in columns)

    print(
        f"depth={args.depth} expansion={args.expansion} samples={args.samples} "
        f"failures={failures} first_failure={first_failure}"
    )


if __name__ == "__main__":
    main()
