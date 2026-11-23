use ratatui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    layout::Rect,
};
use std::io::{self, Write};
use std::time::Instant;
pub fn run(cases: &[&str], mut draw: impl FnMut(&mut Buffer, &str, usize)) {
    ratatui::crossterm::style::force_color_output(true);
    let timed = !std::env::args().any(|a| a == "--allocations");
    let terminal = std::env::args().any(|a| a == "--terminal");
    let mut output = io::stdout().lock();
    for &mode in cases {
        let mut old = Buffer::empty(Rect::ZERO);
        let mut next = Buffer::empty(Rect::new(0, 0, 96, 24));
        let mut bytes = Vec::new();
        let mut samples = Vec::new();
        let mut total = 0;
        for n in 0..220 {
            let w = if mode == "resize" && n % 2 == 0 {
                80
            } else {
                96
            };
            let area = Rect::new(0, 0, w, 24);
            let started = timed.then(Instant::now);
            if next.area == area {
                next.reset();
            } else {
                next = Buffer::empty(area);
            }
            draw(&mut next, mode, n);
            bytes.clear();
            let mut backend = CrosstermBackend::new(&mut bytes);
            if old.area == area {
                backend.draw(old.diff_iter(&next)).unwrap();
            } else {
                backend
                    .draw(next.content.iter().enumerate().map(|(i, c)| {
                        ((i % usize::from(w)) as u16, (i / usize::from(w)) as u16, c)
                    }))
                    .unwrap();
            }
            if terminal {
                output.write_all(&bytes).unwrap();
                output.flush().unwrap();
            }
            std::mem::swap(&mut old, &mut next);
            if n >= 20 {
                if let Some(t) = started {
                    samples.push(t.elapsed().as_nanos());
                }
                total += bytes.len();
            }
        }
        if timed {
            samples.sort_unstable();
            eprintln!("{mode} {} {} {}", samples[99], samples[189], total / 200);
        } else {
            eprintln!("{total}");
        }
    }
}
