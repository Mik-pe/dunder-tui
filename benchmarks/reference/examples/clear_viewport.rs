#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;

use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::Rect;
use ratatui::{Terminal, TerminalOptions, Viewport};
use std::{io, time::Instant};

fn region(mode: &str, offset: u16) -> Rect {
    match mode {
        "bottom" => Rect::new(0, 8 + offset, 96, 16 - offset),
        "rows" => Rect::new(0, 3 + offset, 96, 8),
        "partial" => Rect::new(2 + offset, 3, 91, 8),
        "empty" => Rect::new(0, 3 + offset, 96, 0),
        _ => panic!("unknown viewport"),
    }
}

fn measure<B: Backend>(backend: B, mode: &str, setup: bool) {
    let mut term = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Fixed(region(mode, 0)),
        },
    )
    .unwrap();
    if setup {
        return;
    }
    let mut samples = Vec::with_capacity(100);
    for i in 0..110 {
        let area = region(mode, i % 2);
        let start = Instant::now();
        term.resize(area).unwrap();
        let elapsed = start.elapsed().as_nanos();
        if i >= 10 {
            samples.push(elapsed);
        }
    }
    samples.sort_unstable();
    eprintln!("{mode} {} {}", samples[49], samples[94]);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let setup = args.len() > 3;
    #[cfg(unix)]
    if std::env::var_os("TUI_TERMION").is_some() {
        measure(
            ratatui::backend::TermionBackend::new(io::stdout()),
            &args[2],
            setup,
        );
    } else {
        measure(CrosstermBackend::new(io::stdout()), &args[2], setup);
    }
    #[cfg(not(unix))]
    measure(CrosstermBackend::new(io::stdout()), &args[2], setup);
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
