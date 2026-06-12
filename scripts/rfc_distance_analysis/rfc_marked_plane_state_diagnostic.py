#!/usr/bin/env python3
r"""Scan for marked-line-in-plane savings in the RFC flag recurrence.

This is a diagnostic, not a certificate.  It looks for two-layer child flags

    V >= L

where the dominant one-step row for `V` already creates a child 2-plane `P`
and one marked child line `M <= P`, while the dominant one-step row for `L`
creates another child line `N <= P` with tau zero.  In that situation the
coarse flag bound may pay for the lower layer as an ambient ancestor choice,
but after `P` is fixed the extra line costs at most q+1 choices.

The script reports the resulting local replacement:

    coarse flag bound  ->  carrier state value + inner local row + q

with finite constants intentionally left visible in the CSV columns.
"""

from __future__ import annotations

import argparse
import csv
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from rfc_carried_flag_diagnostic import (  # noqa: E402
    Choice,
    State,
    build_levels,
    format_state,
    split_shape_log2,
    value_at,
)
from rfc_flag_span_moment import (  # noqa: E402
    NEG_INF,
    flag_child_bound_report,
    log2_comb_table,
)
from rfc_diagram_state import (  # noqa: E402
    add_marked_line_in_plane,
    carrier_plane_with_line,
)


CSV_FIELDS = [
    "layer_level",
    "flag_level",
    "outer_state",
    "inner_state",
    "coarse_label",
    "coarse_bound_log2",
    "carrier_state_log2",
    "inner_local_log2",
    "line_choice_log2",
    "line_factor",
    "marked_plane_bound_log2",
    "saving_log2",
    "saving_qdim",
    "carrier_diagram",
    "next_diagram",
    "plane_zero_budget",
    "carrier_line_zero_budget",
    "extra_line_zero_budget",
    "outer_choice",
    "inner_choice",
    "note",
]


def format_choice(choice: Choice | None) -> str:
    if choice is None:
        return ""
    return ":".join(str(part) for part in choice)


def local_row_log2(choice: Choice, child_n: int, comb: list[list[float]], q_log2: float) -> float:
    """Return the non-recursive log2 contribution of one selected row."""

    local_charge = choice[7]
    lift_qdim = choice[14]
    return split_shape_log2(choice, child_n, comb) - local_charge * q_log2 + lift_qdim * q_log2


def is_plane_carrier(choice: Choice) -> bool:
    """Dominant row creates a child 2-plane with a marked child line."""

    outer_span = choice[4]
    inner_span = choice[5]
    return outer_span == 2 and inner_span == 1


def is_tau0_child_line(choice: Choice) -> bool:
    """Dominant row creates only a child line, with no visible singleton quotient."""

    visible_tau = choice[3]
    outer_span = choice[4]
    inner_span = choice[5]
    return visible_tau == 0 and outer_span == 1 and inner_span == 1


def emit_row(writer: csv.DictWriter, **row: object) -> None:
    writer.writerow({key: row.get(key, "") for key in CSV_FIELDS})


def parse_state(text: str) -> State:
    try:
        span_text, zero_text = text.split(",", 1)
        return int(span_text), int(zero_text)
    except ValueError as exc:
        raise argparse.ArgumentTypeError("state must have the form span,zeros") from exc


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
    parser.add_argument("--layer-level", type=int, default=-1)
    parser.add_argument("--limit", type=int, default=40)
    parser.add_argument("--only-positive", action="store_true")
    parser.add_argument("--min-saving-qdim", type=float, default=-1.0e100)
    parser.add_argument("--outer-state", type=parse_state)
    parser.add_argument("--inner-state", type=parse_state)
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

    comb = log2_comb_table(args.expansion * (1 << args.depth))
    rows: list[dict[str, object]] = []
    layer_levels = [args.layer_level] if args.layer_level >= 0 else list(range(1, args.depth))
    for layer_level in layer_levels:
        if layer_level <= 0 or layer_level >= len(levels):
            continue
        layer = levels[layer_level]
        child_n = args.expansion * (1 << (layer_level - 1))
        flag_child_n = args.expansion * (1 << layer_level)
        flag_child_k = 1 << layer_level

        states = sorted(layer.choices)
        for outer_state in states:
            if args.outer_state is not None and outer_state != args.outer_state:
                continue
            outer_choice = layer.choices.get(outer_state)
            if outer_choice is None or not is_plane_carrier(outer_choice):
                continue
            outer_value = value_at(layer.values, outer_state)
            if outer_value <= NEG_INF / 2:
                continue

            for inner_state in states:
                if args.inner_state is not None and inner_state != args.inner_state:
                    continue
                if inner_state[0] >= outer_state[0]:
                    continue
                if inner_state[1] < outer_state[1]:
                    continue
                inner_choice = layer.choices.get(inner_state)
                if inner_choice is None or not is_tau0_child_line(inner_choice):
                    continue

                coarse_label, coarse_bound, _outer_first, _inner_first = flag_child_bound_report(
                    child_by_span=layer.values,
                    child_k=flag_child_k,
                    child_n=flag_child_n,
                    outer_span=outer_state[0],
                    inner_span=inner_state[0],
                    outer_zeros=outer_state[1],
                    inner_zeros=inner_state[1],
                    q_log2=args.q_log2,
                    mode=args.flag_bound,
                )
                if coarse_bound <= NEG_INF / 2:
                    continue

                inner_local = local_row_log2(inner_choice, child_n, comb, args.q_log2)
                plane_zero_budget = outer_choice[6]
                carrier_line_zero_budget = outer_choice[0] + outer_choice[1]
                extra_line_zero_budget = inner_choice[6]
                carrier_diagram = carrier_plane_with_line(
                    plane_name="P",
                    line_name="M",
                    plane_zeros=plane_zero_budget,
                    line_zeros=carrier_line_zero_budget,
                )
                next_diagram, line_choice_qdim, line_factor = add_marked_line_in_plane(
                    carrier_diagram,
                    plane_name="P",
                    line_name="N",
                    line_zeros=extra_line_zero_budget,
                )
                line_choice = line_choice_qdim * args.q_log2
                marked_plane_bound = outer_value + inner_local + line_choice
                saving = coarse_bound - marked_plane_bound
                saving_qdim = saving / args.q_log2
                if args.only_positive and saving <= 0:
                    continue
                if saving_qdim < args.min_saving_qdim:
                    continue

                rows.append(
                    {
                        "layer_level": layer_level,
                        "flag_level": layer_level + 1,
                        "outer_state": format_state(outer_state),
                        "inner_state": format_state(inner_state),
                        "coarse_label": coarse_label,
                        "coarse_bound_log2": f"{coarse_bound:.8f}",
                        "carrier_state_log2": f"{outer_value:.8f}",
                        "inner_local_log2": f"{inner_local:.8f}",
                        "line_choice_log2": f"{line_choice:.8f}",
                        "line_factor": line_factor,
                        "marked_plane_bound_log2": f"{marked_plane_bound:.8f}",
                        "saving_log2": f"{saving:.8f}",
                        "saving_qdim": f"{saving_qdim:.8f}",
                        "carrier_diagram": carrier_diagram.key(),
                        "next_diagram": next_diagram.key(),
                        "plane_zero_budget": plane_zero_budget,
                        "carrier_line_zero_budget": carrier_line_zero_budget,
                        "extra_line_zero_budget": extra_line_zero_budget,
                        "outer_choice": format_choice(outer_choice),
                        "inner_choice": format_choice(inner_choice),
                        "note": (
                            "ordered extra line inside the carrier child plane; zero condition on "
                            "the extra line is ignored, so this is an upper-bound diagnostic"
                        ),
                    }
                )

    rows.sort(key=lambda row: float(row["saving_log2"]), reverse=True)
    writer = csv.DictWriter(sys.stdout, fieldnames=CSV_FIELDS, lineterminator="\n")
    writer.writeheader()
    for row in rows[: args.limit]:
        emit_row(writer, **row)


if __name__ == "__main__":
    main()
