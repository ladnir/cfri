#!/usr/bin/env python3
"""Fixed-survivor rank-tail oracle for the original RFC.

This is the first diagnostic for the "global subspace-evasion" route.  For a fixed survivor
coordinate set S, it enumerates or samples RFC challenge vectors T and measures:

    Pr_T[ rank(G_S(T)) < k ]
    E_T[ # {m != 0 : RFC_T(m)|_S = 0} ] = E_T[q^(k-rank(G_S(T))) - 1].

Here G_S is the k x |S| generator submatrix restricted to survivor coordinates.  The event
rank(G_S) < k is exactly erasure-decoding failure from S.  This avoids the old row-by-row
zero-count recurrence: the object is a fixed coordinate set, not an aggregate over placements.

Keep parameters tiny for exact mode.  For larger depth use --challenge-samples and/or --max-sets.
"""

from __future__ import annotations

import argparse
import itertools
import math
import random
import sys
from collections import Counter, defaultdict
from fractions import Fraction
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
if str(SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPT_DIR))

from rfc_brute_force_moment import challenge_domain, encode, num_challenges  # noqa: E402
from rfc_sufficiency_test import mat_rank_modq  # noqa: E402


def parse_survivor_sizes(text: str, k: int, n: int) -> list[int]:
    out: list[int] = []
    for part in text.split(","):
        part = part.strip()
        if not part:
            continue
        if ".." in part:
            lo_s, hi_s = part.split("..", 1)
            lo = k if lo_s == "k" else (k + int(lo_s[2:]) if lo_s.startswith("k+") else int(lo_s))
            hi = k if hi_s == "k" else (k + int(hi_s[2:]) if hi_s.startswith("k+") else int(hi_s))
            out.extend(range(lo, hi + 1))
        else:
            out.append(k if part == "k" else (k + int(part[2:]) if part.startswith("k+") else int(part)))
    bad = [s for s in out if s < 0 or s > n]
    if bad:
        raise SystemExit(f"survivor sizes outside [0,{n}]: {bad}")
    return sorted(dict.fromkeys(out))


def iter_survivor_sets(n: int, s: int, max_sets: int | None, rng: random.Random):
    total = math.comb(n, s)
    if max_sets is None or total <= max_sets:
        yield from itertools.combinations(range(n), s)
        return
    seen: set[tuple[int, ...]] = set()
    while len(seen) < max_sets:
        cand = tuple(sorted(rng.sample(range(n), s)))
        if cand in seen:
            continue
        seen.add(cand)
        yield cand


def cyclic_block(n: int, start: int, s: int) -> tuple[int, ...]:
    return tuple(sorted((start + i) % n for i in range(s)))


def stride_set(n: int, start: int, stride: int, s: int) -> tuple[int, ...] | None:
    vals = []
    seen = set()
    x = start
    for _ in range(s):
        if x in seen:
            return None
        seen.add(x)
        vals.append(x)
        x = (x + stride) % n
    return tuple(sorted(vals))


def recursive_block(n: int, start: int, s: int, depth: int) -> tuple[int, ...]:
    """Prefer complete aligned blocks at all dyadic scales, then fill the remainder."""
    if s >= n or depth <= 0:
        return tuple(range(n)) if s >= n else cyclic_block(n, start, s)
    half = n // 2
    if s <= half:
        side = 0 if (start // half) % 2 == 0 else half
        return tuple(side + x for x in recursive_block(half, start % half, s, depth - 1))
    left = tuple(x for x in recursive_block(half, start % half, min(half, (s + 1) // 2), depth - 1))
    right_need = s - len(left)
    right = tuple(half + x for x in recursive_block(half, start % half, right_need, depth - 1))
    return tuple(sorted(left + right))


def top_pair_family(n: int, s: int, mode: str, start: int = 0) -> tuple[int, ...] | None:
    half = n // 2
    if s > n:
        return None
    pairs = [((start + i) % half, (start + i) % half + half) for i in range(half)]
    vals: list[int] = []
    if mode == "pair_heavy":
        full_pairs = min(half, s // 2)
        for a, b in pairs[:full_pairs]:
            vals.extend([a, b])
        rem = s - len(vals)
        for a, _b in pairs[full_pairs:full_pairs + rem]:
            vals.append(a)
    elif mode == "singleton_heavy":
        singletons = min(half, s)
        for a, _b in pairs[:singletons]:
            vals.append(a)
        rem = s - len(vals)
        for a, b in pairs[singletons:singletons + (rem + 1) // 2]:
            if len(vals) < s:
                vals.append(a)
            if len(vals) < s:
                vals.append(b)
    elif mode == "balanced":
        # Alternate paired and singleton top positions.
        i = 0
        while len(vals) < s and i < half:
            a, b = pairs[i]
            if i % 2 == 0 and len(vals) + 2 <= s:
                vals.extend([a, b])
            else:
                vals.append(a)
            i += 1
        i = 0
        while len(vals) < s and i < half:
            b = pairs[i][1]
            if b not in vals:
                vals.append(b)
            i += 1
    else:
        raise ValueError(mode)
    if len(vals) != s or len(set(vals)) != s:
        return None
    return tuple(sorted(vals))


def iter_structured_survivor_sets(n: int, s: int, depth: int,
                                  families: set[str]) -> list[tuple[str, tuple[int, ...]]]:
    """Return labeled structured survivor sets, de-duplicated by set."""
    out: list[tuple[str, tuple[int, ...]]] = []
    seen: set[tuple[int, ...]] = set()

    def add(label: str, cand: tuple[int, ...] | None) -> None:
        if cand is None or len(cand) != s or len(set(cand)) != s:
            return
        if any(x < 0 or x >= n for x in cand):
            return
        cand = tuple(sorted(cand))
        if cand in seen:
            return
        seen.add(cand)
        out.append((label, cand))

    want_all = "structured" in families
    if want_all or "blocks" in families:
        for start in range(n):
            add(f"block:{start}", cyclic_block(n, start, s))
    if want_all or "strides" in families:
        for stride in range(2, n):
            if math.gcd(stride, n) != 1 and s > n // math.gcd(stride, n):
                continue
            for start in range(min(n, 2 * stride)):
                add(f"stride:{start}:{stride}", stride_set(n, start % n, stride, s))
    if want_all or "top" in families:
        half = n // 2
        for start in range(half):
            add(f"pair_heavy:{start}", top_pair_family(n, s, "pair_heavy", start))
            add(f"singleton_heavy:{start}", top_pair_family(n, s, "singleton_heavy", start))
            add(f"balanced:{start}", top_pair_family(n, s, "balanced", start))
    if want_all or "recursive" in families:
        for start in range(n):
            add(f"recursive:{start}", recursive_block(n, start, s, depth))
    return out


def iter_challenges(depth: int, c: int, q: int, nonzero_t: bool,
                    samples: int | None, rng: random.Random):
    n_chal = num_challenges(depth, c)
    dom = challenge_domain(q, nonzero_t)
    total = len(dom) ** n_chal
    if samples is None:
        yield from itertools.product(dom, repeat=n_chal)
        return
    for _ in range(samples):
        yield tuple(rng.choice(dom) for _ in range(n_chal))


def basis_generator(depth: int, c: int, q: int, challenges: tuple[int, ...]) -> list[list[int]]:
    """Return k generator rows, one encoded basis vector per row."""
    k = 1 << depth
    rows: list[list[int]] = []
    for i in range(k):
        msg = [0] * k
        msg[i] = 1
        rows.append(encode(tuple(msg), depth, c, q, list(challenges)))
    return rows


def restricted_rank(generator_rows: list[list[int]], survivors: tuple[int, ...], q: int) -> int:
    return mat_rank_modq([[row[j] for j in survivors] for row in generator_rows], q)


def top_pair_profile(survivors: tuple[int, ...], n: int) -> tuple[int, int, int, int]:
    """Return (pairs, singletons, empty_pairs, touched_child_positions) for the top fold."""
    half = n // 2
    surv = set(survivors)
    pairs = singletons = empty = 0
    for j in range(half):
        a = j in surv
        b = (j + half) in surv
        if a and b:
            pairs += 1
        elif a or b:
            singletons += 1
        else:
            empty += 1
    return pairs, singletons, empty, pairs + singletons


def top_child_sets(survivors: tuple[int, ...], n: int) -> tuple[tuple[int, ...], tuple[int, ...], tuple[int, ...]]:
    """Return child positions (P,T,U): paired, singleton, and touched at the top fold."""
    half = n // 2
    surv = set(survivors)
    paired: list[int] = []
    singleton: list[int] = []
    touched: list[int] = []
    for j in range(half):
        a = j in surv
        b = (j + half) in surv
        if a and b:
            paired.append(j)
            touched.append(j)
        elif a or b:
            singleton.append(j)
            touched.append(j)
    return tuple(paired), tuple(singleton), tuple(touched)


def top_singleton_sides(survivors: tuple[int, ...], n: int) -> tuple[int, ...]:
    """Return 0 for a left singleton and 1 for a right singleton, in top_child_sets singleton order."""
    half = n // 2
    surv = set(survivors)
    sides: list[int] = []
    for j in range(half):
        a = j in surv
        b = (j + half) in surv
        if a ^ b:
            sides.append(0 if a else 1)
    return tuple(sides)


def level_pair_profile(survivors: tuple[int, ...], n: int, depth: int,
                       level_from_top: int) -> tuple[int, int, int, int]:
    """Pair/singleton profile across all fold chunks at one recursive level.

    level_from_top=0 is the final/top fold on one chunk of length n.  The last level is the
    bottom fold on 2^(depth-1) chunks of length 2c.
    """
    if level_from_top < 0 or level_from_top >= depth:
        raise ValueError(level_from_top)
    chunk_len = n >> level_from_top
    half = chunk_len >> 1
    surv = set(survivors)
    pairs = singletons = empty = 0
    for base in range(0, n, chunk_len):
        for j in range(half):
            a = (base + j) in surv
            b = (base + half + j) in surv
            if a and b:
                pairs += 1
            elif a or b:
                singletons += 1
            else:
                empty += 1
    return pairs, singletons, empty, pairs + singletons


def multilevel_profile(survivors: tuple[int, ...], n: int, depth: int) -> tuple[tuple[int, int, int, int], ...]:
    return tuple(level_pair_profile(survivors, n, depth, level) for level in range(depth))


def format_multilevel_profile(profile: tuple[tuple[int, int, int, int], ...]) -> str:
    # P/S/E/T per level from top to bottom.
    return "|".join(f"{p}/{s}/{e}/{t}" for p, s, e, t in profile)


def compact_set(survivors: tuple[int, ...], limit: int = 32) -> str:
    if len(survivors) <= limit:
        return " ".join(str(x) for x in survivors)
    head = " ".join(str(x) for x in survivors[:limit])
    return f"{head} ..."


def log_q_fraction(x: Fraction, q: int) -> float:
    if x == 0:
        return float("-inf")
    return (math.log(x.numerator) - math.log(x.denominator)) / math.log(q)


def format_counter(counter: Counter[int] | Counter[tuple[int, int]], limit: int) -> str:
    """Compact deterministic histogram string. Values are raw counts over challenge samples."""
    items = sorted(counter.items(), key=lambda kv: kv[0])
    if len(items) > limit:
        items = items[:limit]
        suffix = ";..."
    else:
        suffix = ""
    parts = []
    for key, val in items:
        if isinstance(key, tuple):
            kstr = "/".join(str(x) for x in key)
        else:
            kstr = str(key)
        parts.append(f"{kstr}:{val}")
    return ";".join(parts) + suffix


def nullspace_basis_modq(rows: list[list[int]], q: int, num_vars: int) -> list[list[int]]:
    """Basis for {x in F_q^num_vars : rows * x = 0}; rows are linear equations."""
    M = [row[:] for row in rows]
    nrows = len(M)
    pivots: list[int] = []
    r = 0
    for col in range(num_vars):
        piv = None
        for i in range(r, nrows):
            if M[i][col] % q != 0:
                piv = i
                break
        if piv is None:
            continue
        M[r], M[piv] = M[piv], M[r]
        inv = pow(M[r][col] % q, q - 2, q)
        M[r] = [(x * inv) % q for x in M[r]]
        for i in range(nrows):
            if i != r and M[i][col] % q != 0:
                f = M[i][col] % q
                M[i] = [(M[i][j] - f * M[r][j]) % q for j in range(num_vars)]
        pivots.append(col)
        r += 1
        if r == nrows:
            break
    pivot_set = set(pivots)
    free_cols = [j for j in range(num_vars) if j not in pivot_set]
    basis: list[list[int]] = []
    for free in free_cols:
        vec = [0] * num_vars
        vec[free] = 1
        for row_idx in range(len(pivots) - 1, -1, -1):
            col = pivots[row_idx]
            acc = 0
            for j in free_cols:
                acc = (acc + M[row_idx][j] * vec[j]) % q
            vec[col] = (-acc) % q
        basis.append(vec)
    return basis


def exact_rootline_failure(generator_rows: list[list[int]], paired_child: tuple[int, ...],
                           singleton_child: tuple[int, ...], singleton_sides: tuple[int, ...],
                           child_deficit: int, q: int, nonzero_t: bool,
                           max_singletons: int,
                           cache: dict[tuple[object, ...], Fraction] | None = None) -> Fraction | None:
    """Exact top-root repair failure for tiny singleton sets, conditioned on the child generator.

    The domain enumerates the actual top fold challenge t_j.  A left singleton uses alpha=t_j; a
    right singleton uses alpha=t_j+1.
    """
    m = len(singleton_child)
    if child_deficit == 0:
        return Fraction(0)
    if m > max_singletons:
        return None
    k_child = len(generator_rows)
    p_equations = [[row[j] % q for row in generator_rows] for j in paired_child]
    kernel_basis = nullspace_basis_modq(p_equations, q, k_child)
    if len(kernel_basis) != child_deficit:
        return None
    eval_on_t: list[list[int]] = []
    for vec in kernel_basis:
        eval_on_t.append([
            sum(vec[i] * generator_rows[i][j] for i in range(k_child)) % q
            for j in singleton_child
        ])
    cache_key = (
        q,
        nonzero_t,
        child_deficit,
        singleton_sides,
        tuple(tuple(row) for row in eval_on_t),
    )
    if cache is not None and cache_key in cache:
        return cache[cache_key]
    t_domain = challenge_domain(q, nonzero_t)
    fail = 0
    total = 0
    for roots in itertools.product(t_domain, repeat=m):
        rows = []
        for j, t_val in enumerate(roots):
            alpha = t_val if singleton_sides[j] == 0 else (t_val + 1) % q
            left = [eval_on_t[i][j] for i in range(child_deficit)]
            right = [(alpha * eval_on_t[i][j]) % q for i in range(child_deficit)]
            rows.append(left + right)
        if mat_rank_modq(rows, q) < 2 * child_deficit:
            fail += 1
        total += 1
    ans = Fraction(fail, total)
    if cache is not None:
        cache[cache_key] = ans
    return ans


def projective_class_counts_d2(generator_rows: list[list[int]], paired_child: tuple[int, ...],
                               singleton_child: tuple[int, ...], singleton_sides: tuple[int, ...],
                               q: int) -> tuple[int, tuple[tuple[int, int], ...]] | None:
    """For D=2, return zero count and side-counts of projective parallel classes.

    Projective direction labels are discarded; only the multiset of `(left_count,right_count)` over
    nonzero parallel classes remains.
    """
    k_child = len(generator_rows)
    p_equations = [[row[j] % q for row in generator_rows] for j in paired_child]
    kernel_basis = nullspace_basis_modq(p_equations, q, k_child)
    if len(kernel_basis) != 2:
        return None
    classes: dict[tuple[int, int], list[int]] = defaultdict(lambda: [0, 0])
    zero_cols = 0
    for col_idx, child_pos in enumerate(singleton_child):
        vec = tuple(
            sum(kernel_basis[row][i] * generator_rows[i][child_pos] for i in range(k_child)) % q
            for row in range(2)
        )
        if vec == (0, 0):
            zero_cols += 1
            continue
        if vec[0] % q != 0:
            inv = pow(vec[0] % q, q - 2, q)
            key = (1, (vec[1] * inv) % q)
        else:
            key = (0, 1)
        classes[key][singleton_sides[col_idx]] += 1
    class_counts = sorted((lr[0], lr[1]) for lr in classes.values())
    return zero_cols, tuple(class_counts)


def format_projective_class_signature_d2(counts: tuple[int, tuple[tuple[int, int], ...]]) -> str:
    zero_cols, class_counts = counts
    return f"zero={zero_cols};classes={'/'.join(f'{l}:{r}' for l, r in class_counts)}"


def parallel_class_formula_failure_d2(counts: tuple[int, tuple[tuple[int, int], ...]],
                                      q: int, nonzero_t: bool,
                                      cache: dict[tuple[object, ...], Fraction] | None = None) -> Fraction | None:
    """Canonical exact root-line failure probability from D=2 parallel-class side counts."""
    _zero_cols, class_counts = counts
    if len(class_counts) > q + 1:
        return None
    cache_key = (q, nonzero_t, class_counts)
    if cache is not None and cache_key in cache:
        return cache[cache_key]
    directions = [(1, a) for a in range(q)] + [(0, 1)]
    rows_by_var: list[tuple[tuple[int, int], int]] = []
    for idx, (left_count, right_count) in enumerate(class_counts):
        ell = directions[idx]
        rows_by_var.extend((ell, 0) for _ in range(left_count))
        rows_by_var.extend((ell, 1) for _ in range(right_count))
    t_domain = challenge_domain(q, nonzero_t)
    fail = 0
    total = 0
    for roots in itertools.product(t_domain, repeat=len(rows_by_var)):
        rows = []
        for (ell, side), t_val in zip(rows_by_var, roots):
            alpha = t_val if side == 0 else (t_val + 1) % q
            rows.append([ell[0], ell[1], (alpha * ell[0]) % q, (alpha * ell[1]) % q])
        if mat_rank_modq(rows, q) < 4:
            fail += 1
        total += 1
    ans = Fraction(fail, total)
    if cache is not None:
        cache[cache_key] = ans
    return ans


def _inactive_common_alpha_count(left_count: int, right_count: int, q: int, nonzero_t: bool) -> int:
    if left_count + right_count == 0:
        return 1
    if not nonzero_t:
        return q
    if left_count and right_count:
        return max(0, q - 2)
    return q - 1


def parallel_class_envelope_failure_d2(counts: tuple[int, tuple[tuple[int, int], ...]],
                                       q: int, nonzero_t: bool) -> Fraction:
    """Cross-ratio-free D=2 root-line repair envelope.

    This is a theorem-shaped upper bound, not an exact probability.  It depends only on the
    projective parallel-class side counts.  For the production determinant-1 model, use
    `nonzero_t=True`.
    """
    _zero_cols, class_counts = counts
    if not class_counts:
        return Fraction(1)
    total_roots = 1
    inactive_counts: list[int] = []
    active_counts: list[int] = []
    for left_count, right_count in class_counts:
        size = left_count + right_count
        class_total = (q - 1 if nonzero_t else q) ** size
        inactive = _inactive_common_alpha_count(left_count, right_count, q, nonzero_t)
        active = max(0, class_total - inactive)
        total_roots *= class_total
        inactive_counts.append(inactive)
        active_counts.append(active)

    inactive_total = math.prod(inactive_counts)
    if len(class_counts) <= 3:
        no_active_bad = inactive_total
    else:
        # No-active D=2 failures lie on a (1,1) divisor in P^1 x P^1.  There are
        # |PGL_2(F_q)| nonconstant maps plus at most q constant affine-alpha maps.
        pgl2_plus_constants = q * (q * q - 1) + q
        no_active_bad = min(inactive_total, pgl2_plus_constants)

    one_active_bad = 0
    for active in active_counts:
        # With one active projective class, failure requires all inactive classes to share one
        # common alpha direction.  There are at most q affine alpha choices.
        one_active_bad += q * active

    bad = min(total_roots, no_active_bad + one_active_bad)
    return Fraction(bad, total_roots)


def child_quotient_rank_signature(generator_rows: list[list[int]], paired_child: tuple[int, ...],
                                  singleton_child: tuple[int, ...], q: int,
                                  child_deficit: int, max_singletons: int = 12) -> tuple[str, int, int, int, int]:
    """Rank-increment and root-line Hall signature for singleton columns modulo paired columns.

    This is a diagnostic, matroid-flavored state: for every subset A of singleton positions, record
    rank(P union A)-rank(P), grouped by (|A|, increment).  It intentionally uses only child-code
    geometry, before the top fold challenges act on singleton parent columns.

    The Hall term is the generic rank obstruction for the two-copy root-line repair map on
    K_P x K_P:

        hall_min = min_A (|T|-|A| + 2 rank(K_P|_A)).

    Full generic repair requires rank(K_P|_T)=D and hall_min >= 2D.
    """
    m = len(singleton_child)
    base_rank = restricted_rank(generator_rows, paired_child, q)
    full_inc = restricted_rank(generator_rows, tuple(sorted(paired_child + singleton_child)), q) - base_rank
    visibility_deficit = max(0, child_deficit - full_inc)
    if m == 0:
        hall_min = 0
        hall_deficit = max(0, 2 * child_deficit - hall_min)
        return (
            f"m=0;D={child_deficit};full=0;vis_def={visibility_deficit};"
            f"hall_min={hall_min};hall_def={hall_deficit}",
            full_inc, hall_min, visibility_deficit, hall_deficit)
    if m > max_singletons:
        return (
            f"m={m};D={child_deficit};full={full_inc};vis_def={visibility_deficit};"
            "hall_min=unknown;hall_def=unknown;subsets=too_many",
            full_inc, -1, visibility_deficit, -1)
    hist: Counter[tuple[int, int]] = Counter()
    hall_min = 10 ** 9
    for mask in range(1 << m):
        subset = tuple(singleton_child[i] for i in range(m) if (mask >> i) & 1)
        inc = restricted_rank(generator_rows, tuple(sorted(paired_child + subset)), q) - base_rank
        subset_size = len(subset)
        hist[(subset_size, inc)] += 1
        hall_min = min(hall_min, m - subset_size + 2 * inc)
    hall_deficit = max(0, 2 * child_deficit - hall_min)
    return (
        f"m={m};D={child_deficit};full={full_inc};vis_def={visibility_deficit};"
        f"hall_min={hall_min};hall_def={hall_deficit};hist={format_counter(hist, 1 << m)}",
        full_inc, hall_min, visibility_deficit, hall_deficit)


def parse_families(text: str) -> set[str]:
    allowed = {"all", "random", "structured", "blocks", "strides", "top", "recursive"}
    families = {part.strip() for part in text.split(",") if part.strip()}
    bad = families - allowed
    if bad:
        raise SystemExit(f"unknown --set-family values {sorted(bad)}; allowed={sorted(allowed)}")
    if not families:
        families = {"all"}
    return families


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--depth", type=int, required=True)
    ap.add_argument("--expansion", type=int, default=2, help="c")
    ap.add_argument("--q", type=int, required=True, help="prime field size")
    ap.add_argument("--t-domain", choices=["nonzero", "all"], default="nonzero")
    ap.add_argument("--survivors", default="k,k+1,k+2",
                    help="comma list/range of survivor sizes, e.g. k,k+1,k+1..k+3,5")
    ap.add_argument("--max-sets", type=int, default=None,
                    help="sample at most this many survivor sets per size")
    ap.add_argument("--set-family", default="all",
                    help=("survivor candidates: all, random, structured, or comma subfamilies "
                          "blocks,strides,top,recursive"))
    ap.add_argument("--challenge-samples", type=int, default=None,
                    help="sample this many challenge vectors instead of exact enumeration")
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--top-k", type=int, default=12, help="number of worst sets to print per survivor size")
    ap.add_argument("--hist-limit", type=int, default=16,
                    help="max histogram bins printed in per-set diagnostic columns")
    ap.add_argument("--quotient-signature", action="store_true",
                    help=("also group singleton repair by child quotient rank signatures; "
                          "slower, intended for small targeted runs"))
    ap.add_argument("--rootline-exact-max", type=int, default=0,
                    help=("with --quotient-signature, exactly enumerate top-root repair failure "
                          "for singleton counts up to this value"))
    args = ap.parse_args()

    if args.q < 2 or any(args.q % d == 0 for d in range(2, int(args.q ** 0.5) + 1)):
        raise SystemExit(f"--q={args.q} must be prime")

    depth, c, q = args.depth, args.expansion, args.q
    k = 1 << depth
    n = c * k
    sizes = parse_survivor_sizes(args.survivors, k, n)
    families = parse_families(args.set_family)
    rng = random.Random(args.seed)
    nonzero_t = args.t_domain == "nonzero"
    n_chal = num_challenges(depth, c)
    chal_dom_size = len(challenge_domain(q, nonzero_t))
    total_challenges = chal_dom_size ** n_chal
    exact_challenges = args.challenge_samples is None

    all_challenges = list(iter_challenges(depth, c, q, nonzero_t, args.challenge_samples, rng))
    generators = [basis_generator(depth, c, q, chal) for chal in all_challenges]
    rootline_exact_cache: dict[tuple[object, ...], Fraction] = {}
    parallel_formula_cache: dict[tuple[object, ...], Fraction] = {}
    if depth > 0:
        child_chal_count = num_challenges(depth - 1, c)
        child_generators = [
            basis_generator(depth - 1, c, q, tuple(chal[:child_chal_count]))
            for chal in all_challenges
        ]
    else:
        child_generators = []
    num_chal_used = len(generators)

    print(f"# fixed-survivor rank-tail: depth={depth} k={k} c={c} n={n} q={q} "
          f"t_domain={args.t_domain} num_challenges={n_chal}")
    print(f"# challenge_vectors={'all' if exact_challenges else num_chal_used} "
          f"total_challenge_space={total_challenges} set_family={args.set_family} "
          f"set_mode={'all' if args.max_sets is None else f'sample<= {args.max_sets}'}")
    print("# random-rank heuristic: Pr[rank<k] roughly q^-(survivors-k+1)")

    for s in sizes:
        set_rng = random.Random(args.seed + 1009 * s)
        labeled_sets: list[tuple[str, tuple[int, ...]]] = []
        if "all" in families:
            labeled_sets.extend(("all", cand) for cand in iter_survivor_sets(n, s, None, set_rng))
        elif "random" in families:
            cap = args.max_sets if args.max_sets is not None else min(1000, math.comb(n, s))
            labeled_sets.extend(("random", cand) for cand in iter_survivor_sets(n, s, cap, set_rng))
        if "structured" in families or any(f in families for f in ("blocks", "strides", "top", "recursive")):
            labeled_sets.extend(iter_structured_survivor_sets(n, s, depth, families))
        # De-duplicate while preserving the first, most informative label.
        dedup: dict[tuple[int, ...], str] = {}
        for label, cand in labeled_sets:
            dedup.setdefault(cand, label)
        if args.max_sets is not None and "random" not in families and "all" in families:
            # Preserve legacy --max-sets behavior for all-mode sampling.
            sampled = list(iter_survivor_sets(n, s, args.max_sets, set_rng))
            dedup = {cand: "all_sample" for cand in sampled}
        labeled_sets = [(label, cand) for cand, label in dedup.items()]
        exact_sets = "all" in families and args.max_sets is None and len(labeled_sets) == math.comb(n, s)
        rows = []
        profile_stats = defaultdict(lambda: {
            "sets": 0,
            "fail_sum": Fraction(0),
            "kernel_sum": Fraction(0),
            "worst_fail": Fraction(0),
        })
        multilevel_stats = defaultdict(lambda: {
            "sets": 0,
            "fail_sum": Fraction(0),
            "kernel_sum": Fraction(0),
            "worst_fail": Fraction(0),
        })
        repair_group_stats = defaultdict(lambda: {
            "events": 0,
            "repair_fail": 0,
            "parent_fail": 0,
            "inc_hist": Counter(),
            "repair_deficit_hist": Counter(),
            "sets": 0,
            "set_rate_sum": Fraction(0),
            "worst_set_repair_fail": Fraction(0),
        })
        repair_profile_group_stats = defaultdict(lambda: {
            "events": 0,
            "repair_fail": 0,
            "parent_fail": 0,
            "inc_hist": Counter(),
            "repair_deficit_hist": Counter(),
            "sets": 0,
            "set_rate_sum": Fraction(0),
            "worst_set_repair_fail": Fraction(0),
        })
        repair_quotient_group_stats = defaultdict(lambda: {
            "events": 0,
            "repair_fail": 0,
            "parent_fail": 0,
            "inc_hist": Counter(),
            "repair_deficit_hist": Counter(),
            "sets": 0,
            "set_rate_sum": Fraction(0),
            "worst_set_repair_fail": Fraction(0),
        })
        rootline_ambiguity = defaultdict(set)
        rootline_parallel_ambiguity = defaultdict(set)
        rootline_parallel_formula_mismatches = Counter()
        for label, survivors in labeled_sets:
            prof = top_pair_profile(survivors, n)
            mprof = multilevel_profile(survivors, n, depth)
            fail_count = 0
            kernel_sum_int = 0
            rank_sum = 0
            child_rp_sum = 0
            child_ru_sum = 0
            child_u_full_count = 0
            diag_exp_sum = 0
            diag_exp_min: int | None = None
            diag_exp_max: int | None = None
            pair_block_rank_sum = 0
            singleton_inc_sum = 0
            pair_rank_mismatch_count = 0
            singleton_inc_hist: Counter[int] = Counter()
            repair_deficit_hist: Counter[int] = Counter()
            pair_inc_hist: Counter[tuple[int, int]] = Counter()
            local_repair_group_stats = defaultdict(lambda: {"events": 0, "repair_fail": 0})
            local_repair_profile_group_stats = defaultdict(lambda: {"events": 0, "repair_fail": 0})
            local_repair_quotient_group_stats = defaultdict(lambda: {"events": 0, "repair_fail": 0})
            min_rank = k
            paired_child, singleton_child, touched_child = top_child_sets(survivors, n)
            singleton_sides = top_singleton_sides(survivors, n)
            half = n // 2
            pair_parent_survivors = tuple(
                sorted([j for j in paired_child] + [j + half for j in paired_child])
            )
            for idx, gen in enumerate(generators):
                rank = restricted_rank(gen, survivors, q)
                pair_block_rank = restricted_rank(gen, pair_parent_survivors, q)
                singleton_inc = rank - pair_block_rank
                pair_block_rank_sum += pair_block_rank
                singleton_inc_sum += singleton_inc
                singleton_inc_hist[singleton_inc] += 1
                repair_deficit_hist[max(0, k - pair_block_rank - singleton_inc)] += 1
                pair_inc_hist[(pair_block_rank, singleton_inc)] += 1
                rank_sum += rank
                min_rank = min(min_rank, rank)
                if rank < k:
                    fail_count += 1
                kernel_sum_int += q ** (k - rank) - 1
                if depth > 0:
                    child_gen = child_generators[idx]
                    r_p = restricted_rank(child_gen, paired_child, q)
                    r_u = restricted_rank(child_gen, touched_child, q)
                    if pair_block_rank != 2 * r_p:
                        pair_rank_mismatch_count += 1
                    child_rp_sum += r_p
                    child_ru_sum += r_u
                    if r_u == min(1 << (depth - 1), len(touched_child)):
                        child_u_full_count += 1
                    child_deficit = (1 << (depth - 1)) - r_p
                    repair_deficit = max(0, 2 * child_deficit - singleton_inc)
                    repair_key = (len(paired_child), len(singleton_child), child_deficit, r_u)
                    repair_profile_key = (mprof, len(paired_child), len(singleton_child), child_deficit, r_u)
                    repair_group_stats[repair_key]["events"] += 1
                    repair_group_stats[repair_key]["repair_fail"] += 1 if repair_deficit > 0 else 0
                    repair_group_stats[repair_key]["parent_fail"] += 1 if rank < k else 0
                    repair_group_stats[repair_key]["inc_hist"][singleton_inc] += 1
                    repair_group_stats[repair_key]["repair_deficit_hist"][repair_deficit] += 1
                    repair_profile_group_stats[repair_profile_key]["events"] += 1
                    repair_profile_group_stats[repair_profile_key]["repair_fail"] += 1 if repair_deficit > 0 else 0
                    repair_profile_group_stats[repair_profile_key]["parent_fail"] += 1 if rank < k else 0
                    repair_profile_group_stats[repair_profile_key]["inc_hist"][singleton_inc] += 1
                    repair_profile_group_stats[repair_profile_key]["repair_deficit_hist"][repair_deficit] += 1
                    if args.quotient_signature:
                        child_qsig, q_full, q_hall_min, q_vis_def, q_hall_def = child_quotient_rank_signature(
                            child_gen, paired_child, singleton_child, q, child_deficit)
                        exact_root_fail = exact_rootline_failure(
                            child_gen, paired_child, singleton_child, singleton_sides, child_deficit,
                            q, nonzero_t, args.rootline_exact_max, rootline_exact_cache)
                        if exact_root_fail is not None:
                            rootline_ambiguity[(singleton_sides, child_qsig)].add(exact_root_fail)
                            pcounts = projective_class_counts_d2(
                                child_gen, paired_child, singleton_child, singleton_sides, q)
                            if pcounts is not None:
                                psig = format_projective_class_signature_d2(pcounts)
                                rootline_parallel_ambiguity[psig].add(exact_root_fail)
                                formula_fail = parallel_class_formula_failure_d2(
                                    pcounts, q, nonzero_t, parallel_formula_cache)
                                if formula_fail != exact_root_fail:
                                    rootline_parallel_formula_mismatches[(psig, formula_fail, exact_root_fail)] += 1
                        repair_quotient_key = (
                            mprof, len(paired_child), len(singleton_child), child_deficit, r_u,
                            q_full, q_hall_min, q_vis_def, q_hall_def, exact_root_fail, child_qsig)
                        repair_quotient_group_stats[repair_quotient_key]["events"] += 1
                        repair_quotient_group_stats[repair_quotient_key]["repair_fail"] += (
                            1 if repair_deficit > 0 else 0)
                        repair_quotient_group_stats[repair_quotient_key]["parent_fail"] += 1 if rank < k else 0
                        repair_quotient_group_stats[repair_quotient_key]["inc_hist"][singleton_inc] += 1
                        repair_quotient_group_stats[repair_quotient_key]["repair_deficit_hist"][repair_deficit] += 1
                        local_repair_quotient_group_stats[repair_quotient_key]["events"] += 1
                        local_repair_quotient_group_stats[repair_quotient_key]["repair_fail"] += (
                            1 if repair_deficit > 0 else 0)
                    local_repair_group_stats[repair_key]["events"] += 1
                    local_repair_group_stats[repair_key]["repair_fail"] += 1 if repair_deficit > 0 else 0
                    local_repair_profile_group_stats[repair_profile_key]["events"] += 1
                    local_repair_profile_group_stats[repair_profile_key]["repair_fail"] += 1 if repair_deficit > 0 else 0
                    # Explorer-B diagonal-transversality heuristic:
                    # singleton_count - 2 * dim(child kernel after P) + 1.
                    diag_exp = len(singleton_child) - 2 * child_deficit + 1
                    diag_exp_sum += diag_exp
                    diag_exp_min = diag_exp if diag_exp_min is None else min(diag_exp_min, diag_exp)
                    diag_exp_max = diag_exp if diag_exp_max is None else max(diag_exp_max, diag_exp)
            fail_prob = Fraction(fail_count, num_chal_used)
            e_kernel = Fraction(kernel_sum_int, num_chal_used)
            avg_rank = rank_sum / num_chal_used
            avg_pair_block_rank = pair_block_rank_sum / num_chal_used
            avg_singleton_inc = singleton_inc_sum / num_chal_used
            if depth > 0:
                avg_child_rp = child_rp_sum / num_chal_used
                avg_child_ru = child_ru_sum / num_chal_used
                child_u_full_prob = Fraction(child_u_full_count, num_chal_used)
                avg_diag_exp = diag_exp_sum / num_chal_used
            else:
                avg_child_rp = avg_child_ru = avg_diag_exp = 0.0
                child_u_full_prob = Fraction(0)
                diag_exp_min = diag_exp_max = 0
            for repair_key, local_st in local_repair_group_stats.items():
                events = local_st["events"]
                if events == 0:
                    continue
                local_rate = Fraction(local_st["repair_fail"], events)
                repair_group_stats[repair_key]["sets"] += 1
                repair_group_stats[repair_key]["set_rate_sum"] += local_rate
                repair_group_stats[repair_key]["worst_set_repair_fail"] = max(
                    repair_group_stats[repair_key]["worst_set_repair_fail"], local_rate)
            for repair_profile_key, local_st in local_repair_profile_group_stats.items():
                events = local_st["events"]
                if events == 0:
                    continue
                local_rate = Fraction(local_st["repair_fail"], events)
                repair_profile_group_stats[repair_profile_key]["sets"] += 1
                repair_profile_group_stats[repair_profile_key]["set_rate_sum"] += local_rate
                repair_profile_group_stats[repair_profile_key]["worst_set_repair_fail"] = max(
                    repair_profile_group_stats[repair_profile_key]["worst_set_repair_fail"], local_rate)
            if args.quotient_signature:
                for repair_quotient_key, local_st in local_repair_quotient_group_stats.items():
                    events = local_st["events"]
                    if events == 0:
                        continue
                    local_rate = Fraction(local_st["repair_fail"], events)
                    repair_quotient_group_stats[repair_quotient_key]["sets"] += 1
                    repair_quotient_group_stats[repair_quotient_key]["set_rate_sum"] += local_rate
                    repair_quotient_group_stats[repair_quotient_key]["worst_set_repair_fail"] = max(
                        repair_quotient_group_stats[repair_quotient_key]["worst_set_repair_fail"], local_rate)
            profile_stats[prof]["sets"] += 1
            profile_stats[prof]["fail_sum"] += fail_prob
            profile_stats[prof]["kernel_sum"] += e_kernel
            profile_stats[prof]["worst_fail"] = max(profile_stats[prof]["worst_fail"], fail_prob)
            multilevel_stats[mprof]["sets"] += 1
            multilevel_stats[mprof]["fail_sum"] += fail_prob
            multilevel_stats[mprof]["kernel_sum"] += e_kernel
            multilevel_stats[mprof]["worst_fail"] = max(multilevel_stats[mprof]["worst_fail"], fail_prob)
            rows.append((
                fail_prob, e_kernel, min_rank, avg_rank, prof, mprof, label, survivors,
                avg_child_rp, avg_child_ru, child_u_full_prob, avg_diag_exp, diag_exp_min, diag_exp_max,
                avg_pair_block_rank, avg_singleton_inc, pair_rank_mismatch_count,
                singleton_inc_hist, repair_deficit_hist, pair_inc_hist,
            ))

        rows.sort(key=lambda r: (r[0], r[1], -r[2]), reverse=True)
        pred_logq = -(s - k + 1)
        print(f"# survivor_size={s} excess={s-k} sets={'all' if exact_sets else len(labeled_sets)} "
              f"total_sets={math.comb(n, s)} random_rank_fail_logq~{pred_logq}")
        print("kind,survivors,excess,pairs,singletons,empty_pairs,touched,fail_prob,fail_logq,"
              "E_kernel,E_kernel_logq,min_rank,avg_rank,avg_child_rP,avg_child_rU,child_U_full_prob,"
              "avg_diag_exp,diag_exp_min,diag_exp_max,avg_pair_block_rank,avg_singleton_inc,"
              "pair_rank_mismatch_count,singleton_inc_hist,repair_deficit_hist,pair_inc_hist,"
              "multilevel_profile,family,set")
        for row in rows[:args.top_k]:
            (fail_prob, e_kernel, min_rank, avg_rank, prof, mprof, label, survivors,
             avg_child_rp, avg_child_ru, child_u_full_prob, avg_diag_exp, diag_exp_min, diag_exp_max,
             avg_pair_block_rank, avg_singleton_inc, pair_rank_mismatch_count,
             singleton_inc_hist, repair_deficit_hist, pair_inc_hist) = row
            pairs, singletons, empty, touched = prof
            print(f"set,{s},{s-k},{pairs},{singletons},{empty},{touched},"
                  f"{fail_prob},{log_q_fraction(fail_prob, q):.6f},"
                  f"{e_kernel},{log_q_fraction(e_kernel, q):.6f},{min_rank},{avg_rank:.6f},"
                  f"{avg_child_rp:.6f},{avg_child_ru:.6f},{child_u_full_prob},"
                  f"{avg_diag_exp:.6f},{diag_exp_min},{diag_exp_max},"
                  f"{avg_pair_block_rank:.6f},{avg_singleton_inc:.6f},{pair_rank_mismatch_count},"
                  f"\"{format_counter(singleton_inc_hist, args.hist_limit)}\","
                  f"\"{format_counter(repair_deficit_hist, args.hist_limit)}\","
                  f"\"{format_counter(pair_inc_hist, args.hist_limit)}\","
                  f"{format_multilevel_profile(mprof)},{label},\"{compact_set(survivors)}\"")

        prof_rows = []
        for prof, st in profile_stats.items():
            count = st["sets"]
            avg_fail = st["fail_sum"] / count
            avg_kernel = st["kernel_sum"] / count
            prof_rows.append((st["worst_fail"], avg_fail, avg_kernel, prof, count))
        prof_rows.sort(key=lambda r: (r[0], r[1], r[2]), reverse=True)
        print(f"# profile_summary survivor_size={s}")
        print("kind,survivors,excess,pairs,singletons,empty_pairs,touched,profile_sets,"
              "worst_fail,worst_fail_logq,avg_fail,avg_fail_logq,avg_E_kernel,avg_E_kernel_logq")
        for worst_fail, avg_fail, avg_kernel, prof, count in prof_rows[:args.top_k]:
            pairs, singletons, empty, touched = prof
            print(f"profile,{s},{s-k},{pairs},{singletons},{empty},{touched},{count},"
                  f"{worst_fail},{log_q_fraction(worst_fail, q):.6f},"
                  f"{avg_fail},{log_q_fraction(avg_fail, q):.6f},"
                  f"{avg_kernel},{log_q_fraction(avg_kernel, q):.6f}")

        mprof_rows = []
        for mprof, st in multilevel_stats.items():
            count = st["sets"]
            avg_fail = st["fail_sum"] / count
            avg_kernel = st["kernel_sum"] / count
            mprof_rows.append((st["worst_fail"], avg_fail, avg_kernel, mprof, count))
        mprof_rows.sort(key=lambda r: (r[0], r[1], r[2]), reverse=True)
        print(f"# multilevel_profile_summary survivor_size={s}")
        print("kind,survivors,excess,multilevel_profile,profile_sets,"
              "worst_fail,worst_fail_logq,avg_fail,avg_fail_logq,avg_E_kernel,avg_E_kernel_logq")
        for worst_fail, avg_fail, avg_kernel, mprof, count in mprof_rows[:args.top_k]:
            print(f"multilevel,{s},{s-k},{format_multilevel_profile(mprof)},{count},"
                  f"{worst_fail},{log_q_fraction(worst_fail, q):.6f},"
                  f"{avg_fail},{log_q_fraction(avg_fail, q):.6f},"
                  f"{avg_kernel},{log_q_fraction(avg_kernel, q):.6f}")

        repair_rows = []
        for key, st in repair_group_stats.items():
            events = st["events"]
            if events == 0:
                continue
            repair_fail = Fraction(st["repair_fail"], events)
            parent_fail = Fraction(st["parent_fail"], events)
            repair_rows.append((repair_fail, parent_fail, events, key, st))
        repair_rows.sort(key=lambda r: (r[0], r[1], r[2]), reverse=True)
        print(f"# singleton_repair_summary survivor_size={s}")
        print("kind,survivors,excess,pairs,singletons,child_deficit_D,child_rank_U,events,"
              "repair_fail,repair_fail_logq,parent_fail,parent_fail_logq,sets,"
              "avg_set_repair_fail,avg_set_repair_fail_logq,worst_set_repair_fail,"
              "worst_set_repair_fail_logq,"
              "singleton_inc_hist,repair_deficit_hist")
        for repair_fail, parent_fail, events, key, st in repair_rows[:args.top_k]:
            pair_count, singleton_count, child_deficit, child_rank_u = key
            set_count = st["sets"]
            avg_set_repair = st["set_rate_sum"] / set_count if set_count else Fraction(0)
            worst_set_repair = st["worst_set_repair_fail"]
            print(f"repair,{s},{s-k},{pair_count},{singleton_count},{child_deficit},{child_rank_u},{events},"
                  f"{repair_fail},{log_q_fraction(repair_fail, q):.6f},"
                  f"{parent_fail},{log_q_fraction(parent_fail, q):.6f},"
                  f"{set_count},{avg_set_repair},{log_q_fraction(avg_set_repair, q):.6f},"
                  f"{worst_set_repair},{log_q_fraction(worst_set_repair, q):.6f},"
                  f"\"{format_counter(st['inc_hist'], args.hist_limit)}\","
                  f"\"{format_counter(st['repair_deficit_hist'], args.hist_limit)}\"")

        if args.quotient_signature:
            repair_quotient_rows = []
            for key, st in repair_quotient_group_stats.items():
                events = st["events"]
                if events == 0:
                    continue
                repair_fail = Fraction(st["repair_fail"], events)
                parent_fail = Fraction(st["parent_fail"], events)
                repair_quotient_rows.append((repair_fail, parent_fail, events, key, st))
            repair_quotient_rows.sort(key=lambda r: (r[0], r[1], r[2]), reverse=True)
            print(f"# singleton_repair_quotient_summary survivor_size={s}")
            print("kind,survivors,excess,multilevel_profile,pairs,singletons,child_deficit_D,"
                  "child_rank_U,quotient_full_rank,hall_min,visibility_deficit,hall_deficit,"
                  "exact_rootline_fail,exact_rootline_fail_logq,child_quotient_rank_signature,events,"
                  "repair_fail,repair_fail_logq,parent_fail,parent_fail_logq,sets,"
                  "avg_set_repair_fail,avg_set_repair_fail_logq,"
                  "worst_set_repair_fail,worst_set_repair_fail_logq,singleton_inc_hist,repair_deficit_hist")
            for repair_fail, parent_fail, events, key, st in repair_quotient_rows[:args.top_k]:
                (mprof, pair_count, singleton_count, child_deficit, child_rank_u,
                 q_full, q_hall_min, q_vis_def, q_hall_def, exact_root_fail, child_qsig) = key
                set_count = st["sets"]
                avg_set_repair = st["set_rate_sum"] / set_count if set_count else Fraction(0)
                worst_set_repair = st["worst_set_repair_fail"]
                exact_fail_text = "" if exact_root_fail is None else str(exact_root_fail)
                exact_fail_logq = (
                    "nan" if exact_root_fail is None else f"{log_q_fraction(exact_root_fail, q):.6f}")
                print(f"repair_quotient,{s},{s-k},{format_multilevel_profile(mprof)},"
                      f"{pair_count},{singleton_count},{child_deficit},{child_rank_u},"
                      f"{q_full},{q_hall_min},{q_vis_def},{q_hall_def},"
                      f"{exact_fail_text},{exact_fail_logq},\"{child_qsig}\",{events},"
                      f"{repair_fail},{log_q_fraction(repair_fail, q):.6f},"
                      f"{parent_fail},{log_q_fraction(parent_fail, q):.6f},"
                      f"{set_count},{avg_set_repair},{log_q_fraction(avg_set_repair, q):.6f},"
                      f"{worst_set_repair},{log_q_fraction(worst_set_repair, q):.6f},"
                  f"\"{format_counter(st['inc_hist'], args.hist_limit)}\","
                  f"\"{format_counter(st['repair_deficit_hist'], args.hist_limit)}\"")
            if args.rootline_exact_max > 0:
                print(f"# rootline_exact_cache_entries={len(rootline_exact_cache)}")
                ambiguous = [
                    (key, vals) for key, vals in rootline_ambiguity.items()
                    if len(vals) > 1
                ]
                print(f"# rootline_signature_ambiguity total_signatures={len(rootline_ambiguity)} "
                      f"ambiguous_signatures={len(ambiguous)}")
                for (sides, qsig), vals in sorted(ambiguous, key=lambda item: (str(item[0]), str(item[1])))[:args.top_k]:
                    val_text = ";".join(str(v) for v in sorted(vals))
                    print(f"rootline_ambiguity,sides={sides},exact_values=\"{val_text}\","
                          f"signature=\"{qsig}\"")
                parallel_ambiguous = [
                    (key, vals) for key, vals in rootline_parallel_ambiguity.items()
                    if len(vals) > 1
                ]
                print(f"# rootline_parallel_class_ambiguity total_signatures={len(rootline_parallel_ambiguity)} "
                      f"ambiguous_signatures={len(parallel_ambiguous)}")
                print(f"# rootline_parallel_formula_mismatches={sum(rootline_parallel_formula_mismatches.values())} "
                      f"distinct_mismatches={len(rootline_parallel_formula_mismatches)} "
                      f"parallel_formula_cache_entries={len(parallel_formula_cache)}")
                for psig, vals in sorted(parallel_ambiguous, key=lambda item: (str(item[0]), str(item[1])))[:args.top_k]:
                    val_text = ";".join(str(v) for v in sorted(vals))
                    print(f"rootline_parallel_ambiguity,exact_values=\"{val_text}\",signature=\"{psig}\"")
                for (psig, formula_fail, exact_fail), count in sorted(
                        rootline_parallel_formula_mismatches.items(),
                        key=lambda item: (str(item[0]), item[1]))[:args.top_k]:
                    print(f"rootline_parallel_formula_mismatch,count={count},formula={formula_fail},"
                          f"exact={exact_fail},signature=\"{psig}\"")
                parallel_rows = [
                    (min(vals), max(vals), psig, vals)
                    for psig, vals in rootline_parallel_ambiguity.items()
                ]
                parallel_rows.sort(key=lambda row: (row[0], row[1], row[2]))
                print("kind,parallel_class_signature,exact_rootline_values")
                for _lo, _hi, psig, vals in parallel_rows[:args.top_k]:
                    val_text = ";".join(str(v) for v in sorted(vals))
                    print(f"rootline_parallel_signature,\"{psig}\",\"{val_text}\"")

        repair_profile_rows = []
        for key, st in repair_profile_group_stats.items():
            events = st["events"]
            if events == 0:
                continue
            repair_fail = Fraction(st["repair_fail"], events)
            parent_fail = Fraction(st["parent_fail"], events)
            repair_profile_rows.append((repair_fail, parent_fail, events, key, st))
        repair_profile_rows.sort(key=lambda r: (r[0], r[1], r[2]), reverse=True)
        print(f"# singleton_repair_profile_summary survivor_size={s}")
        print("kind,survivors,excess,multilevel_profile,pairs,singletons,child_deficit_D,"
              "child_rank_U,events,repair_fail,repair_fail_logq,parent_fail,parent_fail_logq,sets,"
              "avg_set_repair_fail,avg_set_repair_fail_logq,worst_set_repair_fail,"
              "worst_set_repair_fail_logq,singleton_inc_hist,repair_deficit_hist")
        for repair_fail, parent_fail, events, key, st in repair_profile_rows[:args.top_k]:
            mprof, pair_count, singleton_count, child_deficit, child_rank_u = key
            set_count = st["sets"]
            avg_set_repair = st["set_rate_sum"] / set_count if set_count else Fraction(0)
            worst_set_repair = st["worst_set_repair_fail"]
            print(f"repair_profile,{s},{s-k},{format_multilevel_profile(mprof)},"
                  f"{pair_count},{singleton_count},{child_deficit},{child_rank_u},{events},"
                  f"{repair_fail},{log_q_fraction(repair_fail, q):.6f},"
                  f"{parent_fail},{log_q_fraction(parent_fail, q):.6f},"
                  f"{set_count},{avg_set_repair},{log_q_fraction(avg_set_repair, q):.6f},"
                  f"{worst_set_repair},{log_q_fraction(worst_set_repair, q):.6f},"
                  f"\"{format_counter(st['inc_hist'], args.hist_limit)}\","
                  f"\"{format_counter(st['repair_deficit_hist'], args.hist_limit)}\"")


if __name__ == "__main__":
    main()
