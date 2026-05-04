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
  --certificate-path docs/systematic_rfc_c8_depth9_global80_certificate.csv ^
  --full-thresholds-path docs/systematic_rfc_c8_depth9_global80_thresholds.csv
```

Example verify:

```text
build/systematic_rfc_cert_cpp/systematic_rfc_cert.exe ^
  --verify docs/systematic_rfc_c8_depth9_global80_thresholds.csv ^
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
