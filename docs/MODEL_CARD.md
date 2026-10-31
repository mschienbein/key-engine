# Model status and limitations

**Status: 30 October 2026.** There are two distinct model generations. The historical experimental artifact is bundled in this repository; the newer decision models are part of the **unpublished reconstruction**, pending licensing review. None of the new models, weights or training data is released by this documentation update.

## Historical public artifact

`src/weights.json` is a locally trained numerical artifact, not vendor-supplied or recovered proprietary weights. It contains no audio, track titles, paths, account codes or service captures.

Its intended use is research-only offline key prediction from the public CLI's experimental features. It has been superseded and is not suitable for a claim of Mixed In Key compatibility, calibrated confidence or automatic replacement of trusted music metadata.

- Fixed 352-element input, arranged as four banks of 88, with one 64-unit tanh hidden layer and 25 output classes.
- Independent bank normalization has no demonstrated invariance to the reference decision. Training used an approximate feature path, not the subsequently qualified reconstruction.
- The artifact lacks a complete dataset/build/split lineage receipt. Historical row-based folds and exploratory agreement percentages are not accepted independent benchmarks.
- Training and Rust inference used different numerical precision; full decision-boundary numerical equivalence is unqualified.
- Runtime shape/finite-value checks, consistent first-tied-class selection and explicit refusal of empty, all-zero, negative or nonfinite features are safeguards, not a model-accuracy result.

The old CLI's softmax `confidence` is an **uncalibrated local score**, not a probability of musical correctness or the original application's confidence. The new reconstructed feature pipeline is not silently connected to this historical model. None of the results below describes this bundled artifact.

## Unpublished learned decision models

### Purpose

The newer models approximate the vendor's server-side decisions from reconstructed local audio-analysis features: musical key labels and confidence, energy levels and overall energy, and later cue selection/count and cue levels. They are **learned students of observed service decisions**, not the original vendor models and not ground-truth musical annotators.

The key model, L8mix, combines positional and standardized-spectrum ensemble members into one calibrated probability mixture. Energy-level prediction uses a reduced-feature nonlinear model; cue selection uses an ensemble of learned rankers. Separately measured deterministic timing and segment-merging rules surround these predictions, but are exact only conditional on their inputs. This is a high-level description, not a release of model coefficients or service access details.

### Data and fitting scope

- Final key fitting used **3,926 service-labelled examples**, including **428 former development examples**. Grouped cross-validation informed development. The former development score is therefore not an independent test result for the final refit.
- Energy/cue fitting used **3,498 training tracks** plus additional labelled feature/probe rows; energy-level fitting covered **64,098 rows**. That model retained **428 development tracks** outside fitting. Row counts and track counts are different units.
- Final acceptance measurements used separately sealed **499-file lossless** and **450-file MP3/AAC** sets, with the same frozen models and preregistered scoring. Both sets were evaluated once, with no exclusions or refusals.
- Audio, recording identities, per-file labels/predictions, captures and training artifacts are not distributed. The model descriptions and aggregate results do not constitute a publicly reproducible training package.

### Sealed held-out agreement

These metrics compare with the vendor service, not musical annotations:

| Metric | Lossless, 499 files | MP3/AAC, 450 files |
|---|---:|---:|
| Key label | **425/499 (85.2%; 95% interval 81.8–88.0%)** | **388/450 (86.2%; 95% interval 82.7–89.1%)** |
| Energy row level, per row | 83.9% | 84.5% |
| Complete energy segment list, per file | 22.2% | 32.0% |
| Overall energy, per file | 75.2% | 77.8% |
| Complete cue-time list, per file | 26.1% | 25.1% |
| Complete cue list, including levels, per file | 7.2% | 7.8% |
| Complete decisions, including key confidence | **0/499 (0%)** | **0/450 (0%)** |
| Key-confidence mean absolute error | 0.095 | 0.105 |

Energy-row percentages count rows within the named file set; other agreement percentages count files. Confidence errors are absolute errors, not percentages. Key-label agreement within the lossy set was **297/355 MP3 (83.7%)** and **91/95 AAC (95.8%)**.

### Limits

- **Not service parity.** Confidence was never exactly reproduced on either sealed set, so no file had identical complete decisions. Correct labels or conditional deterministic rules must not be relabelled complete compatibility.
- **Unsupported decision behavior.** The no-key rule is unrecovered; secondary-key and single-note behavior are not reproduced. Short key vectors are padded for prediction, but service agreement on that domain is unqualified. Invalid or missing feature/timing inputs have explicit refusals.
- **Confidence is not musical certainty.** Probability calibration for the learned service-label task neither reproduces vendor confidence nor establishes the probability of a musically correct key.
- **Features and final decisions are separate.** Exact local decode/BPM/key/energy feature matches do not make learned key labels, energy levels or cue choices exact. Native numerical-profile ambiguity and input refusals remain separate qualifications.
- **Finite and version-bound evidence.** Held-out agreement does not establish generalization to every genre, recording, encoding or future vendor-service version. Evaluated sealed sets must not be reused as untouched holdouts after tuning.
- **Musical accuracy is a different measurement.** The frozen reconstructed CLI scored **69.0% on 604 GiantSteps key excerpts**. On a separate preregistered 147-excerpt service sample, MIK 11 scored **68.0%** and the same CLI **66.7%**, with no significant paired difference (McNemar **p = 0.7539**). This is not equivalence or end-to-end app parity.
- **Benchmark limits.** GiantSteps was not used for fitting, calibration or model selection, but overlap between excerpt recordings and the service-labelled training corpus was not independently ruled out. Annotation uncertainty, excerpt/genre selection and artist dependence limit interpretation. There is no GiantSteps energy/cue musical-quality ground truth.

See [verification](VERIFICATION.md) for exact comparison scopes and the historical MIK 7 citation, and the [roadmap](ROADMAP.md) for remaining work. The optional vendor-service mode belongs to the unpublished implementation; it is not a model improvement or a public feature.

The final integration/evaluation route was configured as `openai-codex/gpt-6.1-sol`. This is configuration provenance, not self-attested or independently verified runtime model identity.
