# Key Engine

**The code in this repository is an early experimental key-detection CLI. It has been superseded.** The reconstruction described below is developed privately and **is not published, pending licensing review**. Building this checkout does not provide the reconstructed analysis pipeline, its new decision models, or its optional service mode.

This is a documentation-only status update as of **30 October 2026**, not a new implementation release. Do not use the historical CLI to overwrite trusted music metadata.

Key Engine is independent and is not affiliated with, endorsed by, or supplied by Mixed In Key or zplane. No proprietary application binaries, raw decompilation, private audio, service captures or credentials are distributed here.

## Current reconstruction status

### Local analysis: exact on the measured boundaries

The unpublished reconstruction reproduces the app's local analysis exactly on the measured test sets and numerical profiles: decoding for **FLAC, WAV, AIFF, ALAC, MP3 and AAC**, tested sample-rate conversion, BPM and beats, key features, local energy features and cue pre-analysis. **Exact key features are not exact final key decisions**; those decisions are a separate server-side boundary.

| Measured scope | Result |
|---|---|
| Frozen offline suite on macOS arm64, Linux arm64 and Linux x86_64 | 17/17 core cases, 49/49 broad runs and 73/73 app-function captures matched on each target |
| Decoder reference batches on each target | 131/131, 40/40, 208/208 and 120/120 streams matched; these are separate batches, including repeated/profile tests, not a unique-file total |
| Complete local analysis on real files | 40/40 44.1 kHz FLAC files matched the native reference; 60/60 MP3 and 40/40 AAC files matched at every compared stage and the complete pre-decision boundary, across 200 native captures |
| Cross-platform real-file comparison | The frozen reconstruction's complete outputs and native-reference pre-decision comparison matched on 40/40 FLAC files per target |

The real MP3 set includes 11.025, 22.05, 32, 44.1 and 48 kHz inputs; the real AAC set is limited to 44.1 kHz stereo. A separate higher-rate FLAC study includes 48, 88.2, 96 and 192 kHz material, but has documented exceptions, including non-unique native results and unsupported six-channel inputs. This is not a claim for every format/rate/channel combination or every app execution. See [verification and limits](docs/VERIFICATION.md).

### Server-side decisions: learned approximations, not parity

Offline key labels, confidence, energy levels and cue selection use learned approximations of the vendor's server-side decisions, together with separately measured deterministic rules. The models are not the vendor's original models and are not released here.

Two preregistered, sealed held-out sets were evaluated with the same frozen models: **499 lossless files** and **450 MP3/AAC files**. All received reference results; none was excluded or refused.

| Agreement with the vendor service | Lossless set | MP3/AAC set |
|---|---:|---:|
| Key label | **425/499 (85.2%)** | **388/450 (86.2%)** |
| Energy row level, per row | 83.9% | 84.5% |
| Complete energy segment list, per file | 22.2% | 32.0% |
| Overall energy, per file | 75.2% | 77.8% |
| Complete cue-time list, per file | 26.1% | 25.1% |
| Complete cue list, including levels, per file | 7.2% | 7.8% |
| Complete decisions, including key confidence | **0/499** | **0/450** |
| Key-confidence mean absolute error | 0.095 | 0.105 |

Key confidence is not reproduced exactly. Agreement with a service is also not musical accuracy. The [model card](docs/MODEL_CARD.md) explains fitting scope, held-out results and unsupported behavior.

### Optional vendor-service mode

The unpublished reconstruction also has an optional mode that can query the vendor's own service using the user's license rather than relying solely on the offline learned decisions. This is a networked path, subject to the vendor's terms and the user's authorization, not an offline-parity claim. It is **not available in this public CLI**; endpoint, protocol and credential-handling instructions are not published here.

### Musical accuracy: GiantSteps

These figures score key predictions against musical annotations, not against the vendor's labels:

| System and population | Exact key accuracy |
|---|---:|
| Unpublished frozen CLI, full GiantSteps key set | **417/604 (69.0%)** |
| MIK 11 service, preregistered random sample of that set | **100/147 (68.0%)** |
| Same frozen CLI, same 147 excerpts | **98/147 (66.7%)** |
| Published Mixed-In-Key 7 result, historical full set | **67.2%** |

The paired MIK 11-minus-CLI difference is +1.36 percentage points (95% interval −2.72 to +5.44; McNemar **p = 0.7539**): **not significantly different**, but neither superiority nor equivalence is established. The MIK 11 measurement used reconstructed local pre-analysis and the vendor service, not the installed app end to end; the remaining 457 excerpts were not measured with MIK 11.

The historical 67.2% figure is **MIK 7, not MIK 11**: Knees et al., ISMIR 2015, [Table 5, page 6](https://www.cp.jku.at/research/papers/knees_etal_ismir_2015.pdf). Different versions and evaluation details preclude a direct superiority claim. The [verification record](docs/VERIFICATION.md#musical-annotation-evaluation) includes uncertainty and benchmark limits.

## Build and run the historical public CLI

A Rust toolchain supporting edition 2024 is required.

```sh
cargo build --release --locked
./target/release/key analyze example.flac --json
./target/release/key features example.wav --json
```

This older CLI is offline, requires no account code and does not write tags. It uses Symphonia, with WAV/AIFF PCM, FLAC, MP3 and AAC/M4A support enabled. Its decoder and linear resampler are **not** the reconstructed paths measured above, and have no established native equivalence.

Its bundled historical model uses the experimental feature extractor; its `confidence` is an uncalibrated local score, not a probability of musical correctness. Model/input validation, explicit silent-input refusal and per-input JSON errors remain useful safeguards, not compatibility evidence. A failed input produces a nonzero exit status.

## Documentation and publication boundary

- [Verification](docs/VERIFICATION.md): measured results, populations and claim boundaries.
- [Model card](docs/MODEL_CARD.md): the historical bundled artifact versus the unpublished learned decision models.
- [Roadmap](docs/ROADMAP.md): remaining technical, evaluation and licensing work.

Only the early implementation, its historical locally trained model and public documentation are present here. No new reconstruction code, model weights, datasets, audio, native captures or vendor-derived implementation material accompany this refresh.

Project-authored public source remains under the [MIT License](LICENSE). Dependencies retain their own licenses; see [NOTICE](NOTICE). That license does not grant rights in external reference products or imply that the unpublished reconstruction has been cleared for distribution.
