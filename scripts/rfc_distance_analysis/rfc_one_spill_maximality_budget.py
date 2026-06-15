#!/usr/bin/env python3
"""Bookkeep exact-maximality credit for the dominant one-spill row.

The one-spill ledger already includes the residual root exponent

    t - a - 2h + 1.

That exponent is the standard projective witness count q^(2h-1) times one
root constraint for each singleton outside C.  Exact maximality adds the
inequalities "not both evaluations vanish" outside C.  Inequalities can remove
constant factors but do not give q-codimension unless they are replaced by
equations.  This script prints the relevant budget to keep that accounting
explicit.
"""

from __future__ import annotations

import argparse


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--t", type=int, default=1845)
    parser.add_argument("--a", type=int, default=87)
    parser.add_argument("--h", type=int, default=864)
    parser.add_argument("--missing-q", type=float, default=30.196314)
    args = parser.parse_args()

    outside = args.t - args.a
    witness_qdim = 2 * args.h - 1
    residual_root_q = outside - witness_qdim
    inequality_extra_q = 0

    print("quantity,value")
    print(f"outside_singletons_t_minus_a,{outside}")
    print(f"projective_witness_qdim_2h_minus_1,{witness_qdim}")
    print(f"residual_root_q_already_charged,{residual_root_q}")
    print(f"maximality_inequality_extra_qdim,{inequality_extra_q}")
    print(f"missing_qdim_after_structural_ledger,{args.missing_q}")
    print(f"remaining_after_local_maximality,{args.missing_q - inequality_extra_q:.6f}")


if __name__ == "__main__":
    main()
