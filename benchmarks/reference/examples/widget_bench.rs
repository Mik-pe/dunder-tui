#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/widgets_scene.rs"]
mod widgets_scene;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    layout::Rect,
};
use std::io::{self, Write};
use std::time::Instant;
fn main() {
    ratatui::crossterm::style::force_color_output(true);
    let timed = !std::env::args().any(|a| a == "--allocations");
    let terminal = std::env::args().any(|a| a == "--terminal");
    let mut output = io::stdout().lock();
    let mut scenes = (0..8)
        .map(|p| widgets_scene::Scene::new(p, false))
        .collect::<Vec<_>>();
    scenes.push(widgets_scene::Scene::new(0, true));
    for mode in ["idle", "cell", "scroll", "resize", "repaint"] {
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
            let index = if mode == "scroll" {
                n % 8
            } else if mode == "repaint" && n % 2 == 1 {
                8
            } else {
                0
            };
            let started = timed.then(Instant::now);
            if next.area == area {
                next.reset();
            } else {
                next = Buffer::empty(area);
            }
            scenes[index].draw(&mut next, mode == "cell" && n % 2 == 1);
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
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
