#!/usr/bin/env python3
"""Compute recursive split diagnostics for systematic RFC zero-set shapes."""

from __future__ import annotations

import argparse
import csv
import random
from collections import Counter, defaultdict
from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path

from sample_rfc_rank_failure import rank_selected_columns, systematic_generator_prime


@dataclass
class SplitStats:
    full_child_blocks: Counter[int]
    sibling_groups: Counter[int]
    dead_sibling_groups: Counter[int]
    one_live_sibling_groups: Counter[int]
    singleton_groups: Counter[int]
    zero_singleton_groups: Counter[int]

    @classmethod
    def empty(cls) -> "SplitStats":
        return cls(Counter(), Counter(), Counter(), Counter(), Counter(), Counter())

    def add(self, other: "SplitStats") -> None:
        self.full_child_blocks.update(other.full_child_blocks)
        self.sibling_groups.update(other.sibling_groups)
        self.dead_sibling_groups.update(other.dead_sibling_groups)
        self.one_live_sibling_groups.update(other.one_live_sibling_groups)
        self.singleton_groups.update(other.singleton_groups)
        self.zero_singleton_groups.update(other.zero_singleton_groups)


def parse_shape(text: str) -> list[int]:
    return [int(value) for value in text.split(":") if value != ""]


def decode_shape(columns: list[int], depth: int, parity_expansion: int) -> tuple[set[int], list[tuple[int, int]]]:
    k = 1 << depth
    systematic: set[int] = set()
    parity: list[tuple[int, int]] = []
    for column in columns:
        if column < k:
            systematic.add(column)
            continue
        local = column - k
        parity.append((local % parity_expansion, local // parity_expansion))
    return systematic, parity


def profile_node(systematic: set[int], parity: list[tuple[int, int]], depth: int) -> SplitStats:
    stats = SplitStats.empty()
    if depth == 0:
        return stats

    child_size = 1 << (depth - 1)
    child_bit = child_size
    low_mask = child_size - 1

    systematic_left = {path & low_mask for path in systematic if (path & child_bit) == 0}
    systematic_right = {path & low_mask for path in systematic if (path & child_bit) != 0}
    left_full = len(systematic_left) == child_size
    right_full = len(systematic_right) == child_size

    child_level = depth - 1
    if left_full:
        stats.full_child_blocks[child_level] += 1
    if right_full:
        stats.full_child_blocks[child_level] += 1

    groups: dict[tuple[int, int], set[int]] = defaultdict(set)
    for copy, path in parity:
        groups[(copy, path & low_mask)].add(1 if (path & child_bit) else 0)

    left_parity: list[tuple[int, int]] = []
    right_parity: list[tuple[int, int]] = []
    for (copy, lower_path), sides in groups.items():
        if len(sides) == 2:
            stats.sibling_groups[depth] += 1
            left_parity.append((copy, lower_path))
            right_parity.append((copy, lower_path))
            if left_full ^ right_full:
                stats.dead_sibling_groups[depth] += 1
            if left_full or right_full:
                stats.one_live_sibling_groups[depth] += 1
            continue

        stats.singleton_groups[depth] += 1
        if left_full and right_full:
            stats.zero_singleton_groups[depth] += 1
        if not left_full:
            left_parity.append((copy, lower_path))
        if not right_full:
            right_parity.append((copy, lower_path))

    stats.add(profile_node(systematic_left, left_parity, depth - 1))
    stats.add(profile_node(systematic_right, right_parity, depth - 1))
    return stats


def live_rows(systematic: set[int], depth: int) -> int:
    return (1 << depth) - len(systematic)


def certified_rank(systematic: set[int], parity: list[tuple[int, int]], depth: int) -> int:
    """Recursive rank certificate from sibling splits and singleton orientations.

    This is not a rank computation over a sampled field. It is the constructive proof heuristic:
    sibling pairs are split by the determinant-1 local transform, while singleton columns are
    assigned to one live child as their leading-term obligation.
    """

    systematic_key = tuple(sorted(systematic))
    parity_key = tuple(sorted(set(parity)))
    return _certified_rank_cached(depth, systematic_key, parity_key)


@lru_cache(maxsize=None)
def _certified_rank_cached(depth: int, systematic_key: tuple[int, ...], parity_key: tuple[tuple[int, int], ...]) -> int:
    systematic = set(systematic_key)
    parity = list(parity_key)

    if (1 << depth) - len(systematic) == 0:
        return 0
    if depth == 0:
        return 1 if parity else 0

    child_size = 1 << (depth - 1)
    child_bit = child_size
    low_mask = child_size - 1

    systematic_left = {path & low_mask for path in systematic if (path & child_bit) == 0}
    systematic_right = {path & low_mask for path in systematic if (path & child_bit) != 0}
    left_live = len(systematic_left) < child_size
    right_live = len(systematic_right) < child_size

    groups: dict[tuple[int, int], set[int]] = defaultdict(set)
    for copy, path in parity:
        groups[(copy, path & low_mask)].add(1 if (path & child_bit) else 0)

    left_forced: list[tuple[int, int]] = []
    right_forced: list[tuple[int, int]] = []
    choices: list[tuple[int, int]] = []
    for copy_lower, sides in groups.items():
        if len(sides) == 2:
            if left_live:
                left_forced.append(copy_lower)
            if right_live:
                right_forced.append(copy_lower)
            continue
        if left_live and right_live:
            choices.append(copy_lower)
        elif left_live:
            left_forced.append(copy_lower)
        elif right_live:
            right_forced.append(copy_lower)

    best = 0
    choice_count = len(choices)
    for mask in range(1 << choice_count):
        left_parity = left_forced[:]
        right_parity = right_forced[:]
        for index, column in enumerate(choices):
            if (mask >> index) & 1:
                right_parity.append(column)
            else:
                left_parity.append(column)
        rank = _certified_rank_cached(depth - 1, tuple(sorted(systematic_left)), tuple(sorted(set(left_parity)))) + (
            _certified_rank_cached(depth - 1, tuple(sorted(systematic_right)), tuple(sorted(set(right_parity))))
        )
        if rank > best:
            best = rank
            if best == (1 << depth) - len(systematic):
                break
    return best


def stats_key(stats: SplitStats, depth: int) -> tuple[int, ...]:
    values: list[int] = []
    for level in range(depth):
        values.append(stats.full_child_blocks[level])
    for level in range(1, depth + 1):
        values.append(stats.sibling_groups[level])
    for level in range(1, depth + 1):
        values.append(stats.dead_sibling_groups[level])
    for level in range(1, depth + 1):
        values.append(stats.one_live_sibling_groups[level])
    for level in range(1, depth + 1):
        values.append(stats.singleton_groups[level])
    for level in range(1, depth + 1):
        values.append(stats.zero_singleton_groups[level])
    return tuple(values)


def header(depth: int) -> list[str]:
    names: list[str] = []
    names += [f"full_child_blocks_level_{level}" for level in range(depth)]
    names += [f"sibling_groups_level_{level}" for level in range(1, depth + 1)]
    names += [f"dead_sibling_groups_level_{level}" for level in range(1, depth + 1)]
    names += [f"one_live_sibling_groups_level_{level}" for level in range(1, depth + 1)]
    names += [f"singleton_groups_level_{level}" for level in range(1, depth + 1)]
    names += [f"zero_singleton_groups_level_{level}" for level in range(1, depth + 1)]
    return names


def load_shapes(args: argparse.Namespace) -> list[tuple[str, list[int], bool | None]]:
    shapes: list[tuple[str, list[int], bool | None]] = []
    for raw in args.shape:
        name, columns_text = raw.split("=", 1)
        shapes.append((name, parse_shape(columns_text), None))
    if args.shapes_csv is None:
        return shapes

    with Path(args.shapes_csv).open(newline="") as handle:
        for index, row in enumerate(csv.DictReader(handle)):
            if args.max_rows is not None and index >= args.max_rows:
                break
            columns_text = row.get(args.columns_field, "")
            if columns_text == "":
                if args.skip_empty_columns:
                    continue
                raise SystemExit(f"row {index} has empty column field {args.columns_field!r}")
            columns = parse_shape(columns_text)
            bad: bool | None = None
            if "bad_count" in row:
                bad = int(row["bad_count"]) > 0
            shapes.append((row.get("index", str(index)), columns, bad))
    return shapes


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--parity-expansion", type=int, default=7)
    parser.add_argument("--shape", action="append", default=[], help="name=colon-separated full columns")
    parser.add_argument("--shapes-csv", default=None)
    parser.add_argument("--columns-field", default="columns")
    parser.add_argument("--skip-empty-columns", action="store_true")
    parser.add_argument("--out", required=True)
    parser.add_argument("--summary-out", default=None)
    parser.add_argument("--max-rows", type=int, default=None)
    parser.add_argument("--total-expansion", type=int, default=8)
    parser.add_argument("--prime", type=int, default=65537)
    parser.add_argument("--seed-start", type=int, default=1)
    parser.add_argument("--rank-samples", type=int, default=0)
    args = parser.parse_args()

    shapes = load_shapes(args)
    feature_header = header(args.depth)
    summary: Counter[tuple[tuple[int, ...], bool | None]] = Counter()
    generators = []
    if args.rank_samples > 0:
        generators = [
            systematic_generator_prime(args.depth, args.total_expansion, args.prime, random.Random(seed))
            for seed in range(args.seed_start, args.seed_start + args.rank_samples)
        ]

    with Path(args.out).open("w", newline="") as handle:
        writer = csv.writer(handle)
        rank_headers = ["deficient_samples"] if args.rank_samples > 0 else []
        certificate_headers = ["live_rows", "certified_rank", "certified_defect", "certified_full"]
        writer.writerow(["shape", "columns", "is_bad"] + rank_headers + certificate_headers + feature_header)
        for name, columns, is_bad in shapes:
            systematic, parity = decode_shape(columns, args.depth, args.parity_expansion)
            stats = profile_node(systematic, parity, args.depth)
            key = stats_key(stats, args.depth)
            live = live_rows(systematic, args.depth)
            cert_rank = certified_rank(systematic, parity, args.depth)
            cert_defect = live - cert_rank
            cert_key = (live, cert_rank, cert_defect, int(cert_defect == 0))
            summary_key = cert_key + key
            deficient_samples = None
            if generators:
                k = 1 << args.depth
                deficient_samples = sum(
                    1 for generator in generators if rank_selected_columns(generator, columns, args.prime) < k
                )
                summary[(summary_key, deficient_samples == args.rank_samples)] += 1
            else:
                summary[(summary_key, is_bad)] += 1
            writer.writerow(
                [
                    name,
                    ":".join(str(column) for column in columns),
                    "" if is_bad is None else int(is_bad),
                    *([] if deficient_samples is None else [deficient_samples]),
                    live,
                    cert_rank,
                    cert_defect,
                    int(cert_defect == 0),
                    *key,
                ]
            )

    if args.summary_out is not None:
        by_key: dict[tuple[int, ...], Counter[bool | None]] = defaultdict(Counter)
        for (key, is_bad), count in summary.items():
            by_key[key][is_bad] += count
        with Path(args.summary_out).open("w", newline="") as handle:
            writer = csv.writer(handle)
            writer.writerow(certificate_headers + feature_header + ["unknown_count", "good_count", "bad_count", "total_count"])
            for key, counts in sorted(by_key.items(), key=lambda item: (-sum(item[1].values()), item[0])):
                unknown = counts[None]
                good = counts[False]
                bad = counts[True]
                writer.writerow([*key, unknown, good, bad, unknown + good + bad])

    print(f"shapes={len(shapes)} out={args.out}")


if __name__ == "__main__":
    main()
