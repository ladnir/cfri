#!/usr/bin/env python3
"""Exact (brute-force) first moment of the original RFC, used as ground truth.

This is the M1 oracle for the new direction (see docs/rfc_distance_analysis/claude_new_direction.md).
It enumerates the actual random foldable code at small (c, q, depth) and computes the EXACT
aggregate first moment

    B_d(1, z) = E_T[ sum_{m != 0} binom( zeros(codeword(m)), z ) ]

with no relaxation and no charge heuristic.  The expectation is over all per-coordinate fold
challenges T.  The identity above counts, for every nonzero message, every size-z subset of its zero
set, which is exactly the aggregate pair-count B_d(1,z) summed over zero sets of size z.

Construction (faithful to crates/cfri/src/backend/basefold.rs):
  - depth 0: repetition base code [c, 1], message symbol x -> (x, x, ..., x), length c.
  - depth i: encode m_L and m_R recursively (length c*2^{i-1} each); for each child coordinate j
    draw an independent challenge T_j; the codeword is
        [ w_L[j] + T_j     * w_R[j] ]_j   ++   [ w_L[j] + (T_j + 1) * w_R[j] ]_j
    of length c*2^i.  This is the determinant-1 fold the proof normalizes to.

Field: prime field F_q for a prime q (so arithmetic is mod q).  Use small primes.

Cost: q^k messages times |T|^(c*(2^d - 1)) challenge vectors.  Keep c, d, q tiny.
"""

from __future__ import annotations

import argparse
import itertools
import math
import random
from fractions import Fraction


def encode(message: tuple[int, ...], depth: int, c: int, q: int,
           challenges: list[int], cursor: list[int]) -> list[int]:
    """Encode `message` (length 2^depth) into a codeword (length c*2^depth) mod q.

    `challenges` is a flat list of fold challenges; `cursor` is a one-element list used as a
    consuming index so each fold coordinate draws the next challenge in a fixed order.
    """
    if depth == 0:
        return [message[0] % q] * c
    half = 1 << (depth - 1)
    w_l = encode(message[:half], depth - 1, c, q, challenges, cursor)
    w_r = encode(message[half:], depth - 1, c, q, challenges, cursor)
    child_n = len(w_l)
    left = [0] * child_n
    right = [0] * child_n
    for j in range(child_n):
        t = challenges[cursor[0]]
        cursor[0] += 1
        left[j] = (w_l[j] + t * w_r[j]) % q
        right[j] = (w_l[j] + (t + 1) * w_r[j]) % q
    return left + right


def num_challenges(depth: int, c: int) -> int:
    # Fold level i (1..depth) has 2^{d-i} nodes, each drawing c*2^{i-1} challenges.
    # Total = sum_{i=1}^d 2^{d-i} * c*2^{i-1} = d * c * 2^{d-1}.
    if depth == 0:
        return 0
    return depth * c * (1 << (depth - 1))


def challenge_domain(q: int, nonzero: bool) -> list[int]:
    return list(range(1, q)) if nonzero else list(range(q))


def brute_force_moment(depth: int, c: int, q: int, nonzero_t: bool,
                       max_z: int | None, samples: int | None,
                       seed: int) -> tuple[list[Fraction], int, int, bool]:
    """Return (B[z] for z=0..N, N, num_T_vectors, exact?).

    If `samples` is None the full challenge space is enumerated (exact Fractions).  Otherwise
    `samples` random challenge vectors are drawn (Monte-Carlo unbiased estimate of E_T)."""
    k = 1 << depth
    n = c * k
    if max_z is None:
        max_z = n
    n_chal = num_challenges(depth, c)
    t_dom = challenge_domain(q, nonzero_t)

    # Precompute binomial coefficients binom(w, z) for w,z in 0..n.
    binom = [[0] * (n + 1) for _ in range(n + 1)]
    for w in range(n + 1):
        binom[w][0] = 1
        for z in range(1, w + 1):
            binom[w][z] = binom[w - 1][z - 1] + binom[w - 1][z]

    messages = [m for m in itertools.product(range(q), repeat=k) if any(m)]

    if samples is None:
        chal_iter = itertools.product(t_dom, repeat=n_chal)
        exact = True
    else:
        rng = random.Random(seed)
        chal_iter = ([rng.choice(t_dom) for _ in range(n_chal)] for _ in range(samples))
        exact = False

    total = [0] * (n + 1)
    num_t_vectors = 0
    for chal in chal_iter:
        num_t_vectors += 1
        chal = list(chal)
        for m in messages:
            cw = encode(m, depth, c, q, chal, [0])
            zeros = sum(1 for x in cw if x == 0)
            if zeros == 0:
                continue
            row = binom[zeros]
            top = min(zeros, max_z)
            for z in range(1, top + 1):
                total[z] += row[z]
    total[0] = len(messages) * num_t_vectors

    moments = [Fraction(total[z], num_t_vectors) for z in range(n + 1)]
    return moments, n, num_t_vectors, exact


def log2_fraction(x: Fraction) -> float:
    if x == 0:
        return float("-inf")
    return math.log2(x.numerator) - math.log2(x.denominator)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--depth", type=int, required=True)
    ap.add_argument("--expansion", type=int, default=2, help="c (base repetition rate)")
    ap.add_argument("--q", type=int, required=True, help="prime field size")
    ap.add_argument("--t-domain", choices=["nonzero", "all"], default="nonzero",
                    help="challenge domain: F^* (proof model) or F")
    ap.add_argument("--max-z", type=int, default=None)
    ap.add_argument("--samples", type=int, default=None,
                    help="Monte-Carlo: sample this many T-vectors instead of full enumeration")
    ap.add_argument("--seed", type=int, default=0)
    args = ap.parse_args()

    # sanity: q prime
    if args.q < 2 or any(args.q % d == 0 for d in range(2, int(args.q ** 0.5) + 1)):
        raise SystemExit(f"--q={args.q} must be prime")

    k = 1 << args.depth
    n = args.expansion * k
    moments, n, num_t, exact = brute_force_moment(
        args.depth, args.expansion, args.q, args.t_domain == "nonzero", args.max_z,
        args.samples, args.seed)

    q = args.q
    qlog = math.log2(q)
    print(f"# depth={args.depth} k={k} c={args.expansion} n={n} q={q} "
          f"t_domain={args.t_domain} num_T_vectors={num_t} "
          f"mode={'exact' if exact else 'montecarlo'}")
    print(f"# num_nonzero_messages={q**k - 1}  num_challenges={num_challenges(args.depth, args.expansion)}")
    print("z,excess,B_exact,log2_B,log_q_B,ideal_log2_binomNz_minus_qz,charge_per_zero_q")
    top = args.max_z if args.max_z is not None else n
    for z in range(0, top + 1):
        b = moments[z]
        l2 = log2_fraction(b)
        lq = l2 / qlog if l2 != float("-inf") else float("-inf")
        # idealized random-code first moment for a fixed nonzero msg: binom(N,z)*(q^k-1)*q^-z
        ideal_l2 = (math.lgamma(n + 1) - math.lgamma(z + 1) - math.lgamma(n - z + 1)) / math.log(2)
        ideal_l2 += math.log2(q**k - 1) - z * qlog
        # measured average q-suppression per zero relative to binom(N,z)*(q^k-1)
        denom_l2 = ((math.lgamma(n + 1) - math.lgamma(z + 1) - math.lgamma(n - z + 1)) / math.log(2)
                    + math.log2(q**k - 1))
        charge = (denom_l2 - l2) / z / qlog if (z > 0 and l2 != float("-inf")) else float("nan")
        excess = z - k
        l2s = "-inf" if l2 == float("-inf") else f"{l2:.6f}"
        lqs = "-inf" if lq == float("-inf") else f"{lq:.6f}"
        print(f"{z},{excess},{b},{l2s},{lqs},{ideal_l2:.6f},{charge:.4f}")


if __name__ == "__main__":
    main()
