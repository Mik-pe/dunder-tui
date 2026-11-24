use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
};
use std::{hint::black_box, time::Instant};

#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;

fn picture(height: u16, offset: usize, dense: bool) -> Buffer {
    let mut image = Buffer::empty(Rect::new(3, 5, 96, height));
    if dense {
        let tiles = ["AB茶é....XYZ", "CD海ä::::UVZ"];
        let inks = [
            Style::new()
                .fg(Color::Indexed(45))
                .add_modifier(Modifier::BOLD),
            Style::new()
                .fg(Color::Rgb(255, 136, 68))
                .bg(Color::Rgb(25, 42, 55))
                .underline_color(Color::Rgb(85, 255, 170))
                .add_modifier(Modifier::ITALIC | Modifier::UNDERLINED),
        ];
        for y in 0..height {
            for x in 0..8 {
                let i = (usize::from(x + y) + offset) % 2;
                image.set_string(3 + x * 12, 5 + y, tiles[i], inks[i]);
            }
        }
        return image;
    }
    let labels = ["│ 水 é", "│ 火 é", "│ 木 é", "│ 金 é"];
    for y in 0..height {
        image.set_string(
            3,
            5 + y,
            labels[(usize::from(y) + offset) % 4],
            Color::Green,
        );
        image.set_string(87, 5 + y, "STATUS", Style::new());
    }
    image
}

fn measure(mode: &str, timed: bool, count: usize, check: bool) {
    let dense = mode.starts_with("dense-");
    let old = picture(if mode == "resize" { 20 } else { 24 }, 0, dense);
    let mut next = picture(
        24,
        usize::from(matches!(mode, "scroll" | "dense-scroll")),
        dense,
    );
    if matches!(mode, "one" | "resize" | "dense-one") {
        next[(97, 24)].set_char('!');
    }
    if matches!(mode, "repaint" | "dense-repaint") {
        next.set_style(next.area, Style::new().fg(Color::Yellow).bg(Color::Blue));
    }
    if check {
        println!("{mode}");
        println!("{}", serde_json::to_string(&old).unwrap());
        println!("{}", serde_json::to_string(&next).unwrap());
        for (x, y, cell) in old.diff_iter(&next) {
            println!("{x} {y} {}", serde_json::to_string(cell).unwrap());
        }
        return;
    }
    let mut samples = Vec::new();
    let mut sum = 0i64;
    let mut emitted = 0;
    for n in 0..count {
        let start = timed.then(Instant::now);
        emitted = 0;
        let mut value = 0;
        for (x, y, cell) in black_box(&old).diff_iter(black_box(&next)) {
            emitted += 1;
            value += i64::from(y) * 256 + i64::from(x) + i64::from(cell.modifier.bits());
        }
        if let Some(t) = start.filter(|_| n >= 10) {
            samples.push(t.elapsed().as_nanos());
        }
        sum += black_box(value);
    }
    if timed {
        samples.sort_unstable();
        println!("{mode} {emitted} {sum} {} {}", samples[49], samples[94]);
    } else {
        println!("{mode} {emitted} {sum}");
    }
}

fn main() {
    let arg = std::env::args().nth(1);
    let count = if arg.as_deref() == Some("--setup") {
        0
    } else {
        110
    };
    for mode in [
        "idle",
        "one",
        "scroll",
        "resize",
        "repaint",
        "dense-idle",
        "dense-one",
        "dense-scroll",
        "dense-repaint",
    ] {
        measure(
            mode,
            arg.is_none(),
            count,
            arg.as_deref() == Some("--cells"),
        );
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
