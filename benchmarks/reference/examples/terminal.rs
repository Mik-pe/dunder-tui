use ratatui::backend::{Backend, ClearType, TestBackend};
use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier};
use ratatui::{Frame, Terminal};
use std::fmt::Write;
use std::io::{self, BufRead};

fn draw(f: &mut Frame<'_>, seed: u16, phase: u16) {
    let area = f.area();
    for y in 0..area.height {
        for x in 0..area.width {
            let v = (x + 3 * y + seed + phase) % 7;
            if v == 0 {
                continue;
            }
            let cell = &mut f.buffer_mut()[(x, y)];
            cell.set_char(char::from(b'A' + v as u8));
            if v % 2 == 0 {
                cell.set_fg(Color::Red);
                cell.modifier = Modifier::BOLD;
            } else {
                cell.set_fg(Color::Cyan).set_bg(Color::Blue);
            }
        }
    }
    if phase % 3 != 0 {
        f.set_cursor_position((0, 0));
        f.set_cursor_position((
            (phase + seed) % area.width,
            (phase + 2 * seed) % area.height,
        ));
    }
}

fn color(c: Color) -> char {
    match c {
        Color::Reset => 'A',
        Color::Red => 'C',
        Color::Cyan => 'H',
        Color::Blue => 'F',
        _ => panic!("unexpected color {c:?}"),
    }
}
fn cells(out: &mut String, b: &Buffer) {
    for c in &b.content {
        out.push_str(c.symbol());
        out.push(color(c.fg));
        out.push(color(c.bg));
        out.push(if c.modifier.contains(Modifier::BOLD) {
            'B'
        } else {
            '-'
        });
    }
}

fn live() -> io::Result<()> {
    use ratatui::backend::CrosstermBackend;
    use ratatui::crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::time::Duration;
    terminal::enable_raw_mode()?;
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            let _ = execute!(
                io::stdout(),
                ratatui::crossterm::cursor::Show,
                LeaveAlternateScreen
            );
            let _ = terminal::disable_raw_mode();
        }
    }
    let _restore = Restore;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut t = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut step = 0;
    let mut redraw = true;
    let mut last = (0, 0).into();
    loop {
        let size = t.backend().size()?;
        if redraw || size != last {
            t.draw(|f| {
                let area = f.area();
                f.buffer_mut().set_string(
                    0,
                    0,
                    if step < 3 { "ABC" } else { "XYZ" },
                    ratatui::style::Style::default(),
                );
                match step {
                    0 => f.set_cursor_position((1, 1)),
                    1..=3 => {
                        f.set_cursor_position((0, 0));
                        f.set_cursor_position((7, 2));
                    }
                    5 => f.set_cursor_position((area.width - 1, area.height - 1)),
                    _ => {}
                }
            })?;
            last = size;
            redraw = false;
        }
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
                step += 1;
                redraw = true;
            }
        }
    }
    Ok(())
}

fn main() {
    if std::env::args().any(|arg| arg == "--live") {
        return live().unwrap();
    }
    let mut lines = io::stdin().lock().lines();
    let mut checked = 0;
    for seed in 0_u16..60 {
        let (mut w, mut h) = (3 + seed % 8, 2 + seed % 6);
        let mut t = Terminal::new(TestBackend::new(w, h)).unwrap();
        let mut phase = seed;
        let mut count = t.draw(|f| draw(f, seed, phase)).unwrap().count;
        for step in 0..45 {
            let k = (step * 7 + seed) % 15;
            let first = if step % 2 == 0 { 0 } else { (step + seed) % h };
            let end = first + 1 + (seed + step * 3) % (h - first);
            let n = (seed + step) % (h + 3);
            match k {
                0..=3 => {
                    if k != 3 {
                        phase += 1;
                    }
                    count = t.draw(|f| draw(f, seed, phase)).unwrap().count;
                }
                4 => t.backend_mut().scroll_region_up(first..end, n).unwrap(),
                5 => t.backend_mut().scroll_region_down(first..end, n).unwrap(),
                6 => t.backend_mut().append_lines(n).unwrap(),
                7 => {
                    w = 3 + (seed + step) % 8;
                    h = 2 + (seed + step) % 6;
                    t.backend_mut().resize(w, h);
                    t.backend_mut().set_cursor_position((0, 0)).unwrap();
                }
                8 => {
                    t.backend_mut()
                        .set_cursor_position(((seed + step) % w, (seed + step * 2) % h))
                        .unwrap();
                    t.backend_mut().show_cursor().unwrap();
                }
                9 => t.backend_mut().hide_cursor().unwrap(),
                10 => t
                    .backend_mut()
                    .clear_region(ClearType::CurrentLine)
                    .unwrap(),
                11 => t.backend_mut().clear_region(ClearType::All).unwrap(),
                12 => t
                    .backend_mut()
                    .clear_region(ClearType::BeforeCursor)
                    .unwrap(),
                13 => t
                    .backend_mut()
                    .clear_region(ClearType::AfterCursor)
                    .unwrap(),
                _ => t
                    .backend_mut()
                    .clear_region(ClearType::UntilNewLine)
                    .unwrap(),
            }
            let b = t.backend();
            let pos = b.cursor_position();
            let mut expected = String::new();
            write!(
                &mut expected,
                "{seed} {step} {count} {w} {h} {} {} {} {} ",
                u8::from(b.cursor_visible()),
                pos.x,
                pos.y,
                b.scrollback().area.height
            )
            .unwrap();
            cells(&mut expected, b.buffer());
            cells(&mut expected, b.scrollback());
            let actual = lines.next().expect("missing state").unwrap();
            assert_eq!(actual, expected, "seed {seed}, step {step}, op {k}");
            checked += 1;
        }
    }
    assert!(lines.next().is_none());
    println!("{checked} terminal and scrollback states match Ratatui");
}
