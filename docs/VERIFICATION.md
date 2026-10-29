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

- The public experimental extractor still uses four-second windows and heuristic spectral reduction. A separate research implementation now reproduces the recovered paired-frame, tracking, tuning and range path on generated fixtures; the old classifier has not been retrained on it.
- Native four-beat scheduling and cue wire/post-response transforms have implementations and boundary tests, but the actual app-refined inputs and live persistence path remain unqualified.
- Decode/resample differences and codec-specific offsets remain unresolved.
- The current model normalizes banks independently without demonstrated service invariance; its historical folds are row-based, not recording-group-disjoint.
- Model shape, finite coefficients, explicit silent-input refusal, JSON batch errors, bank labels and broken-pipe behavior have been hardened and exercised.
- The public snapshot still provides no accepted end-to-end key, BPM, energy or cue parity result.

## Development checkpoint

The research branch uses hash-bound, network-denied native runs on generated fixed PCM. A coherent 17-case development replay currently has **11 exact combined key/energy/BPM cases and six explicit refusals**: five require smaller FFT profiles, and one reaches an indeterminate beat-tracker state. Refused cases remain failures, not exclusions.

| Exercised boundary | Result and limit |
|---|---|
| First 30-second native key fixture | All 352 raw words and accumulator matched; not a classifier or app-schedule test |
| First local energy fixture | Both segments’ start/end/volume and 183 features matched: 372 words; no remote energy label |
| First raw BPM fixture | All 118 three-word beat records, tempo and signature matched one native run |
| Native repeatability | Two identical inputs differed in 79 trailing beat-record words; positions and tempos agreed |
| Initial Linux arm64 replay | Built and ran offline; energy matched, two of 352 key words differed; numerical correction under verification |
| Research regression suite | 109 tests passed before subsequent app-refinement integration; tests alone are not native proof |

Foundation and Rust parsed some identical numeric JSON range endpoints into different f64 words. Original failures are preserved. A separately labeled replay uses the native emitted endpoint bits to isolate the feature algorithm; it does not claim parser parity.

New review findings also cover long energy-correlation windows and late-onset preanalysis offsets. A short-fixture match cannot qualify those branches. App-owned waveform and cue functions live in the executable rather than the loadable companion library, and still require an independent in-process reference capture. The remote key/energy/cue decision remains a separate opaque boundary.

## Publication checks versus parity

Source/credential scans, a build and a CLI smoke run can qualify a public research snapshot. They do not qualify a compatible product. Verification records must name the exact source/model/toolchain identities and measured scope, preserving failed results and unknowns.
