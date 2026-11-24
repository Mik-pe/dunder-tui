#[path = "../src/raw_cells.rs"]
mod raw_cells;
use ratatui::{
    buffer::{Buffer, CellDiffOption},
    layout::{Offset, Rect},
    style::{Color, Style},
    widgets::{Block, CellEffect, Fill, Shadow, Widget, dimmed},
};
use std::io::{self, BufRead};

fn seeded(a: Rect) -> Buffer {
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

#[derive(Debug)]
struct Mark;
impl CellEffect for Mark {
    fn apply(&self, shade: Rect, base: Rect, buf: &mut Buffer) {
        let at = buf.area.as_position();
        let styled = buf.content.iter().filter(|c| c.fg == Color::Yellow).count();
        let cell = &mut buf[at];
        let value = format!(
            "{}/{styled}/{},{},{},{}/{},{},{},{}",
            cell.symbol(),
            shade.x,
            shade.y,
            shade.width,
            shade.height,
            base.x,
            base.y,
            base.width,
            base.height
        );
        cell.set_symbol(&value)
            .set_diff_option(CellDiffOption::ForcedWidth(1.try_into().unwrap()));
    }
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let mut frames = 0;
    let mut cells = 0;
    for a in [
        Rect::new(0, 0, 9, 5),
        Rect::new(5, 3, 9, 5),
        Rect::new(65526, 65530, 9, 5),
    ] {
        let bases = [
            a,
            Rect::new(a.x, a.y, 2, 2),
            Rect::new(a.x + 3, a.y + 1, 3, 2),
            Rect::new(a.x + 7, a.y + 3, 4, 4),
            Rect::new(a.x + 2, a.y, 0, 3),
        ];
        let prior = seeded(a);
        for base in bases {
            for (dx, dy) in [
                (1, 1),
                (-1, 0),
                (2, -2),
                (0, 0),
                (8, 3),
                (i32::MIN, 1),
                (i32::MAX, i32::MAX),
            ] {
                for mode in 0..9 {
                    for attached in [false, true] {
                        let mut buf = prior.clone();
                        let style = Style::new()
                            .yellow()
                            .underline_color(Color::Indexed(13))
                            .not_bold();
                        let style = match dx {
                            2 => style.bg(Color::Rgb(0xef, 0xfe, 0xfd)).not_dim(),
                            -1 => style.bg(Color::Indexed(17)),
                            _ => style,
                        };
                        let shadow = match mode {
                            0 => Shadow::overlay(),
                            1 => Shadow::custom(dimmed()),
                            2 => Shadow::light_shade(),
                            3 => Shadow::symbol(""),
                            4 => Shadow::symbol("界"),
                            5 => Shadow::symbol("ab"),
                            6 => Shadow::symbol("é"),
                            7 => Shadow::symbol("😶‍🌫️"),
                            _ => Shadow::custom(Mark),
                        }
                        .style(style)
                        .offset(Offset::new(dx, dy));
                        if attached {
                            Block::new().shadow(shadow).render(base, &mut buf);
                        } else {
                            shadow.render(base, &mut buf);
                        }
                        raw_cells::compare(
                            &mut input,
                            &prior,
                            &buf,
                            &format!(
                                "base={base:?} move=({dx},{dy}) mode={mode} attached={attached}"
                            ),
                        );
                        frames += 1;
                        cells += a.area();
                    }
                }
            }
        }
    }
    assert!(input.next().is_none());
    println!("{frames} shadows, {cells} raw cells and ordered diffs match");
}
