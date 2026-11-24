use ratatui::buffer::{Buffer, CellWidth};
use std::{hint::black_box, time::Instant};
#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;

fn measure(mode: &str, arg: &str) {
    let words = match mode {
        "ascii" => ["server-01...", "client-02..."],
        "unicode" => ["AB茶é....XYZ", "CD海ä::::UVZ"],
        _ => ["لا\r\nカﾞ🙂..", "a\tB\n❤️風."],
    };
    let rows: Vec<String> = (0..24)
        .map(|y| (0..8).map(|x| words[(x + y) % 2]).collect())
        .collect();
    if arg == "--cells" {
        println!(
            "{}",
            serde_json::to_string(&Buffer::with_lines(rows.iter().map(String::as_str))).unwrap()
        );
        return;
    }
    let mut samples = Vec::new();
    let mut checksum = 0;
    let timed = arg.is_empty();
    for i in 0..if arg == "--setup" { 0 } else { 110 } {
        let start = timed.then(Instant::now);
        let b = Buffer::with_lines(black_box(&rows).iter().map(String::as_str));
        checksum += u32::from(b.area.width)
            + u32::from(b.area.height)
            + u32::from(black_box(&b)[(0, i % 24)].cell_width());
        if let Some(t) = start.filter(|_| i >= 10) {
            samples.push(t.elapsed().as_nanos());
        }
    }
    if timed {
        samples.sort_unstable();
        println!("{mode} {checksum} {} {}", samples[49], samples[94]);
    } else {
        println!("{mode} {checksum}");
    }
}
fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();
    for mode in ["ascii", "unicode", "multiline"] {
        measure(mode, &arg);
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
