//! Experimental local key classification from a fixed banked feature vector.
//!
//! The bundled model is locally trained, not recovered vendor weights. Its training lineage and
//! generalization are not qualified for a compatibility claim. Softmax output is an uncalibrated
//! model score, not a probability that the musical key is correct.

use serde::Deserialize;

use crate::feature::FEATURES;

/// Camelot wheel position: number 1-12 and side A (minor) or B (major).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CamelotKey {
    pub number: u8,
    pub minor: bool,
}

/// Camelot number for a minor key, indexed by pitch class (C = 0).
const MINOR_NUMBER: [u8; 12] = [5, 12, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10];
/// Camelot number for a major key, indexed by pitch class (C = 0).
const MAJOR_NUMBER: [u8; 12] = [8, 3, 10, 5, 12, 7, 2, 9, 4, 11, 6, 1];

const PITCH_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

impl CamelotKey {
    pub fn new(number: u8, minor: bool) -> Self {
        Self { number, minor }
    }

    /// Build from a pitch class and mode.
    pub fn from_pitch_class(pitch_class: u8, minor: bool) -> Self {
        let pc = (pitch_class % 12) as usize;
        Self {
            number: if minor { MINOR_NUMBER[pc] } else { MAJOR_NUMBER[pc] },
            minor,
        }
    }

    /// Parse Camelot text such as `8A` or `12B`.
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        let (digits, side) = text.split_at(text.len().checked_sub(1)?);
        let number: u8 = digits.parse().ok()?;
        if !(1..=12).contains(&number) {
            return None;
        }
        match side {
            "A" | "a" => Some(Self { number, minor: true }),
            "B" | "b" => Some(Self { number, minor: false }),
            _ => None,
        }
    }

    /// Camelot label, e.g. `8A`.
    pub fn label(&self) -> String {
        format!("{}{}", self.number, if self.minor { 'A' } else { 'B' })
    }

    /// Pitch class and mode.
    pub fn pitch_class(&self) -> (u8, bool) {
        let table = if self.minor { MINOR_NUMBER } else { MAJOR_NUMBER };
        let pc = table
            .iter()
            .position(|&n| n == self.number)
            .unwrap_or(0) as u8;
        (pc, self.minor)
    }

    /// Traditional name, e.g. `A minor`.
    pub fn name(&self) -> String {
        let (pc, minor) = self.pitch_class();
        format!("{} {}", PITCH_NAMES[pc as usize], if minor { "minor" } else { "major" })
    }

    /// Index in the classifier's class list: 0-11 minor, 12-23 major, 24 = no key.
    pub fn class_index(&self) -> usize {
        let (pc, minor) = self.pitch_class();
        if minor { pc as usize } else { 12 + pc as usize }
    }

    /// Inverse of [`Self::class_index`]; `None` for the "no key" class.
    pub fn from_class_index(index: usize) -> Option<Self> {
        match index {
            0..=11 => Some(Self::from_pitch_class(index as u8, true)),
            12..=23 => Some(Self::from_pitch_class((index - 12) as u8, false)),
            _ => None,
        }
    }
}

/// Experimental linear or one-hidden-layer model coefficients.
#[derive(Deserialize)]
pub struct Weights {
    /// `[classes][FEATURES]`, one row per output class.
    pub w: Vec<Vec<f32>>,
    /// One bias per output class.
    pub b: Vec<f32>,
    /// Optional hidden layer: `[hidden][FEATURES]` and `[hidden]`, applied with `tanh` when present.
    #[serde(default)]
    pub w1: Option<Vec<Vec<f32>>>,
    #[serde(default)]
    pub b1: Option<Vec<f32>>,
    /// `[classes][hidden]`, used with the hidden layer.
    #[serde(default)]
    pub w2: Option<Vec<Vec<f32>>>,
    #[serde(default)]
    pub b2: Option<Vec<f32>>,
}

impl Weights {
    pub fn from_json(text: &str) -> Result<Self, String> {
        serde_json::from_str(text).map_err(|e| e.to_string())
    }
}

/// Result of one classification.
#[derive(Clone, Debug)]
pub struct KeyEstimate {
    /// Best key, or `None` for the "no key" class.
    pub key: Option<CamelotKey>,
    /// Confidence of the best key.
    pub confidence: f32,
    /// Per-class scores after softmax; index matches [`CamelotKey::class_index`].
    pub scores: [f32; 25],
}

/// Local stand-in for the vendor analysis service.
pub struct KeyClassifier {
    weights: Weights,
}

impl KeyClassifier {
    pub fn new(weights: Weights) -> Self {
        Self { weights }
    }

    /// Classifier shipped with the crate.
    pub fn bundled() -> Self {
        Self::new(Weights::from_json(include_str!("weights.json")).expect("bundled weights"))
    }

    pub fn classify(&self, features: &[f64; FEATURES]) -> KeyEstimate {
        // Banks hold different numbers of analysis windows, so normalise each separately.
        let mut x = Vec::with_capacity(FEATURES);
        for bank in features.chunks(crate::feature::BINS) {
            let sum: f64 = bank.iter().sum();
            let scale = if sum > 0.0 { 1.0 / sum } else { 0.0 };
            x.extend(bank.iter().map(|&v| (v * scale) as f32));
        }

        let logits: Vec<f32> = match (&self.weights.w1, &self.weights.b1, &self.weights.w2, &self.weights.b2) {
            (Some(w1), Some(b1), Some(w2), Some(b2)) => {
                let hidden: Vec<f32> = w1
                    .iter()
                    .zip(b1.iter())
                    .map(|(row, bias)| {
                        let z: f32 = row.iter().zip(x.iter()).map(|(w, v)| w * v).sum();
                        (z + bias).tanh()
                    })
                    .collect();
                w2.iter()
                    .zip(b2.iter())
                    .map(|(row, bias)| row.iter().zip(hidden.iter()).map(|(w, v)| w * v).sum::<f32>() + bias)
                    .collect()
            }
            _ => self
                .weights
                .w
                .iter()
                .zip(self.weights.b.iter())
                .map(|(row, bias)| row.iter().zip(x.iter()).map(|(w, v)| w * v).sum::<f32>() + bias)
                .collect(),
        };

        let max = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let mut scores = [0.0f32; 25];
        let mut total = 0.0f32;
        for (i, z) in logits.iter().enumerate() {
            let e = (z - max).exp();
            scores[i] = e;
            total += e;
        }
        for s in scores.iter_mut() {
            *s /= total;
        }

        let (best, &confidence) = scores
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .expect("non-empty scores");

        KeyEstimate {
            key: CamelotKey::from_class_index(best),
            confidence,
            scores,
        }
    }
}
