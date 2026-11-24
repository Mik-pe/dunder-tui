#[path = "../src/raw_cells.rs"]
mod raw_cells;
use ratatui::{
    buffer::{Buffer, CellDiffOption},
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Fill, Paragraph, Widget, Wrap},
};
use std::io::{self, BufRead};

fn seeded(a: Rect, flags: bool) -> Buffer {
    let mut b = Buffer::empty(a);
    let style = Style::new()
        .cyan()
        .bg(Color::Indexed(81))
        .underline_color(Color::Rgb(0x73, 0x51, 0xce))
        .bold()
        .underlined();
    Fill::new(".").style(style).render(a, &mut b);
    for y in a.top()..a.bottom() {
        for (offset, s, style) in [
            (1, "界", Style::new().red()),
            (4, "❤️", Style::new().italic()),
        ] {
            if offset + 2 <= a.width {
                b.set_string(a.x + offset, y, s, style);
            }
        }
        if flags {
            for (offset, flag) in [(2, CellDiffOption::Skip), (4, CellDiffOption::AlwaysUpdate)] {
                if offset < a.width {
                    b[(a.x + offset, y)].set_diff_option(flag);
                }
            }
        }
    }
    b
}
fn paint(buf: &mut Buffer, area: Rect, step: usize, wrap: Option<Wrap>) {
    let text = match step {
        0 => Text::raw("ab界\n😶‍🌫️\na 秋"),
        1 => Text::from(
            Line::from(vec![Span::raw("x "), Span::raw("🦊").red().bold()]).right_aligned(),
        ),
        2 => Text::default(),
        _ => Text::raw("字\n  👩🏽‍💻 z"),
    };
    let mut p = Paragraph::new(text);
    if let Some(wrap) = wrap {
        p = p.wrap(wrap);
    }
    if step == 2 {
        p = p.style(
            Style::new()
                .on_blue()
                .underline_color(Color::Indexed(13))
                .not_bold(),
        );
    }
    p.render(area, buf);
}
fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let (mut frames, mut cells) = (0, 0);
    for a in [
        Rect::new(0, 0, 8, 4),
        Rect::new(5, 3, 8, 4),
        Rect::new(2, 1, 3, 3),
    ] {
        for left in 0..3 {
            for w in [0, 1, 2, 3, 5, 8] {
                let area = Rect::new(a.x + left, a.y, w.min(a.width - left), a.height);
                for flags in [false, true] {
                    for wrap in [None, Some(Wrap { trim: true }), Some(Wrap { trim: false })] {
                        let mut buf = seeded(a, flags);
                        for step in 0..4 {
                            let old = buf.clone();
                            paint(&mut buf, area, step, wrap);
                            raw_cells::compare(
                                &mut input,
                                &old,
                                &buf,
                                &format!("{area:?} {wrap:?} flags={flags} step={step}"),
                            );
                            frames += 1;
                            cells += a.area();
                        }
                    }
                }
            }
        }
    }
    assert!(input.next().is_none());
    println!("{frames} paragraph overlays, {cells} raw cells and ordered diffs match");
}
