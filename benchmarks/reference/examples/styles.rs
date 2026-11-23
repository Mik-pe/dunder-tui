use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::palette::{Hsl, Hsluv};
use ratatui::style::{Color, Modifier, Style};
use std::io::{self, Read};

const COLORS: [Color; 7] = [
    Color::LightBlue,
    Color::Indexed(4),
    Color::Rgb(0, 0, 0),
    Color::Reset,
    Color::White,
    Color::Indexed(231),
    Color::Rgb(255, 128, 2),
];

fn reference() -> Buffer {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 32, 32));
    for i in 0..1024 {
        let x = (i % 32) as u16;
        let y = (i / 32) as u16;
        let style = Style::new()
            .fg(COLORS[i % 7])
            .bg(COLORS[(i / 3) % 7])
            .underline_color(COLORS[(i / 5) % 7])
            .add_modifier(Modifier::from_bits_retain(((i * 73) & 511) as u16));
        buffer.set_string(x, y, "x", style);
        if i >= 512 {
            let overlay = if i % 4 == 0 {
                Style::reset().add_modifier(Modifier::ITALIC)
            } else {
                Style::new()
                    .remove_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::DIM)
                    .fg(Color::Indexed(0))
            };
            buffer.set_style(Rect::new(x, y, 1, 1), overlay);
        }
    }
    buffer
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Attributes {
    colors: [Color; 3],
    modifiers: u16,
}

impl Attributes {
    fn sgr(&mut self, args: &[u16]) {
        let mut args = args.iter().copied();
        while let Some(code) = args.next() {
            match code {
                0 => *self = Self::default(),
                1..=9 => self.modifiers |= 1 << (code - 1),
                22 => self.modifiers &= !3,
                25 => self.modifiers &= !48,
                23 | 24 | 27..=29 => self.modifiers &= !(1 << (code - 21)),
                30..=37 | 90..=97 => {
                    self.colors[0] = Color::Indexed((code % 10 + 8 * u16::from(code >= 90)) as u8)
                }
                40..=47 | 100..=107 => {
                    self.colors[1] = Color::Indexed((code % 10 + 8 * u16::from(code >= 100)) as u8)
                }
                39 | 49 | 59 => self.colors[((code - 39) / 10) as usize] = Color::Reset,
                38 | 48 | 58 => {
                    let mut byte = || u8::try_from(args.next().unwrap()).unwrap();
                    self.colors[((code - 38) / 10) as usize] = match byte() {
                        2 => Color::Rgb(byte(), byte(), byte()),
                        5 => Color::Indexed(byte()),
                        mode => panic!("unexpected color mode {mode}"),
                    };
                }
                _ => panic!("unexpected SGR {code}"),
            }
        }
    }
}

fn decode(stream: &[u8]) -> Vec<Attributes> {
    let mut at = 0;
    let mut index = 0;
    let mut screen = vec![None; 1024];
    let mut state = Attributes::default();
    while at < stream.len() {
        if stream[at] == b'x' {
            assert!(
                screen[index].replace(state).is_none(),
                "duplicate cell {index}"
            );
            index += 1;
            at += 1;
            continue;
        }
        assert_eq!(&stream[at..at + 2], b"\x1b[");
        let body = at + 2;
        at = body;
        while stream[at].is_ascii_digit() || stream[at] == b';' {
            at += 1;
        }
        let raw = std::str::from_utf8(&stream[body..at]).unwrap();
        let args: Vec<u16> = raw
            .split(';')
            .map(|v| if v.is_empty() { 0 } else { v.parse().unwrap() })
            .collect();
        match stream[at] {
            b'm' => state.sgr(&args),
            b'H' => index = ((args[0] - 1) * 32 + args[1] - 1) as usize,
            end => panic!("unexpected CSI final byte {end}"),
        }
        at += 1;
    }
    assert_eq!(
        state,
        Attributes::default(),
        "terminal attributes after draw"
    );
    screen.into_iter().map(Option::unwrap).collect()
}

fn terminal_color(color: Color) -> Color {
    match color {
        Color::LightBlue => Color::Indexed(12),
        Color::White => Color::Indexed(15),
        _ => color,
    }
}

fn main() {
    ratatui::crossterm::style::force_color_output(true);
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let actual = decode(lines.next().expect("dunder-tui output").as_bytes());
    let buffer = reference();
    let mut output = vec![];
    CrosstermBackend::new(&mut output)
        .draw(Buffer::empty(buffer.area).diff_iter(&buffer))
        .unwrap();
    let crossterm = decode(&output);
    for (i, cell) in buffer.content.iter().enumerate() {
        let expected = Attributes {
            colors: [cell.fg, cell.bg, cell.underline_color].map(terminal_color),
            modifiers: cell.modifier.bits(),
        };
        assert_eq!(actual[i], expected, "Dunder cell {i}");
        assert_eq!(crossterm[i], expected, "Crossterm cell {i}");
    }
    for i in 0..512 {
        let h = ((i * 113 % 1081) as f32 - 540.0) + 0.375;
        let s = (i * 79 % 130) as f32 - 10.0;
        let l = (i * 61 % 132) as f32 - 14.0;
        let hsl = Color::from_hsl(Hsl::new(h, s / 100.0, l / 100.0));
        let hsluv = Color::from_hsluv(Hsluv::new(h, s, l));
        assert_eq!(
            lines.next().unwrap(),
            format!("{hsl} {hsluv}"),
            "palette case {i} ({h}, {s}, {l})"
        );
    }
    assert!(lines.next().is_none());
    println!(
        "1024 ANSI cells match both the Ratatui buffer and Crossterm output; 1024 palette conversions match"
    );
}
