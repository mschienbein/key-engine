# Reconstruction roadmap

**Status: 30 October 2026.** The public code is the superseded experimental CLI. The current reconstruction is developed privately and is not published, pending licensing review. This roadmap describes remaining work, not features available in this checkout or a promised release date.

## Measured work already completed

- Local decoding, tested sample-rate conversion, BPM/beats, app refinement, scheduling, key/energy features and cue pre-analysis have exact matches on declared generated and real-file boundaries.
- A frozen offline snapshot passed the macOS arm64, Linux arm64 and Linux x86_64 qualification matrix. The final snapshot with all later changes passed every reachable check on the same platforms; re-running the real MP3/AAC and high-resolution file comparisons with that final build is pending.
- Learned key and energy/cue decisions were integrated and evaluated on two sealed held-out sets. They are approximations, not recovered vendor models or service parity.
- The final offline CLI and a separate MIK 11 service sample were evaluated against GiantSteps annotations.
- Optional vendor-service integration has stored-exchange replay and limited live-smoke evidence; it is not an offline reconstruction of the server.

The [verification record](VERIFICATION.md) gives the counts, populations and exceptions. These completed measurements supersede the earlier plan to obtain the first app-function captures and train the first replacement models.

## 1. Licensing and public-distribution review

- Complete licensing and provenance review before distributing reconstructed code, newly learned weights or research-derived assets.
- Keep proprietary binaries, decompilation, audio, service captures and credentials out of the public distribution.
- Distinguish the existing MIT-licensed public source from material that has not been cleared for release. A documentation update is not a license decision or an implementation release.

## 2. Declared input and numerical-profile coverage

- Broaden real-file coverage beyond the measured format/rate/channel combinations. In particular, the real AAC comparison covers 44.1 kHz stereo, not real mono or resampled AAC.
- Preserve explicit malformed-input, empty/short-input and multichannel outcomes. Four measured six-channel cases did not provide a safely defined native result; do not invent one.
- Keep native allocator-dependent numerical behavior visible. A deterministic selected profile must not be called the unique application answer on sensitive inputs.
- Extend numerical and caller coverage without promoting a bounded corpus to an exhaustive arithmetic claim. Additional profile support or sensitivity detection needs its own evidence, not per-file exceptions or guessed majority outcomes.

## 3. Remaining offline decision gaps

- Improve learned key, energy and cue agreement while retaining the measured **85.2% / 86.2%** held-out key-label results and **0%** complete-decision identity as historical facts.
- Address unreproduced confidence, no-key, secondary-key and single-note behavior, and unqualified short key vectors. Do not substitute confident guesses for missing reference behavior.
- Treat energy levels, overall energy, later cue selection/count and cue levels as learned decisions. Conditional timing/merge-rule matches are not complete output equality.
- Preserve recording-family grouping and dataset/model/preprocessing identities. Once an evaluation set informs changes, it is development data; any new acceptance claim needs a fresh independent holdout.

## 4. Integrated qualification after changes

- Re-freeze the complete implementation and models before extending claims beyond the qualified offline snapshot.
- Re-exercise every declared platform and affected boundary after decoder-recovery, numerical-profile, model or service-integration changes. Component-only evidence does not automatically qualify the merged implementation.
- Keep offline predictions and vendor-service results distinguishable. Test network-disabled operation independently from any separately authorized live use.
- Preserve failed runs, refusals and original denominators. A high average score or a rounded value cannot replace a zero-mismatch compatibility gate.

## 5. Broader musical-quality evaluation

- Broaden annotated key/tempo evaluation beyond excerpts and the measured populations, with preregistered metrics and uncertainty.
- Do not claim improvement over MIK 11 from the **68.0% versus 66.7%** result on 147 excerpts: that paired difference was not significant and did not establish equivalence.
- Evaluate energy and cue usefulness against independently authored or blinded criteria. GiantSteps provides no ground truth for those outputs.
- Keep musical-accuracy improvements distinct from vendor-compatibility behavior, and avoid tuning on reported benchmark errors.

## 6. Any future public cutover

If distribution is cleared, replace the historical experimental CLI with an explicitly versioned, qualified implementation and publish only the claims its source/model/platform snapshot supports. Update build instructions, model documentation, dependency notices and results together. Until then, this repository remains the early CLI plus an aggregate status report; no reconstruction code, new models or data are released.
