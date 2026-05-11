#!/usr/bin/env python3
"""Decode systematic/RFC column indices into expansion and tree-path labels."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def bits_lsb_first(value: int, width: int) -> str:
    return "".join(str((value >> bit) & 1) for bit in range(width))


def parity_label(local_index: int, depth: int, expansion: int) -> str:
    copy = local_index % expansion
    path_index = local_index // expansion
    return f"P(copy={copy},path={bits_lsb_first(path_index, depth)},local={local_index})"


def systematic_label(index: int, depth: int) -> str:
    return f"I(path={bits_lsb_first(index, depth)},col={index})"


def label_column(column: int, depth: int, parity_expansion: int, systematic: bool) -> str:
    k = 1 << depth
    if systematic and column < k:
        return systematic_label(column, depth)
    local = column - k if systematic else column
    return parity_label(local, depth, parity_expansion)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--depth", type=int, required=True)
    parser.add_argument("--parity-expansion", type=int, required=True)
    parser.add_argument("--systematic", action="store_true")
    parser.add_argument("--columns", default=None, help="colon-separated column indices")
    parser.add_argument("--bad-shapes-csv", default=None)
    parser.add_argument("--out", default=None)
    parser.add_argument("--max-rows", type=int, default=20)
    args = parser.parse_args()

    if args.columns is not None:
        columns = [int(value) for value in args.columns.split(":") if value]
        for column in columns:
            print(label_column(column, args.depth, args.parity_expansion, args.systematic))
        return

    if args.bad_shapes_csv is None or args.out is None:
        raise SystemExit("provide either --columns or both --bad-shapes-csv and --out")

    with Path(args.bad_shapes_csv).open(newline="") as input_handle, Path(args.out).open("w", newline="") as output_handle:
        reader = csv.DictReader(input_handle)
        writer = csv.writer(output_handle)
        writer.writerow(["index", "columns", "labels"])
        for row_index, row in enumerate(reader):
            if row_index >= args.max_rows:
                break
            if "columns" in row:
                columns = [int(value) for value in row["columns"].split(":")]
            else:
                columns = [
                    int(value)
                    for key, value in sorted(row.items())
                    if key.startswith("full_col_") and value != ""
                ]
            labels = [label_column(column, args.depth, args.parity_expansion, args.systematic) for column in columns]
            writer.writerow([row.get("index", row_index), ":".join(str(column) for column in columns), "; ".join(labels)])


if __name__ == "__main__":
    main()
