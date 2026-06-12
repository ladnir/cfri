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


def choice_kernel_lift_qdim(parent_span: int, choice: tuple[int, ...]) -> int:
    view = choice_view(choice)
    if view.tau <= 0:
        return 0
    kernel_dim = parent_span - view.tau
    return kernel_dim * (2 * view.inner_span - kernel_dim)


def choice_quotient_lift_qdim(parent_span: int, choice: tuple[int, ...]) -> int:
    view = choice_view(choice)
    if view.tau <= 0:
        return 0
    return view.tau * (2 * view.outer_span - parent_span)


def adjusted_local_log2(
    term: TermCandidate,
    *,
    parent_span: int,
    q_log2: float,
    kernel_cover_mode: str,
) -> float:
    if kernel_cover_mode == "none":
        return term.local_log2
    return term.local_log2 - choice_kernel_lift_qdim(parent_span, term.choice) * q_log2


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
) -> tuple[list[PairRow], float]:
    rows: list[PairRow] = []
    pair_sum = NEG_INF
    for outer in outer_terms:
        if exclude_collapsed_active and choice_has_collapsed_active_container(outer.choice):
            continue
        for inner in inner_terms:
            if exclude_collapsed_active and choice_has_collapsed_active_container(inner.choice):
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
            outer_local_log2 = adjusted_local_log2(
                outer,
                parent_span=outer_parent_span,
                q_log2=q_log2,
                kernel_cover_mode=kernel_cover_mode,
            )
            inner_local_log2 = adjusted_local_log2(
                inner,
                parent_span=inner_parent_span,
                q_log2=q_log2,
                kernel_cover_mode=kernel_cover_mode,
            )
            joint_log2 = outer_local_log2 + inner_local_log2 + child_flag_log2
            row = PairRow(
                outer=outer,
                inner=inner,
                child_layers=child_layers,
                child_flag_log2=child_flag_log2,
                outer_local_log2=outer_local_log2,
                inner_local_log2=inner_local_log2,
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
        choices=("none", "posthoc", "unconsumed-container"),
        default="none",
        help=(
            "kernel-lift adjustment mode; unconsumed-container is the theorem-mode diagnostic "
            "that keeps quotient incidence counted and assumes no hidden consumed kernel datum"
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
                f"kernel_cover_mode={kernel_cover_mode}"
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
                else "truncated pair sum; kernel cover subtracts only kernel-lift q-dimensions and keeps quotient incidence counted"
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
