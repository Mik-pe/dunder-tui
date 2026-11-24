#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;

use ratatui::layout::{Constraint, Layout, Rect};
use std::{num::NonZeroUsize, time::Instant};

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap();
    assert!(matches!(
        mode.as_str(),
        "minimums" | "crowded" | "overlap" | "weighted" | "sweep" | "cached"
    ));
    let setup = args.next().is_some_and(|arg| {
        assert_eq!(arg, "setup");
        true
    });
    assert!(args.next().is_none());
    let mut rules: Vec<_> = (0..32)
        .map(|i| match i % 6 {
            0 => Constraint::Length(1 + i % 5),
            1 => Constraint::Percentage(1),
            2 => Constraint::Ratio(1, 96),
            3 => Constraint::Max(2 + i % 4),
            4 => Constraint::Fill(1),
            _ => Constraint::Min(2 + i % 3),
        })
        .collect();
    rules[16] = Constraint::Min(19);
    rules[31] = Constraint::Min(12);
    if mode == "weighted" || mode == "sweep" {
        rules[10] = Constraint::Fill(3);
    }
    Layout::init_cache(NonZeroUsize::MIN);
    let grid = Layout::horizontal(rules)
        .spacing(if mode == "overlap" { -1 } else { 1 })
        .horizontal_margin(2)
        .vertical_margin(1);
    let measured = if mode == "sweep" { 191 } else { 25 };
    let mut samples = Vec::with_capacity(measured);
    let mut sum = 0_u64;
    if !setup {
        for i in 0..measured + 5 {
            let w = match mode.as_str() {
                "sweep" => 35 + (i + 186) % 191,
                "crowded" => 79 + i % 3,
                "cached" => 160,
                _ => 160 + i % 3,
            } as u16;
            let start = Instant::now();
            let (areas, spacers) = grid.split_with_spacers(Rect::new(7, 11, w, 11));
            for r in areas.iter().chain(spacers.iter()) {
                sum += u64::from(r.x + 3 * r.width);
            }
            let elapsed = start.elapsed().as_nanos();
            if i >= 5 {
                samples.push(elapsed);
            }
        }
        samples.sort_unstable();
        println!(
            "{mode} {} {} {sum}",
            samples[measured / 2],
            samples[(measured * 95).div_ceil(100) - 1]
        );
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
