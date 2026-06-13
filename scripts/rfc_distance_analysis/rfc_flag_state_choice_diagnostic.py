#!/usr/bin/env python3
r"""Targeted flag-state choice diagnostic for RFC distance work.

The two-layer value table in `rfc_flag_span_moment.py` still expands flag
states using the dominant scalar choices of each layer.  This script tests the
next possibility: for one requested flag state, enumerate scalar expansion
candidates for the outer and inner layers, combine them as a joint flag
expansion, and score the carried child flag directly.

This is diagnostic only.  The pair enumeration deliberately overcounts local
split data and does not prove compatibility of exact witnesses.  Its job is to
tell us whether the missing saving is even visible once flag states get their
own expansion choices.
"""

from __future__ import annotations

import argparse
import csv
import sys
from dataclasses import dataclass
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from rfc_flag_span_moment import (  # noqa: E402
    INF,
    NEG_INF,
    Choice,
    FlagTable,
    State,
    choice_child_layers,
    choice_local_row_log2,
    compute_two_layer_flag_table,
    flag_child_bound_report,
    flag_table_bound_for_layers,
    get_child_value,
    local_visible_trace_profile,
    log2_add,
    log2_comb_table,
    merge_equal_dimension_chain,
    marked_line_child_bound,
    support2_component_plane_saving_qdim,
    support2_root_kernel_cover_saving_qdim,
    support3_component_plane_saving_qdim,
    support4_root_kernel_cover_saving_qdim,
    tau1_child_line_carry_saving_qdim,
    tau1_root_kernel_cover_saving_qdim,
)


@dataclass(frozen=True)
class LevelData:
    values: dict[int, list[float]]
    choices: dict[tuple[int, int], Choice | None]
    flag_table: FlagTable | None


@dataclass(frozen=True)
class TermCandidate:
    choice: Choice
    local_log2: float
    child_log2: float
    term_log2: float
    child_layers: tuple[State, ...]


CSV_FIELDS = [
    "rank",
    "level",
    "outer_state",
    "inner_state",
    "joint_log2",
    "coarse_log2",
    "saving_log2",
    "saving_qdim",
    "child_flag",
    "child_flag_log2",
    "outer_local_log2",
    "inner_local_log2",
    "outer_choice",
    "inner_choice",
    "note",
]


def parse_state(text: str) -> State:
    parts = text.split(",")
    if len(parts) != 2:
        raise argparse.ArgumentTypeError("state must be span,zeros")
    try:
        return int(parts[0]), int(parts[1])
    except ValueError as exc:
        raise argparse.ArgumentTypeError("state entries must be integers") from exc


def format_state(state: State) -> str:
    return f"({state[0]},{state[1]})"


def format_flag(layers: tuple[State, ...] | list[State]) -> str:
    return ">=".join(format_state(layer) for layer in layers)


def format_choice(choice: Choice) -> str:
    return ":".join(str(part) for part in choice)


def covers_lift(mode: str, visible_tau: int) -> bool:
    if mode == "all":
        return True
    if mode == "tau0":
        return visible_tau == 0
    if mode == "tau1":
        return visible_tau == 1
    if mode == "tau2":
        return visible_tau == 2
    if mode == "tau0tau1":
        return visible_tau in (0, 1)
    if mode == "tau0tau2":
        return visible_tau in (0, 2)
    if mode == "tau1tau2":
        return visible_tau in (1, 2)
    return False


def enumerate_terms_for_state(
    *,
    child_by_span: dict[int, list[float]],
    child_choices: dict[tuple[int, int], tuple[int, ...] | None] | None,
    child_flag_table: FlagTable | None,
    comb: list[list[float]],
    q_log2: float,
    child_k: int,
    parent_span: int,
    zeros: int,
    singleton_charge_mode: str,
    flag_bound_mode: str,
    max_visible_tau: int,
    cover_lift_mode: str,
    cover_kernel_lift: bool,
    exclude_collapsed_active: bool = False,
    support2_line_filter: bool = False,
    support2_root_kernel_cover_mode: str = "none",
    support2_component_plane_mode: str = "none",
    support3_component_plane_mode: str = "none",
    support4_root_kernel_cover_mode: str = "none",
    tau1_child_line_carry_mode: str = "none",
    tau1_root_kernel_cover_mode: str = "none",
) -> list[TermCandidate]:
    child_n = len(next(iter(child_by_span.values()))) - 1
    child_spans = sorted(child_by_span)
    terms: list[TermCandidate] = []
    local_profile_cache: dict[tuple[int, int, int, int], tuple[int, int, int, int, int, int, int]] = {}

    def cached_local_profile(
        visible_tau: int,
        outer_zero_count: int,
        visible_support_size: int,
        quotient_delta_floor: int = 0,
    ) -> tuple[int, int, int, int, int, int, int]:
        key = (visible_tau, outer_zero_count, visible_support_size, quotient_delta_floor)
        if key not in local_profile_cache:
            local_profile_cache[key] = local_visible_trace_profile(
                visible_tau=visible_tau,
                child_k=child_k,
                outer_zero_count=outer_zero_count,
                visible_support_size=visible_support_size,
                singleton_charge_mode=singleton_charge_mode,
                quotient_delta_floor=quotient_delta_floor,
            )
        return local_profile_cache[key]

    for p in range(zeros // 2 + 1):
        singleton_count = zeros - 2 * p
        if p + singleton_count > child_n:
            continue
        orientation_log = float(singleton_count)
        max_visible_support = min(singleton_count, child_n - p)
        for visible_support_size in range(max_visible_support + 1):
            outer_zeros = p + singleton_count - visible_support_size
            inner_zeros = p + singleton_count
            if outer_zeros > child_n:
                continue
            if visible_support_size > child_n - outer_zeros:
                continue
            shape_log = (
                orientation_log
                + comb[outer_zeros][p]
                + comb[child_n - outer_zeros][visible_support_size]
            )
            max_tau = min(parent_span, max_visible_tau)
            for visible_tau in range(max_tau + 1):
                kernel_dim = parent_span - visible_tau
                if visible_tau == 0 and visible_support_size != 0:
                    continue
                if visible_tau > 0 and visible_support_size == 0:
                    continue
                (
                    charge,
                    local_delta,
                    local_components,
                    _local_g,
                    local_theta,
                    local_h,
                    local_gamma,
                ) = cached_local_profile(
                    visible_tau,
                    outer_zeros,
                    visible_support_size,
                )
                incidence_mode = singleton_charge_mode == "endpoint-tau2-layer-incidence"
                if charge >= INF and not incidence_mode:
                    continue
                if kernel_dim == 0:
                    inner_span_candidates = [0]
                else:
                    inner_span_candidates = [
                        r0
                        for r0 in child_spans
                        if kernel_dim <= 2 * r0 and r0 <= 2 * kernel_dim
                    ]
                for inner_span in inner_span_candidates:
                    if visible_tau == 0:
                        outer_span_candidates = [inner_span]
                    else:
                        outer_span_candidates = [
                            r1
                            for r1 in child_spans
                            if inner_span <= r1
                            and parent_span <= 2 * r1
                            and r1 <= 2 * parent_span
                        ]
                    for outer_span in outer_span_candidates:
                        if outer_span == 0:
                            continue
                        if (
                            exclude_collapsed_active
                            and visible_tau > 0
                            and visible_support_size > 0
                            and inner_span == outer_span
                            and inner_zeros > outer_zeros
                        ):
                            continue
                        if incidence_mode:
                            quotient_delta_floor = (
                                max(0, outer_span - inner_span)
                                if visible_tau == 2
                                else 0
                            )
                            (
                                charge,
                                local_delta,
                                local_components,
                                _local_g,
                                local_theta,
                                local_h,
                                local_gamma,
                            ) = cached_local_profile(
                                visible_tau,
                                outer_zeros,
                                visible_support_size,
                                quotient_delta_floor,
                            )
                            if charge >= INF:
                                continue
                        if incidence_mode and visible_tau == 2:
                            grassmann_charge = visible_support_size
                            if grassmann_charge > charge:
                                charge = grassmann_charge
                                local_theta = 4 * local_delta - 4 - charge
                                local_h = -2
                                local_gamma = -2
                        if (
                            support2_line_filter
                            and visible_tau == 2
                            and visible_support_size == 2
                            and local_delta == 2
                            and local_components == 2
                            and outer_span == inner_span + 1
                        ):
                            continue
                        if visible_tau == 0:
                            lift_log = parent_span * (2 * outer_span - parent_span) * q_log2
                            if covers_lift(cover_lift_mode, visible_tau):
                                lift_log = 0.0
                            child_log = get_child_value(
                                child_by_span,
                                child_n,
                                outer_span,
                                outer_zeros,
                            )
                        else:
                            kernel_lift_log = kernel_dim * (2 * inner_span - kernel_dim) * q_log2
                            if cover_kernel_lift:
                                kernel_lift_log = 0.0
                            quotient_lift_log = visible_tau * (2 * outer_span - parent_span) * q_log2
                            lift_log = kernel_lift_log + quotient_lift_log
                            if covers_lift(cover_lift_mode, visible_tau):
                                lift_log = 0.0
                            else:
                                if visible_tau == 1:
                                    lift_log -= (
                                        tau1_root_kernel_cover_saving_qdim(
                                            mode=tau1_root_kernel_cover_mode,
                                            parent_span=parent_span,
                                            visible_tau=visible_tau,
                                            outer_span=outer_span,
                                            local_charge=charge,
                                        )
                                        * q_log2
                                    )
                                component_plane_saving = support2_component_plane_saving_qdim(
                                    mode=support2_component_plane_mode,
                                    parent_span=parent_span,
                                    visible_tau=visible_tau,
                                    visible_support_size=visible_support_size,
                                    kernel_dim=kernel_dim,
                                    inner_span=inner_span,
                                    outer_span=outer_span,
                                    local_delta=local_delta,
                                    local_components=local_components,
                                )
                                component_plane_saving += support3_component_plane_saving_qdim(
                                    mode=support3_component_plane_mode,
                                    parent_span=parent_span,
                                    visible_tau=visible_tau,
                                    visible_support_size=visible_support_size,
                                    kernel_dim=kernel_dim,
                                    inner_span=inner_span,
                                    outer_span=outer_span,
                                    local_delta=local_delta,
                                    local_components=local_components,
                                )
                                component_plane_saving += support4_root_kernel_cover_saving_qdim(
                                    mode=support4_root_kernel_cover_mode,
                                    parent_span=parent_span,
                                    visible_tau=visible_tau,
                                    visible_support_size=visible_support_size,
                                    kernel_dim=kernel_dim,
                                    inner_span=inner_span,
                                    outer_span=outer_span,
                                    local_delta=local_delta,
                                    local_components=local_components,
                                )
                                current_lift_qdim = (
                                    int(round(lift_log / q_log2)) - component_plane_saving
                                )
                                component_plane_saving += support2_root_kernel_cover_saving_qdim(
                                    mode=support2_root_kernel_cover_mode,
                                    parent_span=parent_span,
                                    visible_tau=visible_tau,
                                    visible_support_size=visible_support_size,
                                    kernel_dim=kernel_dim,
                                    inner_span=inner_span,
                                    outer_span=outer_span,
                                    local_delta=local_delta,
                                    local_components=local_components,
                                    current_lift_qdim=current_lift_qdim,
                                    local_charge=charge,
                                )
                                lift_log -= component_plane_saving * q_log2
                            child_log = flag_table_bound_for_layers(
                                values_by_span=child_by_span,
                                flag_table=child_flag_table,
                                child_k=child_k,
                                child_n=child_n,
                                q_log2=q_log2,
                                layers=((outer_span, outer_zeros), (inner_span, inner_zeros)),
                            )
                        if (
                            visible_tau == 1
                            and child_choices is not None
                            and tau1_child_line_carry_mode != "none"
                        ):
                            child_choice = child_choices.get((outer_span, outer_zeros))
                            carry_saving = tau1_child_line_carry_saving_qdim(
                                mode=tau1_child_line_carry_mode,
                                parent_visible_tau=visible_tau,
                                child_parent_span=outer_span,
                                child_choice=child_choice,
                            )
                            child_log -= carry_saving * q_log2
                        if child_log <= NEG_INF / 2:
                            continue
                        marked_line_used = False
                        if (
                            incidence_mode
                            and parent_span == 2
                            and visible_tau == 2
                            and kernel_dim == 0
                            and outer_span == 2
                            and inner_span == 0
                            and visible_support_size == 2
                            and local_delta == 2
                            and local_components == 2
                        ):
                            marked_child_log = marked_line_child_bound(
                                child_by_span=child_by_span,
                                child_k=child_k,
                                child_n=child_n,
                                outer_span=outer_span,
                                outer_zeros=outer_zeros,
                                q_log2=q_log2,
                            )
                            if marked_child_log > NEG_INF / 2:
                                marked_child_log += q_log2
                                if marked_child_log < child_log:
                                    child_log = marked_child_log
                                    marked_line_used = True
                        choice: Choice = (
                            p,
                            singleton_count,
                            visible_support_size,
                            visible_tau,
                            outer_span,
                            inner_span,
                            outer_zeros,
                            charge,
                            local_delta,
                            local_components,
                            0,
                            local_theta,
                            -4 if marked_line_used else local_h,
                            -4 if marked_line_used else local_gamma,
                            int(round(lift_log / q_log2)),
                        )
                        local_log = shape_log - charge * q_log2 + lift_log
                        terms.append(
                            TermCandidate(
                                choice=choice,
                                local_log2=local_log,
                                child_log2=child_log,
                                term_log2=local_log + child_log,
                                child_layers=tuple(merge_equal_dimension_chain(choice_child_layers(choice))),
                            )
                        )
    terms.sort(key=lambda term: term.term_log2, reverse=True)
    return terms


def build_levels(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    singleton_charge: str,
    max_visible_tau: int,
    cover_lift_mode: str,
    cover_kernel_lift: bool,
    prune_to_final_span: int,
) -> list[LevelData]:
    from rfc_flag_span_moment import lift_flag_span_moment

    total_n = expansion * (1 << depth)
    comb = log2_comb_table(total_n)
    values: dict[int, list[float]] = {1: [NEG_INF] * (expansion + 1)}
    values[1][0] = 0.0
    levels = [LevelData(values={span: row[:] for span, row in values.items()}, choices={}, flag_table=None)]
    previous_choices = None
    previous_flag_table = None
    previous_values = {span: row[:] for span, row in values.items()}
    for level in range(1, depth + 1):
        max_parent_span = None
        if prune_to_final_span > 0:
            max_parent_span = prune_to_final_span * (1 << (depth - level))
        values, choices = lift_flag_span_moment(
            values,
            comb,
            q_log2,
            1 << (level - 1),
            singleton_charge,
            "best-two-layer-table",
            max_visible_tau,
            cover_lift_mode,
            cover_kernel_lift,
            max_parent_span,
            previous_choices,
            previous_flag_table,
        )
        flag_table = compute_two_layer_flag_table(
            values_by_span=values,
            choices=choices,
            previous_values_by_span=previous_values,
            previous_flag_table=previous_flag_table,
            comb=comb,
            q_log2=q_log2,
            level=level,
            expansion=expansion,
        )
        levels.append(
            LevelData(
                values={span: row[:] for span, row in values.items()},
                choices=choices,
                flag_table=flag_table,
            )
        )
        previous_choices = choices
        previous_flag_table = flag_table
        previous_values = {span: row[:] for span, row in values.items()}
    return levels


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=5)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--singleton-charge", default="endpoint-tau2-layer-incidence")
    parser.add_argument("--max-visible-tau", type=int, default=2)
    parser.add_argument("--cover-lift-mode", default="tau0")
    parser.add_argument("--cover-kernel-lift", action="store_true")
    parser.add_argument("--prune-to-final-span", type=int, default=1)
    parser.add_argument("--level", type=int, default=3)
    parser.add_argument("--outer-state", type=parse_state, default=(4, 7))
    parser.add_argument("--inner-state", type=parse_state, default=(2, 8))
    parser.add_argument("--term-limit", type=int, default=80)
    parser.add_argument("--pair-limit", type=int, default=20)
    args = parser.parse_args()

    levels = build_levels(
        depth=args.depth,
        expansion=args.expansion,
        q_log2=args.q_log2,
        singleton_charge=args.singleton_charge,
        max_visible_tau=args.max_visible_tau,
        cover_lift_mode=args.cover_lift_mode,
        cover_kernel_lift=args.cover_kernel_lift,
        prune_to_final_span=args.prune_to_final_span,
    )
    if args.level <= 0 or args.level >= len(levels):
        raise SystemExit("--level must be between 1 and depth")

    comb = log2_comb_table(args.expansion * (1 << args.depth))
    child = levels[args.level - 1]
    child_k = 1 << (args.level - 1)
    child_n = args.expansion * (1 << (args.level - 1))
    outer_terms = enumerate_terms_for_state(
        child_by_span=child.values,
        child_choices=child.choices,
        child_flag_table=child.flag_table,
        comb=comb,
        q_log2=args.q_log2,
        child_k=child_k,
        parent_span=args.outer_state[0],
        zeros=args.outer_state[1],
        singleton_charge_mode=args.singleton_charge,
        flag_bound_mode="best-two-layer-table",
        max_visible_tau=args.max_visible_tau,
        cover_lift_mode=args.cover_lift_mode,
        cover_kernel_lift=args.cover_kernel_lift,
    )[: args.term_limit]
    inner_terms = enumerate_terms_for_state(
        child_by_span=child.values,
        child_choices=child.choices,
        child_flag_table=child.flag_table,
        comb=comb,
        q_log2=args.q_log2,
        child_k=child_k,
        parent_span=args.inner_state[0],
        zeros=args.inner_state[1],
        singleton_charge_mode=args.singleton_charge,
        flag_bound_mode="best-two-layer-table",
        max_visible_tau=args.max_visible_tau,
        cover_lift_mode=args.cover_lift_mode,
        cover_kernel_lift=args.cover_kernel_lift,
    )[: args.term_limit]

    _label, coarse, _outer_first, _inner_first = flag_child_bound_report(
        child_by_span=levels[args.level].values,
        child_k=1 << args.level,
        child_n=args.expansion * (1 << args.level),
        outer_span=args.outer_state[0],
        inner_span=args.inner_state[0],
        outer_zeros=args.outer_state[1],
        inner_zeros=args.inner_state[1],
        q_log2=args.q_log2,
        mode="best",
    )
    table_value = levels[args.level].flag_table.get((args.outer_state, args.inner_state), NEG_INF)

    rows: list[dict[str, object]] = []
    pair_sum = NEG_INF
    for outer in outer_terms:
        for inner in inner_terms:
            child_layers = tuple(
                merge_equal_dimension_chain(list(outer.child_layers) + list(inner.child_layers))
            )
            child_flag_log = flag_table_bound_for_layers(
                values_by_span=child.values,
                flag_table=child.flag_table,
                child_k=child_k,
                child_n=child_n,
                q_log2=args.q_log2,
                layers=child_layers,
            )
            if child_flag_log <= NEG_INF / 2:
                continue
            joint = outer.local_log2 + inner.local_log2 + child_flag_log
            pair_sum = log2_add(pair_sum, joint)
            rows.append(
                {
                    "joint_log2": joint,
                    "saving_log2": coarse - joint,
                    "saving_qdim": (coarse - joint) / args.q_log2,
                    "child_flag": format_flag(child_layers),
                    "child_flag_log2": child_flag_log,
                    "outer_local_log2": outer.local_log2,
                    "inner_local_log2": inner.local_log2,
                    "outer_choice": format_choice(outer.choice),
                    "inner_choice": format_choice(inner.choice),
                }
            )
    optimistic_rows = sorted(rows, key=lambda row: float(row["joint_log2"]))
    dominant_rows = sorted(rows, key=lambda row: float(row["joint_log2"]), reverse=True)

    writer = csv.DictWriter(sys.stdout, fieldnames=CSV_FIELDS, lineterminator="\n")
    writer.writeheader()
    writer.writerow(
        {
            "rank": "baseline",
            "level": args.level,
            "outer_state": format_state(args.outer_state),
            "inner_state": format_state(args.inner_state),
            "joint_log2": f"{table_value:.8f}" if table_value > NEG_INF / 2 else "-inf",
            "coarse_log2": f"{coarse:.8f}" if coarse > NEG_INF / 2 else "-inf",
            "saving_log2": f"{coarse - table_value:.8f}" if table_value > NEG_INF / 2 else "",
            "saving_qdim": f"{(coarse - table_value) / args.q_log2:.8f}" if table_value > NEG_INF / 2 else "",
            "note": "dominant-choice two-layer table value",
        }
    )
    writer.writerow(
        {
            "rank": "pair_sum",
            "level": args.level,
            "outer_state": format_state(args.outer_state),
            "inner_state": format_state(args.inner_state),
            "joint_log2": f"{pair_sum:.8f}" if pair_sum > NEG_INF / 2 else "-inf",
            "coarse_log2": f"{coarse:.8f}" if coarse > NEG_INF / 2 else "-inf",
            "saving_log2": f"{coarse - pair_sum:.8f}" if pair_sum > NEG_INF / 2 else "",
            "saving_qdim": f"{(coarse - pair_sum) / args.q_log2:.8f}" if pair_sum > NEG_INF / 2 else "",
            "note": (
                "log-sum over enumerated pair products only; omitted pairs may increase this, "
                "so treat as a lower estimate of the pair-sum upper bound"
            ),
        }
    )
    if optimistic_rows:
        row = optimistic_rows[0]
        writer.writerow(
            {
                "rank": "optimistic_best",
                "level": args.level,
                "outer_state": format_state(args.outer_state),
                "inner_state": format_state(args.inner_state),
                "joint_log2": f"{row['joint_log2']:.8f}",
                "coarse_log2": f"{coarse:.8f}" if coarse > NEG_INF / 2 else "-inf",
                "saving_log2": f"{row['saving_log2']:.8f}",
                "saving_qdim": f"{row['saving_qdim']:.8f}",
                "child_flag": row["child_flag"],
                "child_flag_log2": f"{row['child_flag_log2']:.8f}",
                "outer_local_log2": f"{row['outer_local_log2']:.8f}",
                "inner_local_log2": f"{row['inner_local_log2']:.8f}",
                "outer_choice": row["outer_choice"],
                "inner_choice": row["inner_choice"],
                "note": "minimum enumerated pair; optimistic unless a canonical selection rule is proved",
            }
        )
    for rank, row in enumerate(dominant_rows[: args.pair_limit], start=1):
        writer.writerow(
            {
                "rank": f"dominant_{rank}",
                "level": args.level,
                "outer_state": format_state(args.outer_state),
                "inner_state": format_state(args.inner_state),
                "joint_log2": f"{row['joint_log2']:.8f}",
                "coarse_log2": f"{coarse:.8f}" if coarse > NEG_INF / 2 else "-inf",
                "saving_log2": f"{row['saving_log2']:.8f}",
                "saving_qdim": f"{row['saving_qdim']:.8f}",
                "child_flag": row["child_flag"],
                "child_flag_log2": f"{row['child_flag_log2']:.8f}",
                "outer_local_log2": f"{row['outer_local_log2']:.8f}",
                "inner_local_log2": f"{row['inner_local_log2']:.8f}",
                "outer_choice": row["outer_choice"],
                "inner_choice": row["inner_choice"],
                "note": "largest enumerated pair contribution",
            }
        )


if __name__ == "__main__":
    main()
