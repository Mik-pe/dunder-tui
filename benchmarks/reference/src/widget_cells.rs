use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::Rect,
    style::{Color, Modifier, Stylize},
    widgets::{Fill, Widget},
};
use std::fmt::Write;

pub fn snapshot(buf: &Buffer) -> String {
    let mut out = String::new();
    for y in 0..buf.area.height {
        let mut covered = 0;
        for x in 0..buf.area.width {
            if covered > 0 {
                out.push('~');
                covered -= 1;
                continue;
            }
            let cell = &buf[(x, y)];
            for c in [cell.fg, cell.bg] {
                match c {
                    Color::Indexed(n) => write!(out, "i{n};").unwrap(),
                    Color::Rgb(..) => write!(out, "{c}").unwrap(),
                    basic => {
                        let n = match basic {
                            Color::Reset => 0,
                            Color::Black => 1,
                            Color::Red => 2,
                            Color::Green => 3,
                            Color::Yellow => 4,
                            Color::Blue => 5,
                            Color::Magenta => 6,
                            Color::Cyan => 7,
                            Color::Gray => 8,
                            Color::DarkGray => 9,
                            Color::LightRed => 10,
                            Color::LightGreen => 11,
                            Color::LightYellow => 12,
                            Color::LightBlue => 13,
                            Color::LightMagenta => 14,
                            Color::LightCyan => 15,
                            Color::White => 16,
                            _ => unreachable!(),
                        };
                        out.push(char::from(b'A' + n));
                    }
                }
            }
            let mut flags = 0;
            for (n, m) in [
                Modifier::BOLD,
                Modifier::ITALIC,
                Modifier::REVERSED,
                Modifier::DIM,
            ]
            .iter()
            .enumerate()
            {
                if cell.modifier.contains(*m) {
                    flags |= 1 << n;
                }
            }
            out.push(char::from(65 + flags));
            out.push_str(cell.symbol());
            out.push('|');
            covered = cell.symbol().cell_width().saturating_sub(1);
        }
    }
    out
}
pub fn seeded(area: Rect) -> Buffer {
    let mut buf = Buffer::empty(Rect::new(0, 0, area.width + 2, area.height + 2));
    Fill::new(".")
        .green()
        .on_blue()
        .bold()
        .render(buf.area, &mut buf);
    buf
}
