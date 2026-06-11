# RFC Flag Recurrence Checkpoint

Scope: original non-systematic RFC only.

This note records the first executable checkpoint for the proposed two-layer flag recurrence.  It is
not a distance certificate.  The purpose is to test whether the missing state

```text
pi(K) <= pi(W)
pi(W) zero on P union (S \ A)
pi(K) zero on P union S
```

has visible numerical force compared with the scalar span recurrence.

## Script

```text
scripts/rfc_distance_analysis/rfc_flag_span_moment.py
```

The script computes a small-depth diagnostic.  It keeps the parent transition aware of the visible
singleton quotient dimension `tau <= 2` and then upper-bounds the child flag count using existing
one-layer child counts:

```text
F_child((r1,z1),(r0,z0))
  <= min(
       B_child(r1,z1) + flag choices inside r1,
       B_child(r0,z0) + outer lifts above r0,
       B_child(r1,z1) + B_child(r0,z0)
     ).
```

This is safe as a coarse upper-bound diagnostic, but it is not the final recurrence.  The final
proof still needs an actual flag count/lift lemma.

## Depth-4 Check

Command:

```text
python scripts/rfc_distance_analysis/rfc_flag_span_moment.py --depth 4 --expansion 8 --q-log2 128 --security-bits 80 --print-window 4 --trace-z 23
```

Result:

```text
crossing_z=65
crossing_excess=49
```

For comparison, the scalar span diagnostic with the same endpoint-tau2 singleton charge gives:

```text
python scripts/rfc_distance_analysis/rfc_subspace_span_moment.py --depth 4 --expansion 8 --q-log2 128 --security-bits 80 --singleton-charge endpoint-tau2 --print-window 4 --trace-z 23

crossing_z=121
crossing_excess=105
```

So the flag-state bookkeeping is not cosmetic in this toy model.  It removes the scalar trace where
large invisible kernels pass through singleton-heavy layers without receiving recursive zero
charges.

## Crossing Trace

At the flag checkpoint crossing:

```text
python scripts/rfc_distance_analysis/rfc_flag_span_moment.py --depth 4 --expansion 8 --q-log2 128 --security-bits 80 --print-window 2 --trace-z 65
```

Dominant trace:

```text
level span z  p   s   a   tau outer inner outer_z inner_z
4     1    65 31  3   0   0   2     2     34      34
3     2    34 17  0   0   0   1     1     17      17
2     1    17 1   15  15  1   1     0     1       16
1     1    1  0   1   1   1   1     0     0       1
```

Interpretation: once the parent line enters a singleton-heavy layer, the kernel is zero-dimensional
in this trace, and the visible quotient pays direct singleton charges.  The previous scalar
diagnostic instead allowed a high-zero singleton layer to collapse into a weak child request.

## What This Does And Does Not Say

This checkpoint supports the upper-bound agent's claim that the recurrence must count flags.  It
does not yet answer Volta's seven-copy broad-family warning at production depth.  In particular:

```text
1. The child flag count is still a coarse min of one-layer bounds.
2. The flag-lift exponent is an overcounting placeholder.
3. The diagnostic is intentionally size-guarded and was only run at depth 4.
4. Production e=71 remains fragile until multi-copy broad-family intersections are folded into the
   proof or enough exact-support savings are proved.
```

Near-term conclusion: the right next proof object is the actual flag-lift/count lemma.  The right
next falsification object is whether the seven-copy broad family survives this flag state or loses
the missing `3.66` bits.
