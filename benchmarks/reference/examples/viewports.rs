use ratatui::{
    Terminal, TerminalOptions, Viewport,
    backend::{Backend, TestBackend},
    buffer::{Buffer, Cell},
    layout::Rect,
    style::{Color, Modifier, Style},
};
use std::io::{self, BufRead};

fn cells(output: &mut String, b: &Buffer) {
    for cell in &b.content {
        output.push_str(cell.symbol());
        output.push(if cell.fg == Color::Red { 'R' } else { '-' });
        output.push(if cell.modifier.contains(Modifier::BOLD) {
            'B'
        } else {
            '-'
        });
    }
}

fn live(fixed: bool) -> io::Result<()> {
    use ratatui::backend::CrosstermBackend;
    use ratatui::crossterm::{
        cursor::Show,
        event::{self, Event, KeyCode},
        execute, terminal,
    };
    terminal::enable_raw_mode()?;
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            let _ = execute!(io::stdout(), Show);
            let _ = terminal::disable_raw_mode();
        }
    }
    let _restore = Restore;
    let viewport = if fixed {
        Viewport::Fixed(Rect::new(4, 2, 8, 2))
    } else {
        Viewport::Inline(2)
    };
    let mut t = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions { viewport },
    )?;
    let mut count = 0;
    loop {
        t.draw(|f| {
            let a = f.area();
            if fixed {
                f.buffer_mut().set_string(
                    a.x,
                    a.y,
                    if count == 0 { "NORD" } else { "SYD" },
                    Style::default(),
                );
                f.set_cursor_position((a.right() - 1, a.bottom() - 1));
            } else {
                f.buffer_mut()
                    .set_string(0, a.y, "Loggbok", Style::default().fg(Color::Cyan));
                f.buffer_mut()
                    .set_string(0, a.y + 1, format!("{count} rader"), Style::default());
            }
        })?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
            count += 1;
            if !fixed {
                t.insert_before(1, |b| {
                    b.set_string(
                        0,
                        0,
                        format!("Händelse {count}"),
                        Style::default().fg(Color::Magenta),
                    );
                })?;
            }
        }
    }
}

fn main() {
    let windows = std::env::args().nth(1).as_deref() == Some("--window");
    let direct = std::env::args().nth(1).as_deref() == Some("--direct-cursor");
    match std::env::args().nth(1).as_deref() {
        Some("--probe-window") => {
            let mut backend = ratatui::backend::CrosstermBackend::new(io::stdout());
            match backend.window_size() {
                Ok(m) => println!(
                    "{}x{} {}x{}",
                    m.columns_rows.width, m.columns_rows.height, m.pixels.width, m.pixels.height
                ),
                Err(_) => println!("unavailable"),
            }
            return;
        }
        Some("--live-window") => {
            let mut t =
                Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout())).unwrap();
            for step in 0..5 {
                let m = t.backend_mut().window_size().unwrap();
                println!(
                    "\x1b[H{}x{} {}x{} step {step}\x1b[K",
                    m.columns_rows.width, m.columns_rows.height, m.pixels.width, m.pixels.height
                );
                if step < 4 {
                    io::stdin().read_line(&mut String::new()).unwrap();
                }
            }
            return;
        }
        Some("--fixed") => return live(true).unwrap(),
        Some("--inline") => return live(false).unwrap(),
        _ => {}
    }
    let mut records = io::stdin().lock().lines();
    let mut checked = 0;
    for seed in 0..90u16 {
        let (w, h) = (9 + seed % 3, 5 + seed % 4);
        let mut device = TestBackend::new(w, h);
        for y in 0..h {
            let mut cell = Cell::EMPTY;
            cell.set_char(char::from(b'0' + y as u8));
            device.draw((0..w).map(|x| (x, y, &cell))).unwrap();
        }
        device
            .set_cursor_position((seed % w, (seed / 2) % h))
            .unwrap();
        let viewport = if seed % 3 == 0 {
            Viewport::Fixed(Rect::new(2, 1, w - 4, h - 2))
        } else if (windows || direct) && seed % 3 == 1 {
            Viewport::Fullscreen
        } else {
            Viewport::Inline(1 + seed % (h + 3))
        };
        let mut t = Terminal::with_options(device, TerminalOptions { viewport }).unwrap();
        for stage in 0..16 {
            match stage {
                0 => {}
                1 | 6 | 10 if direct => {
                    let pos = ((seed + stage) % w, (2 * seed + stage) % h);
                    t.set_cursor_position(pos).unwrap();
                    if stage == 6 {
                        t.hide_cursor().unwrap();
                    } else {
                        t.show_cursor().unwrap();
                    }
                    assert_eq!(t.get_cursor_position().unwrap(), pos.into());
                }
                2 | 11 if direct => {
                    t.backend_mut().resize(w + stage % 3, h + stage % 2);
                    t.autoresize().unwrap();
                }
                4 if direct => {
                    t.backend_mut().set_cursor_position((2, 1)).unwrap();
                    assert_eq!(t.get_cursor_position().unwrap(), (2, 1).into());
                    t.hide_cursor().unwrap();
                }
                1 | 3 | 5 | 9 if windows => {
                    match stage {
                        1 => t.backend_mut().resize(w + 3, h + 2),
                        5 => t.backend_mut().resize(w - 1, h - 1),
                        9 => t.backend_mut().resize(w, h),
                        _ => {}
                    }
                    t.autoresize().unwrap();
                }
                2 | 4 | 8 | 10 => {
                    let count = (seed + stage) % (2 * h + 3);
                    t.insert_before(count, |b| {
                        for y in 0..count {
                            b.set_string(
                                0,
                                y,
                                format!("L{y}"),
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            );
                        }
                    })
                    .unwrap();
                }
                6 => t.clear().unwrap(),
                13 if seed % 3 == 0 => t.resize(Rect::new(1, 2, w - 3, 2)).unwrap(),
                14 if seed % 3 == 0 => t.resize(Rect::new(3, 2, 3, 2)).unwrap(),
                _ => {
                    if stage == 7 {
                        t.backend_mut().resize(w + 2, h + 1);
                    }
                    if stage == 11 && seed % 3 != 0 {
                        t.backend_mut().resize(w - 2, h - 1);
                    }
                    t.draw(|f| {
                        let a = f.area();
                        for (x, y) in a.positions().map(|p| (p.x, p.y)) {
                            let v = (3 * x + y + stage) % 5;
                            if v > 1 {
                                let cell = &mut f.buffer_mut()[(x, y)];
                                cell.set_char(char::from(b'A' + v as u8));
                                if v == 3 {
                                    cell.set_fg(Color::Red);
                                    cell.modifier = Modifier::BOLD;
                                }
                            }
                        }
                        if stage % 2 == 0 {
                            f.set_cursor_position((a.x + 1, a.y));
                        }
                    })
                    .unwrap();
                }
            }
            if windows && stage > 0 {
                let s = t.size().unwrap();
                let m = t.backend_mut().window_size().unwrap();
                let line = format!(
                    "{} {} {} {} {} {}",
                    s.width,
                    s.height,
                    m.columns_rows.width,
                    m.columns_rows.height,
                    m.pixels.width,
                    m.pixels.height
                );
                assert_eq!(
                    records.next().unwrap().unwrap(),
                    line,
                    "window {seed}/{stage}"
                );
            }
            if direct && stage > 0 {
                let p = t.get_cursor_position().unwrap();
                assert_eq!(records.next().unwrap().unwrap(), format!("{} {}", p.x, p.y));
            }
            let a = t.get_frame().area();
            let p = t.backend_mut().get_cursor_position().unwrap();
            let b = t.backend();
            let mut out = format!(
                "{seed} {stage} {} {} {} {} {} {} {} {} ",
                a.x,
                a.y,
                a.width,
                a.height,
                p.x,
                p.y,
                b.cursor_visible(),
                b.scrollback().area.height
            );
            cells(&mut out, b.buffer());
            cells(&mut out, b.scrollback());
            assert_eq!(
                records.next().unwrap().unwrap(),
                out,
                "seed {seed}, stage {stage}"
            );
            checked += 1;
        }
    }
    assert!(records.next().is_none());
    println!("{checked} viewport states matched Ratatui");
}
