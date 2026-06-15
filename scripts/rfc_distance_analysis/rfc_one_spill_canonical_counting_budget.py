#!/usr/bin/env python3
"""Canonical-counting budget for the dominant one-spill row.

This diagnostic asks whether the one-spill deficit can be closed by replacing
selected-subset counting with canonical witness/support counting.  It keeps the
same algebraic q-exponents as the structural one-spill ledger and only changes
combinatorial factors that are plausibly duplicate certificates:

* side choices on common-zero singleton coordinates;
* P/C labels inside Z=P union C;
* the outside singleton subset union, replaced by the exact binomial tail.

The output is intentionally a budget, not a certificate.  If even the aggressive
canonical model leaves a large q-dimensional gap, then witness de-duplication is
not enough for this row.
"""

from __future__ import annotations

import argparse
import math

from rfc_common_zero_one_spill_ledger import NEG_INF, log2_add, row_for


def log2_comb(n: int, k: int) -> float:
    if k < 0 or k > n:
        return NEG_INF
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


def outside_tail_log2(n: int, min_successes: int, q_log2: float) -> float:
    """Return log2 sum_{m>=L} binom(n,m) 2^m q^{-m}.

    This is the exact outside-root success union for a fixed projective witness
    when every non-common-zero child coordinate offers at most two singleton
    sides.  Large-q tails should be dominated by the first term.
    """

    total = NEG_INF
    for m in range(min_successes, n + 1):
        total = log2_add(total, log2_comb(n, m) + m - q_log2 * m)
    return total


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--depth", type=int, default=11)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--p", type=int, default=137)
    parser.add_argument("--t", type=int, default=1845)
    parser.add_argument("--D", type=int, default=887)
    parser.add_argument("--h", type=int, default=864)
    parser.add_argument("--flat-excess", type=int, default=41)
    parser.add_argument("--lift-levels", type=int, default=5)
    parser.add_argument("--p0", type=int, default=3)
    parser.add_argument("--s0", type=int, default=1)
    parser.add_argument("--d0", type=int, default=14)
    args = parser.parse_args()

    row = row_for(
        depth=args.depth,
        expansion=args.expansion,
        q_log2=args.q_log2,
        p=args.p,
        t=args.t,
        d=args.D,
        h=args.h,
        flat_excess=args.flat_excess,
        lift_levels=args.lift_levels,
        p0=args.p0,
        s0=args.s0,
        d0=args.d0,
        triple_rank_codim_override=None,
        triple_rank_structural_profile=True,
        label_model="arbitrary",
    )
    if row is None:
        raise SystemExit("dominant row is infeasible under the one-spill ledger")

    n_child = args.expansion * (1 << (args.depth - 1))
    outside_n = n_child - row.z
    outside_l = args.t - row.a
    projective_witness_qdim = 2 * row.h - 1

    selected_outside_log2 = log2_comb(outside_n, outside_l) + outside_l
    exact_tail_log2 = outside_tail_log2(outside_n, outside_l, args.q_log2)
    exact_tail_as_shape_log2 = exact_tail_log2 + args.q_log2 * outside_l
    tail_saving_bits = selected_outside_log2 - exact_tail_as_shape_log2

    common_zero_side_bits = row.a
    label_bits = row.label_log2_count
    aggressive_shape = row.log2_shape - common_zero_side_bits - label_bits - tail_saving_bits
    aggressive_term = aggressive_shape - args.q_log2 * row.total_q
    aggressive_gap_q = max(0.0, (aggressive_term + args.security_bits) / args.q_log2)

    actual_parent_p = args.p + row.a
    actual_parent_t = args.t - row.a
    actual_parent_zero_count = 2 * actual_parent_p + actual_parent_t
    selected_parent_zero_count = 2 * args.p + args.t

    print("metric,value")
    print(f"h,{row.h}")
    print(f"flat_excess,{row.flat_excess}")
    print(f"z,{row.z}")
    print(f"a_common_zero,{row.a}")
    print(f"outside_positions,{outside_n}")
    print(f"outside_required_successes,{outside_l}")
    print(f"total_q,{row.total_q}")
    print(f"baseline_log2_shape,{row.log2_shape:.6f}")
    print(f"baseline_log2_term,{row.log2_term:.6f}")
    print(f"baseline_gap_q,{row.extra_q_needed(args.q_log2, args.security_bits):.6f}")
    print(f"projective_witness_qdim,{projective_witness_qdim}")
    print(f"selected_outside_log2_shape,{selected_outside_log2:.6f}")
    print(f"exact_outside_tail_log2_probability,{exact_tail_log2:.6f}")
    print(f"exact_outside_tail_log2_shape_equivalent,{exact_tail_as_shape_log2:.6f}")
    print(f"outside_tail_saving_bits,{tail_saving_bits:.6f}")
    print(f"outside_tail_saving_qdims,{tail_saving_bits / args.q_log2:.6f}")
    print(f"common_zero_side_saving_bits,{common_zero_side_bits:.6f}")
    print(f"common_zero_side_saving_qdims,{common_zero_side_bits / args.q_log2:.6f}")
    print(f"pc_label_saving_bits,{label_bits:.6f}")
    print(f"pc_label_saving_qdims,{label_bits / args.q_log2:.6f}")
    print(f"aggressive_canonical_log2_shape,{aggressive_shape:.6f}")
    print(f"aggressive_canonical_log2_term,{aggressive_term:.6f}")
    print(f"aggressive_canonical_gap_q,{aggressive_gap_q:.6f}")
    print(f"selected_parent_zero_count,{selected_parent_zero_count}")
    print(f"actual_parent_zero_count_if_C_is_paired,{actual_parent_zero_count}")
    print(f"extra_actual_parent_zeros_from_C,{actual_parent_zero_count - selected_parent_zero_count}")


if __name__ == "__main__":
    main()
