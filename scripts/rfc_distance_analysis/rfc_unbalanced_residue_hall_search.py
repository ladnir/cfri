#!/usr/bin/env python3
"""Search deterministic residue-Hall models for unbalanced RFC splits.

This is a deliberately structure-light adversarial model.  At a parent node of length K and row
weight M=a+b, it chooses arbitrary child coordinate supports U,V with sizes at least the child
uncertainty lower bounds and asks whether every parent residue modulo M has a missing projected
child coordinate.  If such examples exist, a core-preservation proof cannot rely on support sizes
alone.
"""

from __future__ import annotations

import argparse
import itertools
from collections.abc import Iterable


def parent_projection_child_positions(k: int, m: int, rho: int) -> tuple[int, ...]:
    half = k // 2
    positions: list[int] = []
    value = rho
    while value < k:
        positions.append(value if value < half else value - half)
        value += m
    return tuple(positions)


def residue_holes(k: int, m: int, covered: set[int]) -> list[int]:
    holes: list[int] = []
    for rho in range(m):
        holes.append(sum(1 for pos in parent_projection_child_positions(k, m, rho) if pos not in covered))
    return holes


def residue_model_holes(k: int, m: int, a: int, b: int, ra: int, rb: int) -> list[int]:
    """Hole counts when children are optimistic residue classes modulo a and b."""
    half = k // 2
    holes: list[int] = []
    for rho in range(m):
        hole_count = 0
        for pos in parent_projection_child_positions(k, m, rho):
            if (pos - ra) % a != 0 and (pos - rb) % b != 0:
                hole_count += 1
        holes.append(hole_count)
    return holes


def scan_residue_model(k: int, max_m: int) -> None:
    print("K,M,a,b,split_charge,L,child_period,pairs,zero_pairs,min_holes,worst_min_holes,histogram")
    half = k // 2
    for m in range(2, max_m + 1):
        if k % m != 0 or half % m != 0:
            continue
        best: tuple[int, int, int, int] | None = None
        for a in range(1, m):
            b = m - a
            if a == b:
                continue
            split = max((half + a - 1) // a, (half + b - 1) // b) - ((k + m - 1) // m)
            candidate = (split, abs(a - b), a, b)
            if best is None or candidate < best:
                best = candidate
        if best is None:
            continue
        split, _, a, b = best
        hist: dict[int, int] = {}
        zero_pairs = 0
        worst_min = 0
        global_min = 1 << 30
        for ra in range(a):
            for rb in range(b):
                best_for_pair = min(residue_model_holes(k, m, a, b, ra, rb))
                hist[best_for_pair] = hist.get(best_for_pair, 0) + 1
                zero_pairs += int(best_for_pair == 0)
                worst_min = max(worst_min, best_for_pair)
                global_min = min(global_min, best_for_pair)
        histogram = ";".join(f"{key}:{hist[key]}" for key in sorted(hist))
        print(
            f"{k},{m},{a},{b},{split},{k//m},{half//m},{a*b},{zero_pairs},"
            f"{global_min},{worst_min},{histogram}"
        )


def first_counterexample(
    k: int,
    m: int,
    a: int,
    b: int,
    extra: int,
    separated: bool,
) -> tuple[tuple[int, ...], tuple[int, ...], list[int]] | None:
    half = k // 2
    p = (half + a - 1) // a
    q = (half + b - 1) // b
    p += extra
    q += extra
    universe = range(half)
    for u_tuple in itertools.combinations(universe, p):
        u_set = set(u_tuple)
        v_iter: Iterable[tuple[int, ...]]
        if separated:
            remaining = [x for x in universe if x not in u_set]
            if len(remaining) < q:
                continue
            v_iter = itertools.combinations(remaining, q)
        else:
            v_iter = itertools.combinations(universe, q)
        for v_tuple in v_iter:
            covered = u_set | set(v_tuple)
            holes = residue_holes(k, m, covered)
            if min(holes) > 0:
                return u_tuple, v_tuple, holes
    return None


def constructive_size_counterexample(
    k: int,
    m: int,
    a: int,
    b: int,
    extra: int,
    separated: bool,
) -> tuple[tuple[int, ...], tuple[int, ...], list[int]] | None:
    """Fast arbitrary-support counterexample.

    When M divides K/2, parent residue projections are exactly the child residue classes modulo M.
    A set S kills every parent residue iff it omits at least one point from each class.  The largest
    such S has size K/2-M.  Therefore arbitrary child supports can kill every parent residue whenever
    their required union can fit inside such an S.
    """
    half = k // 2
    if half % m != 0:
        return None
    p = (half + a - 1) // a + extra
    q = (half + b - 1) // b + extra
    required_union = p + q if separated else max(p, q)
    if required_union > half - m:
        return None

    s: list[int] = []
    per_class = half // m
    for rho in range(m):
        # Leave the last element of each class uncovered.
        for t in range(per_class - 1):
            s.append(rho + t * m)
    s = s[:required_union]
    if separated:
        u = tuple(s[:p])
        v = tuple(s[p : p + q])
    else:
        u = tuple(s[:p])
        v = tuple(s[:q])
    holes = residue_holes(k, m, set(u) | set(v))
    return u, v, holes


def scan(k: int, max_m: int, max_extra: int, separated: bool) -> None:
    print("K,M,a,b,L,p,q,extra,min_hole,max_hole,counterexample")
    for m in range(2, max_m + 1):
        if k % m != 0:
            continue
        half = k // 2
        for a in range(1, m):
            b = m - a
            if a == b:
                continue
            p0 = (half + a - 1) // a
            q0 = (half + b - 1) // b
            for extra in range(max_extra + 1):
                # Keep the brute-force model small enough to be an interactive proof aide.
                if p0 + extra > half or q0 + extra > half:
                    continue
                found = constructive_size_counterexample(k, m, a, b, extra, separated)
                if found is None and k <= 32:
                    found = first_counterexample(k, m, a, b, extra, separated)
                if found is None:
                    print(f"{k},{m},{a},{b},{k//m},{p0},{q0},{extra},0,0,no")
                else:
                    u, v, holes = found
                    print(
                        f"{k},{m},{a},{b},{k//m},{p0},{q0},{extra},"
                        f"{min(holes)},{max(holes)},yes"
                    )
                    print(f"  U={':'.join(map(str, u))}")
                    print(f"  V={':'.join(map(str, v))}")
                    print(f"  holes={':'.join(map(str, holes))}")
                break


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--k", type=int, required=True)
    parser.add_argument("--max-m", type=int, required=True)
    parser.add_argument("--max-extra", type=int, default=0)
    parser.add_argument(
        "--child-residue-model",
        action="store_true",
        help="scan the optimistic model where child supports are residue classes modulo a and b",
    )
    parser.add_argument(
        "--separated",
        action="store_true",
        help="require U and V to be disjoint, a stronger adversarial restriction",
    )
    args = parser.parse_args()
    if args.child_residue_model:
        scan_residue_model(args.k, args.max_m)
        return
    scan(args.k, args.max_m, args.max_extra, args.separated)


if __name__ == "__main__":
    main()
