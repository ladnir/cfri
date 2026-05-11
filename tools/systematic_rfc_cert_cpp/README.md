# Systematic RFC Certificate Tool

Standalone C++ implementation of the support-stratified systematic RFC distance recurrence.

This tool intentionally does not link against or modify `libOTe`. It follows the same enumerator
style: log-domain combinatorial tables, explicit threshold rows, and a verifier path that checks
the emitted CSV independently from the generator logic.

Build:

```text
cmake -S tools/systematic_rfc_cert_cpp -B build/systematic_rfc_cert_cpp -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build/systematic_rfc_cert_cpp
```

Example generate:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe ^
  --depth 9 ^
  --parity-expansion 7 ^
  --field-bits 128 ^
  --security-bits 84 ^
  --compare ^
  --monotone-thresholds ^
  --certificate-path docs/rfc_distance_analysis/systematic_rfc_c8_depth9_global80_certificate.csv ^
  --full-thresholds-path docs/rfc_distance_analysis/systematic_rfc_c8_depth9_global80_thresholds.csv
```

Example verify:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe ^
  --verify docs/rfc_distance_analysis/systematic_rfc_c8_depth9_global80_thresholds.csv ^
  --field-bits 128 ^
  --security-bits 84
```

Ideal random-code first-moment ceiling:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe ^
  --ideal-first-moment ^
  --depth 11 ^
  --total-expansion 8 ^
  --field-bits 128 ^
  --security-bits 80
```

Tiny-field sampled RFC first-moment calibration:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe ^
  --sample-rfc-first-moment ^
  --prime 5 ^
  --depth 3 ^
  --total-expansion 8 ^
  --samples 100 ^
  --seed 11 ^
  --spectrum-path docs/rfc_distance_analysis/sample_first_moment_gf5_depth3_c8_cpp.csv ^
  --support-spectrum-path docs/rfc_distance_analysis/sample_first_moment_gf5_depth3_c8_by_support_cpp.csv
```

Tiny-field product first-moment calibration:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe ^
  --sample-rfc-product-first-moment ^
  --prime 5 ^
  --depth 3 ^
  --total-expansion 8 ^
  --samples 1000 ^
  --seed 29 ^
  --spectrum-path docs/rfc_distance_analysis/sample_product_first_moment_gf5_depth3_c8_cpp.csv ^
  --support-spectrum-path docs/rfc_distance_analysis/sample_product_first_moment_gf5_depth3_c8_by_support_cpp.csv
```

This samples the single-tree output-weight law for each message and then raises that law to the
requested expansion. It estimates the same first-moment target as full RFC sampling, but separates
the single-tree shape problem from the independent expansion product.

One-step child-pair first-moment transition:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe ^
  --sample-rfc-one-step ^
  --prime 5 ^
  --depth 3 ^
  --total-expansion 8 ^
  --samples 100 ^
  --seed 11 ^
  --spectrum-path docs/rfc_distance_analysis/sample_one_step_first_moment_gf5_depth3_c8_cpp.csv ^
  --category-path docs/rfc_distance_analysis/sample_one_step_categories_gf5_depth3_c8_cpp.csv
```
