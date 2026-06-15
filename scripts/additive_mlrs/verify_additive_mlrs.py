#!/usr/bin/env python3
"""Verify the additive-MLRS distance and fold identities on small fields.

This is a standalone finite-field model for the notes in:

    docs/additive_mlrs/distance_basis_theorem.md
    docs/additive_mlrs/fold_compatibility_theorem.md

It checks three finite versions of the claims:

1. The additive multilinear basis is triangular by degree.
2. Its evaluation row space equals ordinary degree-<k Reed-Solomon.
3. Small configurations are MDS by exhaustive k-column minors.
4. One-step folding agrees with multilinear coefficient folding, including
   after quotienting into the next additive tower.
"""

from __future__ import annotations

import argparse
import itertools
import json
import math
import random
from dataclasses import dataclass
from typing import Iterable


PRIMITIVES = {
    2: 0b111,        # x^2 + x + 1
    3: 0b1011,       # x^3 + x + 1
    4: 0b10011,      # x^4 + x + 1
    5: 0b100101,     # x^5 + x^2 + 1
    6: 0b1000011,    # x^6 + x + 1
    7: 0b10000011,   # x^7 + x + 1
    8: 0b100011101,  # x^8 + x^4 + x^3 + x^2 + 1
}


def gf2_poly_degree(poly: int) -> int:
    return poly.bit_length() - 1


def gf2_poly_mod(poly: int, modulus: int) -> int:
    mod_degree = gf2_poly_degree(modulus)
    while poly.bit_length() - 1 >= mod_degree:
        poly ^= modulus << (gf2_poly_degree(poly) - mod_degree)
    return poly


def gf2_poly_mul_raw(a: int, b: int) -> int:
    out = 0
    while b:
        if b & 1:
            out ^= a
        a <<= 1
        b >>= 1
    return out


def gf2_poly_mul_mod(a: int, b: int, modulus: int) -> int:
    return gf2_poly_mod(gf2_poly_mul_raw(a, b), modulus)


def gf2_poly_gcd(a: int, b: int) -> int:
    while b:
        a, b = b, gf2_poly_mod(a, b)
    return a


def is_irreducible(modulus: int, degree: int) -> bool:
    if gf2_poly_degree(modulus) != degree or (modulus & 1) == 0:
        return False

    x = 0b10
    power = x
    for i in range(1, degree + 1):
        power = gf2_poly_mul_mod(power, power, modulus)
        if i < degree and gf2_poly_gcd(power ^ x, modulus) != 1:
            return False
    return power == x


def find_irreducible(degree: int) -> int:
    if degree in PRIMITIVES and is_irreducible(PRIMITIVES[degree], degree):
        return PRIMITIVES[degree]
    start = (1 << degree) | 1
    stop = 1 << (degree + 1)
    for modulus in range(start, stop, 2):
        if is_irreducible(modulus, degree):
            return modulus
    raise ValueError(f"no irreducible polynomial found for degree {degree}")


@dataclass(frozen=True)
class GF2m:
    m: int
    modulus: int

    @property
    def size(self) -> int:
        return 1 << self.m

    @property
    def mask(self) -> int:
        return self.size - 1

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
            if aa & self.size:
                aa ^= self.modulus
        return out & self.mask

    def pow(self, a: int, exp: int) -> int:
        out = 1
        base = a
        while exp:
            if exp & 1:
                out = self.mul(out, base)
            base = self.mul(base, base)
            exp >>= 1
        return out

    def inv(self, a: int) -> int:
        if a == 0:
            raise ZeroDivisionError("division by zero in GF(2^m)")
        return self.pow(a, self.size - 2)

    def div(self, a: int, b: int) -> int:
        return self.mul(a, self.inv(b))


def f2_rank(values: Iterable[int]) -> int:
    pivots: dict[int, int] = {}
    rank = 0
    for value in values:
        row = value
        while row:
            bit = row.bit_length() - 1
            pivot = pivots.get(bit)
            if pivot is None:
                pivots[bit] = row
                rank += 1
                break
            row ^= pivot
    return rank


def subset_sum(basis: list[int], index: int) -> int:
    out = 0
    bit = 0
    while index:
        if index & 1:
            out ^= basis[bit]
        index >>= 1
        bit += 1
    return out


def subspace_elements(basis: list[int]) -> list[int]:
    return [subset_sum(basis, index) for index in range(1 << len(basis))]


def mat_rank(field: GF2m, rows: list[list[int]]) -> int:
    if not rows:
        return 0
    mat = [row[:] for row in rows]
    n_rows = len(mat)
    n_cols = len(mat[0])
    rank = 0

    for col in range(n_cols):
        pivot = None
        for row in range(rank, n_rows):
            if mat[row][col] != 0:
                pivot = row
                break
        if pivot is None:
            continue

        mat[rank], mat[pivot] = mat[pivot], mat[rank]
        inv = field.inv(mat[rank][col])
        mat[rank] = [field.mul(inv, value) for value in mat[rank]]

        for row in range(n_rows):
            if row == rank or mat[row][col] == 0:
                continue
            scale = mat[row][col]
            mat[row] = [
                value ^ field.mul(scale, pivot_value)
                for value, pivot_value in zip(mat[row], mat[rank])
            ]
        rank += 1
        if rank == n_rows:
            break

    return rank


def poly_trim(poly: list[int]) -> list[int]:
    out = poly[:]
    while len(out) > 1 and out[-1] == 0:
        out.pop()
    return out


def poly_degree(poly: list[int]) -> int:
    for idx in range(len(poly) - 1, -1, -1):
        if poly[idx] != 0:
            return idx
    return -1


def poly_mul(field: GF2m, lhs: list[int], rhs: list[int]) -> list[int]:
    out = [0] * (len(lhs) + len(rhs) - 1)
    for i, a in enumerate(lhs):
        if a == 0:
            continue
        for j, b in enumerate(rhs):
            if b:
                out[i + j] ^= field.mul(a, b)
    return poly_trim(out)


def poly_scale(field: GF2m, poly: list[int], scalar: int) -> list[int]:
    return poly_trim([field.mul(scalar, coeff) for coeff in poly])


def poly_eval(field: GF2m, poly: list[int], x_value: int) -> int:
    out = 0
    for coeff in reversed(poly):
        out = field.mul(out, x_value) ^ coeff
    return out


def subspace_poly(field: GF2m, prefix_basis: list[int]) -> list[int]:
    poly = [1]
    for root in subspace_elements(prefix_basis):
        poly = poly_mul(field, poly, [root, 1])
    return poly


def normalized_subspace_poly(field: GF2m, basis: list[int], index: int) -> list[int]:
    if index >= len(basis):
        raise IndexError("basis index out of range")
    poly = subspace_poly(field, basis[:index])
    denom = poly_eval(field, poly, basis[index])
    return poly_scale(field, poly, field.inv(denom))


def normalized_subspace_eval(field: GF2m, basis: list[int], index: int, x_value: int) -> int:
    prefix = basis[:index]
    out = 1
    for root in subspace_elements(prefix):
        out = field.mul(out, x_value ^ root)
    denom = 1
    beta = basis[index]
    for root in subspace_elements(prefix):
        denom = field.mul(denom, beta ^ root)
    return field.div(out, denom)


def additive_generator(field: GF2m, domain_basis: list[int], message_dim: int) -> list[list[int]]:
    if message_dim > len(domain_basis):
        raise ValueError("message_dim must be <= domain dimension")
    domain = subspace_elements(domain_basis)
    s_rows = [
        [normalized_subspace_eval(field, domain_basis, bit, point) for point in domain]
        for bit in range(message_dim)
    ]

    rows: list[list[int]] = []
    for basis_index in range(1 << message_dim):
        row = [1] * len(domain)
        for bit in range(message_dim):
            if (basis_index >> bit) & 1:
                row = [field.mul(a, b) for a, b in zip(row, s_rows[bit])]
        rows.append(row)
    return rows


def vandermonde_generator(field: GF2m, domain_basis: list[int], message_dim: int) -> list[list[int]]:
    domain = subspace_elements(domain_basis)
    rows: list[list[int]] = []
    for degree in range(1 << message_dim):
        rows.append([field.pow(point, degree) for point in domain])
    return rows


def verify_triangular_basis(field: GF2m, domain_basis: list[int], message_dim: int) -> dict[str, object]:
    s_polys = [
        normalized_subspace_poly(field, domain_basis, bit)
        for bit in range(message_dim)
    ]

    failures = []
    for basis_index in range(1 << message_dim):
        poly = [1]
        for bit in range(message_dim):
            if (basis_index >> bit) & 1:
                poly = poly_mul(field, poly, s_polys[bit])
        degree = poly_degree(poly)
        diagonal = poly[basis_index] if basis_index < len(poly) else 0
        high_coeffs_zero = all(coeff == 0 for coeff in poly[basis_index + 1 :])
        if degree != basis_index or diagonal == 0 or not high_coeffs_zero:
            failures.append(
                {
                    "basis_index": basis_index,
                    "degree": degree,
                    "diagonal": diagonal,
                    "high_coeffs_zero": high_coeffs_zero,
                }
            )
            break

    return {
        "ok": not failures,
        "checked_basis_polys": 1 << message_dim,
        "first_failure": failures[0] if failures else None,
    }


def verify_row_space(field: GF2m, domain_basis: list[int], message_dim: int) -> dict[str, object]:
    additive = additive_generator(field, domain_basis, message_dim)
    vandermonde = vandermonde_generator(field, domain_basis, message_dim)
    rank_additive = mat_rank(field, additive)
    rank_vandermonde = mat_rank(field, vandermonde)
    rank_union = mat_rank(field, additive + vandermonde)
    k = 1 << message_dim
    return {
        "ok": rank_additive == k and rank_vandermonde == k and rank_union == k,
        "rank_additive": rank_additive,
        "rank_vandermonde": rank_vandermonde,
        "rank_union": rank_union,
    }


def sampled_subsets(n_cols: int, size: int, count: int, seed: int) -> Iterable[tuple[int, ...]]:
    rng = random.Random(seed)
    seen: set[tuple[int, ...]] = set()
    while len(seen) < count:
        subset = tuple(sorted(rng.sample(range(n_cols), size)))
        if subset in seen:
            continue
        seen.add(subset)
        yield subset


def verify_mds(
    field: GF2m,
    domain_basis: list[int],
    message_dim: int,
    max_subsets: int,
    seed: int,
    skip: bool,
) -> dict[str, object]:
    k = 1 << message_dim
    n_cols = 1 << len(domain_basis)
    total = math.comb(n_cols, k)
    if skip:
        return {
            "ok": None,
            "mode": "skipped",
            "checked": 0,
            "total": total,
            "defects": None,
            "first_defect": None,
        }

    generator = additive_generator(field, domain_basis, message_dim)
    if total <= max_subsets:
        mode = "exhaustive"
        iterator = itertools.combinations(range(n_cols), k)
        target = total
    else:
        mode = "sampled"
        target = max_subsets
        iterator = sampled_subsets(n_cols, k, max_subsets, seed)

    checked = 0
    first_defect = None
    defects = 0
    for subset in iterator:
        minor = [[row[col] for col in subset] for row in generator]
        rank = mat_rank(field, minor)
        checked += 1
        if rank != k:
            defects += 1
            if first_defect is None:
                first_defect = {
                    "subset": list(subset),
                    "rank": rank,
                }
            if mode == "exhaustive":
                break

    return {
        "ok": defects == 0 if mode == "exhaustive" else None,
        "mode": mode,
        "checked": checked,
        "total": total,
        "defects": defects,
        "first_defect": first_defect,
    }


def quotient_basis(field: GF2m, domain_basis: list[int]) -> list[int]:
    if len(domain_basis) <= 1:
        return []
    return [
        normalized_subspace_eval(field, domain_basis, 1, beta)
        for beta in domain_basis[1:]
    ]


def verify_one_fold(
    field: GF2m,
    domain_basis: list[int],
    message_dim: int,
    r_values: list[int],
) -> dict[str, object]:
    if message_dim == 0:
        return {
            "ok": True,
            "checked_r": 0,
            "checked_basis_vectors": 0,
            "first_failure": None,
        }

    parent_domain = subspace_elements(domain_basis)
    parent_generator = additive_generator(field, domain_basis, message_dim)
    child_basis = quotient_basis(field, domain_basis)
    child_domain = subspace_elements(child_basis)
    child_generator = additive_generator(field, child_basis, message_dim - 1)

    s0_values = [
        normalized_subspace_eval(field, domain_basis, 0, parent_domain[index << 1])
        for index in range(len(child_domain))
    ]

    quotient_failure = None
    if len(domain_basis) > 1:
        for index, child_point in enumerate(child_domain):
            parent_rep = parent_domain[index << 1]
            quotient_point = normalized_subspace_eval(field, domain_basis, 1, parent_rep)
            if quotient_point != child_point:
                quotient_failure = {
                    "index": index,
                    "quotient_point": quotient_point,
                    "child_point": child_point,
                }
                break

    first_failure = None
    if quotient_failure is None:
        for r_value in r_values:
            for parent_basis_index, parent_row in enumerate(parent_generator):
                child_basis_index = parent_basis_index >> 1
                scale = r_value if (parent_basis_index & 1) else 1
                expected = [
                    field.mul(scale, value)
                    for value in child_generator[child_basis_index]
                ]

                actual = []
                for child_index, b_value in enumerate(s0_values):
                    idx0 = child_index << 1
                    idx1 = idx0 | 1
                    diff = parent_row[idx0] ^ parent_row[idx1]
                    actual.append(parent_row[idx0] ^ field.mul(b_value ^ r_value, diff))

                if actual != expected:
                    mismatch = next(
                        idx for idx, (a, e) in enumerate(zip(actual, expected)) if a != e
                    )
                    first_failure = {
                        "r": r_value,
                        "parent_basis_index": parent_basis_index,
                        "child_index": mismatch,
                        "actual": actual[mismatch],
                        "expected": expected[mismatch],
                    }
                    break
            if first_failure is not None:
                break

    return {
        "ok": quotient_failure is None and first_failure is None,
        "checked_r": len(r_values),
        "checked_basis_vectors": len(parent_generator),
        "quotient_failure": quotient_failure,
        "first_failure": first_failure,
        "child_basis_f2_rank": f2_rank(child_basis),
        "child_domain_size": len(child_domain),
    }


def verify_recursive_folds(
    field: GF2m,
    domain_basis: list[int],
    message_dim: int,
    all_r: bool,
    seed: int,
    sampled_r: int,
) -> dict[str, object]:
    if all_r:
        r_values = list(range(field.size))
        r_mode = "all"
    else:
        rng = random.Random(seed)
        r_values = sorted(set([0, 1] + [rng.randrange(field.size) for _ in range(sampled_r)]))
        r_mode = "sampled"

    levels = []
    current_basis = domain_basis[:]
    current_message_dim = message_dim
    while current_message_dim > 0:
        level_result = verify_one_fold(field, current_basis, current_message_dim, r_values)
        level_result["domain_dim"] = len(current_basis)
        level_result["message_dim"] = current_message_dim
        levels.append(level_result)
        if not level_result["ok"]:
            break
        current_basis = quotient_basis(field, current_basis)
        current_message_dim -= 1

    return {
        "ok": all(level["ok"] for level in levels),
        "r_mode": r_mode,
        "r_values_checked": len(r_values),
        "levels": levels,
    }


def run_verification(args: argparse.Namespace) -> dict[str, object]:
    modulus = args.modulus if args.modulus is not None else find_irreducible(args.m)
    field = GF2m(args.m, modulus)
    domain_dim = args.d + args.rho
    if domain_dim > args.m:
        raise ValueError("need d + rho <= m so the additive domain fits in GF(2^m)")

    domain_basis = [1 << bit for bit in range(domain_dim)]
    basis_rank = f2_rank(domain_basis)
    domain = subspace_elements(domain_basis)
    unique_domain_size = len(set(domain))

    triangular = verify_triangular_basis(field, domain_basis, args.d)
    row_space = verify_row_space(field, domain_basis, args.d)
    mds = verify_mds(
        field,
        domain_basis,
        args.d,
        max_subsets=args.max_mds_subsets,
        seed=args.seed,
        skip=args.skip_mds,
    )
    folds = verify_recursive_folds(
        field,
        domain_basis,
        args.d,
        all_r=args.all_r,
        seed=args.seed,
        sampled_r=args.sampled_r,
    )

    k = 1 << args.d
    n = 1 << domain_dim
    return {
        "config": {
            "field": f"GF(2^{args.m})",
            "modulus_binary": bin(modulus),
            "d": args.d,
            "rho": args.rho,
            "k": k,
            "n": n,
            "rate": f"1/{1 << args.rho}",
            "expected_mds_distance": n - k + 1,
        },
        "domain": {
            "basis_f2_rank": basis_rank,
            "basis_size": len(domain_basis),
            "unique_domain_size": unique_domain_size,
        },
        "triangular_basis": triangular,
        "row_space_equals_rs": row_space,
        "mds_columns": mds,
        "folds": folds,
        "overall_ok": (
            basis_rank == len(domain_basis)
            and unique_domain_size == n
            and triangular["ok"]
            and row_space["ok"]
            and (mds["ok"] is True or mds["mode"] == "sampled" or mds["mode"] == "skipped")
            and folds["ok"]
        ),
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Verify additive-MLRS distance and fold identities on small GF(2^m)."
    )
    parser.add_argument("--m", type=int, default=5, help="field degree for GF(2^m)")
    parser.add_argument("--d", type=int, default=2, help="message dimension log2(k)")
    parser.add_argument("--rho", type=int, default=2, help="rate expansion log2(c)")
    parser.add_argument("--modulus", type=lambda text: int(text, 0), default=None)
    parser.add_argument("--max-mds-subsets", type=int, default=200_000)
    parser.add_argument("--skip-mds", action="store_true")
    parser.add_argument("--all-r", action="store_true", help="check every field challenge")
    parser.add_argument("--sampled-r", type=int, default=8)
    parser.add_argument("--seed", type=int, default=1)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    result = run_verification(args)
    print(json.dumps(result, indent=2, sort_keys=True))
    if not result["overall_ok"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
