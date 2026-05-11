#!/usr/bin/env python3
"""Check the integer alpha-2 local inequality for unbalanced virtual cores."""

from __future__ import annotations

import argparse


def check(max_r: int) -> list[tuple[int, int, int, int, int, int, int]]:
    failures: list[tuple[int, int, int, int, int, int, int]] = []
    for r in range(1, max_r + 1):
        for a_missing in range(r + 1):
            min_o = r + a_missing
            for o in range(min_o, max_r * 3 + 1):
                for c_c in range(2):
                    if a_missing == r and c_c:
                        continue
                    for c_o in range(2):
                        if c_c + c_o > 1:
                            continue
                        holes = 2 * a_missing + c_c
                        outside = 2 * o - c_o
                        defect = outside - holes
                        if holes > 2 * defect:
                            failures.append((r, a_missing, o, c_c, c_o, holes, defect))
    return failures


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--max-r", type=int, default=64)
    args = parser.parse_args()
    failures = check(args.max_r)
    print(f"checked_R=1..{args.max_r} failures={len(failures)}")
    for failure in failures[:20]:
        print(
            "failure "
            f"R={failure[0]} A={failure[1]} O={failure[2]} "
            f"c_C={failure[3]} c_O={failure[4]} holes={failure[5]} defect={failure[6]}"
        )
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
