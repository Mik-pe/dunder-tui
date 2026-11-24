use ratatui::backend::{Backend, ClearType, CrosstermBackend, TestBackend};
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::{Terminal, TerminalOptions, Viewport};
use std::io::{self, Read};

const MODES: [ClearType; 5] = [
    ClearType::All,
    ClearType::AfterCursor,
    ClearType::BeforeCursor,
    ClearType::CurrentLine,
    ClearType::UntilNewLine,
];

fn backend(w: u16, h: u16) -> TestBackend {
    let mut backend = TestBackend::new(w, h);
    let mut grid = Buffer::empty(Rect::new(0, 0, w, h));
    for Position { x, y } in grid.area.positions() {
        grid[(x, y)]
            .set_char(char::from(b'A' + ((y * w + x) % 26) as u8))
            .set_style(
                Style::new()
                    .fg(Color::Red)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );
    }
    backend
        .draw(
            grid.area
                .positions()
                .map(|Position { x, y }| (x, y, &grid[(x, y)])),
        )
        .unwrap();
    backend
}

fn decorated() -> TestBackend {
    use ratatui::buffer::CellDiffOption;
    let mut grid = Buffer::empty(Rect::new(0, 0, 8, 2));
    let style = Style::new()
        .fg(Color::Red)
        .add_modifier(Modifier::ITALIC | Modifier::UNDERLINED)
        .underline_color(Color::Indexed(127));
    grid.set_string(0, 0, "A界é", style);
    for (x, symbol) in ["🦊", "B", "é", "界", "S", "a", "X", "!"]
        .into_iter()
        .enumerate()
    {
        let cell = &mut grid[(x as u16, 1)];
        cell.set_symbol(symbol).set_style(style);
        match x {
            4 => {
                cell.set_diff_option(CellDiffOption::Skip);
            }
            5 => {
                cell.set_diff_option(CellDiffOption::AlwaysUpdate);
            }
            6 => {
                cell.set_diff_option(CellDiffOption::ForcedWidth(2u16.try_into().unwrap()));
            }
            _ => {}
        }
    }
    let mut b = TestBackend::new(8, 2);
    b.draw(grid.area.positions().map(|p| (p.x, p.y, &grid[p])))
        .unwrap();
    b
}

fn snapshot(backend: &mut TestBackend) {
    let Position { x, y } = backend.get_cursor_position().unwrap();
    println!(
        "{x} {y} {}",
        serde_json::to_string(backend.buffer()).unwrap()
    );
}

fn commands<B: Backend>(backend: &mut B) {
    for (i, mode) in MODES.into_iter().enumerate() {
        backend.set_cursor_position((1 + i as u16, 2)).unwrap();
        backend.clear_region(mode).unwrap();
    }
    backend.flush().unwrap();
}

fn normalize(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec())
        .unwrap()
        .replace("\x1b[0J", "\x1b[J")
        .replace("\x1b[0K", "\x1b[K")
}

fn main() {
    #[cfg(all(unix, feature = "other-erase-backends"))]
    if std::env::args().nth(1).as_deref() == Some("--terminal") {
        if std::env::args().nth(2).as_deref() == Some("termina") {
            let device = ratatui::termina::PlatformTerminal::new().unwrap();
            commands(&mut ratatui::backend::TerminaBackend::new(device));
        } else if std::env::args().nth(2).as_deref() == Some("termwiz") {
            use ratatui::termwiz::{
                caps::Capabilities,
                terminal::{SystemTerminal, buffered::BufferedTerminal},
            };
            let device =
                SystemTerminal::new_from_stdio(Capabilities::new_from_env().unwrap()).unwrap();
            let mut b = ratatui::backend::TermwizBackend::with_buffered_terminal(
                BufferedTerminal::new(device).unwrap(),
            );
            b.clear_region(ClearType::All).unwrap();
            for mode in &MODES[1..] {
                assert_eq!(
                    b.clear_region(*mode).unwrap_err().kind(),
                    io::ErrorKind::Other
                );
            }
            b.flush().unwrap();
        } else {
            panic!("unknown erase backend");
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--termwiz-commands") {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        let output = normalize(&input);
        assert_eq!(output.matches("\x1b[2J").count(), 1);
        for seq in ["\x1b[J", "\x1b[1J", "\x1b[K", "\x1b[2K"] {
            assert!(!output.contains(seq));
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--commands")
        || std::env::args().nth(1).as_deref() == Some("--termina-commands")
    {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        let mut cross = Vec::new();
        commands(&mut CrosstermBackend::new(&mut cross));
        let expected = normalize(&cross);
        #[cfg(unix)]
        {
            let mut termion = Vec::new();
            commands(&mut ratatui::backend::TermionBackend::new(&mut termion));
            assert_eq!(normalize(&termion), expected);
        }
        let output = normalize(&input);
        if std::env::args().nth(1).as_deref() == Some("--termina-commands") {
            // Termina 0.3.3 omits parameter 1 for ED. Its default is 0.
            assert_eq!(output, expected.replace("\x1b[1J", "\x1b[J"));
        } else {
            assert_eq!(output.matches(&expected).count(), 1);
            assert_eq!(
                output.split_once(&expected).unwrap().1,
                "\x1b[0m\x1b[?25h\x1b[?1049l"
            );
        }
        return;
    }
    for w in [1, 5, 8] {
        for h in [1, 4] {
            for y in 0..h {
                for x in 0..w {
                    for mode in MODES {
                        let mut b = backend(w, h);
                        b.set_cursor_position((x, y)).unwrap();
                        b.clear_region(mode).unwrap();
                        snapshot(&mut b);
                    }
                }
            }
        }
    }
    for rect in [
        Rect::new(0, 0, 8, 5),
        Rect::new(0, 1, 8, 4),
        Rect::new(0, 1, 8, 2),
        Rect::new(1, 1, 6, 3),
        Rect::new(0, 4, 8, 1),
        Rect::new(0, 1, 0, 2),
        Rect::new(0, 1, 8, 0),
    ] {
        let mut b = backend(8, 5);
        b.set_cursor_position((3, 2)).unwrap();
        let mut t = Terminal::with_options(
            b,
            TerminalOptions {
                viewport: Viewport::Fixed(rect),
            },
        )
        .unwrap();
        t.clear().unwrap();
        snapshot(t.backend_mut());
        let cell = Cell::from('+').set_fg(Color::Green).clone();
        t.draw(|f| {
            for pos in f.area().positions() {
                f.buffer_mut()[pos] = cell.clone();
            }
        })
        .unwrap();
        snapshot(t.backend_mut());
        let source = backend(8, 5);
        let original = source.buffer();
        t.backend_mut()
            .draw(
                original
                    .area
                    .positions()
                    .map(|Position { x, y }| (x, y, &original[(x, y)])),
            )
            .unwrap();
        t.set_cursor_position((3, 2)).unwrap();
        t.resize(rect).unwrap();
        snapshot(t.backend_mut());
    }
    snapshot(&mut decorated());
    for y in 0..2 {
        for x in 0..8 {
            for kind in MODES {
                let mut b = decorated();
                b.set_cursor_position((x, y)).unwrap();
                b.clear_region(kind).unwrap();
                snapshot(&mut b);
            }
        }
    }
}
