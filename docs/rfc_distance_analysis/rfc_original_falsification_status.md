# RFC Original Falsification Status

This note records the active attempt to disprove or stress the original non-systematic near-MDS
target.

## Target

Default production target:

```text
c = 8
k = 2048
N = 16384
q = 2^128
lambda = 80
candidate excess e = 71
target zero count z = k + e = 2119
```

The current conditional certificate says this should be safe with idealized slack about `39.68`
bits. The falsification lane asks whether explicit or modeled bad families force a larger excess.

## Script

The report driver is:

```text
scripts/rfc_distance_analysis/rfc_original_falsification_report.py
```

It checks:

```text
1. exact one-copy uncertainty extremizers;
2. matched-core plus extra one-copy near-extremizer models;
3. all-paired compression scaling.
```

It is not a certificate. It is deliberately adversarial: if a modeled family has positive or
near-positive log expectation, we treat that as a warning sign.

## Exact One-Copy Extremizers

A one-copy exact extremizer with live support `m` has one-copy output support `k/m`, hence
deterministic zeros:

```text
k - k/m.
```

The best deterministic case is `m=k`, giving:

```text
deterministic zeros = k - 1 = 2047.
```

This reaches the MDS zero ceiling but does not exceed it. To violate the `e=71` target, it still
needs:

```text
72
```

extra zeros across the other `7k` coordinates. The modeled expected count for this exact family is:

```text
log2 ~= -8552.90.
```

So exact one-copy extremizers are not a counterexample to `e=71`.

## Wide Near-Extremizer Stress Model

The more adversarial scan allows one-copy support:

```text
k/m + h
```

and includes the existing stride-dimension stress factor:

```text
q^floor(h / (k/m)).
```

Command:

```text
python scripts/rfc_distance_analysis/rfc_original_falsification_report.py \
  --depth 11 \
  --expansion 8 \
  --q-log2 128 \
  --target-excess 71 \
  --security-bits 80 \
  --max-near-extra 2047 \
  --stride-dimension-bound \
  --output-csv docs/rfc_distance_analysis/rfc_original_falsification_c8_k2048_q128_e71_wide.csv \
  --output-json docs/rfc_distance_analysis/rfc_original_falsification_c8_k2048_q128_e71_wide.json
```

Best modeled row:

```text
live_rows = 2048
core_support = 1
extra_support = 1783
modeled log2 expected = -111.03256536
slack to 80-bit target = 31.03256536 bits
```

This is close enough to matter. It is not an explicit counterexample, but it says the `e=71`
target is tight against a broad one-copy-support first-moment family.

Sweeping target excess:

```text
e=64: log2= 765.69615016
e=65: log2= 640.45187547
e=66: log2= 515.20670521
e=67: log2= 389.96063978
e=68: log2= 264.71367960
e=69: log2= 139.46582507
e=70: log2=  14.21707662
e=71: log2=-111.03256536
e=72: log2=-236.28232292
```

Saved sweep artifact:

```text
docs/rfc_distance_analysis/rfc_original_falsification_c8_k2048_q128_e64_76_sweep.csv
```

Interpretation:

```text
e=70 is unsafe in this stress model.
e=71 is the first 80-bit-safe value in this stress model.
```

This supports the idea that the desired excess is close to the true first-moment crossing. It also
means we should not expect much spare slack from the original RFC at this parameter set.

Without the stride-dimension factor, the wide scan is harmless:

```text
best log2 expected = -8552.90103338.
```

So the warning is specifically the large-dimensional preimage effect for broad one-copy supports,
not the tiny exact matched family.

## Current Bias Check

This falsification pass does not disprove the near-MDS target. It does show that the target is
tight:

```text
best explicit deterministic family: reaches MDS ceiling only
best adversarial modeled family: crosses between e=70 and e=71
```

That is exactly the kind of evidence we should keep in view while proving the upper bound. A proof
that claims much better than `e=71` for `c=8,k=2048,q=2^128` is probably over-optimistic.

## Next Falsification Step

The next falsification target should be multi-copy correlation:

```text
Can the same message have unusually sparse support in two or more independent RFC copies more often
than the product/first-moment model predicts?
```

Existing helpers:

```text
scripts/rfc_distance_analysis/rfc_extremizer_second_copy_check.py
scripts/rfc_distance_analysis/rfc_near_pair_kernel_dim.py
scripts/rfc_distance_analysis/rfc_global_rank_cancellation_probe.py
```

If multi-copy correlations exceed the modeled tail, the `e=71` statement may be false or may need
additional slack.
