use std::{fs, path::PathBuf};

use pico_args::Arguments;

struct Args {
    srt_path: PathBuf,
}

const HELP: &str = "\
parse <SRT_PATH>
Parse the <SRT_PATH> file and pretty print the `Vec<Subtitle>` debug output

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
    let mut args = Arguments::from_env();
    let srt_path = PathBuf::from(args.free_from_str::<String>().ok()?);
    Some(Args { srt_path })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { srt_path } = parse_args();

    let bytes = fs::read(&srt_path)?;
    // BOM-sniffing
    let text = std::str::from_utf8(if bytes.starts_with(b"\xef\xbb\xbf") {
        &bytes[3..]
    } else {
        &bytes
    })?;
    let subtitles = kiss_srt::from_str(text)?;
    println!("{:#?}", subtitles);

    Ok(())
}
