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
        "minimums" | "crowded" | "overlap" | "weighted" | "cached"
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
    if mode == "weighted" {
        rules[10] = Constraint::Fill(3);
    }
    Layout::init_cache(NonZeroUsize::MIN);
    let grid = Layout::horizontal(rules)
        .spacing(if mode == "overlap" { -1 } else { 1 })
        .horizontal_margin(2)
        .vertical_margin(1);
    let mut samples = Vec::with_capacity(25);
    let mut sum = 0_u64;
    if !setup {
        for i in 0..30 {
            let w = (if mode == "crowded" { 79 } else { 160 })
                + (if mode == "cached" { 0 } else { i % 3 });
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
        println!("{mode} {} {} {sum}", samples[12], samples[23]);
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
