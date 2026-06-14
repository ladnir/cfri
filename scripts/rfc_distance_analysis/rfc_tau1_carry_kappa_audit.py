#!/usr/bin/env python3
"""Audit tau-one carry rows for an actual kappa_phi obligation.

This is deterministic proof bookkeeping, not a profiler.  It records the
dominant tau-one carry candidates currently discussed in the RFC distance notes
and separates three facts that were getting blurred:

* the tau-one quotient ambient dimensions from the scalar recurrence;
* the visible-only fiber dimension, which is known to be too weak;
* whether the row currently has a defined descendant full-line transition map.

The script intentionally does not invent phi.  If the current recurrence state
does not specify a map E'_B -> E_A, the result is "undefined", not a guessed
kernel dimension.
"""

from __future__ import annotations

import argparse
import csv
import sys
from dataclasses import dataclass
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_flag_span_moment import (  # noqa: E402
    local_visible_trace_profile,
    tau1_incidence_profile,
)


Choice = tuple[int, ...]


@dataclass(frozen=True)
class Tau1Row:
    row_id: str
    level: int
    child_k: int
    parent_span: int
    choice: Choice
    note: str

    @property
    def profile(self) -> tuple[int, int, int, int, int, int, int]:
        profile = tau1_incidence_profile(self.parent_span, self.choice)
        if profile is None:
            raise ValueError(f"{self.row_id} is not a tau-one row")
        return profile

    @property
    def choice_text(self) -> str:
        return ":".join(str(part) for part in self.choice)


@dataclass(frozen=True)
class CarryAuditCase:
    case_id: str
    parent: Tau1Row
    descendant: Tau1Row | None
    status: str
    verdict: str


def make_choice(
    *,
    child_k: int,
    parent_span: int,
    p: int,
    singleton_count: int,
    visible_support_size: int,
    visible_tau: int,
    outer_span: int,
    inner_span: int,
    outer_zeros: int,
    singleton_charge_mode: str = "endpoint-tau2-layer-incidence",
) -> Choice:
    (
        charge,
        local_delta,
        local_components,
        local_g,
        local_theta,
        local_h,
        local_gamma,
    ) = local_visible_trace_profile(
        visible_tau=visible_tau,
        child_k=child_k,
        outer_zero_count=outer_zeros,
        visible_support_size=visible_support_size,
        singleton_charge_mode=singleton_charge_mode,
    )
    kernel_dim = parent_span - visible_tau
    kernel_lift = 0 if kernel_dim == 0 else kernel_dim * (2 * inner_span - kernel_dim)
    quotient_lift = visible_tau * (2 * outer_span - parent_span)
    lift_qdim = kernel_lift + quotient_lift
    return (
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
        local_g,
        local_theta,
        local_h,
        local_gamma,
        lift_qdim,
    )


def known_cases() -> tuple[CarryAuditCase, ...]:
    top_parent = Tau1Row(
        row_id="top_tau1_to_2_15",
        level=5,
        child_k=16,
        parent_span=1,
        choice=make_choice(
            child_k=16,
            parent_span=1,
            p=15,
            singleton_count=4,
            visible_support_size=4,
            visible_tau=1,
            outer_span=2,
            inner_span=0,
            outer_zeros=15,
        ),
        note="current block-potential top row feeding state (2,15)",
    )
    old_parent = Tau1Row(
        row_id="level4_2_15_to_4_7_ge_2_8",
        level=4,
        child_k=8,
        parent_span=2,
        choice=make_choice(
            child_k=8,
            parent_span=2,
            p=7,
            singleton_count=1,
            visible_support_size=1,
            visible_tau=1,
            outer_span=4,
            inner_span=2,
            outer_zeros=7,
        ),
        note="old full-line carry parent row before nested tau0 reroute",
    )
    old_descendant = Tau1Row(
        row_id="old_descendant_4_7_ge_2_8_to_4_5_ge_3_4",
        level=3,
        child_k=4,
        parent_span=4,
        choice=(3, 1, 1, 1, 4, 3, 3, 1, 1, 1, 0, -1, -1, -1, 13),
        note="old descendant tau-one row now rerouted by nested tau0 equal-container filter",
    )
    return (
        CarryAuditCase(
            case_id="current_top_block",
            parent=top_parent,
            descendant=None,
            status="no_descendant_tau1_edge",
            verdict=(
                "kappa_phi is undefined for the immediate top_tau1_to_2_15 transition; "
                "do not spend tau1_full_line_carry on this edge without specifying a later "
                "descendant line and phi"
            ),
        ),
        CarryAuditCase(
            case_id="old_4_7_descendant",
            parent=old_parent,
            descendant=old_descendant,
            status="rerouted_by_nested_tau0_equal_container",
            verdict=(
                "dimension envelope permits kappa_phi=0, but the row is no longer the live "
                "target and no actual phi is specified by the recurrence state"
            ),
        ),
    )


def row_fields(prefix: str, row: Tau1Row | None) -> dict[str, object]:
    if row is None:
        return {
            f"{prefix}_row_id": "",
            f"{prefix}_level": "",
            f"{prefix}_parent_span": "",
            f"{prefix}_choice": "",
            f"{prefix}_quotient_qdim": "",
            f"{prefix}_ambient_dim": "",
            f"{prefix}_visible_image_dim_bound": "",
            f"{prefix}_visible_only_kappa": "",
            f"{prefix}_charged_postroot_qdim": "",
            f"{prefix}_note": "",
        }
    (
        quotient_qdim,
        ambient_dim,
        visible_image_dim_bound,
        visible_only_kappa,
        _universal_postroot_qdim,
        _support_saving_qdim,
        charged_postroot_qdim,
    ) = row.profile
    return {
        f"{prefix}_row_id": row.row_id,
        f"{prefix}_level": row.level,
        f"{prefix}_parent_span": row.parent_span,
        f"{prefix}_choice": row.choice_text,
        f"{prefix}_quotient_qdim": quotient_qdim,
        f"{prefix}_ambient_dim": ambient_dim,
        f"{prefix}_visible_image_dim_bound": visible_image_dim_bound,
        f"{prefix}_visible_only_kappa": visible_only_kappa,
        f"{prefix}_charged_postroot_qdim": charged_postroot_qdim,
        f"{prefix}_note": row.note,
    }


def case_row(case: CarryAuditCase) -> dict[str, object]:
    parent_ambient = case.parent.profile[1]
    descendant_ambient = case.descendant.profile[1] if case.descendant is not None else None
    min_possible_kappa = ""
    max_legal_line_saving_if_phi_injective = ""
    if descendant_ambient is not None:
        min_possible_kappa = max(0, descendant_ambient - parent_ambient)
        max_legal_line_saving_if_phi_injective = case.descendant.profile[6] - min_possible_kappa
    return {
        "case_id": case.case_id,
        "status": case.status,
        **row_fields("parent", case.parent),
        **row_fields("descendant", case.descendant),
        "dimension_only_min_possible_kappa": min_possible_kappa,
        "max_legal_line_saving_if_phi_injective": max_legal_line_saving_if_phi_injective,
        "verdict": case.verdict,
    }


def emit_csv(rows: list[dict[str, object]]) -> None:
    fieldnames: list[str] = []
    for row in rows:
        for key in row:
            if key not in fieldnames:
                fieldnames.append(key)
    writer = csv.DictWriter(sys.stdout, fieldnames=fieldnames, lineterminator="\n")
    writer.writeheader()
    writer.writerows(rows)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--case",
        choices=("all", "current_top_block", "old_4_7_descendant"),
        default="all",
    )
    args = parser.parse_args()

    rows = [
        case_row(case)
        for case in known_cases()
        if args.case == "all" or args.case == case.case_id
    ]
    emit_csv(rows)


if __name__ == "__main__":
    main()
