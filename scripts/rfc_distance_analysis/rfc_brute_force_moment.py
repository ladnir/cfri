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
           challenges: list[int], cursor: list[int] | None = None) -> list[int]:
    """Encode `message` (length 2^depth) into a codeword (length c*2^depth) mod q.

    FAITHFUL to crates/cfri/src/backend/basefold.rs::evaluate_over_foldable_domain: the fold
    challenge at level i is `level[j - half_chunk]`, indexed by the position WITHIN a chunk, so the
    SAME challenge vector `level` is reused across all sibling chunks at that level.  Challenges are
    therefore shared across subtrees (total c*(2^depth - 1)), NOT independent per subtree.

    `challenges` is the flat challenge vector; it is sliced into levels: level i (i=0..depth-1) is
    `challenges[off : off + c*2^i]`.  The `cursor` arg is ignored (kept for signature compatibility).
    The base-code repetition layout and per-level det-1 fold match the encoder exactly; the final
    `reverse_index_bits` only permutes coordinates and does not change the zero COUNT, so it is
    omitted (this script only ever counts zeros).
    """
    k = 1 << depth
    cl = c * k
    arr = [0] * cl
    for i in range(k):
        v = message[i] % q
        for j in range(c):
            arr[i * c + j] = v
    off = 0
    chunk_size = c
    for i in range(depth):
        level_len = c * (1 << i)           # = half_chunk for this level
        level = challenges[off:off + level_len]
        off += level_len
        chunk_size <<= 1
        half = chunk_size >> 1
        for base in range(0, cl, chunk_size):
            for jj in range(half):
                t = level[jj]
                u = arr[base + jj]
                w = arr[base + half + jj]
                arr[base + jj] = (u + t * w) % q
                arr[base + half + jj] = (u + (t + 1) * w) % q
    return arr


def num_challenges(depth: int, c: int) -> int:
    # Shared per-level challenges (faithful to the encoder): level i has c*2^i challenges,
    # reused across all sibling chunks.  Total = sum_{i=0}^{d-1} c*2^i = c*(2^d - 1).
    return c * ((1 << depth) - 1)


def challenge_domain(q: int, nonzero: bool) -> list[int]:
    return list(range(1, q)) if nonzero else list(range(q))


def brute_force_moment(depth: int, c: int, q: int, nonzero_t: bool,
                       max_z: int | None, samples: int | None,
                       seed: int, replica: int) -> tuple[list[Fraction], int, int, bool]:
    """Return (B_d(replica, z) for z=0..N, N, num_T_vectors, exact?).

    B_d(R, z) = E_T[ sum over nonzero ordered R-tuples of messages of binom(common_zeros, z) ]
    where common_zeros = #coords where all R codewords vanish.  R=1 is the distance certificate.

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

    # All messages (including zero, at index 0 after sorting) for R-tuple enumeration.
    all_messages = list(itertools.product(range(q), repeat=k))
    zero_idx = all_messages.index((0,) * k)
    num_msgs = len(all_messages)

    if samples is None:
        chal_iter = itertools.product(t_dom, repeat=n_chal)
        exact = True
    else:
        rng = random.Random(seed)
        chal_iter = ([rng.choice(t_dom) for _ in range(n_chal)] for _ in range(samples))
        exact = False

    # hist[w] = total count (over T and nonzero tuples) of tuples with EXACTLY w common zeros.
    # This is the exact-support histogram; A_d(R,w) = hist[w]/num_T, and the binom-weighted moment
    # is recovered EXACTLY as B_d(R,z) = sum_w A_d(R,w)*binom(w,z).  Tracking exact w (rather than
    # the binom-weighted B directly) is what the inclusion-exclusion recurrence needs to avoid the
    # compounding double-count.
    hist = [0] * (n + 1)
    num_t_vectors = 0
    for chal in chal_iter:
        num_t_vectors += 1
        chal = list(chal)
        # zero-set bitmask of each message's codeword under this T
        masks = []
        for m in all_messages:
            cw = encode(m, depth, c, q, chal, [0])
            mask = 0
            for i, x in enumerate(cw):
                if x == 0:
                    mask |= (1 << i)
            masks.append(mask)
        if replica == 1:
            for idx in range(num_msgs):
                if idx == zero_idx:
                    continue
                hist[bin(masks[idx]).count("1")] += 1
        else:
            full = (1 << n) - 1
            for tup in itertools.product(range(num_msgs), repeat=replica):
                if all(i == zero_idx for i in tup):
                    continue
                common = full
                for i in tup:
                    common &= masks[i]
                hist[bin(common).count("1")] += 1

    # B_d(R,z) = sum_{w>=z} hist[w]*binom(w,z) / num_T  (binom-weighted moment, exact identity).
    total = [0] * (n + 1)
    for w in range(n + 1):
        if hist[w] == 0:
            continue
        for z in range(0, min(w, max_z) + 1):
            total[z] += hist[w] * binom[w][z]

    moments = [Fraction(total[z], num_t_vectors) for z in range(n + 1)]
    hist_moments = [Fraction(hist[w], num_t_vectors) for w in range(n + 1)]
    return moments, hist_moments, n, num_t_vectors, exact


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
    ap.add_argument("--replica", type=int, default=1, help="R: ordered R-tuple replica count")
    ap.add_argument("--histogram", action="store_true",
                    help="also print the exact-support histogram A_d(R,w)=E[#tuples with exactly w common zeros]")
    args = ap.parse_args()

    # sanity: q prime
    if args.q < 2 or any(args.q % d == 0 for d in range(2, int(args.q ** 0.5) + 1)):
        raise SystemExit(f"--q={args.q} must be prime")

    k = 1 << args.depth
    n = args.expansion * k
    R = args.replica
    moments, hist_moments, n, num_t, exact = brute_force_moment(
        args.depth, args.expansion, args.q, args.t_domain == "nonzero", args.max_z,
        args.samples, args.seed, R)

    q = args.q
    qlog = math.log2(q)
    num_tuples = q ** (R * k) - 1
    print(f"# depth={args.depth} k={k} c={args.expansion} n={n} q={q} replica={R} "
          f"t_domain={args.t_domain} num_T_vectors={num_t} "
          f"mode={'exact' if exact else 'montecarlo'}")
    print(f"# num_nonzero_tuples={num_tuples}  num_challenges={num_challenges(args.depth, args.expansion)}")
    print("z,excess,B_exact,log2_B,log_q_B,ideal_log2,charge_per_zero_q")
    top = args.max_z if args.max_z is not None else n
    for z in range(0, top + 1):
        b = moments[z]
        l2 = log2_fraction(b)
        lq = l2 / qlog if l2 != float("-inf") else float("-inf")
        binom_l2 = (math.lgamma(n + 1) - math.lgamma(z + 1) - math.lgamma(n - z + 1)) / math.log(2)
        # idealized: a fixed nonzero R-tuple is common-zero on z fixed coords with prob ~ q^-z
        # (one scalar constraint per coord), so B ~ binom(N,z)*(#tuples)*q^-z.
        ideal_l2 = binom_l2 + math.log2(num_tuples) - z * qlog
        denom_l2 = binom_l2 + math.log2(num_tuples)
        charge = (denom_l2 - l2) / z / qlog if (z > 0 and l2 != float("-inf")) else float("nan")
        excess = z - k
        l2s = "-inf" if l2 == float("-inf") else f"{l2:.6f}"
        lqs = "-inf" if lq == float("-inf") else f"{lq:.6f}"
        print(f"{z},{excess},{b},{l2s},{lqs},{ideal_l2:.6f},{charge:.4f}")

    if args.histogram:
        print("# exact-support histogram A_d(R,w) = E_T[#nonzero tuples with EXACTLY w common zeros]")
        print("w,A_exact,log2_A")
        for w in range(0, top + 1):
            a = hist_moments[w]
            a2 = log2_fraction(a)
            a2s = "-inf" if a2 == float("-inf") else f"{a2:.6f}"
            print(f"{w},{a},{a2s}")


if __name__ == "__main__":
    main()
