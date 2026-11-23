#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/chart_scene.rs"]
mod chart_scene;
use ratatui::{
    Terminal,
    backend::{Backend, ClearType, CrosstermBackend, WindowSize},
    buffer::Cell,
    layout::{Position, Size},
};
use std::io::{self, BufRead, Write};
#[allow(dead_code)]
#[path = "../src/widget_cells.rs"]
mod widget_cells;
use std::time::Instant;

struct Output {
    bytes: Vec<u8>,
    size: Size,
    cursor: Position,
}
impl Backend for Output {
    type Error = io::Error;
    fn draw<'a, I>(&mut self, cells: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        CrosstermBackend::new(&mut self.bytes).draw(cells)
    }
    fn hide_cursor(&mut self) -> io::Result<()> {
        CrosstermBackend::new(&mut self.bytes).hide_cursor()
    }
    fn show_cursor(&mut self) -> io::Result<()> {
        CrosstermBackend::new(&mut self.bytes).show_cursor()
    }
    fn get_cursor_position(&mut self) -> io::Result<Position> {
        Ok(self.cursor)
    }
    fn set_cursor_position<P: Into<Position>>(&mut self, p: P) -> io::Result<()> {
        self.cursor = p.into();
        CrosstermBackend::new(&mut self.bytes).set_cursor_position(self.cursor)
    }
    fn clear(&mut self) -> io::Result<()> {
        CrosstermBackend::new(&mut self.bytes).clear()
    }
    fn clear_region(&mut self, kind: ClearType) -> io::Result<()> {
        CrosstermBackend::new(&mut self.bytes).clear_region(kind)
    }
    fn size(&self) -> io::Result<Size> {
        Ok(self.size)
    }
    fn window_size(&mut self) -> io::Result<WindowSize> {
        Ok(WindowSize {
            columns_rows: self.size,
            pixels: Size::ZERO,
        })
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    #[cfg(feature = "terminal-oracle")]
    fn scroll_region_up(&mut self, r: std::ops::Range<u16>, n: u16) -> io::Result<()> {
        CrosstermBackend::new(&mut self.bytes).scroll_region_up(r, n)
    }
    #[cfg(feature = "terminal-oracle")]
    fn scroll_region_down(&mut self, r: std::ops::Range<u16>, n: u16) -> io::Result<()> {
        CrosstermBackend::new(&mut self.bytes).scroll_region_down(r, n)
    }
}
fn main() {
    ratatui::crossterm::style::force_color_output(true);
    let timed = !std::env::args().any(|a| a == "--allocations");
    let physical = std::env::args().any(|a| a == "--terminal");
    let data = chart_scene::Data::new();
    let mut scene = chart_scene::Scene::new(&data);
    if std::env::args().any(|v| v == "--verify") {
        let mut input = io::stdin().lock().lines();
        for mode in ["idle", "cell", "scroll", "resize", "repaint"] {
            let mut t = Terminal::new(ratatui::backend::TestBackend::new(96, 24)).unwrap();
            for n in 0..220 {
                t.backend_mut().resize(
                    if mode == "resize" && n % 2 == 0 {
                        80
                    } else {
                        96
                    },
                    24,
                );
                let count = t
                    .draw(|f| {
                        scene.draw(f.buffer_mut(), mode, n);
                        f.set_cursor_position((0, 23));
                    })
                    .unwrap()
                    .count;
                assert_eq!(count, n);
                assert!(t.backend().cursor_visible());
                assert_eq!(t.backend().cursor_position(), Position::new(0, 23));
                assert_eq!(
                    input.next().unwrap().unwrap(),
                    widget_cells::snapshot(t.backend().buffer()),
                    "{mode} {n}"
                );
            }
        }
        assert!(input.next().is_none());
        println!("1100 completed terminal frames, styles, cursor positions and counts match");
        return;
    }
    let mut out = io::stdout().lock();
    for mode in ["idle", "cell", "scroll", "resize", "repaint"] {
        let mut t = Terminal::new(Output {
            bytes: vec![],
            size: Size::new(96, 24),
            cursor: Position::ORIGIN,
        })
        .unwrap();
        let mut samples = vec![];
        let mut total = 0;
        for n in 0..220 {
            t.backend_mut().size.width = if mode == "resize" && n % 2 == 0 {
                80
            } else {
                96
            };
            let start = timed.then(Instant::now);
            t.backend_mut().bytes.clear();
            t.draw(|f| {
                scene.draw(f.buffer_mut(), mode, n);
                f.set_cursor_position((0, 23));
            })
            .unwrap();
            let bytes = &t.backend().bytes;
            if physical {
                out.write_all(bytes).unwrap();
                out.flush().unwrap();
            }
            if n >= 20 {
                if let Some(start) = start {
                    samples.push(start.elapsed().as_nanos());
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
