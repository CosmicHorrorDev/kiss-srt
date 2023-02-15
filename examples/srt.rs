use std::{env, fs, path::PathBuf};

struct Args {
    command: Command,
    srt_path: PathBuf,
}

enum Command {
    /// Scale the timestamps by some factor (e.g. 1.25)
    Scale { value: f32 },
    /// Increase the timestamps by some value in ms
    Increase { ms: u32 },
    /// Decrease the timestamps by some value in ms
    Decrease { ms: u32 },
}

const HELP: &str = "\
srt <COMMAND> <SRT_PATH>
Applies <COMMAND> to the SRT file piped to stdin

COMMAND:
    scale <VALUE>  Scale the timestamps by <VALUE> 
    increase <MS>  Increase the timestamps by <MS>
    decrease <MS>  Decrease the timestamps by <MS>

ARGS:
    <SRT_PATH>  Path to the srt file
";

fn parse_args() -> Args {
    match try_parse_args() {
        Some(args) => args,
        None => {
            eprintln!("{}", HELP);
            std::process::exit(1);
        }
    }
}

fn try_parse_args() -> Option<Args> {
    let mut args = env::args();

    // the executable
    let _ = args.next();

    let command = match args.next()?.as_str() {
        "scale" => {
            let value = args.next()?.parse().ok()?;
            Command::Scale { value }
        }
        "increase" => {
            let ms = args.next()?.parse().ok()?;
            Command::Increase { ms }
        }
        "decrease" => {
            let ms = args.next()?.parse().ok()?;
            Command::Decrease { ms }
        }
        _ => return None,
    };

    let srt_path = PathBuf::from(args.next()?);

    Some(Args { command, srt_path })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { command, srt_path } = parse_args();

    let bytes = fs::read(&srt_path)?;
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
