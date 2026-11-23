use ratatui::{
    buffer::{Buffer, CellDiffOption, CellWidth},
    layout::Rect,
    style::{Color, Modifier, Style},
};
use std::{
    fmt::Write,
    io::{self, BufRead},
    num::NonZeroU16,
};

fn width(n: u16) -> CellDiffOption {
    CellDiffOption::ForcedWidth(NonZeroU16::new(n).unwrap())
}

fn fixture(content: u16, flags: u16) -> Buffer {
    let a = Rect::new(7, 4, 8, 2);
    let mut buf = Buffer::empty(a);
    let x = a.x + 1;
    let y = a.y;
    for (dy, s) in ["abcd1234", "qRSTUvwx"].iter().enumerate() {
        buf.set_string(a.x, y + dy as u16, *s, Style::new());
    }
    let ink = if content % 2 == 0 {
        Style::new()
    } else {
        Style::new().red().on_blue().underlined()
    };
    match content {
        0 => {}
        1 | 2 => {
            buf.set_string(x, y, "界", ink);
        }
        3 | 4 => {
            buf.set_string(x, y, "❤️", ink);
        }
        5 => {
            for col in x..x + 3 {
                buf[(col, y)].reset();
            }
            buf.set_string(x, y, "z", ink);
        }
        6 => {
            buf.set_string(x, y, "é", ink);
        }
        7 => {
            for cell in &mut buf.content {
                cell.set_symbol(" ").set_style(ink);
            }
        }
        8 => {
            buf[(x, y)]
                .set_symbol("[ok]")
                .set_style(ink)
                .set_diff_option(width(4));
        }
        _ => {
            buf.set_string(a.x + 2, y, "界", ink);
            buf.set_string(a.x + 3, y + 1, "☀️", ink);
        }
    }
    use CellDiffOption::{AlwaysUpdate, Skip};
    match flags {
        0 => {}
        1 => {
            buf[(x, y)].set_diff_option(Skip);
        }
        2 => {
            buf[(x, y)].set_diff_option(AlwaysUpdate);
        }
        3..=5 => {
            buf[(x, y)].set_diff_option(width(flags - 2));
        }
        6 => {
            buf[(x + 1, y)].set_diff_option(Skip);
            buf[(x + 2, y)].set_diff_option(AlwaysUpdate);
        }
        7 => {
            buf[(a.x + 6, y)].set_diff_option(width(5));
            buf[(x, y + 1)].set_diff_option(AlwaysUpdate);
        }
        8 => {
            buf[(a.x + 6, y)].set_diff_option(width(65535));
        }
        _ => {
            buf[(x, y)].set_diff_option(Skip);
            buf[(x + 1, y)].set_diff_option(AlwaysUpdate);
        }
    }
    buf
}

#[derive(Default)]
struct Stream<'a> {
    input: &'a str,
    pos: (u16, u16),
    fg: Color,
    bg: Color,
    underlined: bool,
}
impl Stream<'_> {
    fn controls(&mut self) {
        while let Some(body) = self.input.strip_prefix("\x1b[") {
            let end = body.find(|c: char| c.is_ascii_alphabetic()).unwrap();
            let args: Vec<u16> = body[..end].split(';').map(|s| s.parse().unwrap()).collect();
            match body.as_bytes()[end] {
                b'H' => self.pos = (args[1] - 1, args[0] - 1),
                b'm' => {
                    for a in args {
                        match a {
                            0 => {
                                self.fg = Color::Reset;
                                self.bg = Color::Reset;
                                self.underlined = false;
                            }
                            4 => self.underlined = true,
                            24 => self.underlined = false,
                            31 => self.fg = Color::Red,
                            39 => self.fg = Color::Reset,
                            44 => self.bg = Color::Blue,
                            49 => self.bg = Color::Reset,
                            _ => panic!("unexpected SGR {a}"),
                        }
                    }
                }
                c => panic!("unexpected CSI {c}"),
            }
            self.input = &body[end + 1..];
        }
    }
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut count = 0;
    for old in 0..10 {
        for of in 0..10 {
            let prev = fixture(old, of);
            for new in 0..10 {
                for nf in 0..10 {
                    let next = fixture(new, nf);
                    let pos = lines.next().unwrap().unwrap();
                    let ansi = lines.next().unwrap().unwrap();
                    let mut expected = String::new();
                    for (x, y, _) in prev.diff_iter(&next) {
                        write!(expected, "{x},{y} ").unwrap();
                    }
                    assert_eq!(pos, expected, "positions {old}/{of} -> {new}/{nf}");
                    let mut stream = Stream {
                        input: &ansi,
                        ..Default::default()
                    };
                    for (x, y, c) in prev.diff_iter(&next) {
                        stream.controls();
                        assert_eq!(
                            stream.pos,
                            (x, y),
                            "cursor {old}/{of} -> {new}/{nf} {ansi:?}"
                        );
                        assert_eq!(
                            (stream.fg, stream.bg, stream.underlined),
                            (c.fg, c.bg, c.modifier.contains(Modifier::UNDERLINED)),
                            "style {old}/{of} -> {new}/{nf}"
                        );
                        stream.input = stream
                            .input
                            .strip_prefix(c.symbol())
                            .expect("wrong cell symbol");
                        stream.pos.0 += c.symbol().cell_width();
                    }
                    stream.controls();
                    assert!(stream.input.is_empty());
                    assert_eq!(
                        (stream.fg, stream.bg, stream.underlined),
                        (Color::Reset, Color::Reset, false)
                    );
                    count += 1;
                }
            }
        }
    }
    assert!(lines.next().is_none());
    println!("{count} buffer diffs and decoded terminal streams matched Ratatui");
}
