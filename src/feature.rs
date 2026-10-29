//! Experimental pitch-profile extraction; not a verified native reconstruction.
//!
//! This implementation uses four-second windows and heuristic spectral peaks. The reference
//! application instead schedules ranges in four-beat coordinates and applies temporal peak
//! association, tuning and a weighted range reducer. Those differences remain unresolved.
//! The nominal per-bank grid has 88 semitones from A0 through C8; occupied edge bins depend on
//! the processing path. Fixed-size output here is a local model contract, not a universal
//! statement about the application's serialized arrays.

/// FFT size used at 44.1 kHz by the vendor code.
pub const FFT_SIZE: usize = 4096;
/// Hop between frames (`fft_size / 2`).
pub const HOP: usize = FFT_SIZE / 2;
/// Number of output bins per bank: one semitone each, A0 = 0.
pub const BINS: usize = 88;
/// Number of fixed banks in this experimental model's input.
pub const BANKS: usize = 4;
/// Total length of this implementation's model-input array.
pub const FEATURES: usize = BINS * BANKS;
/// Lowest frequency represented by bin 0.
pub const BASE_HZ: f64 = 27.5;
/// Provisional time window; the native app uses beat-coordinate scheduling instead.
pub const WINDOW_SECONDS: f64 = 4.0;
/// Sample rate the extractor runs at.
pub const RATE: f64 = 44_100.0;

/// Highest FFT bin kept: `5512.5 Hz` rounded to a bin, capped by the vendor's limits.
fn useful_bins(rate: f64, fft_size: usize) -> usize {
    let per_bin = rate / fft_size as f64;
    let mut n = (5512.5 / per_bin + 0.5) as usize;
    let cap = fft_size / 2 - 1;
    if cap < n {
        n = cap;
    }
    if n > 1023 {
        n = 1023;
    }
    n
}

/// A tracked partial inside one frame.
#[derive(Clone, Copy)]
struct Partial {
    /// Peak position in FFT bins (fractional).
    bin: f64,
    /// Magnitude (linear).
    mag: f64,
}

/// Compute the 352-value key feature vector for interleaved PCM at 44.1 kHz, starting at 0 s.
pub fn key_features(samples: &[f32], channels: usize) -> [f64; FEATURES] {
    key_features_from(samples, channels, 0.0)
}

/// Compute the experimental 352-value vector using an explicitly selected time offset.
///
/// An offset alone does not reproduce the native application's refined beat scheduling.
pub fn key_features_from(samples: &[f32], channels: usize, offset_seconds: f64) -> [f64; FEATURES] {
    let frames = samples.len() / channels.max(1);
    let mut mono = Vec::with_capacity(frames);
    for f in 0..frames {
        let mut acc = 0.0f32;
        for c in 0..channels {
            acc += samples[f * channels + c];
        }
        mono.push(acc);
    }
    key_features_mono(&mono, offset_seconds)
}

/// Compute the 352-value key feature vector from mono PCM at 44.1 kHz.
pub fn key_features_mono(mono: &[f32], offset_seconds: f64) -> [f64; FEATURES] {
    let bins = useful_bins(RATE, FFT_SIZE);
    let seconds = mono.len() as f64 / RATE;
    let mut out = [0.0f64; FEATURES];

    // Per-frame partial lists, computed once and reused by every window.
    let mut re = Vec::with_capacity(FFT_SIZE);
    let mut im = Vec::with_capacity(FFT_SIZE);
    let mut frame_partials: Vec<(f64, Vec<Partial>)> = Vec::new();
    let mut offset = 0usize;
    while offset + FFT_SIZE <= mono.len() {
        re.clear();
        re.extend_from_slice(&mono[offset..offset + FFT_SIZE]);
        im.clear();
        im.resize(FFT_SIZE, 0.0);
        crate::fft::fft(&mut re, &mut im);
        // Frame centre time, matching the vendor's frame-index arithmetic.
        let centre = (offset as f64 + FFT_SIZE as f64 / 2.0) / RATE;
        frame_partials.push((centre, track_partials(&re, &im, bins)));
        offset += HOP;
    }

    let mut start = offset_seconds.max(0.0);
    let mut window = 0usize;
    while start + WINDOW_SECONDS <= seconds {
        let end = start + WINDOW_SECONDS;
        let bank = (window % BANKS) * BINS;
        add_window(&mut out[bank..bank + BINS], &frame_partials, start, end);
        window += 1;
        start += WINDOW_SECONDS;
    }
    out
}

/// Heuristic local-maxima selection used by the current experimental model.
///
/// This is not the native temporal-association algorithm. The magnitude floor and cap are
/// empirical choices and must not be presented as recovered vendor constants.
fn track_partials(re: &[f32], im: &[f32], bins: usize) -> Vec<Partial> {
    let mut mag = Vec::with_capacity(bins);
    for b in 0..bins {
        mag.push(((re[b] * re[b] + im[b] * im[b]) as f64).sqrt());
    }

    let peak = mag.iter().cloned().fold(0.0f64, f64::max);
    let floor = peak * 0.5;
    let mut out = Vec::new();
    for i in 1..bins.saturating_sub(1) {
        if mag[i] < floor {
            continue;
        }
        let (m0, m1, m2) = (mag[i - 1], mag[i], mag[i + 1]);
        if m0 <= 0.0 || m2 <= 0.0 || m1 < m0 || m2 >= m1 {
            continue;
        }
        let denom = (m1 * m1) as f32;
        if denom <= 0.0 {
            continue;
        }
        let delta = ((im[i] * (re[i - 1] - re[i + 1]) + re[i] * (im[i + 1] - im[i - 1])) / denom) * 0.45;
        if delta.abs() >= 0.7 {
            continue;
        }
        out.push(Partial {
            bin: i as f64 + delta as f64,
            mag: m1,
        });
        if out.len() >= 24 {
            break;
        }
    }
    out
}

/// Accumulate one 4-second window into `acc`: max over the window's frames, then a peak test, then
/// the vendor's per-bin weighting.
fn add_window(acc: &mut [f64], frames: &[(f64, Vec<Partial>)], start: f64, end: f64) {
    let mut harm = [0.0f32; 90];
    let mut fund = [0.0f32; 90];
    let mut any = false;

    for (centre, partials) in frames {
        if *centre < start || *centre >= end {
            continue;
        }
        any = true;
        for p in partials {
            let pos = 12.0 * (p.bin * RATE / FFT_SIZE as f64 / 440.0).log2() + 48.0;
            let energy = (p.mag * p.mag) as f32;

            let idx = (pos as f32 + 0.5) as i32;
            if idx >= 0 {
                let x = (pos as f32 - idx as f32) / 0.8 + 0.5;
                if let Some(w) = kernel(x) {
                    let slot = idx as usize;
                    if slot < 89 {
                        let v = energy * w;
                        if v > harm[slot] {
                            harm[slot] = v;
                        }
                    }
                }
            }

            let pos2 = pos - 0.5;
            let idx2 = (pos2 as f32 + 0.5) as i32;
            if idx2 >= 0 {
                let x = (pos2 as f32 - idx2 as f32) / 0.8 + 0.5;
                if let Some(w) = kernel(x) {
                    let slot = idx2 as usize;
                    if slot < 89 {
                        let v = energy * w;
                        if v > fund[slot] {
                            fund[slot] = v;
                        }
                    }
                }
            }
        }
    }

    if !any {
        return;
    }

    for i in 1..BINS {
        let neighbours = fund[i - 1] + harm[i] + fund[i] + harm[i + 1] + fund[i + 1] + fund[i + 2];
        if neighbours * 0.4 > harm[i] {
            continue;
        }
        acc[i] += compress(harm[i] as f64, i);
    }
}

/// `Finished`'s per-bin weighting: A-weighting at the bin frequency raised to 3.0031, then a sixth
/// root, then the vendor's `6e6` scale.
fn compress(value: f64, bin: usize) -> f64 {
    let f = BASE_HZ * 2f64.powf(bin as f64 / 12.0);
    let a_weight = {
        let f2 = f * f;
        const F1: f64 = 20.6;
        const F2: f64 = 107.7;
        const F3: f64 = 737.9;
        const F4: f64 = 12194.0;
        let num = f2 * f2 * (F4 * F4);
        let den = (f2 + F4 * F4) * (f2 + F1 * F1) * ((f2 + F2 * F2) * (f2 + F3 * F3)).sqrt();
        num / den
    };
    (value * a_weight.powf(3.0031)).powf(1.0 / 6.0) * 6e6
}

fn kernel(x: f32) -> Option<f32> {
    if !(0.0..1.0).contains(&x) {
        return None;
    }
    Some(0.5 - 0.5 * (x * 2.0 * std::f32::consts::PI).cos())
}

/// Experimental per-bin shaping retained for research comparisons.
///
/// This helper is not evidence that the service applies the same preprocessing.
pub fn shape(raw: &[f64; BINS]) -> [f64; BINS] {
    let mut out = [0.0f64; BINS];
    for i in 0..BINS {
        let tilt = (i as f64 * (11.0 / 240.0)).exp2();
        let mut v = (tilt * raw[i]).powf(0.62);
        if i == 87 || i <= 4 {
            v = 0.0;
        }
        if i == 5 || i == 6 {
            v *= 0.5;
        }
        out[i] = v;
    }
    let sum: f64 = out.iter().sum();
    if sum == 0.0 {
        return [1.0; BINS];
    }
    let scale = 0.9 / sum;
    for v in out.iter_mut() {
        *v *= scale;
    }
    out
}

/// Unit-sum normalization used before classification.
pub fn unit_sum(raw: &[f64; FEATURES]) -> [f64; FEATURES] {
    let sum: f64 = raw.iter().sum();
    let mut out = *raw;
    if sum <= 0.0 {
        return out;
    }
    for v in out.iter_mut() {
        *v /= sum;
    }
    out
}
