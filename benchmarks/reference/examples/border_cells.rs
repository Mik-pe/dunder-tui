#[path = "../src/raw_cells.rs"]
mod raw_cells;
use ratatui::{
    buffer::{Buffer, CellDiffOption},
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols::{border, merge::MergeStrategy},
    text::Line,
    widgets::{Block, BorderType, Borders, Widget},
};
use std::io::{self, BufRead};

fn seed(a: Rect, filled: bool) -> Buffer {
    if !filled {
        return Buffer::empty(a);
    }
    let mut buf = raw_cells::seeded(a);
    let y = a.y + 2;
    buf[(a.x, y)].set_symbol(" ");
    buf[(a.x + 3, y)]
        .set_symbol("─")
        .set_diff_option(CellDiffOption::ForcedWidth(1.try_into().unwrap()));
    buf[(a.x + 6, y)].set_symbol("─a");
    buf
}

fn draw(buf: &mut Buffer, area: Rect, sides: Borders, strategy: MergeStrategy, stage: usize) {
    let mut b = Block::new()
        .borders(sides)
        .merge_borders(strategy)
        .border_style(
            Style::new()
                .yellow()
                .not_bold()
                .underline_color(Color::Indexed(13)),
        );
    if stage != 0 {
        b = b.border_set(border::Set {
            top_left: "",
            top_right: "ab",
            bottom_left: "é",
            bottom_right: "😶‍🌫️",
            vertical_left: "界",
            vertical_right: "╭",
            horizontal_top: "─",
            horizontal_bottom: "─a",
        });
    }
    if stage == 2 {
        b = b.border_type(BorderType::Rounded);
    }
    if stage >= 2 {
        b = b.style(Style::new().bg(Color::Rgb(0x15, 0x34, 0x44)));
    }
    if stage == 3 {
        b = b
            .title_top(Line::from("界T").alignment(Alignment::Center))
            .title_bottom("é")
            .title_style(Style::new().red().italic());
    }
    b.render(area, buf);
}

// A wide title can reset a cell past the last column in Ratatui 0.30.2.
fn draw_with_guard(
    previous: &Buffer,
    area: Rect,
    sides: Borders,
    strategy: MergeStrategy,
    stage: usize,
) -> Buffer {
    let a = previous.area;
    let mut padded = Buffer::empty(Rect::new(0, 0, a.width + 1, a.height));
    for y in 0..a.height {
        for x in 0..a.width {
            padded[(x, y)] = previous[(a.x + x, a.y + y)].clone();
        }
    }
    let clip = area.intersection(a);
    draw(
        &mut padded,
        Rect::new(clip.x - a.x, clip.y - a.y, clip.width, clip.height),
        sides,
        strategy,
        stage,
    );
    let mut out = previous.clone();
    for y in 0..a.height {
        for x in 0..a.width {
            out[(a.x + x, a.y + y)] = padded[(x, y)].clone();
        }
    }
    out
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    if std::env::args().nth(1).as_deref() == Some("--merge") {
        let values = [
            "",
            " ",
            "a",
            "─a",
            "é",
            "́",
            "界",
            "☀",
            "あ",
            "🦊",
            "😶‍🌫️",
            "─",
            "│",
            "╭",
            "╲",
            "═",
            "╂",
            "┈",
            "╹",
            "╋",
            "┿",
        ];
        for rule in [
            MergeStrategy::Replace,
            MergeStrategy::Exact,
            MergeStrategy::Fuzzy,
        ] {
            for before in values {
                for after in values {
                    assert_eq!(
                        input.next().as_deref(),
                        Some(rule.merge(before, after)),
                        "{before:?} {after:?} {rule:?}"
                    );
                }
            }
        }
        assert!(input.next().is_none());
        println!("{} string merges match", 3 * values.len() * values.len());
        return;
    }
    let mut frames = 0;
    let mut overflows = 0;
    let report = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !info.payload().downcast_ref::<String>().is_some_and(|msg| {
            msg.starts_with("index outside of buffer:") && msg.ends_with("index is (65535, 65533)")
        }) {
            report(info);
        }
    }));
    for a in [
        Rect::new(0, 0, 9, 5),
        Rect::new(5, 3, 9, 5),
        Rect::new(65526, 65530, 9, 5),
    ] {
        for (dx, dy, w, h) in [
            (0, 0, 9, 5),
            (0, 0, 1, 1),
            (2, 1, 6, 3),
            (1, 1, 1, 1),
            (2, 0, 1, 5),
            (0, 2, 9, 1),
            (7, 3, 6, 4),
            (2, 2, 0, 1),
        ] {
            let area = Rect::new(a.x + dx, a.y + dy, w, h);
            for mask in 0..16 {
                let mut sides = Borders::NONE;
                for (i, side) in [Borders::LEFT, Borders::RIGHT, Borders::TOP, Borders::BOTTOM]
                    .into_iter()
                    .enumerate()
                {
                    if mask & (1 << i) != 0 {
                        sides |= side;
                    }
                }
                for strategy in [
                    MergeStrategy::Replace,
                    MergeStrategy::Exact,
                    MergeStrategy::Fuzzy,
                ] {
                    for filled in [false, true] {
                        let mut buf = seed(a, filled);
                        for stage in 0..4 {
                            let previous = buf.clone();
                            if let Err(err) =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    draw(&mut buf, area, sides, strategy, stage)
                                }))
                            {
                                assert_eq!((a.x, a.y, dx, dy, stage), (65526, 65530, 7, 3, 3));
                                let msg = err.downcast_ref::<String>().unwrap();
                                assert!(
                                    msg.starts_with("index outside of buffer:")
                                        && msg.ends_with("index is (65535, 65533)"),
                                    "{msg}"
                                );
                                buf = draw_with_guard(&previous, area, sides, strategy, stage);
                                overflows += 1;
                            }
                            raw_cells::compare(
                                &mut input,
                                &previous,
                                &buf,
                                &format!(
                                    "{area:?} mask={mask} {strategy:?} filled={filled} stage={stage}"
                                ),
                            );
                            frames += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(input.next().is_none());
    assert_eq!(overflows, 24);
    println!("{overflows} reference title overflows checked with a guard column");
    println!(
        "{frames} block frames, {} raw cells and ordered diffs match",
        frames * 45
    );
}
