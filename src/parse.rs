use std::str::Bytes;

use crate::{
    error::{Error, Result},
    time::Timestamp,
    Subtitle,
};

fn parse_three_digit_ascii_num(bytes: &mut Bytes<'_>) -> Option<u16> {
    let hundreds = bytes.next().and_then(parse_ascii_digit)?;
    let the_rest = parse_two_digit_ascii_num(bytes)?;

    Some(u16::from(hundreds) * 100 + u16::from(the_rest))
}

fn parse_two_digit_ascii_num(bytes: &mut Bytes<'_>) -> Option<u8> {
    let (tens, ones) = bytes
        .next()
        .and_then(parse_ascii_digit)
        .zip(bytes.next().and_then(parse_ascii_digit))?;

    Some(tens * 10 + ones)
}

fn parse_ascii_digit(b: u8) -> Option<u8> {
    if b.is_ascii_digit() {
        Some(b - b'0')
    } else {
        None
    }
}

// Of the form '01:23:45,678'
fn parse_ts(bytes: &mut Bytes<'_>) -> Option<Timestamp> {
    let hours = parse_two_digit_ascii_num(bytes)?;

    if bytes.next() != Some(b':') {
        return None;
    }

    let minutes = parse_two_digit_ascii_num(bytes)?;

    if bytes.next() != Some(b':') {
        return None;
    }

    let seconds = parse_two_digit_ascii_num(bytes)?;

    if bytes.next() != Some(b',') {
        return None;
    }

    let millis = parse_three_digit_ascii_num(bytes)?;

    Timestamp::new(hours, minutes, seconds, millis)
}

fn parse_ts_divider(bytes: &mut Bytes<'_>) -> Option<()> {
    if &[
        bytes.next()?,
        bytes.next()?,
        bytes.next()?,
        bytes.next()?,
        bytes.next()?,
    ] == b" --> "
    {
        Some(())
    } else {
        None
    }
}

pub fn from_str(subtitles: &str) -> Result<Vec<Subtitle>> {
    let mut parsed: Vec<Subtitle> = Vec::new();
    let mut lines = subtitles.lines().enumerate();

    'outer: while let Some(mut pair) = lines.next() {
        'empty_line_eater: loop {
            if pair.1.is_empty() {
                pair = match lines.next() {
                    Some(pair) => pair,
                    None => {
                        break 'outer;
                    }
                };
            } else {
                break 'empty_line_eater;
            }
        }

        // Parse the id
        let (line_num, line) = pair;
        if !line.bytes().all(|b| b.is_ascii_digit()) {
            return Err(Error::invalid_id(line_num));
        }

        // Parse the timestamp and duration
        let (line_num, line) = lines.next().ok_or(Error::missing_ts_line(line_num + 1))?;
        let mut bytes = line.bytes();
        let start = parse_ts(&mut bytes).ok_or(Error::invalid_ts_start(line_num))?;
        parse_ts_divider(&mut bytes).ok_or(Error::invalid_ts_divider(line_num))?;
        let end = parse_ts(&mut bytes).ok_or(Error::invalid_ts_end(line_num))?;
        if end < start {
            return Err(Error::ts_end_before_start(line_num));
        }
        let duration = end - start;

        let mut text = lines
            .next()
            .ok_or(Error::missing_text(line_num + 1))?
            .1
            .to_owned();
        for (_, line) in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            text.push('\n');
            text.push_str(line);
        }

        parsed.push(Subtitle {
            start,
            duration,
            text,
        });
    }

    Ok(parsed)
}
