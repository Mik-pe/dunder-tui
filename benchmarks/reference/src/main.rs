#[cfg(feature = "allocation-counts")]
mod allocation;

use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use std::time::Instant;

const A: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ--0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRST";
const B: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz++ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrst";

fn bench(mode: &str, timed: bool) {
    let mut old = Buffer::empty(Rect::new(0, 0, 120, 40));
    let mut next = old.clone();
    let mut out = Vec::new();
    let mut samples = Vec::new();
    let mut bytes = 0;
    for n in 0..220 {
        let w = if mode == "resize" && n % 2 == 0 {
            100
        } else {
            120
        };
        let area = Rect::new(0, 0, w, 40);
        let start = timed.then(Instant::now);
        if next.area != area {
            next = Buffer::empty(area);
        } else {
            next.reset();
        }
        for y in 0..40 {
            let use_b = match mode {
                "scroll" => (y + n) % 5 == 0,
                "full" => n % 2 == 0,
                _ => y % 5 == 0,
            };
            next.set_stringn(
                0,
                y,
                if use_b { B } else { A },
                w as usize,
                Style::default(),
            );
        }
        if mode == "cell" {
            next.set_string(60, 20, if n % 2 == 0 { "X" } else { "Y" }, Style::default());
        }
        out.clear();
        let mut backend = CrosstermBackend::new(&mut out);
        if old.area == area {
            backend.draw(old.diff_iter(&next)).unwrap();
        } else {
            backend
                .draw(
                    next.content
                        .iter()
                        .enumerate()
                        .map(|(i, c)| ((i % w as usize) as u16, (i / w as usize) as u16, c)),
                )
                .unwrap();
        }
        std::mem::swap(&mut next, &mut old);
        if n >= 20 {
            if let Some(start) = start {
                samples.push(start.elapsed().as_nanos());
            }
            bytes += out.len();
        }
    }
    if !timed {
        println!("{bytes}");
        return;
    }
    samples.sort_unstable();
    println!("{mode} {} {} {}", samples[99], samples[189], bytes / 200);
}

fn main() {
    let timed = std::env::args().nth(1).as_deref() != Some("--allocations");
    for mode in ["idle", "cell", "scroll", "resize", "full"] {
        bench(mode, timed);
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
