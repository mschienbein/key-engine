//! Provisional radix-2 in-place FFT used by the experimental feature extractor.
//!
//! Native operation order and the paired-frame path have not been reproduced here.
//! Downstream peak refinement uses complex components, so phase and sign require verification.

use std::f32::consts::PI;

/// Bit-reversal permutation plus butterflies, in place.
pub fn fft(re: &mut [f32], im: &mut [f32]) {
    let n = re.len();
    debug_assert_eq!(n, im.len());
    debug_assert!(n.is_power_of_two());
    if n <= 1 {
        return;
    }

    // Bit reversal.
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }

    // Butterflies.
    let mut len = 2usize;
    while len <= n {
        let ang = -2.0 * PI / len as f32;
        let (wr_step, wi_step) = (ang.cos(), ang.sin());
        let half = len / 2;
        let mut start = 0usize;
        while start < n {
            let mut wr = 1.0f32;
            let mut wi = 0.0f32;
            for k in 0..half {
                let i = start + k;
                let jx = i + half;
                let tr = wr * re[jx] - wi * im[jx];
                let ti = wr * im[jx] + wi * re[jx];
                re[jx] = re[i] - tr;
                im[jx] = im[i] - ti;
                re[i] += tr;
                im[i] += ti;
                let next_wr = wr * wr_step - wi * wi_step;
                wi = wr * wi_step + wi * wr_step;
                wr = next_wr;
            }
            start += len;
        }
        len <<= 1;
    }
}

/// Magnitude of the first `bins` FFT bins of `samples`, with the vendor's rectangular window.
pub fn magnitudes(samples: &[f32], scratch_re: &mut Vec<f32>, scratch_im: &mut Vec<f32>, bins: usize) -> Vec<f32> {
    let n = samples.len();
    scratch_re.clear();
    scratch_im.clear();
    scratch_re.extend_from_slice(samples);
    scratch_im.resize(n, 0.0);
    fft(scratch_re, scratch_im);
    (0..bins)
        .map(|b| (scratch_re[b] * scratch_re[b] + scratch_im[b] * scratch_im[b]).sqrt())
        .collect()
}
