use recommender_core::Engine;
use serde_json::Value;
use std::io::{self, Read, Write};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--help"] {
        println!("lol-scoring-core: read a JSON answers object from stdin; write recommendations as JSON.\nExample: printf '%s' '{{\"1\":\"Medium (4-6)\",\"2\":\"Mage\",\"3\":\"Mid\"}}' | cargo run --quiet -p recommender-core --bin lol-scoring-core");
        return Ok(());
    }
    if !args.is_empty() {
        return Err("Usage: lol-scoring-core [--help] (JSON answers on stdin)".into());
    }
    const LIMIT: u64 = 1024 * 1024;
    let mut input = String::new();
    io::stdin().take(LIMIT + 1).read_to_string(&mut input)?;
    if input.len() as u64 > LIMIT {
        return Err("Input exceeds the 1 MiB limit".into());
    }
    let answers: Value = serde_json::from_str(&input)?;
    let result = Engine::reference()?.recommend(&answers)?;
    let stdout = io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer(&mut output, &result)?;
    writeln!(output)?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Scoring failed: {error}");
        std::process::exit(1);
    }
}
