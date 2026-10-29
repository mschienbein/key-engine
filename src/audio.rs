//! Decode an audio file to interleaved `f32` at 44.1 kHz.
//!
//! The vendor extractor is fed 44.1 kHz float PCM by `ExtAudioFile`, and sums channels itself, so the
//! decoder here keeps the source channel count and only resamples the rate.

use std::fs::File;
use std::path::Path;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Sample rate the feature extractor runs at.
pub const ANALYSIS_RATE: u32 = 44_100;

pub struct Decoded {
    /// Interleaved samples in `-1..=1`, `channels` per frame.
    pub samples: Vec<f32>,
    pub channels: usize,
    /// Source sample rate before resampling.
    pub source_rate: u32,
    /// Frames after resampling.
    pub frames: u64,
}

pub fn decode(path: &Path) -> Result<Decoded, String> {
    let file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let stream = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(&hint, stream, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| format!("probe: {e}"))?;
    let mut format = probed.format;

    let (track_id, declared_channels, declared_rate) = {
        let track = format
            .default_track()
            .ok_or_else(|| "no default audio track".to_string())?;
        (
            track.id,
            track.codec_params.channels.map(|c| c.count()),
            track.codec_params.sample_rate,
        )
    };
    let mut decoder = {
        let track = format
            .default_track()
            .ok_or_else(|| "no default audio track".to_string())?;
        symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())
            .map_err(|e| format!("decoder: {e}"))?
    };

    let mut interleaved: Vec<f32> = Vec::new();
    let mut channels = declared_channels.unwrap_or(0);
    let mut source_rate = declared_rate.unwrap_or(ANALYSIS_RATE);

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(ref e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(e) => return Err(format!("read: {e}")),
        };
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(buffer) => {
                let spec = *buffer.spec();
                channels = spec.channels.count();
                source_rate = spec.rate;
                push_buffer(&buffer, &mut interleaved);
            }
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(format!("decode: {e}")),
        }
    }

    if channels == 0 {
        channels = 1;
    }
    if source_rate != ANALYSIS_RATE {
        interleaved = resample(&interleaved, channels, source_rate, ANALYSIS_RATE);
    }

    Ok(Decoded {
        frames: (interleaved.len() / channels) as u64,
        samples: interleaved,
        channels,
        source_rate,
    })
}

fn push_buffer(buffer: &AudioBufferRef<'_>, out: &mut Vec<f32>) {
    /// Interleave every channel, converting to `f32` in `-1..=1`.
    macro_rules! interleave {
        ($b:expr, $conv:expr) => {{
            let b = $b;
            let conv = $conv;
            let frames = b.frames();
            let ch = b.spec().channels.count();
            let start = out.len();
            out.resize(start + frames * ch, 0.0);
            for c in 0..ch {
                let channel = b.chan(c);
                for f in 0..frames {
                    out[start + f * ch + c] = conv(channel[f]);
                }
            }
        }};
    }
    match buffer {
        AudioBufferRef::F32(b) => interleave!(b, |s: f32| s),
        AudioBufferRef::F64(b) => interleave!(b, |s: f64| s as f32),
        AudioBufferRef::S32(b) => interleave!(b, |s: i32| s as f32 / 2_147_483_648.0),
        AudioBufferRef::S16(b) => interleave!(b, |s: i16| s as f32 / 32_768.0),
        AudioBufferRef::S24(b) => interleave!(b, |s: symphonia::core::sample::i24| s.inner() as f32 / 8_388_608.0),
        AudioBufferRef::U8(b) => interleave!(b, |s: u8| (s as f32 - 128.0) / 128.0),
        AudioBufferRef::U16(b) => interleave!(b, |s: u16| (s as f32 - 32_768.0) / 32_768.0),
        AudioBufferRef::U24(b) => interleave!(b, |s: symphonia::core::sample::u24| (s.inner() as f32 - 8_388_608.0) / 8_388_608.0),
        AudioBufferRef::U32(b) => interleave!(b, |s: u32| (s as f64 - 2_147_483_648.0) as f32 / 2_147_483_648.0),
        AudioBufferRef::S8(b) => interleave!(b, |s: i8| s as f32 / 128.0),
    }
}

/// Linear resampling. Adequate for the 48 kHz -> 44.1 kHz case; replace with a windowed-sinc
/// resampler if parity work needs it.
fn resample(input: &[f32], channels: usize, from: u32, to: u32) -> Vec<f32> {
    if from == to || input.is_empty() {
        return input.to_vec();
    }
    let ratio = from as f64 / to as f64;
    let in_frames = input.len() / channels;
    let out_frames = ((in_frames as f64) / ratio) as usize;
    let mut out = vec![0.0f32; out_frames * channels];
    for f in 0..out_frames {
        let pos = f as f64 * ratio;
        let i0 = (pos.floor() as usize).min(in_frames.saturating_sub(1));
        let frac = (pos - i0 as f64) as f32;
        let i1 = (i0 + 1).min(in_frames.saturating_sub(1));
        for c in 0..channels {
            let a = input[i0 * channels + c];
            let b = input[i1 * channels + c];
            out[f * channels + c] = a + (b - a) * frac;
        }
    }
    out
}
