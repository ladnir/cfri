#!/usr/bin/env python3
r"""Targeted carried-flag diagnostic for the depth-5 RFC checkpoint.

This is not a certificate. It reconstructs the corrected bound-following trace in
`rfc_flag_span_moment.py` and tests the first place where an inner-first child flag should be
carried into the next layer instead of being collapsed to a one-layer bound.
"""

from __future__ import annotations

import argparse
import csv
import sys
from dataclasses import dataclass
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from rfc_flag_span_moment import (  # noqa: E402
    NEG_INF,
    flag_child_bound_report,
    lift_flag_span_moment,
    log2_comb_table,
)


Choice = tuple[int, ...]
State = tuple[int, int]


@dataclass(frozen=True)
class LevelData:
    values: dict[int, list[float]]
    choices: dict[tuple[int, int], Choice | None]


def format_state(state: State) -> str:
    return f"({state[0]},{state[1]})"


def value_at(values: dict[int, list[float]], state: State) -> float:
    span, zeros = state
    row = values.get(span)
    if row is None or zeros < 0 or zeros >= len(row):
        return NEG_INF
    return row[zeros]


def choice_child_layers(choice: Choice) -> list[State]:
    p = choice[0]
    singleton_count = choice[1]
    outer_span = choice[4]
    inner_span = choice[5]
    outer_zeros = choice[6]
    inner_zeros = p + singleton_count
    layers = [(outer_span, outer_zeros)]
    if inner_span > 0:
        layers.append((inner_span, inner_zeros))
    return layers


def merge_equal_dimension_chain(layers: list[State]) -> list[State]:
    by_dim: dict[int, int] = {}
    for dim, zeros in layers:
        by_dim[dim] = max(by_dim.get(dim, -1), zeros)
    return sorted(by_dim.items(), reverse=True)


def build_levels(
    *,
    depth: int,
    expansion: int,
    q_log2: float,
    singleton_charge: str,
    flag_bound: str,
    max_visible_tau: int,
    cover_lift_mode: str,
    cover_kernel_lift: bool,
    prune_to_final_span: int,
) -> list[LevelData]:
    total_n = expansion * (1 << depth)
    comb = log2_comb_table(total_n)
    values: dict[int, list[float]] = {1: [NEG_INF] * (expansion + 1)}
    values[1][0] = 0.0
    levels = [LevelData(values={span: row[:] for span, row in values.items()}, choices={})]
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
            flag_bound,
            max_visible_tau,
            cover_lift_mode,
            cover_kernel_lift,
            max_parent_span,
        )
        levels.append(LevelData(values={span: row[:] for span, row in values.items()}, choices=choices))
    return levels


def trace_bound_path(
    *,
    levels: list[LevelData],
    depth: int,
    start_state: State,
    flag_bound: str,
    q_log2: float,
    expansion: int,
) -> list[tuple[int, State, Choice | None, str]]:
    out: list[tuple[int, State, Choice | None, str]] = []
    state = start_state
    for level in range(depth, 0, -1):
        choice = levels[level].choices.get(state)
        if choice is None:
            out.append((level, state, None, "none"))
            break
        p = choice[0]
        singleton_count = choice[1]
        outer_span = choice[4]
        inner_span = choice[5]
        outer_zeros = choice[6]
        inner_zeros = p + singleton_count
        child_bound_choice = "projection"
        if choice[3] > 0:
            label, _best, _outer, _inner = flag_child_bound_report(
                child_by_span=levels[level - 1].values,
                child_k=1 << (level - 1),
                child_n=expansion * (1 << (level - 1)),
                outer_span=outer_span,
                inner_span=inner_span,
                outer_zeros=outer_zeros,
                inner_zeros=inner_zeros,
                q_log2=q_log2,
                mode=flag_bound,
            )
            child_bound_choice = label
        out.append((level, state, choice, child_bound_choice))
        if child_bound_choice in ("inner-first", "shortened-inner-first") and inner_span > 0:
            state = (inner_span, inner_zeros)
        else:
            state = (outer_span, outer_zeros)
    return out


CSV_FIELDS = [
    "kind",
    "level",
    "state",
    "value_log2",
    "choice",
    "child_bound",
    "current_child_flag",
    "carried_child_flag",
    "current_bound_log2",
    "carried_bound_log2",
    "saving_log2",
    "diagram_bound_log2",
    "diagram_saving_log2",
    "note",
]


def emit_row(writer: csv.DictWriter, **row: object) -> None:
    writer.writerow({key: row.get(key, "") for key in CSV_FIELDS})


def split_shape_log2(choice: Choice, child_n: int, comb: list[list[float]]) -> float:
    p = choice[0]
    singleton_count = choice[1]
    visible_support_size = choice[2]
    outer_zeros = choice[6]
    return (
        float(singleton_count)
        + comb[outer_zeros][p]
        + comb[child_n - outer_zeros][visible_support_size]
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=5)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--singleton-charge", default="endpoint-tau2-layer-incidence")
    parser.add_argument("--flag-bound", default="best")
    parser.add_argument("--max-visible-tau", type=int, default=2)
    parser.add_argument("--cover-lift-mode", default="tau0")
    parser.add_argument("--cover-kernel-lift", action="store_true")
    parser.add_argument("--prune-to-final-span", type=int, default=1)
    parser.add_argument("--trace-span", type=int, default=1)
    parser.add_argument("--trace-z", type=int, default=34)
    args = parser.parse_args()

    levels = build_levels(
        depth=args.depth,
        expansion=args.expansion,
        q_log2=args.q_log2,
        singleton_charge=args.singleton_charge,
        flag_bound=args.flag_bound,
        max_visible_tau=args.max_visible_tau,
        cover_lift_mode=args.cover_lift_mode,
        cover_kernel_lift=args.cover_kernel_lift,
        prune_to_final_span=args.prune_to_final_span,
    )
    trace = trace_bound_path(
        levels=levels,
        depth=args.depth,
        start_state=(args.trace_span, args.trace_z),
        flag_bound=args.flag_bound,
        q_log2=args.q_log2,
        expansion=args.expansion,
    )

    writer = csv.DictWriter(sys.stdout, fieldnames=CSV_FIELDS, lineterminator="\n")
    writer.writeheader()
    for level, state, choice, child_bound in trace:
        value = value_at(levels[level].values, state)
        emit_row(
            writer,
            kind="trace",
            level=level,
            state=format_state(state),
            value_log2=f"{value:.8f}" if value > NEG_INF / 2 else "-inf",
            choice="none" if choice is None else ":".join(str(x) for x in choice),
            child_bound=child_bound,
        )

    # Target the first inner-first row on the corrected safe-tau0 trace:
    # F_3((4,7),(2,8)) is produced by the level-4 state (2,15).
    parent_level = 4
    outer_parent_state = (4, 7)
    inner_parent_state = (2, 8)
    outer_choice = levels[parent_level - 1].choices.get(outer_parent_state)
    inner_choice = levels[parent_level - 1].choices.get(inner_parent_state)
    if outer_choice is None or inner_choice is None:
        raise SystemExit("target carried-flag choices are unavailable")

    current_child_flag = ((4, 3), (2, 5))
    carried_layers = merge_equal_dimension_chain(
        choice_child_layers(outer_choice) + choice_child_layers(inner_choice)
    )
    carried_child_flag = tuple(carried_layers)
    child_level = parent_level - 2
    current_label, current_bound, _current_outer, _current_inner = flag_child_bound_report(
        child_by_span=levels[child_level].values,
        child_k=1 << child_level,
        child_n=args.expansion * (1 << child_level),
        outer_span=current_child_flag[0][0],
        inner_span=current_child_flag[1][0],
        outer_zeros=current_child_flag[0][1],
        inner_zeros=current_child_flag[1][1],
        q_log2=args.q_log2,
        mode=args.flag_bound,
    )
    carried_label, carried_bound, _carried_outer, _carried_inner = flag_child_bound_report(
        child_by_span=levels[child_level].values,
        child_k=1 << child_level,
        child_n=args.expansion * (1 << child_level),
        outer_span=carried_child_flag[0][0],
        inner_span=carried_child_flag[1][0],
        outer_zeros=carried_child_flag[0][1],
        inner_zeros=carried_child_flag[1][1],
        q_log2=args.q_log2,
        mode=args.flag_bound,
    )
    emit_row(
        writer,
        kind="carried_merge",
        level=3,
        state=f"{format_state(outer_parent_state)}>={format_state(inner_parent_state)}",
        choice=f"outer={':'.join(str(x) for x in outer_choice)};inner={':'.join(str(x) for x in inner_choice)}",
        child_bound=f"{current_label}->{carried_label}",
        current_child_flag=">=".join(format_state(state) for state in current_child_flag),
        carried_child_flag=">=".join(format_state(state) for state in carried_child_flag),
        current_bound_log2=f"{current_bound:.8f}",
        carried_bound_log2=f"{carried_bound:.8f}",
        saving_log2=f"{current_bound - carried_bound:.8f}",
        note="merge equal 4-dimensional projected layers; carries outer kernel zero budget",
    )

    next_outer_state = carried_child_flag[0]
    next_inner_state = carried_child_flag[1]
    next_outer_choice = levels[2].choices.get(next_outer_state)
    next_inner_choice = levels[2].choices.get(next_inner_state)
    if next_outer_choice is not None and next_inner_choice is not None:
        comb = log2_comb_table(args.expansion * (1 << args.depth))
        inner_shape_log2 = split_shape_log2(
            next_inner_choice,
            args.expansion * (1 << 1),
            comb,
        )
        outer_value = value_at(levels[2].values, next_outer_state)
        # Diagnostic q-exponent replacement for the current q^4 ancestor factor: after the
        # outer tau-two row fixes the child 2-plane and one marked line, the inner tau-zero
        # child line ranges over at most q+1 lines in that plane.  The small split factor is
        # included; finite root/constant refinements are deliberately not claimed here.
        two_marked_line_bound = outer_value + args.q_log2 + inner_shape_log2
        emit_row(
            writer,
            kind="next_diagram",
            level=2,
            state=f"{format_state(next_outer_state)}>={format_state(next_inner_state)}",
            value_log2=(
                f"outer={value_at(levels[2].values, next_outer_state):.8f};"
                f"inner={value_at(levels[2].values, next_inner_state):.8f}"
            ),
            choice=(
                f"outer={':'.join(str(x) for x in next_outer_choice)};"
                f"inner={':'.join(str(x) for x in next_inner_choice)}"
            ),
            carried_child_flag="(2,0) with marked lines (1,4) and (1,3)",
            current_bound_log2=f"{carried_bound:.8f}",
            diagram_bound_log2=f"{two_marked_line_bound:.8f}",
            diagram_saving_log2=f"{carried_bound - two_marked_line_bound:.8f}",
            note=(
                "outer tau2 row and inner tau0 row create two marked child lines inside one "
                "2-plane; q+1 line-choice diagnostic includes inner split factor only"
            ),
        )


if __name__ == "__main__":
    main()
