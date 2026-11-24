use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use std::hint::black_box;
use std::time::Instant;

#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;

fn main() {
    let mode = std::env::args().nth(1).unwrap();
    let mut spans = Vec::new();
    for _ in 0..8 {
        spans.push(Span::raw("0123456789abcdef"));
        spans.push(Span::styled(" e\u{301} 秋 👩🏽‍💻! ", Style::new().italic()));
        spans.push(Span::raw("\r\n🇸🇪\0لا\u{85}"));
        spans.push(Span::styled(" fin", Style::new().green().not_italic()));
    }
    let row = Line::from(spans).style(Style::new().bold());
    let scan = || {
        black_box(&row)
            .styled_graphemes(Style::new())
            .map(|g| {
                g.symbol.len()
                    + usize::from(g.symbol.as_bytes()[0])
                    + usize::from(g.style.add_modifier.contains(Modifier::ITALIC))
            })
            .sum::<usize>()
    };
    let mut sum = 0;
    if mode == "timing" {
        let mut samples = Vec::new();
        for n in 0..220 {
            let start = Instant::now();
            for _ in 0..16 {
                sum += scan();
            }
            let ns = start.elapsed().as_nanos() / 16;
            if n >= 20 {
                samples.push(ns);
            }
        }
        samples.sort_unstable();
        assert_eq!(sum, 82_874_880);
        println!("ratatui {} {} {sum}", samples[99], samples[189]);
    } else {
        match mode.as_str() {
            "count" => {
                for _ in 0..100 {
                    sum += scan();
                }
            }
            "setup" => {}
            _ => panic!("unknown mode"),
        }
        assert_eq!(sum, if mode == "count" { 2_354_400 } else { 0 });
        println!("{sum}");
        #[cfg(feature = "allocation-counts")]
        allocation::report();
    }
}
