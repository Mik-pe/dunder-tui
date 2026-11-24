use ratatui::{buffer::Buffer, layout::Rect, style::Color};
use std::time::Instant;

#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;

fn paint(b: &mut Buffer, area: Rect, phase: u16, mode: &str, palette: &[Color; 4]) {
    let mut tile = |x, y| {
        b[(x, y)].set_fg(palette[usize::from((x + 3 * y + phase) & 3)]);
    };
    if mode == "for" {
        for p in area.positions() {
            tile(p.x, p.y);
        }
    } else {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                tile(x, y);
            }
        }
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap();
    assert!(matches!(mode.as_str(), "for" | "nested"));
    let arg = args.next().unwrap_or_default();
    let area = Rect::new(7, 11, 96, 24);
    let palette = [Color::Cyan, Color::Magenta, Color::Yellow, Color::Green];
    let mut b = Buffer::empty(area);
    if arg == "--cells" {
        for phase in 0..4 {
            paint(&mut b, area, phase, &mode, &palette);
            println!("{}", serde_json::to_string(&b).unwrap());
        }
        return;
    }
    let mut samples = Vec::new();
    let mut checksum = 0;
    for phase in 0..if arg == "--setup" { 0 } else { 110 } {
        let t = arg.is_empty().then(Instant::now);
        paint(std::hint::black_box(&mut b), area, phase, &mode, &palette);
        if let Some(t) = t
            && phase >= 10
        {
            samples.push(t.elapsed().as_nanos());
        }
        checksum += match b[(area.x + phase % area.width, area.y)].fg {
            Color::Cyan => 6,
            Color::Magenta => 5,
            Color::Yellow => 3,
            Color::Green => 2,
            c => panic!("unexpected color {c}"),
        };
    }
    if arg.is_empty() {
        samples.sort_unstable();
        println!("{checksum} {} {}", samples[49], samples[94]);
    } else {
        println!("{checksum}");
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
