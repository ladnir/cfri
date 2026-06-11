#!/usr/bin/env python3
"""Aggregate replica first-moment upper bound for original RFC distance.

For a fixed zero set Z, rank deficiency is equivalent to the existence of a nonzero message
vanishing on Z.  This script studies the aggregate first moment

    B_d(r,z) = sum_{|Z|=z} E[# nonzero ordered r-tuples of messages
                              that all vanish on Z].

The top distance certificate uses r=1.  The root recurrence doubles the replica count because one
parent message has two child halves.  For a root split with p sibling-pair groups and s singleton
groups, z=2p+s.  If C is the subset of singleton groups where all child replicas are already common
zero, then the remaining singleton groups each cost one fresh root challenge.  Summing over shapes:

    B_d(r,z) <= sum 2^s * ((q-2)^c/(q-1)^s)
                     * binom(u,p) * binom(n-u,s-c) * B_{d-1}(2r,u)

where u=p+c, c=|C|, and n is the child block length.

The default `loose` mode is a valid common-zero-only diagnostic but is far too pessimistic.  The
`replica` singleton-charge mode is an optimistic model that charges a generic singleton in an
r-replica parent state by r field equations, matching the local rank-one aggregate identity.  The
`component-uniform` mode keeps the first component correction under a uniform-matroid quotient
model.  These modes are for calibration; they still need a global rank-pattern proof before they
can be used as certificates.
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


def log2_comb_table(n: int) -> list[list[float]]:
    table: list[list[float]] = []
    for row_n in range(n + 1):
        row = [NEG_INF] * (row_n + 1)
        row[0] = 0.0
        row[row_n] = 0.0
        for k in range(1, row_n):
            row[k] = log2_add(table[row_n - 1][k - 1], table[row_n - 1][k])
        table.append(row)
    return table


def log2_q_minus(delta: int, q_log2: float, q_exact: int | None) -> float:
    if q_exact is not None:
        value = q_exact - delta
        if value <= 0:
            return NEG_INF
        return math.log2(value)
    return q_log2


def initial_nonzero_moment(base_expansion: int, replica_count: int, q_log2: float) -> list[float]:
    values = [NEG_INF] * (base_expansion + 1)
    # q^r - 1.  At certificate fields this is indistinguishable from q^r in log scale.
    values[0] = replica_count * q_log2
    return values


def lift_moment(
    child: list[float],
    comb: list[list[float]],
    q_log2: float,
    q_exact: int | None,
    replica_count: int,
    singleton_charge: str,
    child_k: int,
) -> tuple[list[float], list[tuple[int, int, int, int] | None]]:
    child_n = len(child) - 1
    parent_n = 2 * child_n
    parent = [NEG_INF] * (parent_n + 1)
    choices: list[tuple[int, int, int, int] | None] = [None] * (parent_n + 1)
    log_q_minus_1 = log2_q_minus(1, q_log2, q_exact)
    log_q_minus_2 = log2_q_minus(2, q_log2, q_exact)

    for z in range(parent_n + 1):
        best = NEG_INF
        best_term = NEG_INF
        best_choice: tuple[int, int, int, int] | None = None
        for p in range(z // 2 + 1):
            s = z - 2 * p
            if p + s > child_n:
                continue
            orientation_log = float(s)
            singleton_den_log = s * log_q_minus_1
            max_c = min(s, child_n - p)
            for c in range(max_c + 1):
                u = p + c
                if child[u] <= NEG_INF / 2:
                    continue
                extras = s - c
                if extras > child_n - u:
                    continue
                if singleton_charge == "loose":
                    root_charge_log = -singleton_den_log + c * log_q_minus_2
                elif singleton_charge == "replica":
                    # Optimistic rank-one aggregate model: every non-common singleton in an
                    # r-replica state costs r field equations, while common-zero singleton
                    # coordinates are automatic.
                    root_charge_log = -(s - c) * replica_count * q_log2
                elif singleton_charge == "component-uniform":
                    # Component-aware model under a uniform quotient matroid.  For parent
                    # replica_count r >= 2, a non-common singleton block E has charge
                    # |E| + r * rank(E) - comp(E).  For r=1 the rank-one compatibility condition
                    # is vacuous, so only the root equations remain.
                    if replica_count <= 1:
                        charge = extras
                    else:
                        common_rank = min(u, child_k)
                        quotient_rank = max(0, child_k - common_rank)
                        if extras == 0:
                            charge = 0
                        elif quotient_rank == 0:
                            charge = 0
                        elif extras <= quotient_rank:
                            charge = replica_count * extras
                        else:
                            charge = extras + replica_count * quotient_rank - 1
                    root_charge_log = -charge * q_log2
                else:
                    raise ValueError(f"unknown singleton charge {singleton_charge}")
                term = (
                    orientation_log
                    + root_charge_log
                    + comb[u][p]
                    + comb[child_n - u][extras]
                    + child[u]
                )
                best = log2_add(best, term)
                if term > best_term:
                    best_term = term
                    best_choice = (p, s, c, u)
        parent[z] = best
        choices[z] = best_choice
    return parent, choices


def first_crossing(values: list[float], security_bits: float) -> int | None:
    target = -security_bits
    for z, log_value in enumerate(values):
        if log_value <= target:
            return z
    return None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--q-exact", type=int, default=0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--print-window", type=int, default=5)
    parser.add_argument("--trace-z", type=int, default=-1)
    parser.add_argument(
        "--singleton-charge",
        choices=["loose", "replica", "component-uniform"],
        default="loose",
    )
    args = parser.parse_args()

    if args.depth < 0:
        raise SystemExit("--depth must be nonnegative")
    if args.expansion < 1:
        raise SystemExit("--expansion must be positive")
    if args.q_exact and args.q_exact <= 2:
        raise SystemExit("--q-exact must be > 2")

    q_exact = args.q_exact if args.q_exact else None
    if q_exact is not None:
        args.q_log2 = math.log2(q_exact)

    total_n = args.expansion * (1 << args.depth)
    comb = log2_comb_table(total_n)
    replica_count = 1 << args.depth
    values = initial_nonzero_moment(args.expansion, replica_count, args.q_log2)
    trace: list[list[tuple[int, int, int, int] | None]] = []
    print(
        "level,replica_count,k,n,min_log2,max_log2",
        flush=True,
    )
    print(
        f"0,{replica_count},1,{args.expansion},"
        f"{min(v for v in values if v > NEG_INF / 2):.8f},"
        f"{max(v for v in values if v > NEG_INF / 2):.8f}",
        flush=True,
    )

    for level in range(1, args.depth + 1):
        parent_replica_count = replica_count // 2
        values, choices = lift_moment(
            values,
            comb,
            args.q_log2,
            q_exact,
            parent_replica_count,
            args.singleton_charge,
            1 << (level - 1),
        )
        trace.append(choices)
        replica_count = parent_replica_count
        finite = [v for v in values if v > NEG_INF / 2]
        print(
            f"{level},{replica_count},{1 << level},{args.expansion * (1 << level)},"
            f"{min(finite):.8f},{max(finite):.8f}",
            flush=True,
        )

    crossing = first_crossing(values, args.security_bits)
    k = 1 << args.depth
    print("z,excess,log2_nonzero_tuple_moment")
    if crossing is None:
        center = k
    else:
        center = crossing
    start = max(0, center - args.print_window)
    stop = min(total_n, center + args.print_window)
    for z in range(start, stop + 1):
        excess = z - k
        label = "-inf" if values[z] <= NEG_INF / 2 else f"{values[z]:.8f}"
        print(f"{z},{excess},{label}")
    print(
        f"crossing_z={crossing} crossing_excess="
        f"{'' if crossing is None else crossing - k}"
    )
    if args.trace_z >= 0:
        if args.trace_z > total_n:
            raise SystemExit("--trace-z is larger than n")
        print("trace_level,z,p,s,c,u")
        z = args.trace_z
        for level in range(args.depth, 0, -1):
            choice = trace[level - 1][z]
            if choice is None:
                print(f"{level},{z},,,,,")
                break
            p, s, c, u = choice
            print(f"{level},{z},{p},{s},{c},{u}")
            z = u


if __name__ == "__main__":
    main()
