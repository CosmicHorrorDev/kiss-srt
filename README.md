A stupidly simple SRT subtitles parser/renderer

## The Gist

The main entrypoints into this API are [`from_str()`] and [`to_string()`]. Both of those deal
with a sequence of [`Subtitle`]s that can be manipulated to your heart's content

## Quickstart

Here's a simple example that

1. Parses some SRT text
2. Removes the first and last entry
3. Shifts all the timestamps up by 500ms
4. Renders back to SRT text

```rust
const SAMPLE_SRT_TEXT: &str = "\
1
00:00:00,000 --> 00:00:05,250
Will be removed

2
00:00:06,000 --> 00:00:10,000
Will be kept

3
00:00:15,500 --> 00:00:20,750
Will also be removed
";

// 1. Parses some SRT text
let subtitles = kiss_srt::from_str(SAMPLE_SRT_TEXT).unwrap();
// 2. Removes the first and last entry
let mut the_middle = match subtitles.as_slice() {
    [_first, the_middle @ .., _last] => the_middle.to_owned(),
    _ => panic!("Needs at least two entries"),
};

// 3. Shifts all the timestamps up by 500ms
for subtitle in &mut the_middle {
    subtitle.start += kiss_srt::Duration::from_millis(500);
}

// 4. Renders back to SRT text
const PRUNED_AND_SHIFTED: &str = "\
1
00:00:06,500 --> 00:00:10,500
Will be kept
";
assert_eq!(kiss_srt::to_string(&the_middle), PRUNED_AND_SHIFTED);
```

Let's take a moment to appreciate how simple this was

We didn't have to worry about shifting the IDs to make it start at one again (it just gets
inferred from the index), and we didn't have to worry about shifting both the start and end
timestamp because the subtitles use a start and _duration_ instead

There is beauty in simplicity 💕
