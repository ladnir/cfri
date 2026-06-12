#!/usr/bin/env python3
r"""Diagnose support-two tau-two quotient-frame rows.

This script is deliberately diagnostic, not a certificate mode.  It reuses the
pair-table stress rows and asks what would happen if a decomposable
support-two tau-two quotient row were routed through the child diamond

    V >= M1,M2 >= L

instead of a bare two-layer child flag V >= L.

Two estimates are printed:

* child_only keeps the current local quotient-lift exponent and only replaces
  the child flag by a quotient-frame child diagram bound.
* replace_quotient also removes the outer quotient-plane lift.  This is the
  stronger local theorem target and should not be read as proved by this
  script.
"""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))

from rfc_diagram_state import quotient_diamond  # noqa: E402
from rfc_flag_bad_pair_classifier import (  # noqa: E402
    build_pair_rows,
    choice_kernel_lift_qdim,
    choice_quotient_lift_qdim,
    choice_view,
)
from rfc_flag_span_moment import (  # noqa: E402
    NEG_INF,
    State,
    flag_table_bound_for_layers,
    get_child_value,
    log2_add,
    log2_comb_table,
)
from rfc_flag_state_choice_diagnostic import (  # noqa: E402
    enumerate_terms_for_state,
    format_choice,
    format_state,
)
from rfc_pair_flag_table_recurrence import (  # noqa: E402
    build_pair_levels,
    take_terms,
)


def qbinom_exponent(sub_dim: int, ambient_dim: int) -> int:
    if sub_dim < 0 or sub_dim > ambient_dim:
        return 10**9
    return sub_dim * (ambient_dim - sub_dim)


def two_layer_bound(
    *,
    values_by_span: dict[int, list[float]],
    flag_table: dict[tuple[State, State], float] | None,
    child_k: int,
    child_n: int,
    q_log2: float,
    outer: State,
    inner: State,
) -> float:
    return flag_table_bound_for_layers(
        values_by_span=values_by_span,
        flag_table=flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        layers=(outer, inner),
    )


def three_layer_chain_bound(
    *,
    values_by_span: dict[int, list[float]],
    flag_table: dict[tuple[State, State], float] | None,
    child_k: int,
    child_n: int,
    q_log2: float,
    top: State,
    middle: State,
    bottom: State,
) -> tuple[float, str]:
    """Safe coarse bound for a chain top >= middle >= bottom.

    The bound is the minimum of choosing a scalar layer or an available
    two-layer flag and extending the missing layers by Gaussian-binomial
    counts.  Zero requirements on missing layers are ignored, so each candidate
    is an upper bound.
    """

    d0, _z0 = top
    d1, _z1 = middle
    d2, _z2 = bottom
    if not (d0 >= d1 >= d2):
        return NEG_INF, "invalid"

    candidates: list[tuple[float, str]] = []

    def add(value: float, label: str, qdim: int = 0) -> None:
        if value > NEG_INF / 2 and qdim < 10**9:
            candidates.append((value + qdim * q_log2, label))

    top_value = get_child_value(values_by_span, child_n, d0, top[1])
    middle_value = get_child_value(values_by_span, child_n, d1, middle[1])
    bottom_value = get_child_value(values_by_span, child_n, d2, bottom[1])
    top_middle = two_layer_bound(
        values_by_span=values_by_span,
        flag_table=flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        outer=top,
        inner=middle,
    )
    middle_bottom = two_layer_bound(
        values_by_span=values_by_span,
        flag_table=flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        outer=middle,
        inner=bottom,
    )
    top_bottom = two_layer_bound(
        values_by_span=values_by_span,
        flag_table=flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        outer=top,
        inner=bottom,
    )

    add(
        top_bottom,
        "flag(top,bottom)+choose_middle_between",
        qbinom_exponent(d1 - d2, d0 - d2),
    )
    add(
        top_middle,
        "flag(top,middle)+choose_bottom_inside_middle",
        qbinom_exponent(d2, d1),
    )
    add(
        middle_bottom,
        "flag(middle,bottom)+choose_top_over_middle",
        qbinom_exponent(d0 - d1, child_k - d1),
    )
    add(
        top_value,
        "scalar(top)+choose_middle_inside+choose_bottom_inside",
        qbinom_exponent(d1, d0) + qbinom_exponent(d2, d1),
    )
    add(
        middle_value,
        "scalar(middle)+choose_top_over+choose_bottom_inside",
        qbinom_exponent(d0 - d1, child_k - d1) + qbinom_exponent(d2, d1),
    )
    add(
        bottom_value,
        "scalar(bottom)+choose_middle_over+choose_top_over",
        qbinom_exponent(d1 - d2, child_k - d2)
        + qbinom_exponent(d0 - d1, child_k - d1),
    )

    if not candidates:
        return NEG_INF, "none"
    return min(candidates, key=lambda item: item[0])


def is_support_two_tau2_frame_candidate(parent_span: int, choice: tuple[int, ...]) -> bool:
    view = choice_view(choice)
    kernel_dim = parent_span - view.tau
    return (
        view.tau == 2
        and view.visible_support == 2
        and view.delta == 2
        and view.components == 2
        and kernel_dim == view.inner_span
        and view.outer_span == view.inner_span + 2
    )


def quotient_frame_bounds_for_row(
    *,
    row,
    outer_parent_span: int,
    values_by_span: dict[int, list[float]],
    flag_table: dict[tuple[State, State], float] | None,
    child_k: int,
    child_n: int,
    q_log2: float,
    frame_completion_qdim: int,
) -> tuple[float, str, float, float, int, str] | None:
    if not is_support_two_tau2_frame_candidate(outer_parent_span, row.outer.choice):
        return None
    outer = choice_view(row.outer.choice)
    kernel_dim = outer.inner_span
    top_dim = outer.outer_span
    middle_dim = kernel_dim + 1
    bottom_dim = kernel_dim
    top_zeros = outer.outer_zeros
    middle_zeros = outer.outer_zeros + 1
    bottom_zeros = outer.outer_zeros + 2
    for dim, zeros in row.child_layers:
        if dim == top_dim:
            top_zeros = max(top_zeros, zeros)
        elif dim == middle_dim:
            middle_zeros = max(middle_zeros, zeros)
        elif dim == bottom_dim:
            bottom_zeros = max(bottom_zeros, zeros)
        else:
            return None
    top_state = (top_dim, top_zeros)
    middle_state = (middle_dim, middle_zeros)
    bottom_state = (bottom_dim, bottom_zeros)
    chain_bound, chain_label = three_layer_chain_bound(
        values_by_span=values_by_span,
        flag_table=flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        top=top_state,
        middle=middle_state,
        bottom=bottom_state,
    )
    if chain_bound <= NEG_INF / 2:
        return None
    frame_bound = chain_bound + frame_completion_qdim * q_log2
    child_only_joint = row.joint_log2 - row.child_flag_log2 + frame_bound
    outer_quotient_lift = choice_quotient_lift_qdim(outer_parent_span, row.outer.choice)
    replace_quotient_joint = child_only_joint - outer_quotient_lift * q_log2
    diagram = quotient_diamond(
        kernel_dim=kernel_dim,
        top_zeros=top_zeros,
        middle_zeros=middle_zeros,
        kernel_zeros=bottom_zeros,
    ).key()
    return (
        chain_bound,
        chain_label,
        child_only_joint,
        replace_quotient_joint,
        outer_quotient_lift,
        diagram,
    )


def parse_flag_state(text: str) -> tuple[int, int, int, int]:
    parts = text.split(",")
    if len(parts) != 4:
        raise argparse.ArgumentTypeError(
            "table state must be outer_span,outer_z,inner_span,inner_z"
        )
    try:
        return tuple(int(part) for part in parts)  # type: ignore[return-value]
    except ValueError as exc:
        raise argparse.ArgumentTypeError("table-state entries must be integers") from exc


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=5)
    parser.add_argument("--level", type=int, default=3)
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--singleton-charge", default="endpoint-tau2-layer-incidence")
    parser.add_argument("--max-visible-tau", type=int, default=2)
    parser.add_argument("--cover-lift-mode", default="tau0")
    parser.add_argument("--cover-kernel-lift", action="store_true")
    parser.add_argument("--prune-to-final-span", type=int, default=1)
    parser.add_argument("--term-limit", type=int, default=300)
    parser.add_argument("--exclude-collapsed-active", action="store_true")
    parser.add_argument(
        "--kernel-cover-mode",
        choices=("none", "posthoc", "unconsumed-container", "sibling-unconsumed"),
        default="none",
    )
    parser.add_argument(
        "--nested-quotient-mode",
        choices=("none", "inner-in-outer"),
        default="none",
    )
    parser.add_argument(
        "--nested-subspace-mode",
        choices=("none", "inner-in-outer"),
        default="none",
    )
    parser.add_argument(
        "--consumed-kernel-mode",
        choices=("none", "tau0-inner-contained", "inner-kernel-contained"),
        default="none",
    )
    parser.add_argument("--proof-shaped", action="store_true")
    parser.add_argument("--table-state", type=parse_flag_state, default=(4, 7, 2, 8))
    parser.add_argument("--top", type=int, default=12)
    parser.add_argument(
        "--frame-completion-qdim",
        type=int,
        default=1,
        help="q-dimension charged for completing the second quotient-frame line",
    )
    args = parser.parse_args()

    if args.proof_shaped:
        args.exclude_collapsed_active = True
        args.kernel_cover_mode = "sibling-unconsumed"
    if args.level <= 0 or args.level > args.depth:
        raise SystemExit("--level must be in 1..depth")

    outer_span, outer_z, inner_span, inner_z = args.table_state
    target_key = ((outer_span, outer_z), (inner_span, inner_z))
    levels, _stats = build_pair_levels(
        depth=args.depth,
        stop_level=args.level,
        expansion=args.expansion,
        q_log2=args.q_log2,
        singleton_charge=args.singleton_charge,
        max_visible_tau=args.max_visible_tau,
        cover_lift_mode=args.cover_lift_mode,
        cover_kernel_lift=args.cover_kernel_lift,
        prune_to_final_span=args.prune_to_final_span,
        term_limit=args.term_limit,
        exclude_collapsed_active=args.exclude_collapsed_active,
        kernel_cover_mode=args.kernel_cover_mode,
        nested_quotient_mode=args.nested_quotient_mode,
        nested_subspace_mode=args.nested_subspace_mode,
        consumed_kernel_mode=args.consumed_kernel_mode,
        support2_diamond_mode="none",
        support2_line_filter=False,
        exact_filtered_empty=False,
        last_level_keys={target_key},
        demand_next_level=False,
    )

    current = levels[args.level]
    previous = levels[args.level - 1]
    previous_k = 1 << (args.level - 1)
    previous_n = args.expansion * (1 << (args.level - 1))
    comb = log2_comb_table(args.expansion * (1 << args.depth))

    def terms_for_state(state: State) -> list:
        terms = enumerate_terms_for_state(
            child_by_span=previous.values,
            child_flag_table=previous.flag_table,
            comb=comb,
            q_log2=args.q_log2,
            child_k=previous_k,
            parent_span=state[0],
            zeros=state[1],
            singleton_charge_mode=args.singleton_charge,
            flag_bound_mode="best-two-layer-table",
            max_visible_tau=args.max_visible_tau,
            cover_lift_mode=args.cover_lift_mode,
            cover_kernel_lift=args.cover_kernel_lift,
        )
        return take_terms(terms, args.term_limit)

    rows, pair_sum = build_pair_rows(
        outer_terms=terms_for_state((outer_span, outer_z)),
        inner_terms=terms_for_state((inner_span, inner_z)),
        child_values=previous.values,
        child_flag_table=previous.flag_table,
        child_k=previous_k,
        child_n=previous_n,
        q_log2=args.q_log2,
        outer_parent_span=outer_span,
        inner_parent_span=inner_span,
        exclude_collapsed_active=args.exclude_collapsed_active,
        kernel_cover_mode=args.kernel_cover_mode,
        nested_quotient_mode=args.nested_quotient_mode,
        nested_subspace_mode=args.nested_subspace_mode,
        consumed_kernel_mode=args.consumed_kernel_mode,
        support2_diamond_mode="none",
        support2_line_filter=False,
    )

    table_value = current.flag_table.get(target_key) if current.flag_table is not None else None
    table_label = "-inf" if table_value is None or table_value <= NEG_INF / 2 else f"{table_value:.8f}"
    pair_label = "-inf" if pair_sum <= NEG_INF / 2 else f"{pair_sum:.8f}"
    print(f"table_state,{format_state(target_key[0])}>={format_state(target_key[1])}")
    print(f"table_log2,{table_label}")
    print(f"pair_sum_log2,{pair_label}")
    child_only_sum = NEG_INF
    replace_quotient_sum = NEG_INF
    candidate_count = 0
    improved_child_only_count = 0
    improved_replace_count = 0
    for row in rows:
        bounds = quotient_frame_bounds_for_row(
            row=row,
            outer_parent_span=outer_span,
            values_by_span=previous.values,
            flag_table=previous.flag_table,
            child_k=previous_k,
            child_n=previous_n,
            q_log2=args.q_log2,
            frame_completion_qdim=args.frame_completion_qdim,
        )
        child_value = row.joint_log2
        replace_value = row.joint_log2
        if bounds is not None:
            candidate_count += 1
            _chain_bound, _chain_label, child_only_joint, replace_quotient_joint, _oq, _diagram = bounds
            child_value = min(child_value, child_only_joint)
            replace_value = min(replace_value, replace_quotient_joint)
            if child_value < row.joint_log2:
                improved_child_only_count += 1
            if replace_value < row.joint_log2:
                improved_replace_count += 1
        child_only_sum = log2_add(child_only_sum, child_value)
        replace_quotient_sum = log2_add(replace_quotient_sum, replace_value)
    child_only_label = "-inf" if child_only_sum <= NEG_INF / 2 else f"{child_only_sum:.8f}"
    replace_label = (
        "-inf" if replace_quotient_sum <= NEG_INF / 2 else f"{replace_quotient_sum:.8f}"
    )
    print(
        "quotient_diamond_summary,"
        f"candidate_rows={candidate_count},"
        f"child_only_improved_rows={improved_child_only_count},"
        f"replace_quotient_improved_rows={improved_replace_count},"
        f"child_only_pair_sum_log2={child_only_label},"
        f"child_only_saving_qdim={(pair_sum - child_only_sum) / args.q_log2:.8f},"
        f"replace_quotient_pair_sum_log2={replace_label},"
        f"replace_quotient_saving_qdim={(pair_sum - replace_quotient_sum) / args.q_log2:.8f}"
    )
    print(
        "rank,current_joint_log2,child_flag_log2,chain_bound_log2,chain_label,"
        "frame_bound_log2,child_only_joint_log2,child_only_saving_qdim,"
        "replace_quotient_joint_log2,replace_quotient_saving_qdim,"
        "outer_quotient_lift_qdim,outer_kernel_lift_qdim,diagram,"
        "outer_choice,inner_choice,note"
    )

    emitted = 0
    for rank, row in enumerate(rows, start=1):
        if emitted >= args.top:
            break
        bounds = quotient_frame_bounds_for_row(
            row=row,
            outer_parent_span=outer_span,
            values_by_span=previous.values,
            flag_table=previous.flag_table,
            child_k=previous_k,
            child_n=previous_n,
            q_log2=args.q_log2,
            frame_completion_qdim=args.frame_completion_qdim,
        )
        if bounds is None:
            continue
        (
            chain_bound,
            chain_label,
            child_only_joint,
            replace_quotient_joint,
            outer_quotient_lift,
            diagram,
        ) = bounds
        frame_bound = chain_bound + args.frame_completion_qdim * args.q_log2
        print(
            f"{rank},{row.joint_log2:.8f},{row.child_flag_log2:.8f},"
            f"{chain_bound:.8f},{chain_label},{frame_bound:.8f},"
            f"{child_only_joint:.8f},"
            f"{(row.joint_log2 - child_only_joint) / args.q_log2:.8f},"
            f"{replace_quotient_joint:.8f},"
            f"{(row.joint_log2 - replace_quotient_joint) / args.q_log2:.8f},"
            f"{outer_quotient_lift},"
            f"{choice_kernel_lift_qdim(outer_span, row.outer.choice)},"
            f"{diagram},"
            f"{format_choice(row.outer.choice).replace(',', ';')},"
            f"{format_choice(row.inner.choice).replace(',', ';')},"
            "replace_quotient is theorem target, not certified by this diagnostic"
        )
        emitted += 1


if __name__ == "__main__":
    main()
