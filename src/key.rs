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

const HIDDEN: usize = 64;
const CLASSES: usize = 25;

/// Coefficients for the current experimental 352 -> 64 -> 25 model.
///
/// This schema deliberately has no legacy linear-model fallback.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Weights {
    w1: Vec<Vec<f32>>,
    b1: Vec<f32>,
    w2: Vec<Vec<f32>>,
    b2: Vec<f32>,
}

impl Weights {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let weights: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        weights.validate()?;
        Ok(weights)
    }

    fn validate(&self) -> Result<(), String> {
        let shapes_match = self.w1.len() == HIDDEN
            && self.w1.iter().all(|row| row.len() == FEATURES)
            && self.b1.len() == HIDDEN
            && self.w2.len() == CLASSES
            && self.w2.iter().all(|row| row.len() == HIDDEN)
            && self.b2.len() == CLASSES;
        if !shapes_match {
            return Err("model dimensions must be 352 -> 64 -> 25".into());
        }
        if !self.w1.iter().flatten().chain(&self.b1)
            .chain(self.w2.iter().flatten()).chain(&self.b2)
            .all(|value| value.is_finite())
        {
            return Err("model coefficients must be finite".into());
        }
        Ok(())
    }
}

/// Result of one classification.
#[derive(Clone, Debug)]
pub struct KeyEstimate {
    /// Best key, or `None` for the "no key" class.
    pub key: Option<CamelotKey>,
    /// Uncalibrated local softmax score, not vendor confidence.
    pub confidence: f32,
    /// Per-class scores after softmax; index matches [`CamelotKey::class_index`].
    pub scores: [f32; 25],
}

/// Validated evaluator for the experimental local model, not a vendor-equivalence claim.
pub struct KeyClassifier {
    weights: Weights,
}

impl KeyClassifier {
    pub fn new(weights: Weights) -> Result<Self, String> {
        weights.validate()?;
        Ok(Self { weights })
    }

    pub fn bundled() -> Result<Self, String> {
        Self::new(Weights::from_json(include_str!("weights.json"))?)
    }

    pub fn classify(&self, features: &[f64; FEATURES]) -> Result<KeyEstimate, String> {
        if features.iter().any(|v| !v.is_finite() || *v < 0.0) {
            return Err("model input must contain finite, nonnegative features".into());
        }
        if features.iter().all(|v| *v == 0.0) {
            return Err("experimental model is not qualified for empty or silent features".into());
        }

        // Retained experimental preprocessing; this is not an established vendor invariance.
        let mut x = [0.0f32; FEATURES];
        for (bank, output) in features.chunks(crate::feature::BINS)
            .zip(x.chunks_mut(crate::feature::BINS))
        {
            let sum: f64 = bank.iter().sum();
            if !sum.is_finite() {
                return Err("model input bank sum overflowed".into());
            }
            if sum > 0.0 {
                for (value, normalized) in bank.iter().zip(output) {
                    *normalized = (value / sum) as f32;
                }
            }
        }

        let mut hidden = [0.0f32; HIDDEN];
        for (i, value) in hidden.iter_mut().enumerate() {
            let z: f32 = self.weights.w1[i].iter().zip(&x).map(|(w, v)| w * v).sum();
            let z = z + self.weights.b1[i];
            if !z.is_finite() {
                return Err("model hidden activation overflowed".into());
            }
            *value = z.tanh();
        }

        let mut scores = [0.0f32; CLASSES];
        for (i, value) in scores.iter_mut().enumerate() {
            *value = self.weights.w2[i].iter().zip(&hidden).map(|(w, v)| w * v).sum::<f32>()
                + self.weights.b2[i];
            if !value.is_finite() {
                return Err("model output activation overflowed".into());
            }
        }
        let max = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let mut total = 0.0f32;
        for score in &mut scores {
            *score = (*score - max).exp();
            total += *score;
        }
        for score in &mut scores {
            *score /= total;
        }
        // NumPy argmax selects the first maximum; preserve that export/runtime boundary.
        let mut best = 0;
        for i in 1..CLASSES {
            if scores[i] > scores[best] {
                best = i;
            }
        }
        Ok(KeyEstimate {
            key: CamelotKey::from_class_index(best),
            confidence: scores[best],
            scores,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coefficients() -> Weights {
        Weights {
            w1: vec![vec![0.0; FEATURES]; HIDDEN],
            b1: vec![0.0; HIDDEN],
            w2: vec![vec![0.0; HIDDEN]; CLASSES],
            b2: vec![0.0; CLASSES],
        }
    }

    #[test]
    fn rejects_truncated_model_instead_of_truncating_dot_product() {
        let mut weights = coefficients();
        weights.w1[0].pop();
        assert!(KeyClassifier::new(weights).is_err());
    }

    #[test]
    fn rejects_nonfinite_coefficients() {
        let mut weights = coefficients();
        weights.b2[3] = f32::INFINITY;
        assert!(KeyClassifier::new(weights).is_err());
    }

    #[test]
    fn malformed_hidden_model_cannot_select_a_linear_fallback() {
        assert!(Weights::from_json(r#"{"w1":[],"b1":[],"w2":[],"b2":[],"w":[],"b":[]}"#).is_err());
    }

    #[test]
    fn empty_and_invalid_features_do_not_produce_a_confident_key() {
        let model = KeyClassifier::new(coefficients()).unwrap();
        assert!(model.classify(&[0.0; FEATURES]).is_err());
        let mut features = [1.0; FEATURES];
        features[0] = f64::NAN;
        assert!(model.classify(&features).is_err());
        features[0] = -1.0;
        assert!(model.classify(&features).is_err());
    }

    #[test]
    fn ties_select_first_class_like_training_runtime() {
        let model = KeyClassifier::new(coefficients()).unwrap();
        let estimate = model.classify(&[1.0; FEATURES]).unwrap();
        assert_eq!(estimate.key, CamelotKey::from_class_index(0));
        assert!(estimate.scores.iter().all(|score| (*score - 0.04).abs() < 1e-7));
    }
}
