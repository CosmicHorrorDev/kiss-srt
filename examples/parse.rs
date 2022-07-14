use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let _exe = args.next();
    let path = args.next().unwrap();

    let bytes = fs::read(&path).unwrap();
    // BOM-sniffing
    let text = std::str::from_utf8(if bytes.starts_with(b"\xef\xbb\xbf") {
        &bytes[3..]
    } else {
        &bytes
    })?;
    let subtitles = kiss_srt::from_str(text).unwrap();
    print!("{}", kiss_srt::to_string(&subtitles));

    Ok(())
}
