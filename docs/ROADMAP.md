# Reconstruction roadmap

Goal: an offline Rust implementation of key, BPM/beat, and energy/cue analysis on macOS and Linux. No milestone is accepted merely because code exists.

## 1. Trustworthy reference boundary

- Freeze source, reference version/settings, input identities and result receipts.
- Reproduce actual app scheduling and request data, including dynamic feature-bank length and error cases.
- Keep account handling and raw references outside the public distribution.
- Validate repeated native results before claiming a deterministic target.

## 2. Signal, timing and native key features

- Reconstruct decoder offsets, channel summation and resampling.
- Reproduce paired-frame FFT arithmetic, temporal partial association, tuning and weighted time-range reduction.
- Implement the app's four-beat coordinate schedule with exact native rounding and boundary behavior.
- Require exact raw checkpoints on fixed PCM and captured schedules; do not replace missing math with a tuned heuristic.

## 3. BPM and beats

- Implement the actual selected native tempo algorithm and its onset, histogram and beat-tracking dependencies.
- Preserve raw tempo, waveform-refined tempo, beat-grid, displayed precision and stored values as separate observables.
- Cover pickups, silence, short files, tempo changes, half/double decisions and failed estimates.

## 4. Energy and cues

- Reconstruct local energy features, tempo-derived segment boundaries and cue-preanalysis scores.
- Preserve response types, cue ordering/times and post-response beat/segment adjustments.
- Establish an equivalent local decision implementation or independently qualify a learned replacement where the original decision is server-side.
- Never substitute generic loudness labels or evenly spaced cues and call them compatible.

## 5. Corpus and model

- Identify unique recordings and version families before immutable train/validation/holdout splits.
- Label authoritative native inputs rather than an unverified approximate feature representation.
- Bind every model to its data, preprocessing and training identities; validate exported inference numerics in Rust.
- Close every development mismatch and require zero mismatches on a genuinely untouched holdout. A shortfall remains a failed gate.

## 6. Independent accuracy and release

- Compare key and BPM against independent annotations, with genre/class and ambiguity reporting.
- Evaluate energy and cue usefulness through blinded, declared criteria; multiple useful outcomes may exist.
- Keep any improved model explicitly separate from the vendor-compatibility model.
- Run complete offline qualification on macOS arm64 and Linux arm64/x86_64, then publish only the exercised claims.

Current work begins at the reference, numerical and scheduling boundaries. The existing model and heuristic extractor are retained as an explicitly experimental baseline, not as a target to defend.
