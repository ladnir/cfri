#!/usr/bin/env python3
"""D=3 root-line geometry leakage self-test.

This is a falsification probe for the fixed-survivor rank-tail route. It asks whether exact
root-line repair probabilities in K_P of dimension 3 are already sensitive to projective geometry
beyond a coarse rank/matroid signature of the singleton functionals.
"""

from __future__ import annotations

import argparse
import itertools
import random
import sys
from collections import defaultdict
from fractions import Fraction
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_brute_force_moment import challenge_domain  # noqa: E402
from rfc_sufficiency_test import mat_rank_modq  # noqa: E402


def projective_points_pg2(q: int) -> list[tuple[int, int, int]]:
    points: set[tuple[int, int, int]] = set()
    for vec in itertools.product(range(q), repeat=3):
        if vec == (0, 0, 0):
            continue
        first = next(i for i, x in enumerate(vec) if x % q)
        inv = pow(vec[first] % q, q - 2, q)
        points.add(tuple((x * inv) % q for x in vec))
    return sorted(points)


def parse_sides(text: str, rows: int) -> tuple[int, ...]:
    if text == "left":
        return (0,) * rows
    if text == "right":
        return (1,) * rows
    if text == "alternating":
        return tuple(i & 1 for i in range(rows))
    vals = []
    for ch in text:
        if ch in "Ll0":
            vals.append(0)
        elif ch in "Rr1":
            vals.append(1)
    if len(vals) != rows:
        raise SystemExit(f"side pattern has {len(vals)} entries, expected {rows}")
    return tuple(vals)


def exact_failure_for_dirs(q: int, dirs: tuple[tuple[int, int, int], ...],
                           sides: tuple[int, ...], nonzero_t: bool) -> Fraction:
    t_domain = challenge_domain(q, nonzero_t)
    fail = 0
    total = 0
    for roots in itertools.product(t_domain, repeat=len(dirs)):
        rows = []
        for ell, side, t_val in zip(dirs, sides, roots):
            alpha = t_val if side == 0 else (t_val + 1) % q
            rows.append([
                ell[0], ell[1], ell[2],
                (alpha * ell[0]) % q, (alpha * ell[1]) % q, (alpha * ell[2]) % q,
            ])
        if mat_rank_modq(rows, q) < 6:
            fail += 1
        total += 1
    return Fraction(fail, total)


def rank_histogram_signature(q: int, dirs: tuple[tuple[int, int, int], ...]) -> str:
    counts: dict[tuple[int, int], int] = defaultdict(int)
    m = len(dirs)
    for size in range(1, m + 1):
        for subset in itertools.combinations(range(m), size):
            rank = mat_rank_modq([list(dirs[i]) for i in subset], q)
            counts[(size, rank)] += 1
    return ";".join(f"{size}:{rank}:{counts[(size, rank)]}" for size, rank in sorted(counts))


def subset_rank_table(q: int, dirs: tuple[tuple[int, int, int], ...]) -> list[int]:
    m = len(dirs)
    ranks = [0] * (1 << m)
    for mask in range(1, 1 << m):
        rows = [list(dirs[i]) for i in range(m) if (mask >> i) & 1]
        ranks[mask] = mat_rank_modq(rows, q)
    return ranks


def canonical_matroid_signature(q: int, dirs: tuple[tuple[int, int, int], ...],
                                sides: tuple[int, ...] | None = None) -> str:
    """Canonical rank-table signature under row relabeling.

    This is intended for tiny D=3 probes only.  For six rows it checks 720 permutations, which is
    acceptable for small sampled runs and gives a much sharper test than the rank histogram.
    """
    m = len(dirs)
    rank_by_mask = subset_rank_table(q, dirs)
    masks = []
    for size in range(1, m + 1):
        masks.extend(mask for mask in range(1, 1 << m) if mask.bit_count() == size)
    best: tuple[int, ...] | None = None
    for perm in itertools.permutations(range(m)):
        ranks = [sides[i] for i in perm] if sides is not None else []
        for mask in masks:
            old_mask = 0
            for new_idx, old_idx in enumerate(perm):
                if (mask >> new_idx) & 1:
                    old_mask |= 1 << old_idx
            ranks.append(rank_by_mask[old_mask])
        key = tuple(ranks)
        if best is None or key < best:
            best = key
    return ",".join(str(x) for x in best)


def iter_configs(points: list[tuple[int, int, int]], rows: int, max_configs: int | None,
                 seed: int) -> list[tuple[tuple[int, int, int], ...]]:
    total = 1
    for i in range(rows):
        total = total * (len(points) - i) // (i + 1)
    if max_configs is None or max_configs >= total:
        return list(itertools.combinations(points, rows))
    rng = random.Random(seed)
    seen: set[tuple[int, ...]] = set()
    out = []
    while len(out) < max_configs:
        idxs = tuple(sorted(rng.sample(range(len(points)), rows)))
        if idxs in seen:
            continue
        seen.add(idxs)
        out.append(tuple(points[i] for i in idxs))
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--q", type=int, default=3)
    ap.add_argument("--rows", type=int, default=6)
    ap.add_argument("--t-domain", choices=["nonzero", "all"], default="nonzero")
    ap.add_argument("--sides", default="left",
                    help="left, right, alternating, or an L/R string such as LLLRRR")
    ap.add_argument("--max-configs", type=int, default=None)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--signature", choices=["rank-histogram", "canonical-matroid", "colored-matroid"],
                    default="rank-histogram")
    ap.add_argument("--print-limit", type=int, default=20)
    args = ap.parse_args()

    q = args.q
    nonzero_t = args.t_domain == "nonzero"
    sides = parse_sides(args.sides, args.rows)
    points = projective_points_pg2(q)
    configs = iter_configs(points, args.rows, args.max_configs, args.seed)

    group_values: dict[str, set[Fraction]] = defaultdict(set)
    group_sizes: dict[str, int] = defaultdict(int)
    for dirs in configs:
        if args.signature == "canonical-matroid":
            sig = canonical_matroid_signature(q, dirs)
        elif args.signature == "colored-matroid":
            sig = canonical_matroid_signature(q, dirs, sides)
        else:
            sig = rank_histogram_signature(q, dirs)
        value = exact_failure_for_dirs(q, dirs, sides, nonzero_t)
        group_values[sig].add(value)
        group_sizes[sig] += 1

    ambiguous = {sig: vals for sig, vals in group_values.items() if len(vals) > 1}
    print(
        f"# d3 root-line geometry self-test q={q} rows={args.rows} "
        f"t_domain={args.t_domain} sides={args.sides} signature={args.signature}"
    )
    print(f"# configs={len(configs)} groups={len(group_values)} ambiguous_groups={len(ambiguous)}")
    print("group,configs,distinct_values,example_values")
    for idx, (sig, vals) in enumerate(sorted(group_values.items(), key=lambda kv: (-len(kv[1]), kv[0]))):
        if idx >= args.print_limit:
            break
        value_text = ";".join(str(v) for v in sorted(vals))
        print(f"\"{sig}\",{group_sizes[sig]},{len(vals)},\"{value_text}\"")


if __name__ == "__main__":
    main()
