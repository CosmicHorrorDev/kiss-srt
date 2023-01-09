use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kiss_srt::{time::Timestamp, Subtitle};
use oorandom::Rand64;

// Sample text is made up of quotes from _The House of Leaves_ with varying length
const TEXT: &[&str] = &[
    "We all create stories to protect ourselves.",
    "Like patience, passion comes from the Latin root: pati. It",
    " does not mean to flow with exuberance. It means to suffer.",
    "Losing the possibility of something is the exact same thing as",
    " losing hope and without hope nothing can survive",
    "And then the nightmares will begin.",
    "Little solace comes to those who grieve when thoughts keep",
    " drifting as walls keep shifting and this blue world of ours",
    " seems a house of leaves moments before the wind.",
    "No one ever really gets used to nightmares.",
    "Why did god create a dual universe? So he might say 'Be not",
    " like me. I am alone.' And it might be heard.",
    "It might be the wrong decision, but fuck it, it's mine.",
    "Y g g",
    "  d",
    "  r",
    "  s",
    "  i",
    "  l",
    "What miracle is this? This giant tree.",
    " It stands ten thousand feet high",
    " But doesn't reach the ground. Still it stands.",
    " Its roots must hold the sky.",
];

// Generates srt text that looks like
//
// ```srt
// 1
// 00:52:01,482 --> 00:52:03,017
// What miracle is this? This giant tree.
//
// 2
// 01:41:15,468 --> 01:41:20,309
// Why did god create a dual universe? So he might say 'Be not
//
// 3
// 01:25:00,393 --> 01:25:02,559
// Y g g
//   i
//
// 4
// 01:12:46,085 --> 01:12:48,564
// We all create stories to protect ourselves.
//
// 5
// 01:40:45,004 --> 01:40:48,430
//  does not mean to flow with exuberance. It means to suffer.
// Like patience, passion comes from the Latin root: pati. It
// ```
//
// Yes the timestamps aren't increasing, but that doesn't matter for this parser
fn gen_subtitles(num_items: u64) -> String {
    let mut subtitles = Vec::new();
    let mut rng = Rand64::new(0x1234_5678);

    for _ in 0..num_items {
        // Start between 0 and 2 hours, duration between 0 and 10 seconds
        let start_millis =
            rng.rand_range(0..Timestamp::new(2, 0, 0, 1).unwrap().total_millis() as u64);
        let start = Timestamp::from_millis(start_millis as u32);
        let duration = Timestamp::from_millis(rng.rand_range(0..10_001) as u32);

        // 2/3 chance for one line, 1/3 for two
        let num_lines = match rng.rand_range(0..3) {
            0 | 1 => 1,
            2 => 2,
            _ => panic!("Range only includes 0, 1, and 2"),
        };
        let text: String = (0..num_lines)
            .map(|_| TEXT[rng.rand_range(0..TEXT.len() as u64) as usize])
            .collect::<Vec<_>>()
            .join("\n");

        subtitles.push(Subtitle {
            start,
            duration,
            text,
        });
    }

    kiss_srt::to_string(&subtitles)
}

fn parse(c: &mut Criterion) {
    // This winds up being ~125KiB
    let subtitles = gen_subtitles(1_500);

    let mut group = c.benchmark_group("parse throughput");
    group.throughput(criterion::Throughput::Bytes(subtitles.len() as u64));
    group.bench_function("parse", |b| {
        b.iter(|| kiss_srt::from_str(black_box(&subtitles)))
    });
    group.finish();
}

criterion_group!(benches, parse);
criterion_main!(benches);
