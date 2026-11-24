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
fn emoji_log(buf: &mut ratatui::buffer::Buffer, mode: &str, step: usize) {
    use ratatui::style::{Color, Style};
    use ratatui::widgets::{Fill, Widget};
    let lines = [
        "08.32   😶‍🌫️  Dimma vid kusten. Ny rapport efter nästa mätning.    📡  Station 7  🇸🇪",
        "08.46   👩🏾‍⚕️  Jouren har lämnat kajen. Beräknad ankomst 09.10.    🛟  På väg    ✅",
        "09.10   😶‍🌫  Sikten förbättras i väster. Nästa observation 09.40.    ☀︎  Väst     🧭",
        "09.40   👩🏽‍💻  Mätserie mottagen. Loggen innehåller samtliga värden.  🔬  Klart    1️⃣",
    ];
    let offset = if mode == "scroll" { step } else { 0 };
    let style = if mode == "repaint" && step % 2 == 1 {
        Style::new().fg(Color::Yellow).bg(Color::Blue)
    } else {
        Style::new().fg(Color::Cyan)
    };
    for (y, row) in buf.area.rows().take(23).enumerate() {
        Fill::new(".").style(style).render(row, buf);
        buf.set_stringn(
            0,
            row.y,
            lines[(y + offset) % lines.len()],
            usize::from(row.width),
            style,
        );
    }
    buf.set_string(
        0,
        23,
        if mode == "cell" && step % 2 == 1 {
            "+"
        } else {
            "-"
        },
        Style::new(),
    );
}

fn main() {
    ratatui::crossterm::style::force_color_output(true);
    let timed = !std::env::args().any(|a| a == "--allocations");
    let physical = std::env::args().any(|a| a == "--terminal");
    let data =
        (std::env::var("TUI_BENCH_SCENE").as_deref() != Ok("emoji")).then(chart_scene::Data::new);
    let mut chart = data.as_ref().map(chart_scene::Scene::new);
    let shared = std::env::var("TUI_BENCH_DIFF_OPTIONS").as_deref() == Ok("1");
    let mut draw = |buf: &mut ratatui::buffer::Buffer, mode: &str, n| {
        if let Some(scene) = &mut chart {
            scene.draw(buf, mode, n);
        } else {
            emoji_log(buf, mode, n as usize);
        }
        if shared {
            use ratatui::buffer::CellDiffOption;
            buf.set_string(4, 23, "external    +   >", ratatui::style::Style::new());
            for x in 4..12 {
                buf[(x, 23)].set_diff_option(CellDiffOption::Skip);
            }
            buf[(16, 23)].set_diff_option(CellDiffOption::AlwaysUpdate);
            buf[(20, 23)].set_diff_option(CellDiffOption::ForcedWidth(
                std::num::NonZeroU16::new(4).unwrap(),
            ));
        }
    };
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
                        draw(f.buffer_mut(), mode, n);
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
                draw(f.buffer_mut(), mode, n);
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
