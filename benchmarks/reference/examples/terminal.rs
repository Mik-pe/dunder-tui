use ratatui::backend::{Backend, ClearType, TestBackend};
use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier, Style};
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

fn direct_live(fixed: bool, manual: bool) -> io::Result<()> {
    use ratatui::backend::CrosstermBackend;
    use ratatui::crossterm::{cursor, event, execute, terminal};
    use ratatui::layout::Rect;
    use ratatui::{TerminalOptions, Viewport};
    terminal::enable_raw_mode()?;
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            let _ = execute!(io::stdout(), cursor::Show);
            if self.0 {
                let _ = execute!(io::stdout(), terminal::LeaveAlternateScreen);
            }
            let _ = terminal::disable_raw_mode();
        }
    }
    let _restore = Restore(!fixed);
    if !fixed {
        execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    }
    let viewport = if fixed {
        Viewport::Fixed(if manual {
            Rect::new(4, 2, 8, 3)
        } else {
            Rect::new(0, 0, 20, 7)
        })
    } else {
        Viewport::Fullscreen
    };
    let mut t = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions { viewport },
    )?;
    if manual {
        let a = t.get_frame().area();
        for step in 0..10 {
            match step {
                0 => {
                    t.set_cursor_position((2, 1))?;
                    t.show_cursor()?;
                }
                1 => {
                    let mut f = t.get_frame();
                    f.buffer_mut().set_string(a.x, a.y, "ED", Style::default());
                    f.set_cursor_position((9, 4));
                }
                2 => t.flush()?,
                3 | 9 => t.swap_buffers(),
                4 => {
                    t.current_buffer_mut()
                        .set_string(a.x, a.y, "LO", Style::default());
                }
                5 => {
                    t.apply_buffer()?;
                }
                6 => {
                    t.current_buffer_mut()
                        .set_string(a.x, a.y, "UP", Style::default());
                    t.apply_buffer_with_cursor(Some((2, 4).into()))?;
                }
                7 => {
                    t.current_buffer_mut()
                        .set_string(a.x, a.y, "NO", Style::default());
                    assert!(
                        t.try_draw(|f| -> io::Result<()> {
                            f.buffer_mut()[(a.x + 2, a.y)].set_char('x');
                            Err(io::Error::other("reject"))
                        })
                        .is_err()
                    );
                }
                _ => {
                    t.apply_buffer()?;
                }
            }
            t.backend_mut().flush()?;
            t.get_cursor_position()?;
        }
        return Ok(());
    }
    for step in 0..13 {
        match step {
            0 | 4 | 7 | 8 | 10 => {
                t.draw(|f| {
                    f.buffer_mut()[(0, 0)].set_char('D');
                    if step < 8 {
                        f.set_cursor_position((2, 1));
                    }
                })?;
            }
            1 => t.hide_cursor()?,
            2 => t.set_cursor_position((5, 3))?,
            3 | 9 | 12 => t.show_cursor()?,
            5 => {
                t.backend_mut().set_cursor_position((6, 2))?;
                t.backend_mut().hide_cursor()?;
            }
            6 => {
                assert!(
                    t.try_draw(|f| -> io::Result<()> {
                        f.buffer_mut()[(0, 0)].set_char('B');
                        f.set_cursor_position((7, 0));
                        Err(io::Error::other("reject"))
                    })
                    .is_err()
                );
            }
            11 => t.set_cursor_position((8, 5))?,
            _ => unreachable!(),
        }
        let expected = match step {
            2 | 3 => (5, 3),
            5 | 6 => (6, 2),
            11 | 12 => (8, 5),
            _ => (2, 1),
        };
        assert_eq!(t.get_cursor_position()?, expected.into());
    }
    assert!(matches!(event::read()?, event::Event::Key(k) if k.code == event::KeyCode::Char('k')));
    Ok(())
}

fn final_flush_error() -> io::Result<()> {
    use ratatui::backend::CrosstermBackend;
    use ratatui::layout::Rect;
    use ratatui::{TerminalOptions, Viewport};
    struct Writer<'a> {
        dirty: bool,
        fail: &'a std::cell::Cell<bool>,
    }
    impl io::Write for Writer<'_> {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.dirty = true;
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            if self.fail.get() && !self.dirty {
                self.fail.set(false);
                return Err(io::Error::other("backend flush"));
            }
            self.dirty = false;
            Ok(())
        }
    }
    let fail = std::cell::Cell::new(false);
    let backend = CrosstermBackend::new(Writer {
        dirty: false,
        fail: &fail,
    });
    let mut t = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Fixed(Rect::new(0, 0, 4, 2)),
        },
    )?;
    t.current_buffer_mut()
        .set_string(0, 0, "SZ", Style::default());
    assert_eq!(t.apply_buffer()?.count, 0);
    t.current_buffer_mut()
        .set_string(0, 0, "keep", Style::default());
    fail.set(true);
    assert_eq!(t.apply_buffer().unwrap_err().to_string(), "backend flush");
    assert_eq!(t.get_frame().count(), 1);
    assert_eq!(
        t.current_buffer_mut(),
        &Buffer::empty(Rect::new(0, 0, 4, 2))
    );
    t.current_buffer_mut()
        .set_string(0, 0, "next", Style::default());
    assert_eq!(t.apply_buffer()?.count, 1);
    Ok(())
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("--final-flush-error") => return final_flush_error().unwrap(),
        Some("--manual") => return direct_live(false, true).unwrap(),
        Some("--manual-fixed") => return direct_live(true, true).unwrap(),
        Some("--direct-live") => return direct_live(false, false).unwrap(),
        Some("--direct-fixed") => return direct_live(true, false).unwrap(),
        _ => {}
    }
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
