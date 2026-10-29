//! Experimental offline musical-key analysis.
//!
//! [`feature`] computes a provisional pitch profile and [`key`] evaluates a locally trained model.
//! This implementation is not a verified reconstruction of Mixed In Key. Native scheduling,
//! spectral tracking, numerical behavior and model provenance still require qualification.
//! The public project roadmap separates vendor compatibility from musical accuracy.

pub mod audio;
pub mod fft;
pub mod feature;
pub mod key;

pub use feature::key_features;
pub use key::{CamelotKey, KeyClassifier};
