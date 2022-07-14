use std::io::{self, Read};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Scale the timestamps by some factor (e.g. 1.25)
    Scale { value: f32 },
    /// Increase the timestamps by some value in ms
    Increase { ms: u64 },
    /// Decrease the timestamps by some value in ms
    Decrease { ms: u64 },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { command } = Args::parse();

    let mut stdin = io::stdin().lock();
    let mut bytes = Vec::new();
    stdin.read_to_end(&mut bytes)?;
    // BOM-sniffing
    let text = std::str::from_utf8(if bytes.starts_with(b"\xef\xbb\xbf") {
        &bytes[3..]
    } else {
        &bytes
    })?;

    let subtitles = kiss_srt::from_str(text)?;

    // Perform the transformation
    let modified: Vec<_> = subtitles
        .into_iter()
        .map(|mut sub| match command {
            Command::Scale { value } => {
                sub.start *= value;
                sub.duration *= value;
                sub
            }
            Command::Increase { ms } => {
                sub.start += kiss_srt::Duration::from_millis(ms);
                sub
            }
            Command::Decrease { ms } => {
                sub.start -= kiss_srt::Duration::from_millis(ms);
                sub
            }
        })
        .collect();

    print!("{}", kiss_srt::to_string(modified.as_slice()));

    Ok(())
}
