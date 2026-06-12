#!/usr/bin/env python3
r"""Classify high-mass flag-state pair witnesses.

This is a diagnostic companion to `rfc_flag_state_choice_diagnostic.py`.
The choice diagnostic reports the largest individual pair products for one
flag state.  This script groups the same pair products by structural keys so
we can see whether a bad pair sum is spread across many unrelated witnesses or
concentrated in a small family that might admit a canonical-selection or
charging lemma.

The output is intentionally theorem-neutral.  A large grouped mass is not a
counterexample by itself; it identifies the witness family a proof must either
count once, charge, or rule incompatible.
"""

from __future__ import annotations

import argparse
import csv
import sys
from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from rfc_flag_span_moment import (  # noqa: E402
    NEG_INF,
    State,
    flag_child_bound_report,
    flag_table_bound_for_layers,
    get_child_value,
    log2_add,
    log2_comb_table,
    merge_equal_dimension_chain,
)
from rfc_flag_state_choice_diagnostic import (  # noqa: E402
    TermCandidate,
    build_levels,
    enumerate_terms_for_state,
    format_choice,
    format_flag,
    format_state,
    parse_state,
)


CSV_FIELDS = [
    "section",
    "rank",
    "key",
    "kernel_cover_mode",
    "count",
    "logsum_log2",
    "max_log2",
    "coarse_log2",
    "saving_log2",
    "saving_qdim",
    "share_vs_pair_sum_log2",
    "top_child_flag",
    "top_outer_local_log2",
    "top_inner_local_log2",
    "top_child_flag_log2",
    "top_outer_kernel_lift_qdim",
    "top_outer_quotient_lift_qdim",
    "top_inner_kernel_lift_qdim",
    "top_inner_quotient_lift_qdim",
    "top_outer_covered_kernel_lift_qdim",
    "top_inner_covered_kernel_lift_qdim",
    "top_outer_consumed_kernel_cover_qdim",
    "top_inner_nested_quotient_cover_qdim",
    "top_inner_nested_subspace_cover_qdim",
    "top_support2_diamond_saving_qdim",
    "top_outer_kernel_dim",
    "top_inner_kernel_dim",
    "top_outer_kernel_unconsumed_by_inner",
    "top_outer_choice",
    "top_inner_choice",
    "note",
]


@dataclass(frozen=True)
class ChoiceView:
    p: int
    singletons: int
    visible_support: int
    tau: int
    outer_span: int
    inner_span: int
    outer_zeros: int
    charge: int
    delta: int
    components: int
    theta: int
    h: int
    gamma: int
    lift_qdim: int


@dataclass(frozen=True)
class PairRow:
    outer: TermCandidate
    inner: TermCandidate
    child_layers: tuple[State, ...]
    child_flag_log2: float
    outer_local_log2: float
    inner_local_log2: float
    outer_consumed_kernel_cover_qdim: int
    inner_nested_quotient_cover_qdim: int
    inner_nested_subspace_cover_qdim: int
    support2_diamond_saving_qdim: float
    joint_log2: float


@dataclass
class GroupStats:
    count: int = 0
    logsum_log2: float = NEG_INF
    max_log2: float = NEG_INF
    top: PairRow | None = None

    def add(self, row: PairRow) -> None:
        self.count += 1
        self.logsum_log2 = log2_add(self.logsum_log2, row.joint_log2)
        if row.joint_log2 > self.max_log2:
            self.max_log2 = row.joint_log2
            self.top = row


def choice_view(choice: tuple[int, ...]) -> ChoiceView:
    return ChoiceView(
        p=choice[0],
        singletons=choice[1],
        visible_support=choice[2],
        tau=choice[3],
        outer_span=choice[4],
        inner_span=choice[5],
        outer_zeros=choice[6],
        charge=choice[7],
        delta=choice[8],
        components=choice[9],
        theta=choice[11],
        h=choice[12],
        gamma=choice[13],
        lift_qdim=choice[14],
    )


def compact_choice_key(choice: tuple[int, ...]) -> str:
    view = choice_view(choice)
    return (
        f"p={view.p},s={view.singletons},a={view.visible_support},tau={view.tau},"
        f"child=({view.outer_span},{view.inner_span}),z={view.outer_zeros},"
        f"charge={view.charge},delta={view.delta},comp={view.components},"
        f"h={view.h},gamma={view.gamma},lift={view.lift_qdim}"
    )


def choice_has_collapsed_active_container(choice: tuple[int, ...]) -> bool:
    """Return true for tau-positive rows whose child flag collapses the active support.

    If the kernel child container has the same dimension as the outer child
    container but a strictly stronger zero budget, then the two containers are
    equal in an exact flag.  For tau > 0 and nonempty visible support this means
    the outer container is already zero on the claimed active singleton support.
    Such a row is a safe coarse-container overcount, but a canonical exact
    recurrence should route the event through a smaller kernel container or a
    tau-zero row.
    """

    view = choice_view(choice)
    inner_zeros = view.p + view.singletons
    return (
        view.tau > 0
        and view.visible_support > 0
        and view.inner_span == view.outer_span
        and inner_zeros > view.outer_zeros
    )


def choice_has_support2_line_quotient_impossibility(choice: tuple[int, ...]) -> bool:
    """Return true for the decomposable support-two tau-two line-quotient overcount.

    If `dim(V/L)=1`, then a tau-two quotient injects into the doubled child
    line `(V/L)+(V/L)` and is therefore the whole two-dimensional doubled line.
    At every active coordinate where the child quotient line is nonzero, the
    coordinate projection has rank two, so it cannot be compatible with a
    singleton root line.  Thus the exact support-two decomposable row is not a
    real tau-two visible-support row.
    """

    view = choice_view(choice)
    return (
        view.tau == 2
        and view.visible_support == 2
        and view.delta == 2
        and view.components == 2
        and view.outer_span == view.inner_span + 1
    )


def choice_kernel_lift_qdim(parent_span: int, choice: tuple[int, ...]) -> int:
    view = choice_view(choice)
    if view.tau <= 0:
        return 0
    kernel_dim = parent_span - view.tau
    return kernel_dim * (2 * view.inner_span - kernel_dim)


def choice_kernel_dim(parent_span: int, choice: tuple[int, ...]) -> int:
    view = choice_view(choice)
    return parent_span - view.tau


def choice_quotient_lift_qdim(parent_span: int, choice: tuple[int, ...]) -> int:
    view = choice_view(choice)
    if view.tau <= 0:
        return 0
    return view.tau * (2 * view.outer_span - parent_span)


def nested_quotient_cover_qdim(
    *,
    outer_parent_span: int,
    inner_parent_span: int,
    outer_choice: tuple[int, ...],
    inner_choice: tuple[int, ...],
    nested_quotient_mode: str,
) -> int:
    """Diagnostic quotient nesting adjustment for a nested pair of local rows.

    In a parent flag W_inner <= W_outer, tau-positive quotient data should not
    always be chosen independently.  The conservative diagnostic here keeps the
    outer quotient datum fully counted, then lets the inner quotient be chosen
    inside the already-fixed outer visible quotient whenever the displayed
    dimensions are compatible.  It still pays the Grassmann exponent for a
    tau_inner-subspace inside a tau_outer-space, so quotient incidence is not
    erased.
    """

    if nested_quotient_mode == "none":
        return 0
    if nested_quotient_mode != "inner-in-outer":
        raise ValueError(f"unknown nested_quotient_mode: {nested_quotient_mode}")

    outer = choice_view(outer_choice)
    inner = choice_view(inner_choice)
    if outer.tau <= 0 or inner.tau <= 0:
        return 0
    if inner.tau > outer.tau:
        return 0
    if inner.outer_span > outer.outer_span or inner.inner_span > outer.inner_span:
        return 0

    independent_qdim = choice_quotient_lift_qdim(inner_parent_span, inner_choice)
    nested_qdim = inner.tau * (outer.tau - inner.tau)
    return max(0, independent_qdim - nested_qdim)


def nested_subspace_cover_qdim(
    *,
    outer_parent_span: int,
    inner_parent_span: int,
    inner_choice: tuple[int, ...],
    already_covered_lift_qdim: int,
    nested_subspace_mode: str,
) -> int:
    """Diagnostic cap for choosing the lower parent subspace inside the upper one.

    After a parent flag fixes W_outer, the lower layer W_inner is a
    dim(W_inner)-subspace of W_outer.  This diagnostic caps the lower row's
    remaining lift multiplicity by the Grassmann exponent for such subspaces.
    Local root/support charges are still paid separately.
    """

    if nested_subspace_mode == "none":
        return 0
    if nested_subspace_mode != "inner-in-outer":
        raise ValueError(f"unknown nested_subspace_mode: {nested_subspace_mode}")
    if inner_parent_span > outer_parent_span:
        return 0

    counted_lift_qdim = max(0, choice_view(inner_choice).lift_qdim - already_covered_lift_qdim)
    nested_qdim = inner_parent_span * (outer_parent_span - inner_parent_span)
    return max(0, counted_lift_qdim - nested_qdim)


def outer_kernel_unconsumed_by_inner(
    *,
    outer_parent_span: int,
    inner_parent_span: int,
    outer_choice: tuple[int, ...],
    inner_choice: tuple[int, ...],
) -> str:
    """Conservative sibling-consumption audit for the displayed pair.

    In a nested two-layer parent flag, a lower-layer kernel is the obvious
    sibling datum that can consume hidden subspace inside the upper kernel
    lift.  If the lower layer is fully visible, it has no such kernel datum.
    This is only a diagnostic; descendants below the displayed pair still
    need their own consumed-kernel check.
    """

    outer_kernel_dim = choice_kernel_dim(outer_parent_span, outer_choice)
    if outer_kernel_dim <= 0:
        return "vacuous"
    inner_kernel_dim = choice_kernel_dim(inner_parent_span, inner_choice)
    return "yes" if inner_kernel_dim == 0 else "no"


def covered_kernel_lift_qdims(
    *,
    outer_parent_span: int,
    inner_parent_span: int,
    outer_choice: tuple[int, ...],
    inner_choice: tuple[int, ...],
    kernel_cover_mode: str,
) -> tuple[int, int]:
    if kernel_cover_mode == "none":
        return 0, 0
    outer_kernel_lift = choice_kernel_lift_qdim(outer_parent_span, outer_choice)
    inner_kernel_lift = choice_kernel_lift_qdim(inner_parent_span, inner_choice)
    if kernel_cover_mode in ("posthoc", "unconsumed-container"):
        return outer_kernel_lift, inner_kernel_lift
    if kernel_cover_mode == "sibling-unconsumed":
        outer_cover = 0
        if (
            outer_kernel_unconsumed_by_inner(
                outer_parent_span=outer_parent_span,
                inner_parent_span=inner_parent_span,
                outer_choice=outer_choice,
                inner_choice=inner_choice,
            )
            == "yes"
        ):
            outer_cover = outer_kernel_lift
        return outer_cover, 0
    raise ValueError(f"unknown kernel_cover_mode: {kernel_cover_mode}")


def consumed_kernel_cover_qdim(
    *,
    outer_parent_span: int,
    inner_parent_span: int,
    outer_choice: tuple[int, ...],
    inner_choice: tuple[int, ...],
    consumed_kernel_mode: str,
) -> int:
    """Diagnostic for a lower tau-zero layer carried inside an upper kernel.

    This is deliberately narrower than the general consumed-kernel theorem.  If
    the displayed lower layer is tau zero and has dimension at most the upper
    kernel dimension, then a conditional recurrence could count the upper
    kernel as a kappa_outer-subspace containing W_inner instead of as an
    arbitrary kappa_outer-subspace of L_outer+L_outer.
    """

    if consumed_kernel_mode == "none":
        return 0
    if consumed_kernel_mode not in ("tau0-inner-contained", "inner-kernel-contained"):
        raise ValueError(f"unknown consumed_kernel_mode: {consumed_kernel_mode}")
    outer = choice_view(outer_choice)
    inner = choice_view(inner_choice)
    if consumed_kernel_mode == "tau0-inner-contained" and inner.tau != 0:
        return 0
    outer_kernel_dim = outer_parent_span - outer.tau
    inner_kernel_dim = inner_parent_span - inner.tau
    if inner_kernel_dim <= 0:
        return 0
    if consumed_kernel_mode == "tau0-inner-contained":
        inner_kernel_dim = inner_parent_span
    if inner_kernel_dim > outer_kernel_dim:
        return 0
    if consumed_kernel_mode == "inner-kernel-contained" and inner.inner_span > outer.inner_span:
        return 0
    ambient_dim = 2 * outer.inner_span
    if ambient_dim < outer_kernel_dim:
        return 0
    return inner_kernel_dim * (ambient_dim - outer_kernel_dim)


def qbinom_exponent(sub_dim: int, ambient_dim: int) -> int:
    if sub_dim < 0 or sub_dim > ambient_dim:
        return 10**9
    return sub_dim * (ambient_dim - sub_dim)


def three_layer_chain_bound(
    *,
    child_values: dict[int, list[float]],
    child_flag_table,
    child_k: int,
    child_n: int,
    q_log2: float,
    top: State,
    middle: State,
    bottom: State,
) -> float:
    """Safe coarse bound for one chain `top >= middle >= bottom`.

    The minimum ranges over scalar and two-layer flag anchors, extending missing
    layers by Gaussian-binomial counts and ignoring their zero constraints.
    """

    d0, _z0 = top
    d1, _z1 = middle
    d2, _z2 = bottom
    if not (d0 >= d1 >= d2):
        return NEG_INF
    candidates: list[float] = []

    def add(value: float, qdim: int = 0) -> None:
        if value > NEG_INF / 2 and qdim < 10**9:
            candidates.append(value + qdim * q_log2)

    top_value = get_child_value(child_values, child_n, d0, top[1])
    middle_value = get_child_value(child_values, child_n, d1, middle[1])
    bottom_value = get_child_value(child_values, child_n, d2, bottom[1])
    top_middle = flag_table_bound_for_layers(
        values_by_span=child_values,
        flag_table=child_flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        layers=(top, middle),
    )
    middle_bottom = flag_table_bound_for_layers(
        values_by_span=child_values,
        flag_table=child_flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        layers=(middle, bottom),
    )
    top_bottom = flag_table_bound_for_layers(
        values_by_span=child_values,
        flag_table=child_flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        layers=(top, bottom),
    )

    add(top_bottom, qbinom_exponent(d1 - d2, d0 - d2))
    add(top_middle, qbinom_exponent(d2, d1))
    add(middle_bottom, qbinom_exponent(d0 - d1, child_k - d1))
    add(top_value, qbinom_exponent(d1, d0) + qbinom_exponent(d2, d1))
    add(
        middle_value,
        qbinom_exponent(d0 - d1, child_k - d1) + qbinom_exponent(d2, d1),
    )
    add(
        bottom_value,
        qbinom_exponent(d1 - d2, child_k - d2)
        + qbinom_exponent(d0 - d1, child_k - d1),
    )
    return min(candidates) if candidates else NEG_INF


def support2_diamond_child_bound(
    *,
    outer_parent_span: int,
    outer_choice: tuple[int, ...],
    child_layers: tuple[State, ...],
    child_values: dict[int, list[float]],
    child_flag_table,
    child_k: int,
    child_n: int,
    q_log2: float,
) -> float:
    """Return the child-only quotient-diamond bound for decomposable tau-two rows."""

    outer = choice_view(outer_choice)
    kernel_dim = outer_parent_span - outer.tau
    if not (
        outer.tau == 2
        and outer.visible_support == 2
        and outer.delta == 2
        and outer.components == 2
        and kernel_dim == outer.inner_span
        and outer.outer_span == outer.inner_span + 2
    ):
        return NEG_INF

    top_dim = outer.outer_span
    middle_dim = outer.inner_span + 1
    bottom_dim = outer.inner_span
    top_zeros = outer.outer_zeros
    middle_zeros = outer.outer_zeros + 1
    bottom_zeros = outer.outer_zeros + 2
    for dim, zeros in child_layers:
        if dim == top_dim:
            top_zeros = max(top_zeros, zeros)
        elif dim == middle_dim:
            middle_zeros = max(middle_zeros, zeros)
        elif dim == bottom_dim:
            bottom_zeros = max(bottom_zeros, zeros)
        else:
            return NEG_INF

    chain_bound = three_layer_chain_bound(
        child_values=child_values,
        child_flag_table=child_flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=q_log2,
        top=(top_dim, top_zeros),
        middle=(middle_dim, middle_zeros),
        bottom=(bottom_dim, bottom_zeros),
    )
    if chain_bound <= NEG_INF / 2:
        return NEG_INF
    return chain_bound + q_log2


def tau_key(row: PairRow) -> str:
    outer = choice_view(row.outer.choice)
    inner = choice_view(row.inner.choice)
    return f"outer_tau={outer.tau},inner_tau={inner.tau}"


def support_key(row: PairRow) -> str:
    outer = choice_view(row.outer.choice)
    inner = choice_view(row.inner.choice)
    return (
        f"outer_a={outer.visible_support},outer_s={outer.singletons};"
        f"inner_a={inner.visible_support},inner_s={inner.singletons}"
    )


def lift_key(row: PairRow) -> str:
    outer = choice_view(row.outer.choice)
    inner = choice_view(row.inner.choice)
    return f"outer_lift={outer.lift_qdim},inner_lift={inner.lift_qdim}"


def outer_choice_key(row: PairRow) -> str:
    return compact_choice_key(row.outer.choice)


def inner_choice_key(row: PairRow) -> str:
    return compact_choice_key(row.inner.choice)


def exact_pair_key(row: PairRow) -> str:
    return f"outer[{format_choice(row.outer.choice)}]|inner[{format_choice(row.inner.choice)}]"


def child_flag_key(row: PairRow) -> str:
    return format_flag(row.child_layers)


def child_flag_and_outer_key(row: PairRow) -> str:
    return f"{child_flag_key(row)} | outer {compact_choice_key(row.outer.choice)}"


def child_flag_and_tau_key(row: PairRow) -> str:
    return f"{child_flag_key(row)} | {tau_key(row)}"


GROUPERS: dict[str, Callable[[PairRow], str]] = {
    "child_flag": child_flag_key,
    "outer_choice": outer_choice_key,
    "inner_choice": inner_choice_key,
    "tau_pair": tau_key,
    "support_pair": support_key,
    "lift_pair": lift_key,
    "child_flag_outer_choice": child_flag_and_outer_key,
    "child_flag_tau_pair": child_flag_and_tau_key,
    "exact_pair": exact_pair_key,
}


def parse_groupers(text: str) -> list[str]:
    names = [part.strip() for part in text.split(",") if part.strip()]
    bad = [name for name in names if name not in GROUPERS]
    if bad:
        raise argparse.ArgumentTypeError(
            "unknown groupers: "
            + ", ".join(bad)
            + "; valid groupers are "
            + ", ".join(sorted(GROUPERS))
        )
    return names


def build_pair_rows(
    *,
    outer_terms: list[TermCandidate],
    inner_terms: list[TermCandidate],
    child_values: dict[int, list[float]],
    child_flag_table: dict[tuple[State, State], float] | None,
    child_k: int,
    child_n: int,
    q_log2: float,
    outer_parent_span: int,
    inner_parent_span: int,
    exclude_collapsed_active: bool,
    kernel_cover_mode: str,
    nested_quotient_mode: str = "none",
    nested_subspace_mode: str = "none",
    consumed_kernel_mode: str = "none",
    support2_diamond_mode: str = "none",
    support2_line_filter: bool = False,
) -> tuple[list[PairRow], float]:
    if support2_diamond_mode not in ("none", "child-only"):
        raise ValueError(f"unknown support2_diamond_mode: {support2_diamond_mode}")
    rows: list[PairRow] = []
    pair_sum = NEG_INF
    for outer in outer_terms:
        if exclude_collapsed_active and choice_has_collapsed_active_container(outer.choice):
            continue
        if support2_line_filter and choice_has_support2_line_quotient_impossibility(outer.choice):
            continue
        for inner in inner_terms:
            if exclude_collapsed_active and choice_has_collapsed_active_container(inner.choice):
                continue
            if support2_line_filter and choice_has_support2_line_quotient_impossibility(inner.choice):
                continue
            child_layers = tuple(
                merge_equal_dimension_chain(list(outer.child_layers) + list(inner.child_layers))
            )
            child_flag_log2 = flag_table_bound_for_layers(
                values_by_span=child_values,
                flag_table=child_flag_table,
                child_k=child_k,
                child_n=child_n,
                q_log2=q_log2,
                layers=child_layers,
            )
            if child_flag_log2 <= NEG_INF / 2:
                continue
            outer_covered_kernel, inner_covered_kernel = covered_kernel_lift_qdims(
                outer_parent_span=outer_parent_span,
                inner_parent_span=inner_parent_span,
                outer_choice=outer.choice,
                inner_choice=inner.choice,
                kernel_cover_mode=kernel_cover_mode,
            )
            outer_local_log2 = outer.local_log2 - outer_covered_kernel * q_log2
            inner_local_log2 = inner.local_log2 - inner_covered_kernel * q_log2
            outer_consumed_kernel_cover = consumed_kernel_cover_qdim(
                outer_parent_span=outer_parent_span,
                inner_parent_span=inner_parent_span,
                outer_choice=outer.choice,
                inner_choice=inner.choice,
                consumed_kernel_mode=consumed_kernel_mode,
            )
            outer_local_log2 -= outer_consumed_kernel_cover * q_log2
            inner_nested_quotient_cover = nested_quotient_cover_qdim(
                outer_parent_span=outer_parent_span,
                inner_parent_span=inner_parent_span,
                outer_choice=outer.choice,
                inner_choice=inner.choice,
                nested_quotient_mode=nested_quotient_mode,
            )
            inner_local_log2 -= inner_nested_quotient_cover * q_log2
            inner_nested_subspace_cover = nested_subspace_cover_qdim(
                outer_parent_span=outer_parent_span,
                inner_parent_span=inner_parent_span,
                inner_choice=inner.choice,
                already_covered_lift_qdim=(
                    inner_covered_kernel + inner_nested_quotient_cover
                ),
                nested_subspace_mode=nested_subspace_mode,
            )
            inner_local_log2 -= inner_nested_subspace_cover * q_log2
            support2_diamond_saving = 0.0
            selected_child_flag_log2 = child_flag_log2
            if support2_diamond_mode == "child-only":
                diamond_child_log2 = support2_diamond_child_bound(
                    outer_parent_span=outer_parent_span,
                    outer_choice=outer.choice,
                    child_layers=child_layers,
                    child_values=child_values,
                    child_flag_table=child_flag_table,
                    child_k=child_k,
                    child_n=child_n,
                    q_log2=q_log2,
                )
                if diamond_child_log2 > NEG_INF / 2 and diamond_child_log2 < child_flag_log2:
                    support2_diamond_saving = (child_flag_log2 - diamond_child_log2) / q_log2
                    selected_child_flag_log2 = diamond_child_log2
            joint_log2 = outer_local_log2 + inner_local_log2 + selected_child_flag_log2
            row = PairRow(
                outer=outer,
                inner=inner,
                child_layers=child_layers,
                child_flag_log2=selected_child_flag_log2,
                outer_local_log2=outer_local_log2,
                inner_local_log2=inner_local_log2,
                outer_consumed_kernel_cover_qdim=outer_consumed_kernel_cover,
                inner_nested_quotient_cover_qdim=inner_nested_quotient_cover,
                inner_nested_subspace_cover_qdim=inner_nested_subspace_cover,
                support2_diamond_saving_qdim=support2_diamond_saving,
                joint_log2=joint_log2,
            )
            rows.append(row)
            pair_sum = log2_add(pair_sum, joint_log2)
    rows.sort(key=lambda row: row.joint_log2, reverse=True)
    return rows, pair_sum


def summarize_group(
    rows: list[PairRow],
    key_fn: Callable[[PairRow], str],
) -> list[tuple[str, GroupStats]]:
    groups: dict[str, GroupStats] = {}
    for row in rows:
        key = key_fn(row)
        stats = groups.get(key)
        if stats is None:
            stats = GroupStats()
            groups[key] = stats
        stats.add(row)
    return sorted(groups.items(), key=lambda item: item[1].logsum_log2, reverse=True)


def make_output_row(
    *,
    section: str,
    rank: str | int,
    key: str,
    stats: GroupStats,
    coarse: float,
    pair_sum: float,
    q_log2: float,
    outer_parent_span: int,
    inner_parent_span: int,
    kernel_cover_mode: str,
    note: str = "",
) -> dict[str, object]:
    top = stats.top
    saving = coarse - stats.logsum_log2 if stats.logsum_log2 > NEG_INF / 2 else NEG_INF
    share = stats.logsum_log2 - pair_sum if pair_sum > NEG_INF / 2 else NEG_INF
    return {
        "section": section,
        "rank": rank,
        "key": key,
        "kernel_cover_mode": kernel_cover_mode,
        "count": stats.count,
        "logsum_log2": f"{stats.logsum_log2:.8f}" if stats.logsum_log2 > NEG_INF / 2 else "-inf",
        "max_log2": f"{stats.max_log2:.8f}" if stats.max_log2 > NEG_INF / 2 else "-inf",
        "coarse_log2": f"{coarse:.8f}" if coarse > NEG_INF / 2 else "-inf",
        "saving_log2": f"{saving:.8f}" if stats.logsum_log2 > NEG_INF / 2 else "",
        "saving_qdim": f"{saving / q_log2:.8f}" if stats.logsum_log2 > NEG_INF / 2 else "",
        "share_vs_pair_sum_log2": f"{share:.8f}" if pair_sum > NEG_INF / 2 else "",
        "top_child_flag": format_flag(top.child_layers) if top is not None else "",
        "top_outer_local_log2": f"{top.outer_local_log2:.8f}" if top is not None else "",
        "top_inner_local_log2": f"{top.inner_local_log2:.8f}" if top is not None else "",
        "top_child_flag_log2": f"{top.child_flag_log2:.8f}" if top is not None else "",
        "top_outer_kernel_lift_qdim": (
            choice_kernel_lift_qdim(outer_parent_span, top.outer.choice) if top is not None else ""
        ),
        "top_outer_quotient_lift_qdim": (
            choice_quotient_lift_qdim(outer_parent_span, top.outer.choice) if top is not None else ""
        ),
        "top_inner_kernel_lift_qdim": (
            choice_kernel_lift_qdim(inner_parent_span, top.inner.choice) if top is not None else ""
        ),
        "top_inner_quotient_lift_qdim": (
            choice_quotient_lift_qdim(inner_parent_span, top.inner.choice) if top is not None else ""
        ),
        "top_outer_covered_kernel_lift_qdim": (
            covered_kernel_lift_qdims(
                outer_parent_span=outer_parent_span,
                inner_parent_span=inner_parent_span,
                outer_choice=top.outer.choice,
                inner_choice=top.inner.choice,
                kernel_cover_mode=kernel_cover_mode,
            )[0]
            if top is not None
            else ""
        ),
        "top_inner_covered_kernel_lift_qdim": (
            covered_kernel_lift_qdims(
                outer_parent_span=outer_parent_span,
                inner_parent_span=inner_parent_span,
                outer_choice=top.outer.choice,
                inner_choice=top.inner.choice,
                kernel_cover_mode=kernel_cover_mode,
            )[1]
            if top is not None
            else ""
        ),
        "top_outer_consumed_kernel_cover_qdim": (
            top.outer_consumed_kernel_cover_qdim if top is not None else ""
        ),
        "top_inner_nested_quotient_cover_qdim": (
            top.inner_nested_quotient_cover_qdim if top is not None else ""
        ),
        "top_inner_nested_subspace_cover_qdim": (
            top.inner_nested_subspace_cover_qdim if top is not None else ""
        ),
        "top_support2_diamond_saving_qdim": (
            f"{top.support2_diamond_saving_qdim:.8f}" if top is not None else ""
        ),
        "top_outer_kernel_dim": (
            choice_kernel_dim(outer_parent_span, top.outer.choice) if top is not None else ""
        ),
        "top_inner_kernel_dim": (
            choice_kernel_dim(inner_parent_span, top.inner.choice) if top is not None else ""
        ),
        "top_outer_kernel_unconsumed_by_inner": (
            outer_kernel_unconsumed_by_inner(
                outer_parent_span=outer_parent_span,
                inner_parent_span=inner_parent_span,
                outer_choice=top.outer.choice,
                inner_choice=top.inner.choice,
            )
            if top is not None
            else ""
        ),
        "top_outer_choice": format_choice(top.outer.choice) if top is not None else "",
        "top_inner_choice": format_choice(top.inner.choice) if top is not None else "",
        "note": note,
    }


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
    parser.add_argument("--term-limit", type=int, default=300)
    parser.add_argument("--top-groups", type=int, default=8)
    parser.add_argument("--top-pairs", type=int, default=8)
    parser.add_argument(
        "--exclude-collapsed-active",
        action="store_true",
        help="diagnostic: drop tau-positive rows where equal-dimension child containers merge away the active support",
    )
    parser.add_argument(
        "--posthoc-cover-kernel-lift",
        action="store_true",
        help="diagnostic: keep child tables fixed but subtract tau-positive kernel-lift q-dimensions from enumerated pair choices",
    )
    parser.add_argument(
        "--kernel-cover-mode",
        choices=("none", "posthoc", "unconsumed-container", "sibling-unconsumed"),
        default="none",
        help=(
            "kernel-lift adjustment mode; unconsumed-container covers all displayed kernel lifts, "
            "while sibling-unconsumed covers only an upper kernel lift whose displayed lower layer "
            "is fully visible"
        ),
    )
    parser.add_argument(
        "--nested-quotient-mode",
        choices=("none", "inner-in-outer"),
        default="none",
        help=(
            "diagnostic: keep outer quotient incidence counted but choose a nested inner "
            "tau-positive quotient inside the outer visible quotient when dimensions permit"
        ),
    )
    parser.add_argument(
        "--nested-subspace-mode",
        choices=("none", "inner-in-outer"),
        default="none",
        help=(
            "diagnostic: after the upper parent row is fixed, cap remaining lower "
            "lift multiplicity by the Grassmann count of W_inner <= W_outer"
        ),
    )
    parser.add_argument(
        "--consumed-kernel-mode",
        choices=("none", "tau0-inner-contained", "inner-kernel-contained"),
        default="none",
        help=(
            "diagnostic: count the upper kernel as containing a lower kernel "
            "subspace instead of as a fresh kernel lift; tau0-inner-contained "
            "applies only to lower tau-zero rows"
        ),
    )
    parser.add_argument(
        "--support2-diamond-mode",
        choices=("none", "child-only"),
        default="none",
        help=(
            "diagnostic: for decomposable support-two tau-two rows, keep quotient "
            "incidence counted but replace the bare child flag by a quotient-diamond "
            "chain bound"
        ),
    )
    parser.add_argument(
        "--support2-line-quotient-filter",
        action="store_true",
        help=(
            "diagnostic: exclude decomposable support-two tau-two rows with "
            "dim(V/L)=1, where a nonempty exact root-line support is impossible"
        ),
    )
    parser.add_argument(
        "--groupers",
        type=parse_groupers,
        default=parse_groupers(
            "child_flag,outer_choice,inner_choice,tau_pair,support_pair,lift_pair,"
            "child_flag_outer_choice,child_flag_tau_pair"
        ),
        help="comma-separated grouping keys; include exact_pair for per-choice pairs",
    )
    args = parser.parse_args()
    if args.posthoc_cover_kernel_lift and args.kernel_cover_mode != "none":
        raise SystemExit("use either --posthoc-cover-kernel-lift or --kernel-cover-mode, not both")
    kernel_cover_mode = "posthoc" if args.posthoc_cover_kernel_lift else args.kernel_cover_mode

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
    pair_rows, pair_sum = build_pair_rows(
        outer_terms=outer_terms,
        inner_terms=inner_terms,
        child_values=child.values,
        child_flag_table=child.flag_table,
        child_k=child_k,
        child_n=child_n,
        q_log2=args.q_log2,
        outer_parent_span=args.outer_state[0],
        inner_parent_span=args.inner_state[0],
        exclude_collapsed_active=args.exclude_collapsed_active,
        kernel_cover_mode=kernel_cover_mode,
        nested_quotient_mode=args.nested_quotient_mode,
        nested_subspace_mode=args.nested_subspace_mode,
        consumed_kernel_mode=args.consumed_kernel_mode,
        support2_diamond_mode=args.support2_diamond_mode,
        support2_line_filter=args.support2_line_quotient_filter,
    )

    writer = csv.DictWriter(sys.stdout, fieldnames=CSV_FIELDS, lineterminator="\n")
    writer.writeheader()
    all_stats = GroupStats()
    for row in pair_rows:
        all_stats.add(row)
    writer.writerow(
        make_output_row(
            section="summary",
            rank="pair_sum",
            key=(
                f"level={args.level},outer={format_state(args.outer_state)},"
                f"inner={format_state(args.inner_state)},"
                f"outer_terms={len(outer_terms)},inner_terms={len(inner_terms)},"
                f"exclude_collapsed_active={args.exclude_collapsed_active},"
                f"kernel_cover_mode={kernel_cover_mode},"
                f"nested_quotient_mode={args.nested_quotient_mode},"
                f"nested_subspace_mode={args.nested_subspace_mode},"
                f"consumed_kernel_mode={args.consumed_kernel_mode},"
                f"support2_diamond_mode={args.support2_diamond_mode},"
                f"support2_line_quotient_filter={args.support2_line_quotient_filter}"
            ),
            stats=all_stats,
            coarse=coarse,
            pair_sum=pair_sum,
            q_log2=args.q_log2,
            outer_parent_span=args.outer_state[0],
            inner_parent_span=args.inner_state[0],
            kernel_cover_mode=kernel_cover_mode,
            note=(
                "truncated pair sum; omitted terms can only increase the full naive sum"
                if kernel_cover_mode == "none"
                else "truncated pair sum; kernel cover subtracts selected kernel-lift q-dimensions and keeps quotient incidence counted"
            ),
        )
    )

    top_stats = GroupStats()
    for row in pair_rows[: args.top_pairs]:
        top_stats.add(row)
    writer.writerow(
        make_output_row(
            section="summary",
            rank=f"top_{args.top_pairs}_pairs",
            key="largest individual pair products",
            stats=top_stats,
            coarse=coarse,
            pair_sum=pair_sum,
            q_log2=args.q_log2,
            outer_parent_span=args.outer_state[0],
            inner_parent_span=args.inner_state[0],
            kernel_cover_mode=kernel_cover_mode,
            note="how concentrated the visible top of the pair sum is",
        )
    )

    for grouper_name in args.groupers:
        grouped = summarize_group(pair_rows, GROUPERS[grouper_name])
        for rank, (key, stats) in enumerate(grouped[: args.top_groups], start=1):
            writer.writerow(
                make_output_row(
                    section=grouper_name,
                    rank=rank,
                    key=key,
                    stats=stats,
                    coarse=coarse,
                    pair_sum=pair_sum,
                    q_log2=args.q_log2,
                    outer_parent_span=args.outer_state[0],
                    inner_parent_span=args.inner_state[0],
                    kernel_cover_mode=kernel_cover_mode,
                )
            )


if __name__ == "__main__":
    main()
