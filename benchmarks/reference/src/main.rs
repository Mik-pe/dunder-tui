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
            #[cfg(feature = "style-oracle")]
            let style = if mode.starts_with("styled-") {
                use ratatui::style::{Color, Modifier};
                let phase = usize::from(mode == "styled-full") * (n % 2);
                Style::new()
                    .fg(Color::Rgb(84, 220, 190))
                    .bg(Color::Rgb(28, 22, 45))
                    .add_modifier(Modifier::ITALIC | Modifier::UNDERLINED)
                    .underline_color(Color::Indexed((32 + (y + phase) % 120) as u8))
            } else {
                Style::default()
            };
            #[cfg(not(feature = "style-oracle"))]
            let style = Style::default();
            let use_b = match mode {
                "scroll" => (y + n) % 5 == 0,
                "full" => n % 2 == 0,
                _ => y % 5 == 0,
            };
            next.set_stringn(0, y as u16, if use_b { B } else { A }, w as usize, style);
        }
        if mode == "cell" {
            next.set_string(60, 20, if n % 2 == 0 { "X" } else { "Y" }, Style::default());
        }
        #[cfg(feature = "style-oracle")]
        if mode == "styled-cell" {
            next.set_style(
                Rect::new(60, 20, 1, 1),
                Style::new().underline_color(ratatui::style::Color::Indexed(200 + (n % 2) as u8)),
            );
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
    ratatui::crossterm::style::force_color_output(true);
    let timed = !std::env::args().any(|arg| arg == "--allocations");
    let modes: &[&str] = if std::env::args().any(|arg| arg == "--styles") {
        assert!(
            cfg!(feature = "style-oracle"),
            "--styles requires --features style-oracle"
        );
        &["styled-idle", "styled-cell", "styled-full"]
    } else {
        &["idle", "cell", "scroll", "resize", "full"]
    };
    for mode in modes {
        bench(mode, timed);
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
