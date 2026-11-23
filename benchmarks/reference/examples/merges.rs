use ratatui::{
    buffer::{Buffer, CellDiffOption},
    layout::Rect,
    style::{Color, Modifier, Style},
};
use std::{
    fmt::Write,
    io::{self, BufRead},
    num::NonZeroU16,
};

fn painted(a: Rect, seed: u16) -> Buffer {
    let mut b = Buffer::empty(a);
    let base = if seed % 2 == 0 {
        Style::new().green().on_black()
    } else {
        Style::new().yellow().on_blue()
    };
    for c in &mut b.content {
        c.set_char((b'a' + seed as u8) as char).set_style(base);
    }
    for y in a.y..a.bottom() {
        let mut x = a.x;
        while x < a.right() {
            let k = (x + 3 * y + seed) % 6;
            let style = Style::new()
                .red()
                .underlined()
                .underline_color(Color::Indexed(130 + k as u8));
            match k {
                0 if x + 1 < a.right() => {
                    b.set_string(x, y, "界", style);
                    x += 2;
                    continue;
                }
                1 => {
                    b.set_string(x, y, "é", style);
                }
                2 => b[(x, y)].reset(),
                3 => {
                    b[(x, y)].set_diff_option(CellDiffOption::Skip);
                }
                4 => {
                    b[(x, y)].set_diff_option(CellDiffOption::AlwaysUpdate);
                }
                5 => {
                    b[(x, y)]
                        .set_diff_option(CellDiffOption::ForcedWidth(NonZeroU16::new(3).unwrap()));
                }
                _ => {}
            }
            x += 1;
        }
    }
    b
}
fn code(c: Color) -> i32 {
    match c {
        Color::Reset => -1,
        Color::Black => 0,
        Color::Red => 1,
        Color::Green => 2,
        Color::Yellow => 3,
        Color::Blue => 4,
        Color::Indexed(n) => i32::from(n),
        _ => panic!("{c:?}"),
    }
}
fn snapshot(b: &Buffer) -> String {
    let a = b.area;
    let mut s = format!("{},{},{},{} ", a.x, a.y, a.width, a.height);
    for c in &b.content {
        let flag = match c.diff_option {
            CellDiffOption::None => 0,
            CellDiffOption::Skip => -1,
            CellDiffOption::AlwaysUpdate => -2,
            CellDiffOption::ForcedWidth(n) => i32::from(n.get()),
        };
        write!(
            s,
            "{}:{},{},{},{},{}|",
            c.symbol(),
            code(c.fg),
            code(c.bg),
            code(c.underline_color),
            c.modifier.contains(Modifier::UNDERLINED),
            flag
        )
        .unwrap();
    }
    s
}
fn main() {
    let mut lines = io::stdin().lock().lines();
    for i in 0..120 {
        for d in 0..12 {
            let mut a = painted(Rect::new(2 + i % 7, 2 + (i / 7) % 5, i % 8, (i / 8) % 6), 0);
            let b = painted(
                Rect::new(
                    1 + (i + 3 * d) % 11,
                    1 + (2 * i + d) % 8,
                    (i + d) % 8,
                    (i + 2 * d) % 5,
                ),
                1,
            );
            let c = painted(Rect::new(4, 3, 2 + d % 5, 3), 2);
            for other in [&b, &c] {
                a.merge(other);
                assert_eq!(
                    lines.next().unwrap().unwrap(),
                    snapshot(&a),
                    "merge {i}/{d} {:?}",
                    other.area
                );
            }
        }
    }
    assert!(lines.next().is_none());
    println!("2880 merged buffers matched in all cells, styles and diff options");
}
