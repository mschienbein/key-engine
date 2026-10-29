use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use key_engine::audio;
use key_engine::feature::{key_features, BINS};
use key_engine::key::KeyClassifier;

#[derive(Parser)]
#[command(name = "key", about = "Experimental offline musical-key analysis; compatibility unverified")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Detect the musical key of one or more audio files.
    Analyze {
        files: Vec<PathBuf>,
        /// Emit JSON instead of text.
        #[arg(long)]
        json: bool,
    },
    /// Print the experimental banked key feature vector.
    Features {
        file: PathBuf,
        /// Emit JSON instead of a table.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Analyze { files, json } => analyze(&files, json),
        Command::Features { file, json } => features(&file, json),
    }
}

fn features(path: &PathBuf, json: bool) -> ExitCode {
    let decoded = match audio::decode(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}: {e}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let vector = key_features(&decoded.samples, decoded.channels);

    let stdout = io::stdout();
    let mut output = stdout.lock();
    let result = if json {
        writeln!(output, "{}", serde_json::json!({
            "file": path.display().to_string(), "features": vector.to_vec()
        }))
    } else {
        let mut result = Ok(());
        for (i, value) in vector.iter().enumerate() {
            let bin = i % BINS;
            let bank = i / BINS;
            let hz = 27.5 * 2f64.powf(bin as f64 / 12.0);
            if let Err(error) = writeln!(output, "bank {bank}  bin {bin:2}  {hz:8.2} Hz  {value:.6}") {
                result = Err(error);
                break;
            }
        }
        result
    };
    output_status(result)
}

fn analyze(paths: &[PathBuf], json: bool) -> ExitCode {
    if paths.is_empty() {
        eprintln!("no files given");
        return ExitCode::FAILURE;
    }
    let classifier = match KeyClassifier::bundled() {
        Ok(model) => model,
        Err(error) => {
            eprintln!("invalid experimental model: {error}");
            return ExitCode::FAILURE;
        }
    };
    let mut failures = 0usize;
    let mut rows = Vec::new();
    let stdout = io::stdout();
    let mut output = stdout.lock();

    for path in paths {
        let decoded = match audio::decode(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{}: {e}", path.display());
                failures += 1;
                if json {
                    rows.push(serde_json::json!({ "file": path.display().to_string(), "error": e }));
                }
                continue;
            }
        };
        let features = key_features(&decoded.samples, decoded.channels);
        let estimate = match classifier.classify(&features) {
            Ok(estimate) => estimate,
            Err(error) => {
                eprintln!("{}: {error}", path.display());
                failures += 1;
                if json {
                    rows.push(serde_json::json!({ "file": path.display().to_string(), "error": error }));
                }
                continue;
            }
        };
        let (label, name, confidence) = match estimate.key {
            Some(k) => (k.label(), k.name(), estimate.confidence),
            None => ("0".to_string(), "no key".to_string(), estimate.confidence),
        };
        if json {
            rows.push(serde_json::json!({
                "file": path.display().to_string(),
                "key": label,
                "name": name,
                "confidence": confidence,
                "seconds": decoded.frames as f64 / audio::ANALYSIS_RATE as f64,
            }));
        } else {
            if let Err(error) = writeln!(output, "{label:>3}  {confidence:5.3}  {name:<10}  {}", path.display()) {
                return output_status(Err(error));
            }
        }
    }

    if json {
        if let Err(error) = writeln!(output, "{}", serde_json::Value::Array(rows)) {
            return output_status(Err(error));
        }
    }
    if failures > 0 { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

fn output_status(result: io::Result<()>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("output: {error}");
            ExitCode::FAILURE
        }
    }
}
