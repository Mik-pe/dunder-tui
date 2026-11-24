#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/frames.rs"]
mod frames;
#[path = "../src/widget_cells.rs"]
mod widget_cells;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
};
use std::{num::NonZeroUsize, time::Instant};

fn bands() -> Layout {
    Layout::horizontal([Constraint::Length(8); 12])
        .flex(Flex::SpaceAround)
        .spacing(2)
}

fn draw(grid: &Layout, buf: &mut Buffer, mode: &str, n: usize) {
    let view = buf.area;
    let regions = grid.split(view);
    let shift = if mode == "scroll" { n % 12 } else { 0 };
    if mode == "repaint" {
        buf.set_style(
            view,
            Style::new().bg(if n % 2 == 0 {
                Color::Black
            } else {
                Color::Blue
            }),
        );
    }
    let colors = [Color::Cyan, Color::Green, Color::Yellow, Color::Magenta];
    for (i, pane) in regions.iter().enumerate() {
        let channel = (i + shift) % 12;
        let height = 2 + (channel * 7 % 18) as u16;
        for y in view.height - 2 - height..view.height - 2 {
            for x in pane.x..pane.right() {
                buf[(x, y)].set_symbol("=").set_fg(colors[channel % 4]);
            }
        }
    }
    buf[(0, 0)].set_symbol(if mode == "cell" && n % 2 == 1 {
        "+"
    } else {
        "-"
    });
    buf.set_string(0, view.height - 1, "FREQUENCY", Style::default());
}

fn cold() {
    Layout::init_cache(NonZeroUsize::new(1).unwrap());
    let grid = bands();
    let mut samples = Vec::new();
    let mut sum = 0u64;
    for batch in 0..30 {
        let start = Instant::now();
        for j in 0..4 {
            let area = Rect::new(0, 0, 80 + (4 * batch + j) % 32, 24);
            let (parts, spacers) = grid.split_with_spacers(area);
            for r in parts.iter().chain(spacers.iter()) {
                sum += u64::from(r.x + 3 * r.width);
            }
        }
        if batch >= 5 {
            samples.push(start.elapsed().as_nanos() / 4);
        }
    }
    samples.sort_unstable();
    println!("{} {} {sum}", samples[12], samples[23]);
}

fn main() {
    let modes = ["idle", "cell", "scroll", "resize", "repaint"];
    let grid = bands();
    match std::env::args().nth(1).as_deref() {
        Some("--cold") => return cold(),
        Some("--check") => {
            for mode in modes {
                for n in 0..220 {
                    let mut buf = Buffer::empty(Rect::new(
                        0,
                        0,
                        if mode == "resize" && n % 2 == 0 {
                            80
                        } else {
                            96
                        },
                        24,
                    ));
                    draw(&grid, &mut buf, mode, n);
                    println!("{}", widget_cells::snapshot(&buf));
                }
            }
            return;
        }
        _ => {}
    }
    frames::run(&modes, |buf, mode, n| draw(&grid, buf, mode, n));
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
