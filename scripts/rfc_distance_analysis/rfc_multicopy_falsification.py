#!/usr/bin/env python3
"""Multi-copy stress model for original non-systematic RFC distance.

This is a falsification diagnostic, not a certificate.  It extends the one-copy
near-extremizer stress model by asking whether the same projective message line
can lie in sparse-family subspaces for several independent RFC copies.

The symmetric model is intentionally adversarial:

* choose r active copies among c;
* in each active copy choose the same one-copy support shape;
* count one-copy families either by the old matched-core/extra-support overcount
  or by exact support-set size;
* assign projective dimension D=floor(extra_support / core_support);
* estimate the common-line exponent by generic projective intersection:
      r*D - (r-1)*(k-1);
* ask the remaining c-r copies to supply the residual zeros randomly.

Positive output rows are warning signs.  They do not prove a counterexample.
For flag-state comparisons, use --support-count-model exact-size.
"""

from __future__ import annotations

import argparse
import csv
import math
import sys
from dataclasses import asdict, dataclass


NEG_INF = -math.inf


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


def fmt(value: float) -> str:
    if value == NEG_INF:
        return "-inf"
    return f"{value:.8f}"


@dataclass
class StressRow:
    support_count_model: str
    target_excess: int
    active_copies: int
    depth: int
    expansion: int
    k: int
    n: int
    target_zero_count: int
    live_rows: int
    core_support: int
    extra_support: int
    one_copy_support: int
    one_copy_projective_dim: int
    intersection_projective_exponent: int
    deterministic_zero_count: int
    needed_tail_zeros: int
    remaining_positions: int
    family_log2_count: float
    matched_core_family_log2_count: float
    exact_size_family_log2_count: float
    support_overlap_penalty_log2: float
    flag_recovery_needed_log2: float
    intersection_log2_count: float
    tail_log2: float
    modeled_log2_expected: float
    notes: str


def row_for_shape(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    target_excess: int,
    active_copies: int,
    live_rows: int,
    extra_support: int,
    support_count_model: str,
    recovery_target_bits: float,
) -> StressRow:
    k = 1 << depth
    n = expansion * k
    target_zero_count = k + target_excess
    core_support = k // live_rows
    one_copy_support = core_support + extra_support
    one_copy_dim = extra_support // core_support if core_support > 0 else 0
    intersection_exp = active_copies * one_copy_dim - (active_copies - 1) * (k - 1)
    deterministic_zero_count = active_copies * (k - one_copy_support)
    needed_tail = max(0, target_zero_count - deterministic_zero_count)
    remaining_positions = (expansion - active_copies) * k

    per_copy_matched_core = math.log2(k) + log2_comb(k - core_support, extra_support)
    per_copy_exact_size = log2_comb(k, one_copy_support)
    if support_count_model == "matched-core":
        per_copy_family = per_copy_matched_core
    elif support_count_model == "exact-size":
        per_copy_family = per_copy_exact_size
    else:
        raise ValueError(f"unknown support count model: {support_count_model}")
    copy_choice_log = log2_comb(expansion, active_copies)
    family_log = copy_choice_log + active_copies * per_copy_family
    matched_core_family_log = copy_choice_log + active_copies * per_copy_matched_core
    exact_size_family_log = copy_choice_log + active_copies * per_copy_exact_size
    support_overlap_penalty = matched_core_family_log - exact_size_family_log
    intersection_log = intersection_exp * q_log2
    tail_log = log2_comb(remaining_positions, needed_tail) - needed_tail * q_log2
    modeled = family_log + intersection_log + tail_log
    if tail_log == NEG_INF:
        modeled = NEG_INF

    return StressRow(
        support_count_model=support_count_model,
        target_excess=target_excess,
        active_copies=active_copies,
        depth=depth,
        expansion=expansion,
        k=k,
        n=n,
        target_zero_count=target_zero_count,
        live_rows=live_rows,
        core_support=core_support,
        extra_support=extra_support,
        one_copy_support=one_copy_support,
        one_copy_projective_dim=one_copy_dim,
        intersection_projective_exponent=intersection_exp,
        deterministic_zero_count=deterministic_zero_count,
        needed_tail_zeros=needed_tail,
        remaining_positions=remaining_positions,
        family_log2_count=family_log,
        matched_core_family_log2_count=matched_core_family_log,
        exact_size_family_log2_count=exact_size_family_log,
        support_overlap_penalty_log2=support_overlap_penalty,
        flag_recovery_needed_log2=max(0.0, modeled + recovery_target_bits),
        intersection_log2_count=intersection_log,
        tail_log2=tail_log,
        modeled_log2_expected=modeled,
        notes="symmetric multicopy stride-dimension stress model",
    )


def best_rows(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    target_excesses: list[int],
    max_near_extra: int,
    max_active_copies: int,
    support_count_model: str,
    recovery_target_bits: float,
    min_core_support: int,
    max_core_support: int | None,
) -> list[StressRow]:
    k = 1 << depth
    rows: list[StressRow] = []
    for target_excess in target_excesses:
        best: StressRow | None = None
        for active_copies in range(1, min(expansion, max_active_copies) + 1):
            for live_bits in range(depth + 1):
                live_rows = 1 << live_bits
                core_support = k // live_rows
                if core_support < min_core_support:
                    continue
                if max_core_support is not None and core_support > max_core_support:
                    continue
                shape_max_extra = min(max_near_extra, k - core_support)
                for extra_support in range(shape_max_extra + 1):
                    row = row_for_shape(
                        depth=depth,
                        expansion=expansion,
                        q_log2=q_log2,
                        target_excess=target_excess,
                        active_copies=active_copies,
                        live_rows=live_rows,
                        extra_support=extra_support,
                        support_count_model=support_count_model,
                        recovery_target_bits=recovery_target_bits,
                    )
                    if best is None or row.modeled_log2_expected > best.modeled_log2_expected:
                        best = row
        assert best is not None
        rows.append(best)
    return rows


def best_rows_by_active(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    target_excesses: list[int],
    max_near_extra: int,
    max_active_copies: int,
    support_count_model: str,
    recovery_target_bits: float,
    min_core_support: int,
    max_core_support: int | None,
) -> list[StressRow]:
    k = 1 << depth
    rows: list[StressRow] = []
    for target_excess in target_excesses:
        for active_copies in range(1, min(expansion, max_active_copies) + 1):
            best: StressRow | None = None
            for live_bits in range(depth + 1):
                live_rows = 1 << live_bits
                core_support = k // live_rows
                if core_support < min_core_support:
                    continue
                if max_core_support is not None and core_support > max_core_support:
                    continue
                shape_max_extra = min(max_near_extra, k - core_support)
                for extra_support in range(shape_max_extra + 1):
                    row = row_for_shape(
                        depth=depth,
                        expansion=expansion,
                        q_log2=q_log2,
                        target_excess=target_excess,
                        active_copies=active_copies,
                        live_rows=live_rows,
                        extra_support=extra_support,
                        support_count_model=support_count_model,
                        recovery_target_bits=recovery_target_bits,
                    )
                    if best is None or row.modeled_log2_expected > best.modeled_log2_expected:
                        best = row
            assert best is not None
            rows.append(best)
    return rows


def parse_target_excesses(text: str) -> list[int]:
    if ".." in text:
        start_text, end_text = text.split("..", 1)
        start = int(start_text)
        end = int(end_text)
        step = 1 if end >= start else -1
        return list(range(start, end + step, step))
    return [int(part) for part in text.split(",") if part]


def csv_dict(row: StressRow) -> dict[str, str | int | float]:
    out = asdict(row)
    for key in [
        "family_log2_count",
        "matched_core_family_log2_count",
        "exact_size_family_log2_count",
        "support_overlap_penalty_log2",
        "flag_recovery_needed_log2",
        "intersection_log2_count",
        "tail_log2",
        "modeled_log2_expected",
    ]:
        value = out[key]
        if isinstance(value, float):
            out[key] = fmt(value)
    return out


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=11)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--target-excesses", default="70,71,72")
    parser.add_argument("--max-near-extra", type=int, default=None)
    parser.add_argument("--max-active-copies", type=int, default=8)
    parser.add_argument("--min-core-support", type=int, default=1)
    parser.add_argument("--max-core-support", type=int, default=None)
    parser.add_argument(
        "--support-count-model",
        choices=["matched-core", "exact-size"],
        default="matched-core",
        help="matched-core preserves the old adversarial overcount; exact-size counts support sets once",
    )
    parser.add_argument(
        "--recovery-target-bits",
        type=float,
        default=80.0,
        help="bits needed for a target expectation <= 2^-bits",
    )
    parser.add_argument("--by-active", action="store_true")
    args = parser.parse_args()

    k = 1 << args.depth
    max_near_extra = k - 1 if args.max_near_extra is None else args.max_near_extra
    if args.min_core_support < 1:
        raise SystemExit("--min-core-support must be positive")
    if args.max_core_support is not None and args.max_core_support < args.min_core_support:
        raise SystemExit("--max-core-support must be >= --min-core-support")
    row_fn = best_rows_by_active if args.by_active else best_rows
    rows = row_fn(
        depth=args.depth,
        expansion=args.expansion,
        q_log2=args.q_log2,
        target_excesses=parse_target_excesses(args.target_excesses),
        max_near_extra=max_near_extra,
        max_active_copies=args.max_active_copies,
        support_count_model=args.support_count_model,
        recovery_target_bits=args.recovery_target_bits,
        min_core_support=args.min_core_support,
        max_core_support=args.max_core_support,
    )

    writer = csv.DictWriter(sys.stdout, fieldnames=list(asdict(rows[0]).keys()))
    writer.writeheader()
    writer.writerows(csv_dict(row) for row in rows)


if __name__ == "__main__":
    main()
