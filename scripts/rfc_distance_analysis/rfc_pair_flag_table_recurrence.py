#!/usr/bin/env python3
r"""Pair-enumerating two-layer flag-table recurrence diagnostic.

This script is the next step after `rfc_flag_bad_pair_classifier.py`.
Instead of auditing one requested flag state, it builds a two-layer flag table
by enumerating expansion pairs for every reachable two-layer state.  The table
can then be fed into the next scalar lift level.

This is still diagnostic, not a certificate.  With `--term-limit`, the pair
sum is truncated to the largest scalar expansion terms for each layer.  The
proof-shaped mode of interest is:

    --exclude-collapsed-active --kernel-cover-mode sibling-unconsumed

which mirrors the current theorem target: exact-support rerouting for collapsed
active rows plus kernel-fiber covering only when the displayed lower sibling is
fully visible.
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))

from rfc_flag_bad_pair_classifier import (  # noqa: E402
    build_pair_rows,
    choice_has_collapsed_active_container,
    choice_has_support2_line_quotient_impossibility,
    choice_kernel_lift_qdim,
    choice_quotient_lift_qdim,
    covered_kernel_lift_qdims,
)
from rfc_flag_span_moment import (  # noqa: E402
    INF,
    NEG_INF,
    FlagTable,
    State,
    choice_child_layers,
    choice_split_shape_log2,
    first_crossing,
    flag_child_bound_report,
    flag_table_bound_for_layers,
    get_child_value,
    lift_flag_span_moment,
    local_visible_trace_profile,
    log2_comb_table,
    merge_equal_dimension_chain,
    tau1_incidence_profile,
)
from rfc_flag_state_choice_diagnostic import (  # noqa: E402
    TermCandidate,
    enumerate_terms_for_state,
    format_choice,
    format_state,
)


def adjusted_lift_minus_charge_qdim(
    choice: tuple[int, ...],
    *,
    covered_kernel_qdim: int = 0,
    consumed_kernel_qdim: int = 0,
    nested_quotient_qdim: int = 0,
    nested_subspace_qdim: int = 0,
) -> int:
    """Return the q-dimensional local exponent after diagnostic adjustments.

    This intentionally ignores finite split-shape factors.  It is a trace
    decoder for understanding which quotient/kernel part remains load-bearing,
    not a recurrence rule.
    """

    local_charge = choice[7]
    lift_qdim = choice[14]
    return (
        lift_qdim
        - local_charge
        - covered_kernel_qdim
        - consumed_kernel_qdim
        - nested_quotient_qdim
        - nested_subspace_qdim
    )


@dataclass(frozen=True)
class LevelData:
    values: dict[int, list[float]]
    choices: dict[tuple[int, int], tuple[int, ...] | None]
    flag_table: FlagTable | None


@dataclass
class TableStats:
    entries: int = 0
    improved_entries: int = 0
    best_saving_log2: float = 0.0
    best_state: tuple[State, State] | None = None
    best_value_log2: float = NEG_INF
    best_baseline_log2: float = NEG_INF


def parse_flag_state(text: str) -> tuple[int, int, int, int]:
    parts = text.split(",")
    if len(parts) != 4:
        raise argparse.ArgumentTypeError(
            "flag state must be outer_span,outer_z,inner_span,inner_z"
        )
    try:
        outer_span, outer_z, inner_span, inner_z = (int(part) for part in parts)
    except ValueError as exc:
        raise argparse.ArgumentTypeError("flag state entries must be integers") from exc
    return outer_span, outer_z, inner_span, inner_z


def parse_table_state(text: str) -> tuple[int, int, int, int, int]:
    parts = text.split(",")
    if len(parts) != 5:
        raise argparse.ArgumentTypeError(
            "table state must be level,outer_span,outer_z,inner_span,inner_z"
        )
    try:
        level, outer_span, outer_z, inner_span, inner_z = (int(part) for part in parts)
    except ValueError as exc:
        raise argparse.ArgumentTypeError("table state entries must be integers") from exc
    return level, outer_span, outer_z, inner_span, inner_z


def clean_csv_field(text: str) -> str:
    return text.replace(",", ";")


def finite_label(value: float) -> str:
    return "-inf" if value <= NEG_INF / 2 else f"{value:.8f}"


def saving_labels(coarse: float, value: float, q_log2: float) -> tuple[str, str]:
    if coarse <= NEG_INF / 2:
        return "", ""
    if value <= NEG_INF / 2:
        return "inf", "inf"
    saving = coarse - value
    return f"{saving:.8f}", f"{saving / q_log2:.8f}"


def finite_states(values_by_span: dict[int, list[float]]) -> list[State]:
    states: list[State] = []
    for span, values in values_by_span.items():
        for zeros, value in enumerate(values):
            if value > NEG_INF / 2:
                states.append((span, zeros))
    return sorted(states, key=lambda state: (state[0], state[1]))


def take_terms(terms: list[TermCandidate], limit: int) -> list[TermCandidate]:
    if limit <= 0:
        return terms
    return terms[:limit]


def compute_pair_enumerated_flag_table(
    *,
    current_values_by_span: dict[int, list[float]],
    previous_values_by_span: dict[int, list[float]],
    previous_flag_table: FlagTable | None,
    comb: list[list[float]],
    q_log2: float,
    level: int,
    expansion: int,
    singleton_charge: str,
    max_visible_tau: int,
    cover_lift_mode: str,
    cover_kernel_lift: bool,
    term_limit: int,
    exclude_collapsed_active: bool,
    kernel_cover_mode: str,
    nested_quotient_mode: str,
    nested_subspace_mode: str,
    consumed_kernel_mode: str,
    support2_diamond_mode: str,
    support2_line_filter: bool,
    exact_filtered_empty: bool,
    allowed_keys: set[tuple[State, State]] | None = None,
) -> tuple[FlagTable, TableStats]:
    if allowed_keys is not None and len(allowed_keys) == 0:
        return {}, TableStats()

    child_k = 1 << level
    child_n = expansion * (1 << level)
    previous_k = 1 << (level - 1)
    previous_n = expansion * (1 << (level - 1))
    states = finite_states(current_values_by_span)
    table: FlagTable = {}
    stats = TableStats()
    term_cache: dict[State, list[TermCandidate]] = {}
    full_term_cache: dict[State, list[TermCandidate]] = {}

    def enumerate_state_terms(state: State) -> list[TermCandidate]:
        terms = enumerate_terms_for_state(
            child_by_span=previous_values_by_span,
            child_flag_table=previous_flag_table,
            comb=comb,
            q_log2=q_log2,
            child_k=previous_k,
            parent_span=state[0],
            zeros=state[1],
            singleton_charge_mode=singleton_charge,
            flag_bound_mode="best-two-layer-table",
            max_visible_tau=max_visible_tau,
            cover_lift_mode=cover_lift_mode,
            cover_kernel_lift=cover_kernel_lift,
        )
        return terms

    def terms_for_state(state: State) -> list[TermCandidate]:
        cached = term_cache.get(state)
        if cached is not None:
            return cached
        terms = enumerate_state_terms(state)
        cached = take_terms(terms, term_limit)
        term_cache[state] = cached
        return cached

    def full_terms_for_state(state: State) -> list[TermCandidate]:
        cached = full_term_cache.get(state)
        if cached is not None:
            return cached
        cached = enumerate_state_terms(state)
        full_term_cache[state] = cached
        return cached

    for outer_state in states:
        for inner_state in states:
            if inner_state[0] > outer_state[0] or inner_state[1] < outer_state[1]:
                continue
            key = (outer_state, inner_state)
            if allowed_keys is not None and key not in allowed_keys:
                continue
            _label, baseline, _outer_first, _inner_first = flag_child_bound_report(
                child_by_span=current_values_by_span,
                child_k=child_k,
                child_n=child_n,
                outer_span=outer_state[0],
                inner_span=inner_state[0],
                outer_zeros=outer_state[1],
                inner_zeros=inner_state[1],
                q_log2=q_log2,
                mode="best",
            )
            if baseline <= NEG_INF / 2:
                continue

            outer_terms = terms_for_state(outer_state)
            inner_terms = terms_for_state(inner_state)
            _rows, pair_sum = build_pair_rows(
                outer_terms=outer_terms,
                inner_terms=inner_terms,
                child_values=previous_values_by_span,
                child_flag_table=previous_flag_table,
                child_k=previous_k,
                child_n=previous_n,
                q_log2=q_log2,
                outer_parent_span=outer_state[0],
                inner_parent_span=inner_state[0],
                exclude_collapsed_active=exclude_collapsed_active,
                kernel_cover_mode=kernel_cover_mode,
                nested_quotient_mode=nested_quotient_mode,
                nested_subspace_mode=nested_subspace_mode,
                consumed_kernel_mode=consumed_kernel_mode,
                support2_diamond_mode=support2_diamond_mode,
                support2_line_filter=support2_line_filter,
            )
            exact_empty_impossible = False
            if (
                exact_filtered_empty
                and support2_line_filter
                and pair_sum <= NEG_INF / 2
            ):
                if term_limit > 0:
                    outer_terms = full_terms_for_state(outer_state)
                    inner_terms = full_terms_for_state(inner_state)
                    _rows, pair_sum = build_pair_rows(
                        outer_terms=outer_terms,
                        inner_terms=inner_terms,
                        child_values=previous_values_by_span,
                        child_flag_table=previous_flag_table,
                        child_k=previous_k,
                        child_n=previous_n,
                        q_log2=q_log2,
                        outer_parent_span=outer_state[0],
                        inner_parent_span=inner_state[0],
                        exclude_collapsed_active=exclude_collapsed_active,
                        kernel_cover_mode=kernel_cover_mode,
                        nested_quotient_mode=nested_quotient_mode,
                        nested_subspace_mode=nested_subspace_mode,
                        consumed_kernel_mode=consumed_kernel_mode,
                        support2_diamond_mode=support2_diamond_mode,
                        support2_line_filter=support2_line_filter,
                    )
                exact_empty_impossible = pair_sum <= NEG_INF / 2
            if exact_empty_impossible:
                value = NEG_INF
            else:
                value = min(baseline, pair_sum) if pair_sum > NEG_INF / 2 else baseline
            table[key] = value
            stats.entries += 1
            saving = baseline - value
            if saving > 0:
                stats.improved_entries += 1
                if saving > stats.best_saving_log2:
                    stats.best_saving_log2 = saving
                    stats.best_state = (outer_state, inner_state)
                    stats.best_value_log2 = value
                    stats.best_baseline_log2 = baseline
    return table, stats


def collect_pair_row_child_flag_keys(
    *,
    current_values_by_span: dict[int, list[float]],
    previous_values_by_span: dict[int, list[float]],
    previous_flag_table: FlagTable | None,
    comb: list[list[float]],
    q_log2: float,
    level: int,
    expansion: int,
    singleton_charge: str,
    max_visible_tau: int,
    cover_lift_mode: str,
    cover_kernel_lift: bool,
    term_limit: int,
    exclude_collapsed_active: bool,
    kernel_cover_mode: str,
    nested_quotient_mode: str,
    nested_subspace_mode: str,
    consumed_kernel_mode: str,
    support2_diamond_mode: str,
    support2_line_filter: bool,
    demanded_keys: set[tuple[State, State]],
) -> set[tuple[State, State]]:
    """Collect lower two-layer keys queried by pair rows for demanded entries."""

    if previous_flag_table is None or not demanded_keys:
        return set()

    child_k = 1 << level
    child_n = expansion * (1 << level)
    previous_k = 1 << (level - 1)
    previous_n = expansion * (1 << (level - 1))
    states = set(finite_states(current_values_by_span))
    term_cache: dict[State, list[TermCandidate]] = {}

    def terms_for_state(state: State) -> list[TermCandidate]:
        cached = term_cache.get(state)
        if cached is not None:
            return cached
        terms = enumerate_terms_for_state(
            child_by_span=previous_values_by_span,
            child_flag_table=previous_flag_table,
            comb=comb,
            q_log2=q_log2,
            child_k=previous_k,
            parent_span=state[0],
            zeros=state[1],
            singleton_charge_mode=singleton_charge,
            flag_bound_mode="best-two-layer-table",
            max_visible_tau=max_visible_tau,
            cover_lift_mode=cover_lift_mode,
            cover_kernel_lift=cover_kernel_lift,
        )
        cached = take_terms(terms, term_limit)
        term_cache[state] = cached
        return cached

    child_keys: set[tuple[State, State]] = set()
    for outer_state, inner_state in demanded_keys:
        if outer_state not in states or inner_state not in states:
            continue
        if inner_state[0] > outer_state[0] or inner_state[1] < outer_state[1]:
            continue
        _label, baseline, _outer_first, _inner_first = flag_child_bound_report(
            child_by_span=current_values_by_span,
            child_k=child_k,
            child_n=child_n,
            outer_span=outer_state[0],
            inner_span=inner_state[0],
            outer_zeros=outer_state[1],
            inner_zeros=inner_state[1],
            q_log2=q_log2,
            mode="best",
        )
        if baseline <= NEG_INF / 2:
            continue
        for outer in terms_for_state(outer_state):
            if exclude_collapsed_active and choice_has_collapsed_active_container(outer.choice):
                continue
            if (
                support2_line_filter
                and choice_has_support2_line_quotient_impossibility(outer.choice)
            ):
                continue
            for inner in terms_for_state(inner_state):
                if exclude_collapsed_active and choice_has_collapsed_active_container(inner.choice):
                    continue
                if (
                    support2_line_filter
                    and choice_has_support2_line_quotient_impossibility(inner.choice)
                ):
                    continue
                child_layers = tuple(
                    merge_equal_dimension_chain(
                        list(outer.child_layers) + list(inner.child_layers)
                    )
                )
                if len(child_layers) != 2:
                    continue
                child_flag_log2 = flag_table_bound_for_layers(
                    values_by_span=previous_values_by_span,
                    flag_table=previous_flag_table,
                    child_k=previous_k,
                    child_n=previous_n,
                    q_log2=q_log2,
                    layers=child_layers,
                )
                if child_flag_log2 > NEG_INF / 2:
                    child_keys.add((child_layers[0], child_layers[1]))
    return child_keys


def collect_next_lift_flag_keys(
    *,
    child_by_span: dict[int, list[float]],
    child_k: int,
    singleton_charge: str,
    max_visible_tau: int,
    max_parent_span: int | None,
) -> set[tuple[State, State]]:
    """Collect two-layer child flag keys that the next scalar lift may query."""

    child_n = len(next(iter(child_by_span.values()))) - 1
    parent_n = 2 * child_n
    parent_k = 2 * child_k
    if max_parent_span is None:
        max_parent_span = parent_k
    max_parent_span = max(0, min(max_parent_span, parent_k))
    child_spans = sorted(child_by_span)
    incidence_mode = singleton_charge == "endpoint-tau2-layer-incidence"
    keys: set[tuple[State, State]] = set()
    local_profile_cache: dict[
        tuple[int, int, int, int], tuple[int, int, int, int, int, int, int]
    ] = {}

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
                singleton_charge_mode=singleton_charge,
                quotient_delta_floor=quotient_delta_floor,
            )
        return local_profile_cache[key]

    for parent_span in range(1, max_parent_span + 1):
        for z in range(parent_n + 1):
            for p in range(z // 2 + 1):
                singleton_count = z - 2 * p
                if p + singleton_count > child_n:
                    continue
                max_visible_support = min(singleton_count, child_n - p)
                for visible_support_size in range(max_visible_support + 1):
                    outer_zeros = p + singleton_count - visible_support_size
                    inner_zeros = p + singleton_count
                    if outer_zeros > child_n:
                        continue
                    if visible_support_size > child_n - outer_zeros:
                        continue
                    max_tau = min(parent_span, max_visible_tau)
                    for visible_tau in range(1, max_tau + 1):
                        if visible_support_size == 0:
                            continue
                        kernel_dim = parent_span - visible_tau
                        (
                            charge,
                            _local_delta,
                            _local_components,
                            _local_g,
                            _local_theta,
                            _local_h,
                            _local_gamma,
                        ) = cached_local_profile(
                            visible_tau,
                            outer_zeros,
                            visible_support_size,
                        )
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
                                if incidence_mode:
                                    quotient_delta_floor = (
                                        max(0, outer_span - inner_span)
                                        if visible_tau == 2
                                        else 0
                                    )
                                    (
                                        charge,
                                        _local_delta,
                                        _local_components,
                                        _local_g,
                                        _local_theta,
                                        _local_h,
                                        _local_gamma,
                                    ) = cached_local_profile(
                                        visible_tau,
                                        outer_zeros,
                                        visible_support_size,
                                        quotient_delta_floor,
                                    )
                                    if charge >= INF:
                                        continue
                                outer_value = get_child_value(
                                    child_by_span,
                                    child_n,
                                    outer_span,
                                    outer_zeros,
                                )
                                inner_value = get_child_value(
                                    child_by_span,
                                    child_n,
                                    inner_span,
                                    inner_zeros,
                                )
                                if outer_value <= NEG_INF / 2 or inner_value <= NEG_INF / 2:
                                    continue
                                keys.add(
                                    (
                                        (outer_span, outer_zeros),
                                        (inner_span, inner_zeros),
                                    )
                                )
    return keys


def _build_pair_levels_once(
    *,
    depth: int,
    stop_level: int,
    expansion: int,
    q_log2: float,
    singleton_charge: str,
    max_visible_tau: int,
    cover_lift_mode: str,
    cover_kernel_lift: bool,
    prune_to_final_span: int,
    term_limit: int,
    exclude_collapsed_active: bool,
    kernel_cover_mode: str,
    nested_quotient_mode: str,
    nested_subspace_mode: str,
    consumed_kernel_mode: str,
    support2_diamond_mode: str,
    support2_line_filter: bool,
    exact_filtered_empty: bool,
    last_level_keys: set[tuple[State, State]] | None,
    demand_next_level: bool,
    full_table_until: int,
    extra_allowed_keys_by_level: dict[int, set[tuple[State, State]]] | None,
) -> tuple[
    list[LevelData],
    list[TableStats],
    dict[int, set[tuple[State, State]] | None],
]:
    total_n = expansion * (1 << depth)
    comb = log2_comb_table(total_n)
    values: dict[int, list[float]] = {1: [NEG_INF] * (expansion + 1)}
    values[1][0] = 0.0
    levels = [LevelData(values={1: values[1][:]}, choices={}, flag_table=None)]
    stats_by_level = [TableStats()]
    previous_choices: dict[tuple[int, int], tuple[int, ...] | None] | None = None
    previous_flag_table: FlagTable | None = None
    previous_values = {span: row[:] for span, row in values.items()}
    used_allowed_keys_by_level: dict[int, set[tuple[State, State]] | None] = {}

    for level in range(1, stop_level + 1):
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
            exclude_collapsed_active=exclude_collapsed_active,
        )
        allowed_keys = last_level_keys if level == stop_level else None
        if allowed_keys is None and level == depth and level == stop_level:
            allowed_keys = set()
        if (
            allowed_keys is None
            and demand_next_level
            and level < depth
            and level > full_table_until
        ):
            next_max_parent_span = None
            if prune_to_final_span > 0:
                next_max_parent_span = prune_to_final_span * (1 << (depth - (level + 1)))
            allowed_keys = collect_next_lift_flag_keys(
                child_by_span=values,
                child_k=1 << level,
                singleton_charge=singleton_charge,
                max_visible_tau=max_visible_tau,
                max_parent_span=next_max_parent_span,
            )
        extra_allowed_keys = (
            extra_allowed_keys_by_level.get(level)
            if extra_allowed_keys_by_level is not None
            else None
        )
        if allowed_keys is not None and extra_allowed_keys:
            allowed_keys = set(allowed_keys)
            allowed_keys.update(extra_allowed_keys)
        used_allowed_keys_by_level[level] = allowed_keys
        flag_table, table_stats = compute_pair_enumerated_flag_table(
            current_values_by_span=values,
            previous_values_by_span=previous_values,
            previous_flag_table=previous_flag_table,
            comb=comb,
            q_log2=q_log2,
            level=level,
            expansion=expansion,
            singleton_charge=singleton_charge,
            max_visible_tau=max_visible_tau,
            cover_lift_mode=cover_lift_mode,
            cover_kernel_lift=cover_kernel_lift,
            term_limit=term_limit,
            exclude_collapsed_active=exclude_collapsed_active,
            kernel_cover_mode=kernel_cover_mode,
            nested_quotient_mode=nested_quotient_mode,
            nested_subspace_mode=nested_subspace_mode,
            consumed_kernel_mode=consumed_kernel_mode,
            support2_diamond_mode=support2_diamond_mode,
            support2_line_filter=support2_line_filter,
            exact_filtered_empty=exact_filtered_empty,
            allowed_keys=allowed_keys,
        )
        levels.append(
            LevelData(
                values={span: row[:] for span, row in values.items()},
                choices=choices,
                flag_table=flag_table,
            )
        )
        stats_by_level.append(table_stats)
        previous_choices = choices
        previous_flag_table = flag_table
        previous_values = {span: row[:] for span, row in values.items()}
    return levels, stats_by_level, used_allowed_keys_by_level


def build_pair_levels(
    *,
    depth: int,
    stop_level: int,
    expansion: int,
    q_log2: float,
    singleton_charge: str,
    max_visible_tau: int,
    cover_lift_mode: str,
    cover_kernel_lift: bool,
    prune_to_final_span: int,
    term_limit: int,
    exclude_collapsed_active: bool,
    kernel_cover_mode: str,
    nested_quotient_mode: str,
    nested_subspace_mode: str,
    consumed_kernel_mode: str,
    support2_diamond_mode: str,
    support2_line_filter: bool,
    exact_filtered_empty: bool,
    last_level_keys: set[tuple[State, State]] | None,
    demand_next_level: bool,
    full_table_until: int,
    demand_closure_passes: int = 0,
) -> tuple[list[LevelData], list[TableStats]]:
    extra_allowed_keys_by_level: dict[int, set[tuple[State, State]]] = {}
    closure_passes = max(0, demand_closure_passes)
    total_n = expansion * (1 << depth)
    comb = log2_comb_table(total_n)

    for pass_index in range(closure_passes + 1):
        levels, stats_by_level, used_allowed_keys_by_level = _build_pair_levels_once(
            depth=depth,
            stop_level=stop_level,
            expansion=expansion,
            q_log2=q_log2,
            singleton_charge=singleton_charge,
            max_visible_tau=max_visible_tau,
            cover_lift_mode=cover_lift_mode,
            cover_kernel_lift=cover_kernel_lift,
            prune_to_final_span=prune_to_final_span,
            term_limit=term_limit,
            exclude_collapsed_active=exclude_collapsed_active,
            kernel_cover_mode=kernel_cover_mode,
            nested_quotient_mode=nested_quotient_mode,
            nested_subspace_mode=nested_subspace_mode,
            consumed_kernel_mode=consumed_kernel_mode,
            support2_diamond_mode=support2_diamond_mode,
            support2_line_filter=support2_line_filter,
            exact_filtered_empty=exact_filtered_empty,
            last_level_keys=last_level_keys,
            demand_next_level=demand_next_level,
            full_table_until=full_table_until,
            extra_allowed_keys_by_level=extra_allowed_keys_by_level,
        )
        if not demand_next_level or pass_index >= closure_passes:
            return levels, stats_by_level

        changed = False
        for level in range(2, stop_level + 1):
            demanded_keys = used_allowed_keys_by_level.get(level)
            if demanded_keys is None or not demanded_keys:
                continue
            child_level = level - 1
            if child_level <= full_table_until:
                continue
            child_keys = collect_pair_row_child_flag_keys(
                current_values_by_span=levels[level].values,
                previous_values_by_span=levels[child_level].values,
                previous_flag_table=levels[child_level].flag_table,
                comb=comb,
                q_log2=q_log2,
                level=level,
                expansion=expansion,
                singleton_charge=singleton_charge,
                max_visible_tau=max_visible_tau,
                cover_lift_mode=cover_lift_mode,
                cover_kernel_lift=cover_kernel_lift,
                term_limit=term_limit,
                exclude_collapsed_active=exclude_collapsed_active,
                kernel_cover_mode=kernel_cover_mode,
                nested_quotient_mode=nested_quotient_mode,
                nested_subspace_mode=nested_subspace_mode,
                consumed_kernel_mode=consumed_kernel_mode,
                support2_diamond_mode=support2_diamond_mode,
                support2_line_filter=support2_line_filter,
                demanded_keys=demanded_keys,
            )
            if not child_keys:
                continue
            extra_keys = extra_allowed_keys_by_level.setdefault(child_level, set())
            before = len(extra_keys)
            extra_keys.update(child_keys)
            changed = changed or len(extra_keys) > before
        if not changed:
            return levels, stats_by_level

    return levels, stats_by_level


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, default=5)
    parser.add_argument(
        "--stop-level",
        type=int,
        default=0,
        help="if positive, build only through this level while keeping --depth as the pruning horizon",
    )
    parser.add_argument("--expansion", type=int, default=8)
    parser.add_argument("--q-log2", type=float, default=128.0)
    parser.add_argument("--security-bits", type=float, default=80.0)
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
        help=(
            "diagnostic: count a compatible inner tau-positive quotient inside the "
            "already-counted outer visible quotient"
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
            "diagnostic: route decomposable support-two tau-two rows through a "
            "joint child-diamond chain bound while keeping quotient incidence counted"
        ),
    )
    parser.add_argument(
        "--support2-line-quotient-filter",
        action="store_true",
        help=(
            "diagnostic: exclude decomposable support-two tau-two rows with "
            "dim(V/L)=1, where nonempty exact root-line support is impossible"
        ),
    )
    parser.add_argument(
        "--exact-filtered-empty",
        action="store_true",
        help=(
            "diagnostic: when support2-line filtering leaves no pair rows, "
            "recompute that table entry exhaustively and set it to -inf if still empty"
        ),
    )
    parser.add_argument(
        "--proof-shaped",
        action="store_true",
        help="alias for --exclude-collapsed-active --kernel-cover-mode sibling-unconsumed",
    )
    parser.add_argument(
        "--report-flag-state",
        type=parse_flag_state,
        action="append",
        default=[],
    )
    parser.add_argument(
        "--trace-table-state",
        type=parse_table_state,
        action="append",
        default=[],
    )
    parser.add_argument("--trace-table-top", type=int, default=8)
    parser.add_argument("--report-final-z", type=int, action="append", default=[])
    parser.add_argument("--trace-z", type=int, default=-1)
    parser.add_argument("--trace-span", type=int, default=1)
    parser.add_argument(
        "--trace-follow",
        choices=("projection", "coarse-bound"),
        default="coarse-bound",
    )
    parser.add_argument(
        "--last-level-report-only",
        action="store_true",
        help="at --stop-level, compute only the requested --report-flag-state table entries",
    )
    parser.add_argument(
        "--demand-next-level",
        action="store_true",
        help="compute only table entries that can be queried by the next depth-pruned scalar lift",
    )
    parser.add_argument(
        "--full-table-until",
        type=int,
        default=0,
        help=(
            "diagnostic: with --demand-next-level, compute complete pair tables "
            "through this level before switching to sparse demanded keys"
        ),
    )
    parser.add_argument(
        "--demand-closure-passes",
        type=int,
        default=0,
        help=(
            "diagnostic: after each sparse run, add lower pair-table keys queried "
            "by demanded pair rows and rebuild for this many passes"
        ),
    )
    args = parser.parse_args()

    if args.proof_shaped:
        args.exclude_collapsed_active = True
        args.kernel_cover_mode = "sibling-unconsumed"
    stop_level = args.stop_level if args.stop_level > 0 else args.depth
    if stop_level < 0 or stop_level > args.depth:
        raise SystemExit("--stop-level must be between 0 and --depth")
    if args.full_table_until < 0 or args.full_table_until > args.depth:
        raise SystemExit("--full-table-until must be between 0 and --depth")
    if args.demand_closure_passes < 0:
        raise SystemExit("--demand-closure-passes must be nonnegative")
    last_level_keys = None
    if args.last_level_report_only:
        if not args.report_flag_state:
            raise SystemExit("--last-level-report-only requires --report-flag-state")
        last_level_keys = {
            ((outer_span, outer_z), (inner_span, inner_z))
            for outer_span, outer_z, inner_span, inner_z in args.report_flag_state
        }

    levels, stats_by_level = build_pair_levels(
        depth=args.depth,
        stop_level=stop_level,
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
        support2_diamond_mode=args.support2_diamond_mode,
        support2_line_filter=args.support2_line_quotient_filter,
        exact_filtered_empty=args.exact_filtered_empty,
        last_level_keys=last_level_keys,
        demand_next_level=args.demand_next_level,
        full_table_until=args.full_table_until,
        demand_closure_passes=args.demand_closure_passes,
    )

    print(
        "level,k,n,span_count,table_entries,improved_entries,best_saving_log2,"
        "best_saving_qdim,best_state"
    )
    for level, data in enumerate(levels):
        k = 1 << level
        n = args.expansion * (1 << level)
        stats = stats_by_level[level]
        best_state = ""
        if stats.best_state is not None:
            outer_state, inner_state = stats.best_state
            best_state = f"{format_state(outer_state)}>={format_state(inner_state)}"
        best_saving = f"{stats.best_saving_log2:.8f}"
        best_saving_qdim = f"{stats.best_saving_log2 / args.q_log2:.8f}"
        if stats.best_value_log2 <= NEG_INF / 2 and stats.best_state is not None:
            best_saving = "inf"
            best_saving_qdim = "inf"
        print(
            f"{level},{k},{n},{len(data.values)},"
            f"{stats.entries},{stats.improved_entries},"
            f"{best_saving},{best_saving_qdim},"
            f"{best_state}"
        )

    final_values = levels[-1].values.get(1)
    if final_values is not None:
        crossing = first_crossing(final_values, args.security_bits)
        print(f"final_span_1_crossing_z,{crossing if crossing is not None else ''}")
        for z in args.report_final_z:
            value = final_values[z] if 0 <= z < len(final_values) else NEG_INF
            value_label = "-inf" if value <= NEG_INF / 2 else f"{value:.8f}"
            print(f"final_span_1_z_report,{z},{value_label}")

    for target in args.report_flag_state:
        outer_span, outer_z, inner_span, inner_z = target
        print(
            "flag_state_report_level,outer_span,outer_z,inner_span,inner_z,"
            "table_log2,coarse_log2,saving_log2,saving_qdim"
        )
        key = ((outer_span, outer_z), (inner_span, inner_z))
        for level, data in enumerate(levels):
            table_value = data.flag_table.get(key, NEG_INF) if data.flag_table is not None else NEG_INF
            if data.flag_table is None or key not in data.flag_table:
                continue
            _label, coarse, _outer_first, _inner_first = flag_child_bound_report(
                child_by_span=data.values,
                child_k=1 << level,
                child_n=args.expansion * (1 << level),
                outer_span=outer_span,
                inner_span=inner_span,
                outer_zeros=outer_z,
                inner_zeros=inner_z,
                q_log2=args.q_log2,
                mode="best",
            )
            saving_label, saving_qdim_label = saving_labels(coarse, table_value, args.q_log2)
            print(
                f"{level},{outer_span},{outer_z},{inner_span},{inner_z},"
                f"{finite_label(table_value)},"
                f"{finite_label(coarse)},"
                f"{saving_label},"
                f"{saving_qdim_label}"
            )

    for target in args.trace_table_state:
        level, outer_span, outer_z, inner_span, inner_z = target
        print(
            "table_trace_summary_level,outer_span,outer_z,inner_span,inner_z,"
            "baseline_log2,table_log2,pair_sum_log2,row_count"
        )
        if level <= 0 or level >= len(levels):
            print(f"{level},{outer_span},{outer_z},{inner_span},{inner_z},-inf,-inf,-inf,0")
            continue
        current = levels[level]
        previous = levels[level - 1]
        key = ((outer_span, outer_z), (inner_span, inner_z))
        table_value = current.flag_table.get(key, NEG_INF) if current.flag_table is not None else NEG_INF
        current_k = 1 << level
        current_n = args.expansion * (1 << level)
        _label, baseline, _outer_first, _inner_first = flag_child_bound_report(
            child_by_span=current.values,
            child_k=current_k,
            child_n=current_n,
            outer_span=outer_span,
            inner_span=inner_span,
            outer_zeros=outer_z,
            inner_zeros=inner_z,
            q_log2=args.q_log2,
            mode="best",
        )
        previous_k = 1 << (level - 1)
        previous_n = args.expansion * (1 << (level - 1))
        comb = log2_comb_table(args.expansion * (1 << args.depth))
        outer_terms = take_terms(
            enumerate_terms_for_state(
                child_by_span=previous.values,
                child_flag_table=previous.flag_table,
                comb=comb,
                q_log2=args.q_log2,
                child_k=previous_k,
                parent_span=outer_span,
                zeros=outer_z,
                singleton_charge_mode=args.singleton_charge,
                flag_bound_mode="best-two-layer-table",
                max_visible_tau=args.max_visible_tau,
                cover_lift_mode=args.cover_lift_mode,
                cover_kernel_lift=args.cover_kernel_lift,
            ),
            args.term_limit,
        )
        inner_terms = take_terms(
            enumerate_terms_for_state(
                child_by_span=previous.values,
                child_flag_table=previous.flag_table,
                comb=comb,
                q_log2=args.q_log2,
                child_k=previous_k,
                parent_span=inner_span,
                zeros=inner_z,
                singleton_charge_mode=args.singleton_charge,
                flag_bound_mode="best-two-layer-table",
                max_visible_tau=args.max_visible_tau,
                cover_lift_mode=args.cover_lift_mode,
                cover_kernel_lift=args.cover_kernel_lift,
            ),
            args.term_limit,
        )
        rows, pair_sum = build_pair_rows(
            outer_terms=outer_terms,
            inner_terms=inner_terms,
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
            support2_diamond_mode=args.support2_diamond_mode,
            support2_line_filter=args.support2_line_quotient_filter,
        )
        baseline_label = "-inf" if baseline <= NEG_INF / 2 else f"{baseline:.8f}"
        table_label = "-inf" if table_value <= NEG_INF / 2 else f"{table_value:.8f}"
        pair_sum_label = "-inf" if pair_sum <= NEG_INF / 2 else f"{pair_sum:.8f}"
        print(
            f"{level},{outer_span},{outer_z},{inner_span},{inner_z},"
            f"{baseline_label},{table_label},{pair_sum_label},{len(rows)}"
        )
        print(
            "table_trace_row_level,rank,joint_log2,outer_local_log2,"
            "inner_local_log2,child_flag,child_flag_log2,"
            "outer_kernel_lift_qdim,outer_quotient_lift_qdim,"
            "inner_kernel_lift_qdim,inner_quotient_lift_qdim,"
            "outer_covered_kernel_lift_qdim,inner_covered_kernel_lift_qdim,"
            "outer_consumed_kernel_cover_qdim,"
            "inner_nested_quotient_cover_qdim,inner_nested_subspace_cover_qdim,"
            "support2_diamond_saving_qdim,"
            "outer_adjusted_lift_minus_charge_qdim,"
            "inner_adjusted_lift_minus_charge_qdim,"
            "pair_adjusted_lift_minus_charge_qdim,"
            "outer_remaining_quotient_lift_qdim,"
            "inner_remaining_quotient_lift_qdim,"
            "outer_choice,inner_choice"
        )
        for rank, row in enumerate(rows[: args.trace_table_top], start=1):
            outer_covered, inner_covered = covered_kernel_lift_qdims(
                outer_parent_span=outer_span,
                inner_parent_span=inner_span,
                outer_choice=row.outer.choice,
                inner_choice=row.inner.choice,
                kernel_cover_mode=args.kernel_cover_mode,
            )
            outer_adjusted_qdim = adjusted_lift_minus_charge_qdim(
                row.outer.choice,
                covered_kernel_qdim=outer_covered,
                consumed_kernel_qdim=row.outer_consumed_kernel_cover_qdim,
            )
            inner_adjusted_qdim = adjusted_lift_minus_charge_qdim(
                row.inner.choice,
                covered_kernel_qdim=inner_covered,
                nested_quotient_qdim=row.inner_nested_quotient_cover_qdim,
                nested_subspace_qdim=row.inner_nested_subspace_cover_qdim,
            )
            outer_remaining_quotient = choice_quotient_lift_qdim(outer_span, row.outer.choice)
            inner_remaining_quotient = max(
                0,
                choice_quotient_lift_qdim(inner_span, row.inner.choice)
                - row.inner_nested_quotient_cover_qdim,
            )
            child_flag = clean_csv_field(">=".join(format_state(layer) for layer in row.child_layers))
            outer_choice = clean_csv_field(format_choice(row.outer.choice))
            inner_choice = clean_csv_field(format_choice(row.inner.choice))
            print(
                f"{level},{rank},{row.joint_log2:.8f},"
                f"{row.outer_local_log2:.8f},{row.inner_local_log2:.8f},"
                f"{child_flag},{row.child_flag_log2:.8f},"
                f"{choice_kernel_lift_qdim(outer_span, row.outer.choice)},"
                f"{choice_quotient_lift_qdim(outer_span, row.outer.choice)},"
                f"{choice_kernel_lift_qdim(inner_span, row.inner.choice)},"
                f"{choice_quotient_lift_qdim(inner_span, row.inner.choice)},"
                f"{outer_covered},{inner_covered},"
                f"{row.outer_consumed_kernel_cover_qdim},"
                f"{row.inner_nested_quotient_cover_qdim},"
                f"{row.inner_nested_subspace_cover_qdim},"
                f"{row.support2_diamond_saving_qdim:.8f},"
                f"{outer_adjusted_qdim},"
                f"{inner_adjusted_qdim},"
                f"{outer_adjusted_qdim + inner_adjusted_qdim},"
                f"{outer_remaining_quotient},"
                f"{inner_remaining_quotient},"
                f"{outer_choice},{inner_choice}"
            )

    if args.trace_z >= 0:
        comb = log2_comb_table(args.expansion * (1 << args.depth))
        print(
            "trace_level,span,z,log2_state,p,s,a,tau,outer_span,inner_span,"
            "outer_zeros,inner_zeros,local_charge,delta,comp,theta,dominant_h,"
            "gamma,lift_qdim,tau1_quotient_qdim,tau1_support_saving_qdim,"
            "tau1_charged_postroot_qdim,raw_child_flag,raw_child_table_log2,"
            "merged_child_flag,merged_child_table_log2,selected_child_log2,"
            "shape_log2,charge_log2,lift_log2,term_log2,logsum_overhead_log2,"
            "coarse_child_choice,coarse_child_log2,coarse_outer_first_log2,"
            "coarse_inner_first_log2"
        )
        span = args.trace_span
        z = args.trace_z
        for level in range(len(levels) - 1, 0, -1):
            values = levels[level].values.get(span)
            state_value = NEG_INF
            if values is not None and 0 <= z < len(values):
                state_value = values[z]
            state_label = "-inf" if state_value <= NEG_INF / 2 else f"{state_value:.8f}"
            choice = levels[level].choices.get((span, z))
            if choice is None:
                print(f"{level},{span},{z},{state_label},,,,,,,,,,,,,,,,,,,,,,,")
                break
            (
                p,
                singleton_count,
                visible_support_size,
                visible_tau,
                outer_span,
                inner_span,
                outer_zeros,
                local_charge,
                local_delta,
                local_components,
                _local_g,
                local_theta,
                local_h,
                local_gamma,
                lift_qdim,
            ) = choice
            inner_zeros = p + singleton_count
            tau1_profile = tau1_incidence_profile(span, choice)
            tau1_quotient_qdim = ""
            tau1_support_saving_qdim = ""
            tau1_charged_postroot_qdim = ""
            child_n = args.expansion * (1 << (level - 1))
            shape_log2 = choice_split_shape_log2(choice, child_n, comb)
            charge_log2 = -local_charge * args.q_log2
            lift_log2 = lift_qdim * args.q_log2
            selected_child_value = NEG_INF
            if tau1_profile is not None:
                (
                    tau1_quotient_qdim,
                    _tau1_quotient_ambient_dim,
                    _tau1_visible_image_dim_bound,
                    _tau1_invisible_fiber_dim_min,
                    _tau1_universal_postroot_qdim,
                    tau1_support_saving_qdim,
                    tau1_charged_postroot_qdim,
                ) = tau1_profile

            raw_child_flag = ""
            raw_child_table_log2 = ""
            merged_child_flag = ""
            merged_child_table_log2 = ""
            selected_child_log2 = ""
            term_log2 = ""
            logsum_overhead_log2 = ""
            coarse_child_choice = ""
            coarse_child_log2 = ""
            coarse_outer_first_log2 = ""
            coarse_inner_first_log2 = ""
            child_level = levels[level - 1]
            child_k = 1 << (level - 1)
            if visible_tau == 0:
                selected_child_value = get_child_value(
                    child_level.values,
                    child_n,
                    outer_span,
                    outer_zeros,
                )
            else:
                raw_child_layers = tuple(choice_child_layers(choice))
                merged_child_layers = tuple(
                    merge_equal_dimension_chain(list(raw_child_layers))
                )
                raw_child_flag = ">=".join(format_state(layer) for layer in raw_child_layers)
                merged_child_flag = ">=".join(
                    format_state(layer) for layer in merged_child_layers
                )
                raw_child_table_value = flag_table_bound_for_layers(
                    values_by_span=child_level.values,
                    flag_table=child_level.flag_table,
                    child_k=child_k,
                    child_n=child_n,
                    q_log2=args.q_log2,
                    layers=raw_child_layers,
                )
                if raw_child_table_value > NEG_INF / 2:
                    raw_child_table_log2 = f"{raw_child_table_value:.8f}"
                merged_child_table_value = flag_table_bound_for_layers(
                    values_by_span=child_level.values,
                    flag_table=child_level.flag_table,
                    child_k=child_k,
                    child_n=child_n,
                    q_log2=args.q_log2,
                    layers=merged_child_layers,
                )
                if merged_child_table_value > NEG_INF / 2:
                    merged_child_table_log2 = f"{merged_child_table_value:.8f}"
                (
                    coarse_child_choice,
                    coarse_child_value,
                    coarse_outer_first_value,
                    coarse_inner_first_value,
                ) = flag_child_bound_report(
                    child_by_span=child_level.values,
                    child_k=child_k,
                    child_n=child_n,
                    outer_span=outer_span,
                    inner_span=inner_span,
                    outer_zeros=outer_zeros,
                    inner_zeros=inner_zeros,
                    q_log2=args.q_log2,
                    mode="best",
                )
                if coarse_child_value > NEG_INF / 2:
                    coarse_child_log2 = f"{coarse_child_value:.8f}"
                if coarse_outer_first_value > NEG_INF / 2:
                    coarse_outer_first_log2 = f"{coarse_outer_first_value:.8f}"
                if coarse_inner_first_value > NEG_INF / 2:
                    coarse_inner_first_log2 = f"{coarse_inner_first_value:.8f}"
                selected_child_value = coarse_child_value
                if (
                    raw_child_table_value > NEG_INF / 2
                    and raw_child_table_value < selected_child_value
                ):
                    selected_child_value = raw_child_table_value
            if selected_child_value > NEG_INF / 2:
                selected_child_log2 = f"{selected_child_value:.8f}"
                term_value = shape_log2 + charge_log2 + lift_log2 + selected_child_value
                term_log2 = f"{term_value:.8f}"
                if state_value > NEG_INF / 2:
                    logsum_overhead_log2 = f"{state_value - term_value:.8f}"
            print(
                f"{level},{span},{z},{state_label},{p},{singleton_count},"
                f"{visible_support_size},{visible_tau},{outer_span},{inner_span},"
                f"{outer_zeros},{inner_zeros},{local_charge},{local_delta},"
                f"{local_components},{local_theta},{local_h},{local_gamma},"
                f"{lift_qdim},{tau1_quotient_qdim},{tau1_support_saving_qdim},"
                f"{tau1_charged_postroot_qdim},{raw_child_flag},{raw_child_table_log2},"
                f"{merged_child_flag},{merged_child_table_log2},{selected_child_log2},"
                f"{shape_log2:.8f},{charge_log2:.8f},{lift_log2:.8f},"
                f"{term_log2},{logsum_overhead_log2},"
                f"{coarse_child_choice},{coarse_child_log2},{coarse_outer_first_log2},"
                f"{coarse_inner_first_log2}"
            )
            next_span = outer_span
            next_z = outer_zeros
            if (
                args.trace_follow == "coarse-bound"
                and coarse_child_choice == "inner-first"
                and inner_span > 0
            ):
                next_span = inner_span
                next_z = inner_zeros
            span = next_span
            z = next_z


if __name__ == "__main__":
    main()
