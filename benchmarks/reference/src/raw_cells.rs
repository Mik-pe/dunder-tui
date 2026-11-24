use ratatui::buffer::{Buffer, CellDiffOption};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Fill, Widget},
};
use std::fmt::Write;

#[allow(dead_code)]
pub fn seeded(a: Rect) -> Buffer {
    let mut buf = Buffer::empty(a);
    for y in a.top()..a.bottom() {
        let bg = if (y - a.y) % 2 == 0 {
            Color::Rgb(53, 90, 143)
        } else {
            Color::Indexed(81)
        };
        let style = Style::new()
            .cyan()
            .bg(bg)
            .underline_color(Color::Rgb(0x73, 0x51, 0xce))
            .bold();
        Fill::new(".")
            .style(style)
            .render(Rect::new(a.x, y, a.width, 1), &mut buf);
        buf.set_string(a.x + 1, y, "界", Style::new().red());
        buf.set_string(a.x + 4, y, "❤️", Style::new().italic());
        for (x, flag) in [
            (2, CellDiffOption::Skip),
            (4, CellDiffOption::AlwaysUpdate),
            (7, CellDiffOption::ForcedWidth(2.try_into().unwrap())),
        ] {
            buf[(a.x + x, y)].set_diff_option(flag);
        }
    }
    buf
}

pub fn compare(
    input: &mut impl Iterator<Item = String>,
    previous: &Buffer,
    buf: &Buffer,
    label: &str,
) {
    let a = buf.area;
    for y in a.top()..a.bottom() {
        let line = input.next().expect("missing Dunder row");
        let mut row = line.split_terminator('|');
        for x in a.left()..a.right() {
            let c = &buf[(x, y)];
            let flag = match c.diff_option {
                CellDiffOption::None => 0,
                CellDiffOption::Skip => -1,
                CellDiffOption::AlwaysUpdate => -2,
                CellDiffOption::ForcedWidth(n) => i32::from(n.get()),
            };
            let expected = format!(
                "{}:{}/{}/{}/{}/{}",
                c.symbol(),
                c.fg,
                c.bg,
                c.underline_color,
                c.modifier.bits(),
                flag
            );
            assert_eq!(
                row.next(),
                Some(expected.as_str()),
                "buffer {a:?} area {label} cell ({x},{y})"
            );
        }
        assert!(row.next().is_none());
    }
    let mut coords = String::new();
    for (x, y, _) in previous.diff_iter(&buf) {
        write!(coords, "{x},{y} ").unwrap();
        if x + 1 == a.right() && y + 1 == a.bottom() {
            break;
        }
    }
    assert_eq!(
        input.next().as_deref(),
        Some(coords.as_str()),
        "diff {a:?} {label}"
    );
}
