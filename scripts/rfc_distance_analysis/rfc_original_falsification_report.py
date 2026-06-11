#!/usr/bin/env python3
"""Falsification-oriented bad-family report for original non-systematic RFC.

This is not a certificate.  It tracks explicit and modeled bad families that could contradict a
near-MDS target:

* one-copy exact uncertainty extremizers;
* matched-core plus extra one-copy near-extremizers;
* all-paired compression scaling.

The output is intended to make the "maybe the statement is false" lane concrete.  A positive
family log is a warning sign; a very negative family log is evidence that this family is not the
obstruction.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
from dataclasses import asdict, dataclass
from pathlib import Path


NEG_INF = -math.inf


def log2_add(left: float, right: float) -> float:
    if left == NEG_INF:
        return right
    if right == NEG_INF:
        return left
    if right > left:
        left, right = right, left
    return left + math.log2(1.0 + 2.0 ** (right - left))


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


def fmt(value: float) -> str:
    if value == NEG_INF:
        return "-inf"
    return f"{value:.8f}"


@dataclass
class CandidateRow:
    category: str
    depth: int
    expansion: int
    k: int
    n: int
    q_log2: float
    target_excess: int
    target_zero_count: int
    live_rows: int
    one_copy_core_support: int
    one_copy_extra_support: int
    deterministic_zero_count: int
    deterministic_distance_upper_bound: int
    needed_other_zeros: int
    family_log2_count: float
    other_tail_log2: float
    modeled_log2_expected: float
    modeled_slack_to_security: float
    relative_zero_count: float
    relative_distance_upper_bound: float
    notes: str


@dataclass
class CompressionRow:
    effective_depth: int
    effective_k: int
    effective_target_excess: int
    scaled_excess_to_top: int
    scaled_relative_gap: float


def exact_family_count_log2(expansion: int, k: int) -> float:
    # One parity copy choice times k matched block/stride pairs.  This intentionally overcounts
    # duplicate projective lines; for falsification, overcounting is conservative.
    return math.log2(expansion) + math.log2(k)


def near_family_count_log2(expansion: int, k: int, core_support: int, extra_support: int) -> float:
    extras_available = k - core_support
    return exact_family_count_log2(expansion, k) + log2_comb(extras_available, extra_support)


def modeled_near_dimension_excess(core_support: int, extra_support: int) -> int:
    # Existing near-extremizer diagnostics use this as the simplest stress model: a full extra
    # stride class can increase the one-copy kernel dimension.
    if core_support <= 0:
        return 0
    return extra_support // core_support


def candidate_row(
    category: str,
    depth: int,
    expansion: int,
    q_log2: float,
    target_excess: int,
    live_rows: int,
    one_copy_extra_support: int,
    use_stride_dimension: bool,
    security_bits: float,
) -> CandidateRow:
    k = 1 << depth
    n = expansion * k
    target_zero_count = k + target_excess
    core_support = k // live_rows
    one_copy_support = core_support + one_copy_extra_support
    deterministic_zero_count = k - one_copy_support
    needed_other_zeros = max(0, target_zero_count - deterministic_zero_count)
    other_positions = (expansion - 1) * k
    if one_copy_extra_support == 0:
        family_log = exact_family_count_log2(expansion, k)
    else:
        family_log = near_family_count_log2(expansion, k, core_support, one_copy_extra_support)
    dimension_excess = (
        modeled_near_dimension_excess(core_support, one_copy_extra_support)
        if use_stride_dimension
        else 0
    )
    other_tail_log = (
        log2_comb(other_positions, needed_other_zeros)
        - needed_other_zeros * q_log2
        + dimension_excess * q_log2
    )
    modeled = family_log + other_tail_log
    distance_upper = n - deterministic_zero_count
    return CandidateRow(
        category=category,
        depth=depth,
        expansion=expansion,
        k=k,
        n=n,
        q_log2=q_log2,
        target_excess=target_excess,
        target_zero_count=target_zero_count,
        live_rows=live_rows,
        one_copy_core_support=core_support,
        one_copy_extra_support=one_copy_extra_support,
        deterministic_zero_count=deterministic_zero_count,
        deterministic_distance_upper_bound=distance_upper,
        needed_other_zeros=needed_other_zeros,
        family_log2_count=family_log,
        other_tail_log2=other_tail_log,
        modeled_log2_expected=modeled,
        modeled_slack_to_security=-security_bits - modeled,
        relative_zero_count=deterministic_zero_count / n,
        relative_distance_upper_bound=distance_upper / n,
        notes=(
            "exact matched one-copy family"
            if one_copy_extra_support == 0
            else "matched-core plus modeled extra one-copy support"
        ),
    )


def best_candidates(
    depth: int,
    expansion: int,
    q_log2: float,
    target_excess: int,
    max_near_extra: int,
    use_stride_dimension: bool,
    security_bits: float,
) -> tuple[list[CandidateRow], CandidateRow]:
    rows: list[CandidateRow] = []
    best: CandidateRow | None = None
    k = 1 << depth
    for live_bits in range(depth + 1):
        live_rows = 1 << live_bits
        core_support = k // live_rows
        max_extra = min(max_near_extra, k - core_support)
        for extra in range(max_extra + 1):
            category = "one_copy_exact" if extra == 0 else "one_copy_near_model"
            row = candidate_row(
                category,
                depth,
                expansion,
                q_log2,
                target_excess,
                live_rows,
                extra,
                use_stride_dimension,
                security_bits,
            )
            if extra == 0:
                rows.append(row)
            if best is None or row.modeled_log2_expected > best.modeled_log2_expected:
                best = row
    assert best is not None
    rows.append(
        CandidateRow(
            **{
                **asdict(best),
                "category": "best_one_copy_near_model",
                "notes": "maximum modeled log2 expected over scanned live_rows/extras",
            }
        )
    )
    return rows, best


def paired_compression_trace(depth: int, expansion: int, target_excess: int) -> list[CompressionRow]:
    top_k = 1 << depth
    rows: list[CompressionRow] = []
    for effective_depth in range(depth + 1):
        effective_k = 1 << effective_depth
        scale = top_k // effective_k
        effective_excess = math.ceil(target_excess / scale)
        rows.append(
            CompressionRow(
                effective_depth=effective_depth,
                effective_k=effective_k,
                effective_target_excess=effective_excess,
                scaled_excess_to_top=scale * effective_excess,
                scaled_relative_gap=(scale * effective_excess) / (expansion * top_k),
            )
        )
    return rows


def row_to_csv_dict(row: CandidateRow) -> dict[str, str | int | float]:
    out = asdict(row)
    for key in [
        "family_log2_count",
        "other_tail_log2",
        "modeled_log2_expected",
        "modeled_slack_to_security",
        "relative_zero_count",
        "relative_distance_upper_bound",
    ]:
        out[key] = fmt(out[key]) if isinstance(out[key], float) else out[key]
    return out


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=11)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--target-excess", type=int, default=71)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--max-near-extra", type=int, default=160)
    parser.add_argument("--stride-dimension-bound", action="store_true")
    parser.add_argument("--output-csv", default=None)
    parser.add_argument("--output-json", default=None)
    args = parser.parse_args()

    if args.depth < 0:
        raise SystemExit("--depth must be nonnegative")
    if args.expansion < 2:
        raise SystemExit("--expansion must be at least 2")
    if args.target_excess < 0:
        raise SystemExit("--target-excess must be nonnegative")

    rows, best = best_candidates(
        args.depth,
        args.expansion,
        args.q_log2,
        args.target_excess,
        args.max_near_extra,
        args.stride_dimension_bound,
        args.security_bits,
    )
    compression = paired_compression_trace(args.depth, args.expansion, args.target_excess)

    fieldnames = list(asdict(rows[0]).keys())
    stdout = csv.DictWriter(__import__("sys").stdout, fieldnames=fieldnames)
    stdout.writeheader()
    stdout.writerows(row_to_csv_dict(row) for row in rows)
    print()
    print(
        "summary,"
        f"best_live_rows={best.live_rows},"
        f"best_extra={best.one_copy_extra_support},"
        f"best_modeled_log2_expected={fmt(best.modeled_log2_expected)},"
        f"best_modeled_slack_to_security={fmt(best.modeled_slack_to_security)}"
    )

    if args.output_csv is not None:
        with Path(args.output_csv).open("w", newline="", encoding="utf-8") as handle:
            writer = csv.DictWriter(handle, fieldnames=fieldnames)
            writer.writeheader()
            writer.writerows(row_to_csv_dict(row) for row in rows)
    if args.output_json is not None:
        payload = {
            "config": {
                "depth": args.depth,
                "expansion": args.expansion,
                "q_log2": args.q_log2,
                "target_excess": args.target_excess,
                "security_bits": args.security_bits,
                "max_near_extra": args.max_near_extra,
                "stride_dimension_bound": args.stride_dimension_bound,
            },
            "best_candidate": asdict(best),
            "candidate_rows": [asdict(row) for row in rows],
            "paired_compression_trace": [asdict(row) for row in compression],
        }
        Path(args.output_json).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
