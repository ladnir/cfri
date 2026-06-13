"""Emit the canonical RFC diagram block grammar and current test cases.

This is deterministic bookkeeping for the proof reset in
docs/rfc_distance_analysis/rfc_canonical_diagram_certificate_plan.md.  It is not
a profiler and does not consume empirical data.  The goal is to give future LP
or certificate scripts a small stable table of block identities instead of
letting proof obligations live only in prose.
"""

from __future__ import annotations

import argparse
import csv
import json
import sys
from dataclasses import asdict, dataclass


@dataclass(frozen=True)
class Block:
    block_id: str
    category: str
    diagram_data: str
    qdim_rule: str
    proof_note: str
    status: str


@dataclass(frozen=True)
class TestCase:
    case_id: str
    row_shape: str
    blocks: tuple[str, ...]
    expected_signal: str
    open_risk: str


BLOCKS: tuple[Block, ...] = (
    Block(
        "paired_descent",
        "descent",
        "child container only",
        "exact child zero propagation",
        "rfc_canonical_diagram_certificate_plan.md",
        "closed algebra",
    ),
    Block(
        "tau0_container",
        "container",
        "V zero on P union S",
        "count child V once; no parent lift",
        "rfc_covering_flag_lift_lemma.md",
        "theorem-shaped",
    ),
    Block(
        "collapsed_active",
        "exact-support",
        "L=V with nonempty active A",
        "reroute to smaller exact support",
        "rfc_exact_support_quotient_state.md",
        "theorem target",
    ),
    Block(
        "nested_tau0_equal_container",
        "exact-support",
        "lower tau0 node equals upper projection",
        "reroute upper active support",
        "rfc_nested_tau0_equal_container_filter.md",
        "local lemma target",
    ),
    Block(
        "tau1_line",
        "quotient-root",
        "full quotient line R plus root labels",
        "support-subcode/projective line incidence",
        "rfc_tau1_quotient_line_incidence_lemma.md",
        "local lemma target",
    ),
    Block(
        "tau1_full_line_carry",
        "quotient-root",
        "carried full line R and transition map phi",
        "conditional line count q^dim ker(phi)",
        "rfc_tau1_full_line_carry_lemma.md",
        "proof sketch",
    ),
    Block(
        "support2_diamond",
        "diagram",
        "V >= M1,M2 >= L",
        "one joint child diagram, not product of moments",
        "rfc_support_two_tau2_quotient_frame_lemma.md",
        "local theorem target",
    ),
    Block(
        "support2_high_lift",
        "incidence",
        "two component planes in codim-one slices of fixed V",
        "q^4 instead of q^8 post-root",
        "rfc_support_two_high_lift_component_plane_bound.md",
        "local theorem target",
    ),
    Block(
        "support3_stratified",
        "incidence",
        "three component planes plus rank-defect marker",
        "q^2 post-root in rank-three and defect strata",
        "rfc_support_three_rank_defect_incidence.md",
        "local theorem target",
    ),
    Block(
        "support4_root_kernel",
        "container-fiber",
        "root-compatible 4D container",
        "count container once after quotient/root data",
        "rfc_root_kernel_container_cover_diagnostic.md",
        "diagnostic theorem target",
    ),
    Block(
        "kernel_fiber_cover",
        "container-fiber",
        "unconsumed K_parent <= L+L",
        "remove duplicate Gaussian kernel-lift multiplier",
        "rfc_covering_flag_lift_lemma.md",
        "theorem target",
    ),
    Block(
        "consumed_kernel",
        "kernel",
        "lower node marks hidden kernel subspace",
        "carry node or charge containment",
        "rfc_exact_support_quotient_state.md",
        "theorem target",
    ),
    Block(
        "shortened_rank_defect",
        "rank-tail",
        "enlarged child kernel H(B)",
        "expose recursive rank event rho_h(D,z)",
        "rfc_theta_minus_one_isolation_lemma.md",
        "conditional target",
    ),
)


TEST_CASES: tuple[TestCase, ...] = (
    TestCase(
        "level3_4_7_ge_2_8",
        "(4,7)>=(2,8): support2 diamond plus tau1 line",
        ("support2_diamond", "tau1_line", "tau1_full_line_carry", "kernel_fiber_cover"),
        "old stress row becomes a joint diagram/carry problem, not pair-table plumbing",
        "prove full-line transition map and consumed/unconsumed kernel audit",
    ),
    TestCase(
        "support3_2_19_to_4_8",
        "(2,19) -> (4,8): a=3,delta=3,comp=3,K=0,dim V=4",
        ("support3_stratified",),
        "rank-defect incidence recovers the two qdim gap between safe and rank3",
        "import finite constants and exact delta=3 normalization",
    ),
    TestCase(
        "root_kernel_cover_rows",
        "support2/support4/tau1 root-compatible container probes",
        ("support4_root_kernel", "kernel_fiber_cover", "tau1_full_line_carry"),
        "duplicate post-root fibers should be counted once after quotient/root data",
        "hidden fibers must not be consumed by descendants unless carried",
    ),
    TestCase(
        "stratified_residual_3_6",
        "(3,6) -> (3,2)>=(1,4) under stratified checkpoint",
        ("support2_diamond", "tau1_line", "kernel_fiber_cover"),
        "lower child table is strong; residual is higher container/fiber/carry accounting",
        "audit whether any remaining support2 diamond fiber is consumed",
    ),
)


def validate() -> None:
    block_ids = {block.block_id for block in BLOCKS}
    if len(block_ids) != len(BLOCKS):
        raise ValueError("duplicate block_id in BLOCKS")
    case_ids = {case.case_id for case in TEST_CASES}
    if len(case_ids) != len(TEST_CASES):
        raise ValueError("duplicate case_id in TEST_CASES")
    for case in TEST_CASES:
        unknown = sorted(set(case.blocks) - block_ids)
        if unknown:
            raise ValueError(f"{case.case_id} references unknown blocks: {unknown}")


def rows_for(section: str) -> list[dict[str, object]]:
    if section == "blocks":
        return [asdict(block) for block in BLOCKS]
    if section == "tests":
        rows: list[dict[str, object]] = []
        for case in TEST_CASES:
            row = asdict(case)
            row["blocks"] = ";".join(case.blocks)
            rows.append(row)
        return rows
    if section == "all":
        return [
            {"section": "blocks", **row}
            for row in rows_for("blocks")
        ] + [
            {"section": "tests", **row}
            for row in rows_for("tests")
        ]
    raise ValueError(f"unknown section: {section}")


def emit_csv(rows: list[dict[str, object]]) -> None:
    if not rows:
        return
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
    parser.add_argument("--section", choices=("blocks", "tests", "all"), default="blocks")
    parser.add_argument("--format", choices=("csv", "json"), default="csv")
    parser.add_argument("--validate-only", action="store_true")
    args = parser.parse_args()

    validate()
    if args.validate_only:
        return
    rows = rows_for(args.section)
    if args.format == "json":
        print(json.dumps(rows, indent=2, sort_keys=True))
    else:
        emit_csv(rows)


if __name__ == "__main__":
    main()
