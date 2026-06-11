#!/usr/bin/env python3
"""Optimistic shortened-kernel rank recurrence for RFC fixed witnesses.

This script computes a lower-cost diagnostic for

    rho_h(D,z) = -log_q Pr[dim H_h(B) >= D]

for a fixed zero witness B of size z.  It is not a certificate.  It keeps only
deterministic survivor mechanisms:

* paired compression;
* forcing u singleton child coordinates to vanish in the child, then using the
  fact that the remaining singleton equations cut at most s-u dimensions.

For a split z=2p+s and chosen u<=s, the recurrence term is

    rho_{h-1}(ceil((D+s-u)/2), p+u).

This gives an optimistic cost for bad shortened kernels.  If this cheap model
still gives enough charge for a theta-chain defect slice, the real proof has
room.  If it does not, the proof must use sharper local root-rank costs or
accept a genuine loss.
"""

from __future__ import annotations

import argparse
import csv
import math
import sys
from dataclasses import asdict, dataclass
from pathlib import Path


INF = 10**9


def ceil_div(num: int, den: int) -> int:
    return (num + den - 1) // den


def generic_rank_cost(k: int, dim: int, zeros: int) -> int:
    """Random-linear calibration for dim H(B) >= dim."""

    return max(0, dim * (zeros - k + dim))


@dataclass(frozen=True)
class Choice:
    cost: int
    parent_depth: int
    parent_k: int
    parent_z: int
    parent_dim: int
    paired: int
    singletons: int
    forced_singletons: int
    child_depth: int
    child_k: int
    child_z: int
    child_dim: int
    generic_cost: int
    slack_to_generic: int
    hard_theta_compatible: int
    theta_residue_if_a5: int


class ShortenedRankRecurrence:
    def __init__(
        self,
        *,
        depth: int,
        expansion: int,
        allowed_singletons: tuple[int, ...] | None = None,
    ) -> None:
        self.depth = depth
        self.expansion = expansion
        self.allowed_singletons = allowed_singletons
        self.costs: list[list[list[int]]] = []
        self.choices: list[list[list[Choice | None]]] = []
        self._build()

    def _split_allowed(self, singletons: int) -> bool:
        return self.allowed_singletons is None or singletons in self.allowed_singletons

    def _build(self) -> None:
        # Base depth 0: repetition code of dimension 1.  With no zero requests,
        # the kernel has dimension 1.  With any zero request, it has dimension 0.
        base_n = self.expansion
        base_cost = [[INF] * (base_n + 1) for _ in range(2)]
        base_choice: list[list[Choice | None]] = [[None] * (base_n + 1) for _ in range(2)]
        for z in range(base_n + 1):
            base_cost[0][z] = 0
        base_cost[1][0] = 0
        self.costs.append(base_cost)
        self.choices.append(base_choice)

        if self.depth >= 1:
            self._append_exact_depth_one()

        for h in range(2, self.depth + 1):
            self._append_recursive_depth(h)

    def _append_exact_depth_one(self) -> None:
        """Exact worst-shape rank cost for one determinant-1 fold over repetition.

        At depth one the message space has dimension two.  A sibling pair is
        invertible and kills the whole message.  With no sibling pair, z
        singleton coordinates impose z projective line constraints; a surviving
        line exists at cost z-1.
        """

        k = 2
        n = 2 * self.expansion
        table = [[INF] * (n + 1) for _ in range(k + 1)]
        choice_table: list[list[Choice | None]] = [[None] * (n + 1) for _ in range(k + 1)]
        for z in range(n + 1):
            table[0][z] = 0
        table[1][0] = 0
        table[2][0] = 0
        for z in range(1, min(self.expansion, n) + 1):
            table[1][z] = z - 1
            choice_table[1][z] = Choice(
                cost=z - 1,
                parent_depth=1,
                parent_k=k,
                parent_z=z,
                parent_dim=1,
                paired=0,
                singletons=z,
                forced_singletons=0,
                child_depth=0,
                child_k=1,
                child_z=0,
                child_dim=0,
                generic_cost=generic_rank_cost(k, 1, z),
                slack_to_generic=generic_rank_cost(k, 1, z) - (z - 1),
                hard_theta_compatible=int(z == 5),
                theta_residue_if_a5=max(0, z - 5),
            )
        self.costs.append(table)
        self.choices.append(choice_table)

    def _append_recursive_depth(self, h: int) -> None:
        child_cost = self.costs[h - 1]
        child_k = 1 << (h - 1)
        child_n = self.expansion * child_k
        parent_k = 2 * child_k
        parent_n = 2 * child_n
        table = [[INF] * (parent_n + 1) for _ in range(parent_k + 1)]
        choice_table: list[list[Choice | None]] = [
            [None] * (parent_n + 1) for _ in range(parent_k + 1)
        ]
        for z in range(parent_n + 1):
            table[0][z] = 0
            for dim in range(1, parent_k + 1):
                best_cost = INF
                best_choice: Choice | None = None
                for paired in range(z // 2 + 1):
                    singletons = z - 2 * paired
                    if not self._split_allowed(singletons):
                        continue
                    if paired + singletons > child_n:
                        continue
                    for forced in range(singletons + 1):
                        child_z = paired + forced
                        child_dim = ceil_div(dim + singletons - forced, 2)
                        if child_dim > child_k:
                            continue
                        cost = child_cost[child_dim][child_z]
                        if cost >= INF:
                            continue
                        if cost < best_cost:
                            gen = generic_rank_cost(parent_k, dim, z)
                            best_cost = cost
                            best_choice = Choice(
                                cost=cost,
                                parent_depth=h,
                                parent_k=parent_k,
                                parent_z=z,
                                parent_dim=dim,
                                paired=paired,
                                singletons=singletons,
                                forced_singletons=forced,
                                child_depth=h - 1,
                                child_k=child_k,
                                child_z=child_z,
                                child_dim=child_dim,
                                generic_cost=gen,
                                slack_to_generic=gen - cost,
                                hard_theta_compatible=int(singletons == 5),
                                theta_residue_if_a5=max(0, singletons - 5),
                            )
                table[dim][z] = best_cost
                choice_table[dim][z] = best_choice
        self.costs.append(table)
        self.choices.append(choice_table)

    def cost(self, depth: int, dim: int, zeros: int) -> int:
        return self.costs[depth][dim][zeros]

    def choice(self, depth: int, dim: int, zeros: int) -> Choice | None:
        return self.choices[depth][dim][zeros]

    def trace(self, depth: int, dim: int, zeros: int) -> list[Choice]:
        out: list[Choice] = []
        while depth > 0 and dim > 0:
            choice = self.choice(depth, dim, zeros)
            if choice is None:
                break
            out.append(choice)
            depth = choice.child_depth
            dim = choice.child_dim
            zeros = choice.child_z
        return out

    def constrained_top_choice(
        self,
        *,
        depth: int,
        dim: int,
        paired: int,
        singletons: int,
    ) -> Choice | None:
        if depth <= 1:
            raise ValueError("constrained top choices are only recursive for depth >= 2")
        child_k = 1 << (depth - 1)
        child_n = self.expansion * child_k
        parent_k = 2 * child_k
        parent_z = 2 * paired + singletons
        if paired + singletons > child_n:
            return None
        best: Choice | None = None
        for forced in range(singletons + 1):
            if not self._split_allowed(singletons):
                return None
            child_z = paired + forced
            child_dim = ceil_div(dim + singletons - forced, 2)
            if child_dim > child_k:
                continue
            cost = self.costs[depth - 1][child_dim][child_z]
            if cost >= INF:
                continue
            if best is None or cost < best.cost:
                gen = generic_rank_cost(parent_k, dim, parent_z)
                best = Choice(
                    cost=cost,
                    parent_depth=depth,
                    parent_k=parent_k,
                    parent_z=parent_z,
                    parent_dim=dim,
                    paired=paired,
                    singletons=singletons,
                    forced_singletons=forced,
                    child_depth=depth - 1,
                    child_k=child_k,
                    child_z=child_z,
                    child_dim=child_dim,
                    generic_cost=gen,
                    slack_to_generic=gen - cost,
                    hard_theta_compatible=int(singletons == 5),
                    theta_residue_if_a5=max(0, singletons - 5),
                )
        return best


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--dim", type=int, required=True)
    parser.add_argument("--zeros", type=int, required=True)
    parser.add_argument("--paired", type=int)
    parser.add_argument("--singletons", type=int)
    parser.add_argument(
        "--allowed-singletons",
        help="comma-separated singleton counts allowed in recursive rho splits, e.g. 0,5",
    )
    parser.add_argument("--output-csv", type=Path)
    parser.add_argument("--trace", action="store_true")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    allowed_singletons = None
    if args.allowed_singletons:
        allowed_singletons = tuple(
            int(part.strip()) for part in args.allowed_singletons.split(",") if part.strip()
        )
    recurrence = ShortenedRankRecurrence(
        depth=args.depth,
        expansion=args.expansion,
        allowed_singletons=allowed_singletons,
    )
    k = 1 << args.depth
    n = args.expansion * k
    if args.dim < 0 or args.dim > k:
        raise SystemExit(f"--dim must be in [0,{k}]")
    if args.zeros < 0 or args.zeros > n:
        raise SystemExit(f"--zeros must be in [0,{n}]")

    top_choice: Choice | None = None
    if args.paired is not None or args.singletons is not None:
        if args.paired is None or args.singletons is None:
            raise SystemExit("--paired and --singletons must be supplied together")
        if 2 * args.paired + args.singletons != args.zeros:
            raise SystemExit("--zeros must equal 2*--paired + --singletons")
        top_choice = recurrence.constrained_top_choice(
            depth=args.depth,
            dim=args.dim,
            paired=args.paired,
            singletons=args.singletons,
        )
        cost = top_choice.cost if top_choice is not None else INF
    else:
        cost = recurrence.cost(args.depth, args.dim, args.zeros)
    generic = generic_rank_cost(k, args.dim, args.zeros)
    print(
        "depth,k,n,dim,zeros,optimistic_cost,generic_cost,generic_minus_optimistic",
        file=sys.stderr,
    )
    print(
        f"{args.depth},{k},{n},{args.dim},{args.zeros},{cost},{generic},{generic-cost}",
        file=sys.stderr,
    )

    if args.trace and top_choice is not None:
        rows = [top_choice] + recurrence.trace(
            top_choice.child_depth,
            top_choice.child_dim,
            top_choice.child_z,
        )
    elif args.trace:
        rows = recurrence.trace(args.depth, args.dim, args.zeros)
    else:
        rows = []
    if args.output_csv:
        with args.output_csv.open("w", newline="") as handle:
            if rows:
                writer = csv.DictWriter(handle, fieldnames=list(asdict(rows[0]).keys()))
                writer.writeheader()
                for row in rows:
                    writer.writerow(asdict(row))
            else:
                handle.write("")

    for row in rows:
        print(
            "h={parent_depth} k={parent_k} D={parent_dim} z={parent_z} "
            "cost={cost} generic={generic_cost} gap={slack_to_generic} "
            "split p={paired} s={singletons} forced={forced_singletons} "
            "hard_theta={hard_theta_compatible} residue_if_a5={theta_residue_if_a5} "
            "-> child D={child_dim} z={child_z}".format(**asdict(row))
        )


if __name__ == "__main__":
    main()
