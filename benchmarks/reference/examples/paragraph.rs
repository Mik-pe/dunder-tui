#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;

use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::buffer::{Buffer, CellWidth};
use ratatui::layout::Rect;
#[path = "../src/paragraph_document.rs"]
mod paragraph_document;
use ratatui::widgets::Widget;
use std::time::Instant;

fn bench(mode: &str, timed: bool) {
    let mut old = Buffer::empty(Rect::new(0, 0, 60, 18));
    let mut next = old.clone();
    let mut doc = paragraph_document::document();
    let mut samples = Vec::new();
    let mut out = Vec::new();
    let mut total = 0;
    for i in 0..220 {
        let columns = if mode == "resize" && i % 2 == 0 {
            44
        } else {
            60
        };
        let top = if mode == "scroll" { i % 48 } else { 0 };
        let area = Rect::new(0, 0, columns, 18);
        let started = timed.then(Instant::now);
        if next.area == area {
            next.reset();
        } else {
            next = Buffer::empty(area);
        }
        doc = doc.scroll((top, 0));
        (&doc).render(area, &mut next);
        out.clear();
        let mut backend = CrosstermBackend::new(&mut out);
        if old.area == area {
            backend.draw(old.diff_iter(&next)).unwrap();
        } else {
            let all = next
                .content
                .chunks_exact(usize::from(columns))
                .enumerate()
                .flat_map(|(y, row)| {
                    let mut skip = 0;
                    row.iter().enumerate().filter_map(move |(x, cell)| {
                        if skip > 0 {
                            skip -= 1;
                            return None;
                        }
                        skip = cell.symbol().cell_width().saturating_sub(1);
                        Some((x as u16, y as u16, cell))
                    })
                });
            backend.draw(all).unwrap();
        }
        std::mem::swap(&mut old, &mut next);
        if i >= 20 {
            if let Some(start) = started {
                samples.push(start.elapsed().as_nanos());
            }
            total += out.len();
        }
    }
    if !timed {
        println!("{total}");
        return;
    }
    samples.sort_unstable();
    println!("{mode} {} {} {}", samples[99], samples[189], total / 200);
}

fn main() {
    ratatui::crossterm::style::force_color_output(true);
    let timed = !std::env::args().any(|arg| arg == "--allocations");
    for mode in ["idle", "scroll", "resize"] {
        bench(mode, timed);
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
