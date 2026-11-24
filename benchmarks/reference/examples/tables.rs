#[path = "../src/layout_optimum.rs"]
mod layout_optimum;
#[path = "../src/table_scene.rs"]
mod table_scene;
#[path = "../src/widget_cells.rs"]
mod widget_cells;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Cell, HighlightSpacing, Row, StatefulWidget, Table, TableState},
};
use std::collections::HashSet;
use std::io::{self, BufRead};
fn rows(d: usize) -> Vec<Row<'static>> {
    let r = |values: Vec<&'static str>| Row::new(values);
    if d == 0 {
        return vec![];
    }
    if d == 1 {
        return vec![
            r(vec!["A", "B", "C"]),
            r(vec!["D"]),
            r(vec![]),
            r(vec!["1", "2", "3"]),
        ];
    }
    let word = Cell::new(Text::from(vec![
        Line::from(vec![
            Span::styled("茶", Style::new().cyan()),
            Span::raw("é"),
        ])
        .alignment(Alignment::Right),
        Line::raw("line").alignment(Alignment::Center),
    ]))
    .yellow()
    .italic()
    .column_span(if d == 3 { 2 } else { 1 });
    vec![
        Row::new([word, Cell::from("XY"), Cell::from("Z")])
            .height(2)
            .top_margin(1)
            .on_black(),
        Row::new([
            Cell::from("skip").column_span(0),
            Cell::from("one\ntwo"),
            Cell::from("end").column_span(9),
        ])
        .height(3)
        .bottom_margin(1),
        r(vec!["0"]).height(0),
        r(vec!["Tall\nrow", "B"]).height(7),
        r(vec!["last"]),
    ]
}
fn widths(k: usize) -> Vec<Constraint> {
    use Constraint::*;
    match k % 4 {
        0 => vec![],
        1 => vec![Length(3), Fill(1), Min(2)],
        2 => vec![Percentage(35), Ratio(1, 3)],
        _ => vec![Length(2), Length(3), Length(1), Fill(1)],
    }
}
fn header() -> Row<'static> {
    Row::new([
        Cell::from("H").column_span(2),
        Cell::from("name"),
        Cell::from("N"),
    ])
    .cyan()
}
fn footer() -> Row<'static> {
    Row::new(["F", "end"]).red()
}
fn make(wk: usize, d: usize, k: usize) -> Table<'static> {
    let mut t = Table::new(rows(d), widths(wk))
        .on_black()
        .row_highlight_style(Style::new().magenta().reversed())
        .column_highlight_style(Style::new().on_blue().dim())
        .cell_highlight_style(Style::new().on_yellow().italic())
        .highlight_symbol(if k % 2 == 0 {
            Text::raw(">")
        } else {
            Text::raw("界>\n!").red()
        })
        .column_spacing((k % 3) as u16)
        .flex(
            [
                Flex::Start,
                Flex::Center,
                Flex::End,
                Flex::SpaceBetween,
                Flex::SpaceAround,
                Flex::SpaceEvenly,
                Flex::Legacy,
            ][k % 7],
        )
        .highlight_spacing(
            [
                HighlightSpacing::Always,
                HighlightSpacing::WhenSelected,
                HighlightSpacing::Never,
            ][(k / 2) % 3]
                .clone(),
        );
    if k % 3 != 0 {
        t = t.header(header().top_margin((k % 2) as u16));
    }
    if k % 4 != 0 {
        t = t.footer(footer().bottom_margin((k % 2) as u16));
    }
    if k % 5 == 0 {
        t = t.block(Block::bordered().title("T"));
    }
    t
}
fn ix(v: Option<usize>) -> i64 {
    v.map_or(-1, |x| {
        if x > usize::MAX - 1000 {
            i64::from(i32::MAX) - (usize::MAX - x) as i64
        } else {
            x as i64
        }
    })
}
fn inner(area: Rect, k: usize) -> Rect {
    if k % 5 == 0 {
        Block::bordered().title("T").inner(area)
    } else {
        area
    }
}
fn vertical(k: usize) -> [Constraint; 7] {
    let h = u16::from(k % 3 != 0);
    let f = u16::from(k % 4 != 0);
    let m = (k % 2) as u16;
    [
        Constraint::Length(h * m),
        Constraint::Length(h),
        Constraint::Length(0),
        Constraint::Min(0),
        Constraint::Length(0),
        Constraint::Length(f),
        Constraint::Length(f * m),
    ]
}

fn rectify(
    view: &Table<'static>,
    inner: Rect,
    k: usize,
    heights: [u16; 7],
    proved: &mut HashSet<(usize, u16, [u16; 7], [u16; 7])>,
) -> Option<Table<'static>> {
    let rules = vertical(k);
    let (native, gaps) = Layout::vertical(rules).split_with_spacers(inner);
    let native_h: [u16; 7] = std::array::from_fn(|i| native[i].height);
    if native_h == heights {
        return None;
    }
    assert_eq!(
        heights.iter().map(|&x| u32::from(x)).sum::<u32>(),
        u32::from(inner.height)
    );
    if proved.insert((k, inner.height, native_h, heights)) {
        let mut y = inner.y;
        let mut pieces = Vec::new();
        let mut separators = Vec::new();
        for h in heights {
            separators.push(Rect::new(inner.x, y, inner.width, 0));
            pieces.push(Rect::new(inner.x, y, inner.width, h));
            y += h;
        }
        separators.push(Rect::new(inner.x, y, inner.width, 0));
        pieces.extend(separators);
        let expected = native
            .iter()
            .chain(gaps.iter())
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(
            layout_optimum::optimal_projections(
                &rules,
                Flex::Start,
                0,
                inner,
                Direction::Vertical,
                &expected,
                &pieces
            ),
            (true, true)
        );
    }
    let mut replay = view.clone();
    if k % 3 != 0 {
        replay = replay.header(
            header()
                .height(heights[1])
                .top_margin(heights[0])
                .bottom_margin(heights[2]),
        );
    }
    if k % 4 != 0 {
        replay = replay.footer(
            footer()
                .height(heights[5])
                .top_margin(heights[4])
                .bottom_margin(heights[6]),
        );
    }
    Some(replay)
}

fn benchmark(input: &mut impl Iterator<Item = String>) {
    let mut scene = table_scene::Scene::new(4096);
    for mode in ["idle", "cell", "scroll", "resize", "repaint", "jump"] {
        for n in 0..220 {
            let w = if mode == "resize" && n % 2 == 0 {
                80
            } else {
                96
            };
            let mut buf = Buffer::empty(Rect::new(0, 0, w, 24));
            scene.draw(&mut buf, mode, n);
            assert_eq!(
                input.next().unwrap(),
                format!(
                    "{} {} {}",
                    scene.state.offset(),
                    ix(scene.state.selected()),
                    ix(scene.state.selected_column())
                ),
                "state for {mode} frame {n}"
            );
            assert_eq!(
                input.next().unwrap(),
                widget_cells::snapshot(&buf),
                "{mode} frame {n}"
            );
        }
    }
    assert!(input.next().is_none());
    println!("1320 table benchmark frames and states match");
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    if std::env::args().any(|a| a == "--benchmark") {
        benchmark(&mut input);
        return;
    }
    let mut exact = 0;
    let mut different = 0;
    let mut ties = 0;
    let mut proved = HashSet::new();
    for w in [0, 1, 3, 7, 14, 27] {
        for h in [0, 1, 3, 6, 11] {
            for k in 0..14 {
                for d in 0..4 {
                    let mut view = make(k, d, k);
                    let mut state = TableState::new();
                    for step in 0..20 {
                        match step {
                            1 => state.select_previous(),
                            2 => state.select_previous_column(),
                            3 => {
                                state.select_first();
                                state.select_next_column();
                            }
                            4 => state.scroll_down_by(2),
                            5 => state.select_cell(Some((1, 0))),
                            6 => state.scroll_right_by(7),
                            7 => {
                                state.scroll_left_by(1);
                                *state.offset_mut() = 99;
                            }
                            8 => {
                                state.select(None);
                                *state.offset_mut() = 2;
                            }
                            9 => state.select_column(None),
                            10 => {
                                state.scroll_up_by(4);
                                state.select_first_column();
                            }
                            11 => {
                                state.select_last_column();
                                state.select_last();
                            }
                            12 => view = view.rows(rows((d + 1) % 4)),
                            13 => {
                                state.select_next();
                                state.select_next_column();
                            }
                            14 => state.select_cell(None),
                            15 => state.select_cell(Some((2, 2))),
                            16 => view = view.widths(widths(k + 1)),
                            17 => {
                                view = view.rows(rows(d));
                                *state.offset_mut() = 3;
                            }
                            18 => {
                                state.select_previous();
                                state.select_previous_column();
                            }
                            19 => state.select(None),
                            _ => {}
                        }
                        let area = Rect::new(
                            1,
                            1,
                            if step == 7 { w / 2 } else { w },
                            if step == 4 || step == 13 { h / 2 } else { h },
                        );
                        let prior = state;
                        let mut buf = widget_cells::seeded(area);
                        StatefulWidget::render(&view, area, &mut buf, &mut state);
                        let state_line = format!(
                            "{} {} {}",
                            state.offset(),
                            ix(state.selected()),
                            ix(state.selected_column())
                        );
                        let spec = input.next().unwrap();
                        let v: Vec<_> = spec.split_whitespace().collect();
                        assert_eq!(v.len(), 10);
                        let a = v[..3].join(" ");
                        let heights = std::array::from_fn(|i| v[i + 3].parse::<u16>().unwrap());
                        let img = input.next().unwrap();
                        let expected = widget_cells::snapshot(&buf);
                        assert_eq!(a, state_line, "{w} {h} {k} {d} {step}");
                        if img == expected {
                            exact += 1;
                            continue;
                        }
                        let inner = inner(area, k);
                        let fixed = rectify(&view, inner, k, heights, &mut proved);
                        let candidate = fixed.as_ref().unwrap_or(&view);
                        let mut replay = widget_cells::seeded(area);
                        let mut cursor = prior;
                        StatefulWidget::render(candidate, area, &mut replay, &mut cursor);
                        assert_eq!(cursor, state);
                        if fixed.is_some() && img == widget_cells::snapshot(&replay) {
                            ties += 1;
                            continue;
                        }
                        if different < 30 {
                            eprintln!(
                                "{w} {h} {k} {d} {step} tied={}\nD {img}\nR {}",
                                fixed.is_some(),
                                widget_cells::snapshot(&replay)
                            );
                        }
                        different += 1;
                    }
                }
            }
        }
    }
    assert!(input.next().is_none());
    println!("{exact} exact, {ties} equivalent band layouts, {different} unresolved");
    assert_eq!(exact + ties + different, 33600);
    assert_eq!(different, 0);
}
