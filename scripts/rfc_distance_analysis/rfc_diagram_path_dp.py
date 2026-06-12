#!/usr/bin/env python3
r"""Path diagnostic for carrying RFC child diagrams through the flag trace.

This is not a certificate.  It follows the current bound-selected trace from
`rfc_flag_span_moment.py`, carries each exposed two-layer child flag one more
level when possible, and then applies the marked-plane transition when the
carried child flag expands to two marked lines inside one child plane.

The goal is to test whether the local marked-plane brick compounds on the
actual dominant path, before building a full finite diagram-state recurrence.
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
    choice_child_layers,
    format_state,
    merge_equal_dimension_chain,
    trace_bound_path,
    value_at,
)
from rfc_diagram_state import two_layer_child_diagram  # noqa: E402
from rfc_flag_span_moment import (  # noqa: E402
    NEG_INF,
    flag_child_bound_report,
    log2_comb_table,
)
from rfc_marked_plane_state_diagnostic import (  # noqa: E402
    is_plane_carrier,
    is_tau0_child_line,
    local_row_log2,
)


CSV_FIELDS = [
    "kind",
    "source_level",
    "flag_level",
    "source_state",
    "source_child_bound",
    "current_flag",
    "carried_flag",
    "transition_diagram",
    "coarse_bound_log2",
    "replacement_bound_log2",
    "saving_log2",
    "saving_qdim",
    "cumulative_saving_log2",
    "adjusted_vector_log2",
    "residual_to_target_bits",
    "residual_to_target_qdim",
    "note",
]


def emit_row(writer: csv.DictWriter, **row: object) -> None:
    writer.writerow({key: row.get(key, "") for key in CSV_FIELDS})


def format_flag(layers: tuple[State, ...] | list[State]) -> str:
    return ">=".join(format_state(layer) for layer in layers)


def selected_child_state(choice: Choice, child_bound: str) -> State:
    p = choice[0]
    singleton_count = choice[1]
    outer_span = choice[4]
    inner_span = choice[5]
    outer_zeros = choice[6]
    inner_zeros = p + singleton_count
    if child_bound in ("inner-first", "shortened-inner-first") and inner_span > 0:
        return inner_span, inner_zeros
    return outer_span, outer_zeros


def flag_bound_for_layers(
    *,
    levels,
    flag_level: int,
    expansion: int,
    q_log2: float,
    flag_bound: str,
    layers: tuple[State, ...] | list[State],
) -> tuple[str, float]:
    if len(layers) == 0:
        return "empty", NEG_INF
    if len(layers) == 1:
        return "scalar", value_at(levels[flag_level].values, layers[0])
    if len(layers) != 2:
        return "unsupported", NEG_INF
    outer_state, inner_state = layers
    label, bound, _outer_first, _inner_first = flag_child_bound_report(
        child_by_span=levels[flag_level].values,
        child_k=1 << flag_level,
        child_n=expansion * (1 << flag_level),
        outer_span=outer_state[0],
        inner_span=inner_state[0],
        outer_zeros=outer_state[1],
        inner_zeros=inner_state[1],
        q_log2=q_log2,
        mode=flag_bound,
    )
    return label, bound


def diagram_replacement_bound(
    *,
    levels,
    flag_level: int,
    expansion: int,
    q_log2: float,
    comb: list[list[float]],
    carried_layers: tuple[State, ...],
) -> tuple[float, str, str]:
    """Return a marked-plane replacement for a two-layer carried flag."""

    if len(carried_layers) != 2 or flag_level <= 0:
        return NEG_INF, "", "carried flag is not a two-layer positive-depth chain"
    outer_state, inner_state = carried_layers
    outer_choice = levels[flag_level].choices.get(outer_state)
    inner_choice = levels[flag_level].choices.get(inner_state)
    if outer_choice is None or inner_choice is None:
        return NEG_INF, "", "missing dominant choices for carried flag layers"
    if not is_plane_carrier(outer_choice):
        return NEG_INF, "", "outer carried layer is not a plane-plus-line carrier"
    if not is_tau0_child_line(inner_choice):
        return NEG_INF, "", "inner carried layer is not a tau-zero child line"

    transition_diagram = two_layer_child_diagram(
        outer_choice=outer_choice,
        inner_choice=inner_choice,
    )
    child_n = expansion * (1 << (flag_level - 1))
    outer_value = value_at(levels[flag_level].values, outer_state)
    inner_local = local_row_log2(inner_choice, child_n, comb, q_log2)
    return (
        outer_value + inner_local + q_log2,
        transition_diagram.key(),
        "marked-plane q+1 replacement",
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=5)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
    parser.add_argument("--singleton-charge", default="endpoint-tau2-layer-incidence")
    parser.add_argument("--flag-bound", default="best")
    parser.add_argument("--max-visible-tau", type=int, default=2)
    parser.add_argument("--cover-lift-mode", default="tau0")
    parser.add_argument("--cover-kernel-lift", action="store_true")
    parser.add_argument("--prune-to-final-span", type=int, default=1)
    parser.add_argument("--trace-span", type=int, default=1)
    parser.add_argument("--trace-z", type=int, default=34)
    parser.add_argument("--positive-only", action="store_true")
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
    comb = log2_comb_table(args.expansion * (1 << args.depth))
    top_state = (args.trace_span, args.trace_z)
    original_state = value_at(levels[args.depth].values, top_state)
    original_vector = original_state + args.q_log2
    cumulative_saving = 0.0

    writer = csv.DictWriter(sys.stdout, fieldnames=CSV_FIELDS, lineterminator="\n")
    writer.writeheader()
    for source_level, source_state, choice, child_bound in trace:
        if choice is None or source_level < 2:
            continue
        inner_span = choice[5]
        if inner_span <= 0:
            continue
        outer_state = (choice[4], choice[6])
        inner_state = (inner_span, choice[0] + choice[1])
        outer_choice = levels[source_level - 1].choices.get(outer_state)
        inner_choice = levels[source_level - 1].choices.get(inner_state)
        if outer_choice is None or inner_choice is None:
            continue

        current_state = selected_child_state(choice, child_bound)
        current_choice = levels[source_level - 1].choices.get(current_state)
        if current_choice is None:
            continue
        current_layers = tuple(merge_equal_dimension_chain(choice_child_layers(current_choice)))
        carried_layers = tuple(
            merge_equal_dimension_chain(
                choice_child_layers(outer_choice) + choice_child_layers(inner_choice)
            )
        )
        flag_level = source_level - 2
        _current_label, current_bound = flag_bound_for_layers(
            levels=levels,
            flag_level=flag_level,
            expansion=args.expansion,
            q_log2=args.q_log2,
            flag_bound=args.flag_bound,
            layers=current_layers,
        )
        _carried_label, carried_bound = flag_bound_for_layers(
            levels=levels,
            flag_level=flag_level,
            expansion=args.expansion,
            q_log2=args.q_log2,
            flag_bound=args.flag_bound,
            layers=carried_layers,
        )
        if current_bound <= NEG_INF / 2 or carried_bound <= NEG_INF / 2:
            continue
        carry_saving = current_bound - carried_bound
        if carry_saving > 0 or not args.positive_only:
            cumulative_saving += max(0.0, carry_saving)
            adjusted_vector = original_vector - cumulative_saving
            residual = adjusted_vector + args.security_bits
            emit_row(
                writer,
                kind="carry_merge",
                source_level=source_level,
                flag_level=flag_level,
                source_state=format_state(source_state),
                source_child_bound=child_bound,
                current_flag=format_flag(current_layers),
                carried_flag=format_flag(carried_layers),
                coarse_bound_log2=f"{current_bound:.8f}",
                replacement_bound_log2=f"{carried_bound:.8f}",
                saving_log2=f"{carry_saving:.8f}",
                saving_qdim=f"{carry_saving / args.q_log2:.8f}",
                cumulative_saving_log2=f"{cumulative_saving:.8f}",
                adjusted_vector_log2=f"{adjusted_vector:.8f}",
                residual_to_target_bits=f"{residual:.8f}",
                residual_to_target_qdim=f"{residual / args.q_log2:.8f}",
                note="carry both child layers before collapsing equal dimensions",
            )

        diagram_bound, diagram_key, diagram_note = diagram_replacement_bound(
            levels=levels,
            flag_level=flag_level,
            expansion=args.expansion,
            q_log2=args.q_log2,
            comb=comb,
            carried_layers=carried_layers,
        )
        if diagram_bound <= NEG_INF / 2:
            continue
        diagram_saving = carried_bound - diagram_bound
        if diagram_saving <= 0 and args.positive_only:
            continue
        cumulative_saving += max(0.0, diagram_saving)
        adjusted_vector = original_vector - cumulative_saving
        residual = adjusted_vector + args.security_bits
        emit_row(
            writer,
            kind="marked_plane",
            source_level=source_level,
            flag_level=flag_level,
            source_state=format_state(source_state),
            source_child_bound=child_bound,
            carried_flag=format_flag(carried_layers),
            transition_diagram=diagram_key,
            coarse_bound_log2=f"{carried_bound:.8f}",
            replacement_bound_log2=f"{diagram_bound:.8f}",
            saving_log2=f"{diagram_saving:.8f}",
            saving_qdim=f"{diagram_saving / args.q_log2:.8f}",
            cumulative_saving_log2=f"{cumulative_saving:.8f}",
            adjusted_vector_log2=f"{adjusted_vector:.8f}",
            residual_to_target_bits=f"{residual:.8f}",
            residual_to_target_qdim=f"{residual / args.q_log2:.8f}",
            note=diagram_note,
        )


if __name__ == "__main__":
    main()
