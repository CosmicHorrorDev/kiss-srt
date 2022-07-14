use std::fmt::Write;

use crate::Subtitle;

pub fn to_string(subtitles: &[Subtitle]) -> String {
    let mut it = (1..).zip(subtitles.iter());

    it.next()
        .map(|(i, subtitle)| {
            let mut rendered = format!("{}\n{}\n", i, subtitle);

            for (i, subtitle) in it {
                write!(rendered, "\n{}\n{}\n", i, subtitle).expect("OOM ;-;");
            }
            rendered
        })
        .unwrap_or_default()
}
