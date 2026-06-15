#!/usr/bin/env python3
"""Self-test for the D=2 root-line parallel-class formula.

This checks the possible cross-ratio leak directly, outside RFC.  For each requested parallel-class
side-count signature, it compares the canonical formula against every choice of distinct projective
directions in P^1(F_q).  If any direction set gives a different exact failure probability, the
parallel-class abstraction is too coarse.
"""

from __future__ import annotations

import argparse
import itertools
import sys
from fractions import Fraction
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_brute_force_moment import challenge_domain  # noqa: E402
from rfc_fixed_survivor_rank_tail import (  # noqa: E402
    parallel_class_envelope_failure_d2,
    parallel_class_formula_failure_d2,
)
from rfc_sufficiency_test import mat_rank_modq  # noqa: E402


def parse_signature(text: str) -> tuple[tuple[int, int], ...]:
    """Parse e.g. '1:0/1:0/0:1' into class side counts."""
    out = []
    for part in text.split("/"):
        part = part.strip()
        if not part:
            continue
        left_s, right_s = part.split(":", 1)
        out.append((int(left_s), int(right_s)))
    if not out:
        raise SystemExit("empty signature")
    return tuple(sorted(out))


def integer_compositions(n: int, k: int):
    if k == 1:
        yield (n,)
        return
    for first in range(1, n - k + 2):
        for rest in integer_compositions(n - first, k - 1):
            yield (first,) + rest


def side_splits(size: int) -> list[tuple[int, int]]:
    return [(left, size - left) for left in range(size + 1)]


def exhaustive_signatures(max_total: int, max_classes: int) -> list[tuple[tuple[int, int], ...]]:
    seen: set[tuple[tuple[int, int], ...]] = set()
    out: list[tuple[tuple[int, int], ...]] = []
    for total in range(1, max_total + 1):
        for classes in range(1, min(total, max_classes) + 1):
            for sizes in integer_compositions(total, classes):
                for counts in itertools.product(*(side_splits(size) for size in sizes)):
                    sig = tuple(sorted(counts))
                    if sig in seen:
                        continue
                    seen.add(sig)
                    out.append(sig)
    return out


def exact_failure_for_directions(q: int, nonzero_t: bool, dirs: tuple[tuple[int, int], ...],
                                 counts: tuple[tuple[int, int], ...]) -> Fraction:
    rows_by_var: list[tuple[tuple[int, int], int]] = []
    for ell, (left_count, right_count) in zip(dirs, counts):
        rows_by_var.extend((ell, 0) for _ in range(left_count))
        rows_by_var.extend((ell, 1) for _ in range(right_count))
    t_domain = challenge_domain(q, nonzero_t)
    fail = 0
    total = 0
    for roots in itertools.product(t_domain, repeat=len(rows_by_var)):
        rows = []
        for (ell, side), t_val in zip(rows_by_var, roots):
            alpha = t_val if side == 0 else (t_val + 1) % q
            rows.append([ell[0], ell[1], (alpha * ell[0]) % q, (alpha * ell[1]) % q])
        if mat_rank_modq(rows, q) < 4:
            fail += 1
        total += 1
    return Fraction(fail, total)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--q", type=int, default=5)
    ap.add_argument("--t-domain", choices=["nonzero", "all"], default="nonzero")
    ap.add_argument("--signature", action="append",
                    help="parallel class side counts, e.g. 1:0/1:0/0:1")
    ap.add_argument("--exhaustive-total", type=int, default=None,
                    help="also test every side-count signature with at most this many singleton rows")
    ap.add_argument("--max-direction-sets", type=int, default=None)
    args = ap.parse_args()

    q = args.q
    nonzero_t = args.t_domain == "nonzero"
    projective_dirs = [(1, a) for a in range(q)] + [(0, 1)]
    signatures = [parse_signature(sig) for sig in (args.signature or [])]
    if args.exhaustive_total is not None:
        signatures.extend(exhaustive_signatures(args.exhaustive_total, q + 1))
    if not signatures:
        raise SystemExit("provide --signature or --exhaustive-total")

    print(f"# d2 root-line formula self-test q={q} t_domain={args.t_domain}")
    print("signature,classes,direction_sets,canonical,envelope,distinct_values,verdict")
    failures = 0
    for counts in dict.fromkeys(signatures):
        sig_text = "/".join(f"{left}:{right}" for left, right in counts)
        canonical = parallel_class_formula_failure_d2((0, counts), q, nonzero_t)
        envelope = parallel_class_envelope_failure_d2((0, counts), q, nonzero_t)
        values: set[Fraction] = set()
        checked = 0
        for dirs in itertools.combinations(projective_dirs, len(counts)):
            values.add(exact_failure_for_directions(q, nonzero_t, dirs, counts))
            checked += 1
            if args.max_direction_sets is not None and checked >= args.max_direction_sets:
                break
        formula_ok = values == {canonical}
        bound_ok = all(value <= envelope for value in values)
        if formula_ok and bound_ok:
            verdict = "OK"
        elif bound_ok:
            verdict = "FORMULA_MISMATCH"
        else:
            verdict = "BOUND_FAIL"
        failures += 1 if verdict != "OK" else 0
        value_text = ";".join(str(v) for v in sorted(values))
        print(f"\"{sig_text}\",{len(counts)},{checked},{canonical},{envelope},\"{value_text}\",{verdict}")
    print(f"# summary signatures={len(dict.fromkeys(signatures))} failures={failures}")


if __name__ == "__main__":
    main()
