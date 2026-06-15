#!/usr/bin/env python3
"""Weak-form PA-chain scale check.

This checks the theorem-safe consequence of multi-PA closure:

    b parent PA closure marks
      -> child marked rank increment <= b-1

rather than the stronger child closure of all b marks.

It is a one-step scale check, not a recurrence or certificate.  In
``closure_union`` mode, it charges the child one-rank drop by the deterministic
minimal-circuit reduction to one-mark closure, adding a union factor over
marked circuits.
"""

from __future__ import annotations

import argparse
import math


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
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2.0)


def fmt(value: float) -> str:
    if value <= NEG_INF / 2:
        return "-inf"
    return f"{value:.6f}"


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--depth", type=int, default=10)
    ap.add_argument("--expansion", type=int, default=8)
    ap.add_argument("--q-log2", type=float, default=128.0)
    ap.add_argument("--p", type=int, default=137)
    ap.add_argument("--a", type=int, required=True)
    ap.add_argument("--residual-q-exp", type=int, default=0)
    ap.add_argument(
        "--charge-mode",
        choices=["random_rank", "closure_union"],
        default="random_rank",
        help="How to charge the child one-rank marked tail.",
    )
    args = ap.parse_args()

    k_parent = 1 << args.depth
    n_parent = args.expansion * k_parent
    k_child = k_parent >> 1
    n_child = n_parent >> 1

    total = NEG_INF
    best: tuple[float, int, int, int, int, float, float] | None = None
    for b in range(1, args.a + 1):
        direct_marks = args.a - b
        max_y = min(args.p - b, n_child - b)
        for y in range(max_y + 1):
            remaining_p = args.p - b - y
            if remaining_p < 0:
                continue
            # Child weak marked-rank event: b marked columns have quotient rank <= b-1
            # modulo a y-column core. Random exponent: (b-(b-1))*(D-(b-1)).
            child_D = max(0, k_child - y)
            child_q_exp = max(0, child_D - (b - 1))
            child_count = log2_comb(n_child, b) + log2_comb(n_child - b, y)
            if args.charge_mode == "closure_union":
                # Minimal circuit B subset J and distinguished j in B.  The weakest closure
                # exponent occurs at |B|=b; summing all B,j costs at most b*2^(b-1).
                child_count += math.log2(max(1, b)) + max(0, b - 1)
            child_log = child_count - child_q_exp * args.q_log2

            # Parent lift, matching the loose multi-mark recurrence.
            available_for_extra_p = 2 * (n_child - b) - y
            pa_lift = b + y + log2_comb(available_for_extra_p, remaining_p)

            direct_q_exp = max(0, direct_marks * (k_parent - args.p))
            direct_lift = log2_comb(n_parent, direct_marks) - direct_q_exp * args.q_log2
            term = child_log + pa_lift + direct_lift
            total = log2_add(total, term)
            if best is None or term > best[0]:
                best = (term, b, y, child_q_exp, direct_q_exp, child_log, pa_lift + direct_lift)

    combined = total - args.residual_q_exp * args.q_log2
    status = "pass" if combined <= -1.0 else "fail"
    print(
        "depth,expansion,q_log2,p,a,residual_q_exp,charge_mode,log2_weak_pa,"
        "combined_log2,status,best_b,best_y,best_child_q_exp,best_direct_q_exp,"
        "best_child_log2,best_lift_log2,best_term"
    )
    if best is None:
        print(
            f"{args.depth},{args.expansion},{args.q_log2:g},{args.p},{args.a},"
            f"{args.residual_q_exp},{args.charge_mode},-inf,-inf,empty,,,,,,,"
        )
        return
    term, b, y, child_q_exp, direct_q_exp, child_log, lift_log = best
    print(
        f"{args.depth},{args.expansion},{args.q_log2:g},{args.p},{args.a},"
        f"{args.residual_q_exp},{args.charge_mode},{fmt(total)},{fmt(combined)},{status},"
        f"{b},{y},{child_q_exp},{direct_q_exp},{fmt(child_log)},"
        f"{fmt(lift_log)},{fmt(term)}"
    )


if __name__ == "__main__":
    main()
