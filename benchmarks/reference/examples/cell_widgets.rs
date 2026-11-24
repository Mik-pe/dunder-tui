use ratatui::{
    buffer::{Buffer, CellDiffOption},
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Clear, Fill, Widget},
};
use std::{
    fmt::Write,
    io::{self, BufRead},
};

fn seeded(a: Rect, flags: bool) -> Buffer {
    let mut buf = Buffer::empty(a);
    Fill::new(".").green().on_blue().bold().render(a, &mut buf);
    for y in a.top()..a.bottom() {
        for (offset, s, style) in [
            (1, "界", Style::new().red()),
            (5, "❤️", Style::new().italic()),
        ] {
            if a.width > offset {
                buf.set_string(a.x + offset, y, s, style);
            }
        }
        if flags && a.width >= 7 {
            buf[(a.x + 2, y)].set_diff_option(CellDiffOption::Skip);
            buf[(a.x + 3, y)].set_diff_option(CellDiffOption::AlwaysUpdate);
            buf[(a.x + 6, y)].set_diff_option(CellDiffOption::ForcedWidth(3.try_into().unwrap()));
        }
    }
    buf
}

fn region(a: Rect, n: usize) -> Rect {
    match n {
        0 => a,
        1 => Rect::new(a.x + 2, a.y, 4, 1),
        2 => Rect::new(0, 0, a.x + 2, a.y + 1),
        3 => Rect::new(a.right() - 1, a.bottom() - 1, 10, 5),
        4 => Rect::new(a.right(), a.y, 3, 3),
        _ => Rect::new(a.x, a.y, 0, 1),
    }
}

fn snapshot(buf: &Buffer) -> String {
    let color = |c| match c {
        Color::Reset => -1,
        Color::Red => 1,
        Color::Green => 2,
        Color::Blue => 4,
        other => panic!("unexpected color {other:?}"),
    };
    let mut out = String::new();
    for cell in &buf.content {
        let bits = u8::from(cell.modifier.contains(Modifier::BOLD))
            + 2 * u8::from(cell.modifier.contains(Modifier::ITALIC));
        let option = match cell.diff_option {
            CellDiffOption::Skip => -1,
            CellDiffOption::AlwaysUpdate => -2,
            CellDiffOption::ForcedWidth(n) => i32::from(n.get()),
            CellDiffOption::None => 0,
        };
        write!(
            out,
            "{}/{}/{bits}/{option}:{}|",
            color(cell.fg),
            color(cell.bg),
            cell.symbol()
        )
        .unwrap();
    }
    out
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--last-cell") {
        let area = Rect::new(0, 0, 1, 1);
        let mut old = Buffer::empty(area);
        Fill::new("xy").on_blue().render(area, &mut old);
        let new = Buffer::empty(area);
        let mut changes = old.diff_iter(&new);
        assert_eq!(changes.next().map(|(x, y, _)| (x, y)), Some((0, 0)));
        assert!(changes.next().is_none());
        return;
    }
    let mut input = io::stdin().lock().lines();
    let mut count = 0;
    for a in [
        Rect::new(0, 0, 8, 3),
        Rect::new(7, 4, 8, 3),
        Rect::new(2, 1, 0, 1),
    ] {
        for flags in [false, true] {
            let initial = seeded(a, flags);
            for symbol in [
                "",
                " ",
                ".",
                "x",
                "é",
                "界",
                "xy",
                "é",
                "لَا",
                "😶‍🌫️",
                "゛",
                "ｶﾞ",
                "\u{301}",
                "👩🏾‍⚕️",
            ] {
                let widget = Fill::new("initial")
                    .symbol(symbol)
                    .red()
                    .italic()
                    .not_bold();
                for n in 0..6 {
                    let mut filled = initial.clone();
                    (&widget).render(region(a, n), &mut filled);
                    let mut overlay = filled.clone();
                    Clear.render(region(a, (n + 1) % 6), &mut overlay);
                    for (step, (prev, curr)) in [
                        (&initial, &filled),
                        (&filled, &overlay),
                        (&overlay, &initial),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        let id = (a, flags, symbol, n, step);
                        assert_eq!(
                            input.next().unwrap().unwrap(),
                            snapshot(curr),
                            "cells {id:?}"
                        );
                        let mut pos = String::new();
                        for (x, y, _) in prev.diff_iter(curr) {
                            write!(pos, "{x},{y} ").unwrap();
                            // The clipped comparison is complete at the last cell. In 0.30.2
                            // advancing again can index past the buffer (see --last-cell).
                            if x + 1 == a.right() && y + 1 == a.bottom() {
                                break;
                            }
                        }
                        assert_eq!(input.next().unwrap().unwrap(), pos, "diff {id:?}");
                        count += 1;
                    }
                }
            }
        }
    }
    assert!(input.next().is_none());
    println!("{count} Fill and Clear buffers and ordered cell diffs match");
}
