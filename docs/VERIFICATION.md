# Verification and claim boundaries

This project has not passed its compatibility gates. A successful build or a plausible key prediction is not proof of reconstruction.

## Separate measurements

1. **Signal/numerical fidelity:** identical admitted PCM, channel handling, numerical checkpoints, scheduling and serialized feature values against a pinned native reference.
2. **Decision fidelity:** identical features passed to the reference decision and local implementation, evaluated on disjoint recording groups.
3. **End-to-end compatibility:** original file through the offline CLI, compared with exactly identified app results for the same bytes and configuration.
4. **Musical accuracy:** independently annotated key/tempo and adjudicated energy/cue usefulness. Another application's tag is a comparison prediction, not ground truth.

Cosine similarity, adjacent Camelot positions and half/double tempo credit are diagnostics. They do not count as exact matches. Vendor confidence, local model scores and annotation confidence are different quantities.

## Acceptance target

Every declared input and outcome is counted. A candidate passes only with zero mismatches on the sealed holdout and regression/edge suites on every required runtime target. Track copies, alternate encodings, excerpts and augmentations stay in the same split. A test set used to tune a model becomes development data; a fresh independent set is then required.

Refusals, crashes, no-key/no-tempo and empty versus missing results are reported separately. No silent service fallback, per-track lookup, denominator changes or post-hoc rounding is permitted. Full-response parity also includes secondary keys, confidence and segment/cue fields if advertised; a key-label match alone is not that claim.

The initial target matrix is macOS arm64, Linux aarch64 and Linux x86_64. Each requires an actual offline runtime exercise with the vendor frameworks absent, not cross-compilation alone.

## Known current gaps

- The experimental extractor uses four-second windows. Native scheduling is expressed in four-beat coordinates over refined beat data.
- Spectral peak heuristics, single-frame FFT use, tuning and max pooling do not reproduce the recovered native temporal processing.
- Decode/resample differences and codec-specific offsets are unresolved.
- The current model normalizes banks independently without a demonstrated service invariance; its historical folds are row-based, not proven recording-group-disjoint.
- Silent/short/error cases and model-shape validation need hardening.
- The public snapshot provides no accepted key, BPM, energy or cue parity result.

## Publication checks versus parity

Source/credential scans, a build and a CLI smoke run can qualify a public research snapshot. They do not qualify a compatible product. Verification records must name the exact source/model/toolchain identities and measured scope, preserving failed results and unknowns.
