# Verification and claim boundaries

**Status: 30 October 2026.** This document reports aggregate measurements of the **unpublished reconstruction**, not the superseded experimental CLI in this repository. The reconstructed implementation and new models remain private pending licensing review.

Results belong to their measured, frozen snapshots. The final frozen snapshot (30 October 2026), which adds the later decoder-recovery, alignment-detection and service-mode changes, passed every reachable check on all three platforms. Re-running the real MP3/AAC and high-resolution file comparisons with that final build is still pending, because it needs the private music library, which was unreachable at the time. No new build, test or native-reference run is implied by this documentation refresh.

## Separate measurements

1. **Local signal/numerical fidelity:** decoding, sample-rate conversion, BPM/beats, scheduling, key and energy features, and cue pre-analysis compared against original native code at a declared boundary.
2. **Decision agreement:** learned offline predictions compared with the vendor's server-side decisions. A matching key label is not a matching complete decision.
3. **End-to-end application compatibility:** the same original file, settings and full application behavior. Headless execution of original analysis functions is not a measurement of every GUI, persistence or execution-layout behavior.
4. **Musical accuracy:** predictions compared with independently authored musical annotations. A vendor prediction is not ground truth.

Exact numerical comparisons use the measured words or bytes, not rounded values or similarity credit. MIREX-weighted key scores and tempo-ratio-tolerant accuracy are separate musical-evaluation metrics, not exact compatibility.

## Local-analysis results

The reconstructed paths cover FLAC/WAV/AIFF/ALAC/MP3/AAC decoding and tested sample-rate conversion. Coverage is finite and profile-specific, not a claim that every combination is supported.

| Boundary and population | Measured result | Scope and limits |
|---|---|---|
| Frozen offline three-platform matrix | All 63 suite × platform cells passed on macOS arm64, Linux arm64 and Linux x86_64 | Qualifies that offline source/model snapshot, not every subsequent change or universal app/service parity |
| Final frozen snapshot (all later changes) | 96 checks passed and none failed on macOS arm64, Linux arm64 and Linux x86_64: library tests 459/458/456; 73/73 app captures; 14,896 alignment cases per build in release and debug; 1,832 short-FFT cases; 40/40 real FLAC outputs byte-identical across the three platforms, and 40/40 app requests exact; 144/144 corruption cases. 15 real-audio re-checks are pending | The pending cells need the private library; earlier builds passed the same comparisons |
| Generated core and broad callers | 17/17 core cases and 49/49 broad runs matched per target | Compared key features, local energy and app-consumed BPM/beat output; development fixtures are not independent accuracy data |
| App-function captures | 73/73 captures; 1,990,443 comparisons per target, zero mismatches | Captured app-owned numerical boundaries, not complete GUI execution |
| Waveform and refinement | 13/13 waveform captures, 310,120 comparisons; 13/13 refinement captures, 340,195 comparisons per target | Separate component boundaries |
| Decoder reference batches | 131/131, 40/40, 208/208 and 120/120 exact streams per target; 110/110 comparable AAC class-table streams; 142/142 offset records | Batches include repeats and numerical profiles; do not sum them into a unique-file count. Expected refusals are tracked separately |
| Short FFT corpus, debug and release | 1,832/1,832 cases and 257,920/257,920 words matched per target in each build mode | Bounded corpus; no exhaustive claim for larger-size nonfinite arithmetic |
| Real 44.1 kHz FLAC | 40/40 files matched all compared local stages and the complete pre-decision boundary | Complete frozen CLI outputs were also byte-identical across the three targets; final key/energy/cue decisions remain approximate |
| Real MP3 and AAC | 60/60 MP3 and 40/40 AAC files matched every compared local stage and complete pre-decision output in 200 native captures | MP3 includes mono/stereo and 11.025/22.05/32/44.1/48 kHz; AAC is 44.1 kHz stereo only. No real AAC mono or resampled-AAC claim |
| Higher-rate FLAC, original frozen comparison | 33/36 comparable files matched the complete pre-decision output; 67/72 native captures matched | Selected sample: 40 files, including 48/88.2/96/192 kHz strata and 44.1 kHz controls. Four six-channel cases had no safely defined native comparison and remain explicit CLI refusals |
| Higher-rate FLAC, admitted decode/energy boundary | 34/34 admitted files matched PCM, lengths, delay and local energy | Does not make every beat grid or complete local output identical |
| Later FLAC recovery qualification | 144/144 generated corruption cases matched: 136 exact PCM/length/pre-decision outputs and 8 matching native failures; 36/36 zero-total-length-metadata cases matched | Separately measured later change; both real malformed cases from the earlier study also matched PCM and complete pre-decision output. This does not rewrite the original 33/36 result |
| Frozen offline library tests | 420/420, 419/419 and 417/417 passed on macOS arm64, Linux arm64 and Linux x86_64 respectively | Platform-gated counts differ; tests are not substitutes for native comparisons |

### Native non-uniqueness

The higher-rate comparison's three unequal files comprised two malformed files subsequently matched by the separate recovery work and one alignment-sensitive native result. On the latter input, identical decoded PCM in 32 fresh native worker processes produced two beat-analysis outcomes, observed 22 and 10 times. Controlled buffer-alignment experiments reproduced both outcomes.

The reconstruction retains an explicit aligned numerical profile; it must not be described as the unique file-defined app answer on such inputs. The observed frequencies are not a probability model of normal GUI use. A second real-file case had raw-beat sensitivity that disappeared after refinement. Undefined six-channel reads are not a deterministic reference to imitate.

## Sealed held-out decision agreement

The final offline key and energy/cue models were frozen before the two preregistered sealed evaluations. Both were sent and scored once, with **499/499 lossless** and **450/450 MP3/AAC** reference results, no exclusions and no refusals.

| Metric | Lossless held-out set | MP3/AAC held-out set |
|---|---:|---:|
| Exact key label | **425/499 (85.2%; 95% interval 81.8–88.0%)** | **388/450 (86.2%; 95% interval 82.7–89.1%)** |
| Energy row level | 83.9% | 84.5% |
| Identical complete energy segment list | 22.2% | 32.0% |
| Exact overall energy | 75.2% | 77.8% |
| Identical complete cue-time list | 26.1% | 25.1% |
| Identical complete cue list, including levels | 7.2% | 7.8% |
| Complete decision identity, including key confidence | **0/499 (0%)** | **0/450 (0%)** |
| Key-confidence mean absolute error | 0.095 | 0.105 |

Energy-row agreement is counted per row within each set; list, overall-energy and complete-decision agreement are counted per file. Confidence errors are absolute errors, not percentages. The lossy key-label breakdown is **297/355 MP3 (83.7%)** and **91/95 AAC (95.8%)**.

The confidence mismatch alone prevents complete identity in every held-out case. Conditional deterministic-rule matches do not remove mistakes in learned key labels, energy levels or cue selection. No-key, secondary-key, single-note and short-feature-vector behavior remain incompletely reproduced; see the [model card](MODEL_CARD.md).

## Optional service-mode evidence

The unpublished reconstruction can optionally obtain decisions from the vendor's own service using the user's license. This path is networked and separate from offline model agreement; it is not part of this public CLI.

- Offline replay matched **3,926/3,926** stored exchanges on each of the three platforms.
- An earlier mode's **3/3** approved live smoke cases matched the compared decisions and numeric results. This was not complete response-byte equality or a broad reliability study.
- A later mode-selection change made no additional live requests. Its unset-license and forced-offline regressions each preserved **75/75** outputs across 25 fixtures and three output modes.

No endpoint, request/response format, service sample or credential-handling procedure is provided here.

## Musical-annotation evaluation

The frozen reconstructed CLI was evaluated descriptively, without using GiantSteps for fitting, calibration or model selection. These are not results of the historical public classifier.

| Reference and population | System | Result |
|---|---|---|
| GiantSteps key, 604 excerpts | Frozen reconstructed CLI | **417/604 (69.0%; 95% interval 65.2–72.6%)**; MIREX weighted **0.757** |
| GiantSteps key, preregistered random 147-excerpt sample | MIK 11 service with reconstructed local pre-analysis | **100/147 (68.0%; 60.1–75.0%)**; MIREX weighted **0.739** |
| Same 147 excerpts | Same frozen CLI | **98/147 (66.7%; 58.7–73.8%)**; MIREX weighted **0.733** |
| GiantSteps key, historical full 604-excerpt dataset | Published Mixed-In-Key 7 | **67.2%**; MIREX weighted **0.7460** |
| GiantSteps+, 566 mappable excerpts | Frozen CLI, first-listed key | **79.3%**; MIREX weighted **0.847** |
| Same GiantSteps+ population | Frozen CLI, any listed key | **86.2%**; MIREX weighted **0.902** |
| GiantSteps tempo v2, 661 annotated excerpts | Frozen CLI's stored refined BPM | Accuracy1 **68.1%**; Accuracy2 **94.1%** |

Published MIK 7 citation: Knees et al., ISMIR 2015, [Table 5, page 6](https://www.cp.jku.at/research/papers/knees_etal_ismir_2015.pdf). The published 67.2% figure is not MIK 11.

The formal paired comparison uses the **same 147 excerpts**, not the full CLI set against a service subsample. MIK 11-minus-CLI accuracy difference: **+1.36 percentage points**, paired 95% interval **−2.72 to +5.44**, exact two-sided McNemar **p = 0.7539**. The difference is not statistically significant; superiority, inferiority and equivalence are not established. MIK 11 was not measured on the remaining 457 excerpts, and the service sample did not revalidate native-app local analysis on each excerpt.

Accuracy1 means tempo within 4% of the reference; Accuracy2 also admits reference multiples 2, 1/2, 3 and 1/3. These concern stored BPM, not display preferences. All 664 tempo files were processed; three zero/no-annotation v2 cases are outside the 661-file scoring population. Both key and tempo populations used the reconstructed decoder path, without a fallback population in the final run.

The data are excerpts, not a representative all-music or full-track sample. Annotation noise and artist dependence remain limits; recording overlap with the service-labelled training corpus was not independently ruled out. Historical MIK 7 versions and evaluation details are not assumed identical. GiantSteps has no energy/cue ground truth, so **no musical energy accuracy or cue-usefulness claim** follows from its output summaries.

## Acceptance and publication boundaries

The compatibility target remains zero mismatches on a frozen independent suite for every declared output and runtime. The offline learned decisions **do not pass a full service-parity gate**. Finite component matches do not prove universal equivalence or future vendor behavior.

Refusals, crashes, no-key/no-tempo outcomes, missing/extra list entries and failures remain visible. Do not narrow denominators, round away mismatches, treat a network result as an offline prediction, or use an evaluated holdout again for model selection.

Builds, source/credential checks and CLI smoke runs can qualify a public snapshot for publication; they cannot establish compatibility. No new source or model is being published by this refresh.

The final integration/evaluation model route was configured as `openai-codex/gpt-6.1-sol`. This records configuration provenance, not self-attested or independently proven runtime model identity.
