# Experimental bundled model

`src/weights.json` is a locally trained numerical artifact, not vendor-supplied or recovered proprietary weights. It contains no audio, track titles, paths, account codes or request captures.

## Intended use

Research-only offline key predictions from the current experimental feature implementation. Not suitable for a claim of Mixed In Key compatibility, calibrated confidence, or automatic replacement of trusted library metadata.

## Known structure and limitations

- Fixed 352-element input, arranged as four banks of 88 in this implementation.
- One 64-unit tanh hidden layer and 25 output classes.
- Runtime independently normalizes each bank; equivalence of that transformation to the reference decision is unproved.
- Training labels came from queries using an approximate local feature path, not a fully qualified native application boundary.
- The existing artifact lacks a complete dataset/build/split lineage receipt. Historical evaluation is not an independent accepted benchmark.
- Training used different numeric precision from Rust inference; decision-boundary/tie behavior is not fully qualified.
- No adequate no-key/short/silent/error qualification is established.

The CLI's softmax `confidence` is an uncalibrated model score. It must not be interpreted as the probability of a musically correct answer or the original application's confidence.

A future accepted model requires authoritative teacher inputs, content-identified and disjoint recording groups, immutable preprocessing/class order, numerical export checks and a sealed independent test result. Until then, no 100% or near-100% claim is made.
