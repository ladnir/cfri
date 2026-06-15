#!/usr/bin/env python3
"""Conditional RFC distance certificate calculator.

This driver intentionally uses theorem exponents, not sampled profiler counts.  It reports a
conditional certificate under the local/global theorem package described in
docs/rfc_distance_analysis/rfc_distance_certificate_theorem.md.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
import sys
from dataclasses import asdict, dataclass
from pathlib import Path


NEG_INF = -1.0e300


def log2_add(left: float, right: float) -> float:
    if left <= NEG_INF / 2:
        return right
    if right <= NEG_INF / 2:
        return left
    if right > left:
        left, right = right, left
    return left + math.log2(1.0 + 2.0 ** (right - left))


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


def log2_systematic_split_count(k: int, parity_n: int, z: int) -> tuple[float, int]:
    total = NEG_INF
    best_s = 0
    best = NEG_INF
    for systematic_zeros in range(max(0, z - parity_n), min(k, z) + 1):
        parity_zeros = z - systematic_zeros
        term = log2_comb(k, systematic_zeros) + log2_comb(parity_n, parity_zeros)
        total = log2_add(total, term)
        if term > best:
            best = term
            best_s = systematic_zeros
    return total, best_s


def log2_zero_set_count(mode: str, k: int, n: int, z: int) -> tuple[float, int | None]:
    if mode == "original":
        return log2_comb(n, z), None
    if mode == "systematic_all_levels":
        return log2_systematic_split_count(k, n - k, z)
    raise ValueError(f"unknown mode {mode}")


def theorem_log2_bad(
    mode: str,
    k: int,
    n: int,
    z: int,
    q_log2: float,
    log2_poly_factor: float,
) -> tuple[float, int | None]:
    zero_set_log2, best_systematic_zeros = log2_zero_set_count(mode, k, n, z)
    excess = z - k
    # Conditional rank-pattern theorem exponent.  The systematic mode keeps identity coordinates
    # explicit in zero_set_log2; for z>=k, the parity exponent is still z-k+1.
    rank_tail_log2 = -(excess + 1) * q_log2
    return zero_set_log2 + log2_poly_factor + rank_tail_log2, best_systematic_zeros


def find_crossing(
    mode: str,
    k: int,
    n: int,
    q_log2: float,
    security_bits: float,
    log2_poly_factor: float,
) -> tuple[int, float, int | None]:
    for z in range(k, n + 1):
        log2_bad, best_systematic_zeros = theorem_log2_bad(mode, k, n, z, q_log2, log2_poly_factor)
        if log2_bad <= -security_bits:
            return z, log2_bad, best_systematic_zeros
    raise SystemExit(f"no crossing found for mode={mode}")


@dataclass
class TraceRow:
    effective_depth: int
    effective_k: int
    effective_n: int
    crossing_excess: int
    scaled_excess_to_top: int
    scaled_relative_gap: float


@dataclass
class CertificateRow:
    mode: str
    status: str
    bound_model: str
    polynomial_factor_status: str
    systematic_model: str
    depth: int
    expansion: int
    k: int
    n: int
    q_log2: float
    security_bits: float
    log2_poly_factor: float
    z: int
    excess: int
    log2_bad: float
    slack_bits: float
    distance_lower_bound: int
    relative_distance_lower_bound: float
    mds_relative_distance: float
    relative_gap_to_mds: float
    best_systematic_zeros: int | None
    local_theorems_used: str
    paired_compression_trace_json: str


LOCAL_THEOREMS_USED = "; ".join(
    [
        "tau=1 support-subcode lemma",
        "high-rank flat-excess h=1 endpoint reduced to child line-zero",
        "tau=2 layer-codimension root-line endpoint theorem assumed",
        "component/full-rank endpoint via diagonal endomorphisms",
        "generic two-copy matroid-union endpoint included as one layer",
        "intermediate tau=2 rank-drop layers charged by theta_2(A)",
        "finite-replica rank-pattern recurrence assumed",
        "paired compression preserves relative excess assumed",
    ]
)


def paired_trace(
    mode: str,
    depth: int,
    expansion: int,
    q_log2: float,
    security_bits: float,
    log2_poly_factor: float,
) -> list[TraceRow]:
    top_k = 1 << depth
    rows: list[TraceRow] = []
    for effective_depth in range(0, depth + 1):
        effective_k = 1 << effective_depth
        effective_n = expansion * effective_k
        z, _log2_bad, _best_s = find_crossing(
            mode,
            effective_k,
            effective_n,
            q_log2,
            security_bits,
            log2_poly_factor,
        )
        excess = z - effective_k
        scale = top_k // effective_k
        rows.append(
            TraceRow(
                effective_depth=effective_depth,
                effective_k=effective_k,
                effective_n=effective_n,
                crossing_excess=excess,
                scaled_excess_to_top=scale * excess,
                scaled_relative_gap=(scale * excess) / (expansion * top_k),
            )
        )
    return rows


def polynomial_factor_status(log2_poly_factor: float) -> str:
    if log2_poly_factor == 0:
        return "C(d,N)=1 idealization; use --log2-poly-factor after constants are bounded"
    return "user_supplied_log2_C(d,N)"


def systematic_model(mode: str) -> str:
    if mode == "original":
        return "not_applicable"
    return "explicit_identity_split; assumes same final rank-tail exponent after split"


def certificate(
    mode: str,
    depth: int,
    expansion: int,
    q_log2: float,
    security_bits: float,
    log2_poly_factor: float,
) -> CertificateRow:
    k = 1 << depth
    n = expansion * k
    if mode == "systematic_all_levels" and expansion < 2:
        raise SystemExit("systematic_all_levels requires expansion >= 2")
    z, log2_bad, best_systematic_zeros = find_crossing(
        mode,
        k,
        n,
        q_log2,
        security_bits,
        log2_poly_factor,
    )
    excess = z - k
    distance = n - z + 1
    trace = paired_trace(mode, depth, expansion, q_log2, security_bits, log2_poly_factor)
    return CertificateRow(
        mode=mode,
        status="idealized_conditional_on_theorem_package",
        bound_model="final_shape_union_bound_not_full_recurrence",
        polynomial_factor_status=polynomial_factor_status(log2_poly_factor),
        systematic_model=systematic_model(mode),
        depth=depth,
        expansion=expansion,
        k=k,
        n=n,
        q_log2=q_log2,
        security_bits=security_bits,
        log2_poly_factor=log2_poly_factor,
        z=z,
        excess=excess,
        log2_bad=log2_bad,
        slack_bits=-security_bits - log2_bad,
        distance_lower_bound=distance,
        relative_distance_lower_bound=distance / n,
        mds_relative_distance=(n - k + 1) / n,
        relative_gap_to_mds=((n - k + 1) - distance) / n,
        best_systematic_zeros=best_systematic_zeros,
        local_theorems_used=LOCAL_THEOREMS_USED,
        paired_compression_trace_json=json.dumps([asdict(row) for row in trace], separators=(",", ":")),
    )


def row_to_json_dict(row: CertificateRow) -> dict[str, object]:
    out = asdict(row)
    out["paired_compression_trace"] = json.loads(row.paired_compression_trace_json)
    del out["paired_compression_trace_json"]
    return out


def parse_modes(value: str) -> list[str]:
    if value == "both":
        return ["original", "systematic_all_levels"]
    modes = [mode.strip() for mode in value.split(",") if mode.strip()]
    allowed = {"original", "systematic_all_levels"}
    unknown = [mode for mode in modes if mode not in allowed]
    if unknown:
        raise SystemExit(f"unknown mode(s): {', '.join(unknown)}")
    return modes


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=11)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--log2-poly-factor", type=float, default=0.0)
    parser.add_argument(
        "--mode",
        default="original",
        help="original, systematic_all_levels, comma-separated list, or both",
    )
    parser.add_argument("--output-json", default=None)
    parser.add_argument("--output-csv", default=None)
    args = parser.parse_args()

    if args.depth < 0:
        raise SystemExit("--depth must be nonnegative")
    if args.expansion < 1:
        raise SystemExit("--expansion must be positive")
    if args.q_log2 <= 0:
        raise SystemExit("--q-log2 must be positive")

    rows = [
        certificate(
            mode=mode,
            depth=args.depth,
            expansion=args.expansion,
            q_log2=args.q_log2,
            security_bits=args.security_bits,
            log2_poly_factor=args.log2_poly_factor,
        )
        for mode in parse_modes(args.mode)
    ]

    stdout_writer = csv.DictWriter(sys.stdout, fieldnames=list(asdict(rows[0]).keys()))
    stdout_writer.writeheader()
    stdout_writer.writerows(asdict(row) for row in rows)

    if args.output_json is not None:
        Path(args.output_json).write_text(
            json.dumps([row_to_json_dict(row) for row in rows], indent=2) + "\n",
            encoding="utf-8",
        )
    if args.output_csv is not None:
        with Path(args.output_csv).open("w", newline="", encoding="utf-8") as handle:
            writer = csv.DictWriter(handle, fieldnames=list(asdict(rows[0]).keys()))
            writer.writeheader()
            writer.writerows(asdict(row) for row in rows)


if __name__ == "__main__":
    main()
