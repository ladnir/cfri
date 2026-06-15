#!/usr/bin/env python3
"""Audit the current BaseFold `binary_rs` encoder path on small binary fields.

The goal is narrow: distinguish three possibilities for the existing code path:

1. the generator is exactly a Reed-Solomon evaluation code on the obvious
   additive-subspace domain, up to row operations and column ordering;
2. the generator is MDS on small cases but not that obvious RS code;
3. the generator is only fold-consistent and not MDS.

This script is a standalone model of the relevant Rust routines:

* `get_table_additive_binary`
* `evaluate_over_foldable_domain`

over a small GF(2^m), with the same polynomial-basis elements `1 << i`.
"""

from __future__ import annotations

import argparse
import itertools
from dataclasses import dataclass


PRIMITIVES = {
    4: 0b10011,      # x^4 + x + 1
    5: 0b100101,     # x^5 + x^2 + 1
    6: 0b1000011,    # x^6 + x + 1
    7: 0b10000011,   # x^7 + x + 1
    8: 0b100011101,  # x^8 + x^4 + x^3 + x^2 + 1
}


@dataclass(frozen=True)
class GF2m:
    m: int
    modulus: int

    @property
    def mask(self) -> int:
        return (1 << self.m) - 1

    def add(self, a: int, b: int) -> int:
        return a ^ b

    def mul(self, a: int, b: int) -> int:
        out = 0
        aa = a
        bb = b
        while bb:
            if bb & 1:
                out ^= aa
            bb >>= 1
            aa <<= 1
            if aa & (1 << self.m):
                aa ^= self.modulus
        return out & self.mask

    def square(self, a: int) -> int:
        return self.mul(a, a)

    def pow(self, a: int, e: int) -> int:
        out = 1
        base = a
        while e:
            if e & 1:
                out = self.mul(out, base)
            base = self.mul(base, base)
            e >>= 1
        return out

    def inv(self, a: int) -> int:
        if a == 0:
            raise ZeroDivisionError
        return self.pow(a, (1 << self.m) - 2)

    def div(self, a: int, b: int) -> int:
        return self.mul(a, self.inv(b))


def subset_sum(values: list[int], n_bits: int, index: int) -> int:
    out = 0
    for bit in range(n_bits):
        if (index >> bit) & 1:
            out ^= values[bit]
    return out


def bit_reverse(value: int, bits: int) -> int:
    out = 0
    for i in range(bits):
        out = (out << 1) | ((value >> i) & 1)
    return out


def reverse_index_bits(values: list[int]) -> list[int]:
    bits = (len(values) - 1).bit_length()
    out = [0] * len(values)
    for i, value in enumerate(values):
        out[bit_reverse(i, bits)] = value
    return out


def subspace_map(field: GF2m, elem: int, constant: int) -> int:
    return field.square(elem) ^ field.mul(constant, elem)


def precompute_subspace_evals(field: GF2m, dim: int) -> list[list[int]]:
    basis = [1 << i for i in range(dim)]
    normalization_consts = [1]
    s_evals = [basis[1:]]
    for _ in range(1, dim):
        norm_prev = normalization_consts[-1]
        prev = s_evals[-1]
        norm_const_i = subspace_map(field, prev[0], norm_prev)
        s_i = [subspace_map(field, value, norm_prev) for value in prev[1:]]
        normalization_consts.append(norm_const_i)
        s_evals.append(s_i)
    out: list[list[int]] = []
    for norm, row in zip(normalization_consts, s_evals):
        inv_norm = field.inv(norm)
        out.append([field.mul(value, inv_norm) for value in row])
    return out


def get_table_additive_binary(field: GF2m, poly_size: int, log_rate: int) -> list[list[int]]:
    lg_n = log_rate + (poly_size.bit_length() - 1)
    s_evals_all = precompute_subspace_evals(field, lg_n + 1)
    flat: list[int] = []
    for level in range(lg_n):
        s_evals = s_evals_all[lg_n - level - 1]
        for i in range(1 << level):
            flat.append(subset_sum(s_evals, level, i))
    table: list[list[int]] = []
    for level in range(lg_n):
        start = (1 << level) - 1
        end = (1 << (level + 1)) - 1
        table.append(flat[start:end])
    return table


def encode_binary_rs_path(field: GF2m, coeffs: list[int], log_rate: int) -> list[int]:
    k = len(coeffs)
    logk = k.bit_length() - 1
    rate = 1 << log_rate
    table = get_table_additive_binary(field, k, log_rate)
    word = [0] * (k * rate)
    for i, coeff in enumerate(coeffs):
        for j in range(rate):
            word[i * rate + j] = coeff
    chunk_size = rate
    for i in range(logk):
        table_level = i + log_rate
        level = table[table_level]
        chunk_size <<= 1
        half = chunk_size >> 1
        for base in range(0, len(word), chunk_size):
            for off in range(half, chunk_size):
                t = level[off - half]
                upper = word[base + off]
                lower = word[base + off - half]
                word[base + off] = lower ^ field.mul(upper, t ^ 1)
                word[base + off - half] = lower ^ field.mul(upper, t)
    return reverse_index_bits(word)


def generator_matrix(field: GF2m, num_vars: int, log_rate: int) -> list[list[int]]:
    k = 1 << num_vars
    rows = []
    for i in range(k):
        coeffs = [0] * k
        coeffs[i] = 1
        rows.append(encode_binary_rs_path(field, coeffs, log_rate))
    return rows


def rank(field: GF2m, rows: list[list[int]]) -> int:
    mat = [row[:] for row in rows if any(row)]
    if not mat:
        return 0
    h = 0
    w = len(mat[0])
    for col in range(w):
        pivot = None
        for r in range(h, len(mat)):
            if mat[r][col] != 0:
                pivot = r
                break
        if pivot is None:
            continue
        mat[h], mat[pivot] = mat[pivot], mat[h]
        inv = field.inv(mat[h][col])
        mat[h] = [field.mul(x, inv) for x in mat[h]]
        for r in range(len(mat)):
            if r != h and mat[r][col] != 0:
                factor = mat[r][col]
                mat[r] = [x ^ field.mul(factor, y) for x, y in zip(mat[r], mat[h])]
        h += 1
        if h == len(mat):
            break
    return h


def mds_defects(field: GF2m, gen: list[list[int]], max_checks: int | None) -> tuple[int, int, tuple[int, ...] | None]:
    k = len(gen)
    n = len(gen[0])
    checked = 0
    defects = 0
    first_bad = None
    for cols in itertools.combinations(range(n), k):
        checked += 1
        sub = [[row[c] for c in cols] for row in gen]
        if rank(field, sub) < k:
            defects += 1
            if first_bad is None:
                first_bad = cols
        if max_checks is not None and checked >= max_checks:
            break
    return checked, defects, first_bad


def additive_domain(dim: int) -> list[int]:
    return [i for i in range(1 << dim)]


def vandermonde(field: GF2m, points: list[int], k: int) -> list[list[int]]:
    rows: list[list[int]] = []
    for degree in range(k):
        rows.append([field.pow(x, degree) for x in points])
    return rows


def rowspace_equal(field: GF2m, left: list[list[int]], right: list[list[int]]) -> bool:
    return rank(field, left) == rank(field, right) == rank(field, left + right)


def candidate_rs_matches(field: GF2m, gen: list[list[int]], log_n: int) -> list[tuple[str, bool]]:
    points = additive_domain(log_n)
    k = len(gen)
    candidates = [
        ("subspace-natural", points),
        ("subspace-bitrev", [points[bit_reverse(i, log_n)] for i in range(1 << log_n)]),
    ]
    out = []
    for name, domain in candidates:
        out.append((name, rowspace_equal(field, gen, vandermonde(field, domain, k))))
    return out


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--m", type=int, default=6)
    parser.add_argument("--num-vars", type=int, action="append", default=None)
    parser.add_argument("--log-rate", type=int, action="append", default=None)
    parser.add_argument("--max-checks", type=int, default=None)
    args = parser.parse_args()

    field = GF2m(args.m, PRIMITIVES[args.m])
    num_vars_list = args.num_vars if args.num_vars is not None else [1, 2, 3]
    log_rate_list = args.log_rate if args.log_rate is not None else [1, 2]
    print("m,num_vars,log_rate,k,n,rank,checked_k_subsets,defective_k_subsets,first_bad,rs_matches")
    for num_vars in num_vars_list:
        for log_rate in log_rate_list:
            log_n = num_vars + log_rate
            if log_n + 1 > args.m:
                continue
            gen = generator_matrix(field, num_vars, log_rate)
            k = len(gen)
            n = len(gen[0])
            gen_rank = rank(field, gen)
            checked, defects, first_bad = mds_defects(field, gen, args.max_checks)
            matches = ";".join(f"{name}:{ok}" for name, ok in candidate_rs_matches(field, gen, log_n))
            bad_text = "" if first_bad is None else ":".join(str(c) for c in first_bad)
            print(f"{args.m},{num_vars},{log_rate},{k},{n},{gen_rank},{checked},{defects},{bad_text},{matches}")


if __name__ == "__main__":
    main()
