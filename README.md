# Key Engine

An experimental, offline Rust CLI for musical analysis. The project is working toward a compatible local implementation of three Mixed In Key analysis features:

1. **Musical key detection**
2. **BPM and beat detection**
3. **Energy levels and cue points**

**This is not a 1:1 replacement.** The public CLI still runs the experimental key detector. Separate native-style key, BPM and local energy implementations now run in the research tree, but full app refinement, server decisions and independent accuracy remain unqualified. Do not use this snapshot to overwrite trusted music-library metadata.

This project is independent and is not affiliated with, endorsed by, or supplied by Mixed In Key or zplane. No proprietary application binaries, raw decompilation, private music or credentials are included.

## Build and run

A Rust toolchain supporting edition 2024 is required.

```sh
cargo build --release --locked
./target/release/key analyze track.flac --json
./target/release/key features track.wav --json
```

Analysis does not contact a remote service, require an account code, or write tags. Decoding uses Symphonia. The package enables WAV/AIFF PCM, FLAC, MP3 and AAC/M4A support, but native-decoder equivalence across those formats is **not** established. The current linear resampler is not a faithful reconstruction of the reference converter.

The emitted `confidence` is an uncalibrated local model score, not a probability of musical correctness. Feature tables use bank-relative pitch labels. Model shape and coefficients are validated; empty/silent features are refused rather than assigned a confident key. JSON batch output retains a result or explicit error for each input, and a failed input produces a nonzero exit status.

## Status

| Component | Current status |
|---|---|
| Offline key CLI | Experimental baseline with strict model/input errors; no accepted parity score |
| Native key feature pipeline | Separate streaming reconstruction exercised against exact native feature words; not connected to the old classifier |
| BPM and beat detection | Complete version-0 research pipeline exercised; short-FFT and indeterminate-state cases remain open |
| Energy and cue points | Local energy features exercised; actual app waveform/cue refiners are awaiting native qualification; final service decisions remain opaque |
| macOS arm64 | Research tests and fixed-PCM native comparisons exercised; full acceptance not passed |
| Linux arm64 | Offline research build and runtime exercised; numerical mismatches remain tracked |
| Linux x86_64 | Planned; not yet runtime-qualified |

Earlier exploratory agreement percentages are not release results. Their data identities and evaluation splits were insufficient for a compatibility claim. See [verification boundaries](docs/VERIFICATION.md) and the [model card](docs/MODEL_CARD.md).

The [verification record](docs/VERIFICATION.md#development-checkpoint) distinguishes exact component matches from refused cases and unresolved application behavior. These measurements are development evidence, not a 100% product claim.

## Roadmap and acceptance

The [roadmap](docs/ROADMAP.md) separates native signal reconstruction, authoritative reference data, model qualification and portable integration. The release target is **zero mismatches on a frozen independent acceptance suite** for every declared output and platform. A high average score, rounded result, excluded hard example or network fallback does not pass that gate.

Finite 100% observed agreement is not proof of equivalence on every possible file or a future vendor service version. Independently improving musical accuracy is a different objective and must not silently change compatibility behavior.

## Repository boundary

This repository contains curated implementation source, a provisional locally trained model and public status documents. Raw reference artifacts, licensed application images, account-bearing requests and corpus data remain outside the public repository. Only reviewed snapshots are exported here; no private repository history is merged.

## License

Project-authored source is provided under the [MIT License](LICENSE). Dependencies retain their own licenses; see [NOTICE](NOTICE). Experimental status and the lack of qualified compatibility are not changed by the license.
