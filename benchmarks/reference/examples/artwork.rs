use ratatui::{
    buffer::{Buffer, CellDiffOption},
    layout::Rect,
    style::{Color, Style},
    widgets::{Fill, MascotEyeColor, RatatuiLogo, RatatuiMascot, Widget},
};
use std::{
    fmt::Write,
    io::{self, BufRead},
};

fn seeded(a: Rect) -> Buffer {
    let mut buf = Buffer::empty(a);
    for y in a.top()..a.bottom() {
        let style = if (y - a.y) % 2 == 0 {
            Style::new()
                .cyan()
                .on_blue()
                .bold()
                .underlined()
                .underline_color(Color::Indexed(81))
        } else {
            Style::new()
                .fg(Color::Rgb(21, 42, 63))
                .bg(Color::Indexed(13))
                .italic()
                .reversed()
        };
        Fill::new(".")
            .style(style)
            .render(Rect::new(a.x, y, a.width, 1), &mut buf);
        for col in [1, 12, 20] {
            if col + 2 <= a.width {
                buf.set_string(
                    a.x + col,
                    y,
                    if col == 12 { "❤️" } else { "界" },
                    Style::new().green(),
                );
            }
        }
        for (col, flag) in [
            (2, CellDiffOption::Skip),
            (4, CellDiffOption::AlwaysUpdate),
            (11, CellDiffOption::ForcedWidth(3.try_into().unwrap())),
        ] {
            if col < a.width {
                buf[(a.x + col, y)].set_diff_option(flag);
            }
        }
    }
    buf
}

fn region(a: Rect, n: usize) -> Rect {
    match n {
        0 => a,
        1 => Rect::new(0, 0, a.x + 14, a.y + 8),
        2 => Rect::new(a.x + 2, a.y + 1, 32, 16),
        3 => Rect::new(a.right(), a.y, 4, 4),
        4 => Rect::new(a.x, a.y, 8, 0),
        _ => Rect::new(
            a.x + 1,
            a.y + 2,
            [0, 1, 14, 15, 26, 27, 31, 32, 33][n - 5],
            [2, 1, 15, 16, 17, 1, 15, 16, 17][n - 5],
        ),
    }
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let (mut frames, mut cells) = (0, 0);
    for a in [
        Rect::new(0, 0, 34, 18),
        Rect::new(6, 4, 34, 18),
        Rect::new(3, 2, 8, 3),
        Rect::new(1, 2, 0, 1),
    ] {
        for n in 0..14 {
            let area = region(a, n);
            let mut buf = seeded(a);
            for step in 0..5 {
                let previous = buf.clone();
                match step {
                    0 => RatatuiLogo::default().render(area, &mut buf),
                    1 => RatatuiLogo::small().render(area, &mut buf),
                    3 => RatatuiMascot::new()
                        .set_eye(MascotEyeColor::Red)
                        .render(area, &mut buf),
                    _ => RatatuiMascot::default().render(area, &mut buf),
                }
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
                            "buffer {a:?} area {area:?} step {step} cell ({x},{y})"
                        );
                        cells += 1;
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
                    "diff {a:?} {area:?} step {step}"
                );
                frames += 1;
            }
        }
    }
    assert!(input.next().is_none());
    println!("{frames} logo and mascot frames, {cells} cells and all ordered diffs match");
}
