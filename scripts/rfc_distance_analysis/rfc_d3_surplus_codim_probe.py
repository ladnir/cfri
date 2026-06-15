#!/usr/bin/env python3
"""D=3 surplus-codimension probe for root-line repair.

This checks whether Hall-OK D=3 root-line repair failures scale like q^{-(t-5)} rather than merely
paying one q-factor.  It is a diagnostic, not a certificate.
"""

from __future__ import annotations

import argparse
import itertools
import math
import random
import sys
from fractions import Fraction
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_brute_force_moment import challenge_domain  # noqa: E402
from rfc_d3_rootline_geometry_selftest import parse_sides, projective_points_pg2  # noqa: E402
from rfc_sufficiency_test import mat_rank_modq  # noqa: E402


def subset_rank_table(q: int, dirs: tuple[tuple[int, int, int], ...]) -> list[int]:
    m = len(dirs)
    ranks = [0] * (1 << m)
    for mask in range(1, 1 << m):
        rows = [list(dirs[i]) for i in range(m) if (mask >> i) & 1]
        ranks[mask] = mat_rank_modq(rows, q)
    return ranks


def hall_min_from_ranks(ranks: list[int], t: int) -> int:
    return min(t - mask.bit_count() + 2 * ranks[mask] for mask in range(1 << t))


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


def iter_sampled_configs(points: list[tuple[int, int, int]], rows: int, max_configs: int,
                         seed: int):
    rng = random.Random(seed)
    seen: set[tuple[int, ...]] = set()
    while len(seen) < max_configs:
        idxs = tuple(sorted(rng.sample(range(len(points)), rows)))
        if idxs in seen:
            continue
        seen.add(idxs)
        yield tuple(points[i] for i in idxs)


def log_q_fraction(value: Fraction, q: int) -> float:
    if value == 0:
        return math.inf
    return -(math.log(value.numerator) - math.log(value.denominator)) / math.log(q)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--q", type=int, default=5)
    ap.add_argument("--rows", type=int, default=7)
    ap.add_argument("--sides", default="alternating")
    ap.add_argument("--t-domain", choices=["nonzero", "all"], default="nonzero")
    ap.add_argument("--max-configs", type=int, default=40)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--print-limit", type=int, default=20)
    args = ap.parse_args()

    q = args.q
    t = args.rows
    sides = parse_sides(args.sides, t)
    points = projective_points_pg2(q)
    nonzero_t = args.t_domain == "nonzero"
    target_codim = max(0, t - 5)

    checked = hall_ok = 0
    worst_ratio = Fraction(0)
    best_codim = math.inf
    rows_out = []
    for dirs in iter_sampled_configs(points, t, args.max_configs, args.seed):
        checked += 1
        ranks = subset_rank_table(q, dirs)
        full_rank = ranks[(1 << t) - 1]
        hall_min = hall_min_from_ranks(ranks, t)
        if full_rank < 3 or hall_min < 6:
            continue
        hall_ok += 1
        fail = exact_failure_for_dirs(q, dirs, sides, nonzero_t)
        denom = q ** target_codim
        ratio = fail * denom
        codim = log_q_fraction(fail, q)
        if ratio > worst_ratio:
            worst_ratio = ratio
        if codim < best_codim:
            best_codim = codim
        if len(rows_out) < args.print_limit:
            rows_out.append((fail, ratio, codim, hall_min))

    print(
        "q,rows,sides,t_domain,target_codim,checked,hall_ok,"
        "worst_ratio_to_q_minus_target,best_observed_codim"
    )
    print(
        f"{q},{t},{args.sides},{args.t_domain},{target_codim},{checked},{hall_ok},"
        f"{worst_ratio},{best_codim:.6f}"
    )
    print("failure_probability,ratio_to_q_minus_target,observed_codim,hall_min")
    for fail, ratio, codim, hall_min in rows_out:
        print(f"{fail},{ratio},{codim:.6f},{hall_min}")


if __name__ == "__main__":
    main()
