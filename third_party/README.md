# Third-Party Reference Snapshots

This directory holds frozen upstream snapshots for provenance, audit, and manual comparison.

These projects are not part of the top-level Cargo workspace. Do not import them from `crates/cfri`
and do not make normal tests depend on compiling them. The owned implementation lives under
`crates/cfri`; reference snapshots stay here so we can inspect original behavior, regenerate
compatibility fixtures, or run upstream tests manually when that is useful.

When changing owned code, prefer small fixtures or focused compatibility tests in `crates/cfri/tests`
over wiring these snapshots into the core build.
