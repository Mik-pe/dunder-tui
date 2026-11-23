#[path = "../src/list_scene.rs"]
mod list_scene;
#[path = "../src/widget_cells.rs"]
mod widget_cells;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Fill, HighlightSpacing, List, ListDirection, ListItem, ListState, StatefulWidget,
        Widget,
    },
};
use std::io::{self, BufRead};

fn items(kind: usize) -> Vec<ListItem<'static>> {
    match kind {
        0 => vec![],
        1 => ["one", "two\nTWO", "", "four"]
            .into_iter()
            .map(ListItem::new)
            .collect(),
        _ => vec![
            ListItem::new(Text::default()),
            ListItem::new(
                Text::from(vec![
                    Line::from(vec![
                        Span::raw("A"),
                        Span::styled("界B", Style::new().cyan().italic()),
                    ])
                    .alignment(Alignment::Right),
                    Line::from("é").alignment(Alignment::Center),
                ])
                .on_magenta(),
            )
            .yellow(),
            ListItem::new("c\nd\ne\nf\ng"),
            ListItem::new(Text::default()),
            ListItem::new("茶"),
            ListItem::new("x\ny"),
            ListItem::new("z"),
        ],
    }
}

fn index(selected: Option<usize>) -> i64 {
    selected.map_or(-1, |v| {
        if v > usize::MAX - 1000 {
            i64::from(i32::MAX) - (usize::MAX - v) as i64
        } else {
            v as i64
        }
    })
}

fn benchmark(input: &mut impl Iterator<Item = String>) {
    let mut scene = list_scene::Scene::new(16384);
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
                format!("{} {}", scene.state.offset(), index(scene.state.selected())),
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
    println!("1320 list benchmark frames and states match");
}

fn configured(w: usize, h: usize, kind: usize, dataset: usize) -> List<'static> {
    let marker = if kind < 12 {
        Line::from(">")
    } else {
        Line::from(vec![
            Span::styled("界", Style::new().red().bold()),
            Span::raw(":"),
        ])
    };
    let mut view = List::new(items(dataset))
        .style(Style::new().on_black())
        .highlight_style(Style::new().magenta().reversed())
        .highlight_symbol(marker)
        .direction(if kind % 2 == 0 {
            ListDirection::TopToBottom
        } else {
            ListDirection::BottomToTop
        })
        .repeat_highlight_symbol(kind % 4 >= 2)
        .highlight_spacing(
            [
                HighlightSpacing::Always,
                HighlightSpacing::WhenSelected,
                HighlightSpacing::Never,
            ][(kind / 4) % 3]
                .clone(),
        )
        .scroll_padding([0, 1, 2, 8][(kind + w + h) % 4]);
    if (kind + w + h) % 7 == 0 {
        view = view.block(Block::bordered().title("L"));
    }
    view
}

fn sequence(
    input: &mut impl Iterator<Item = String>,
    w: usize,
    h: usize,
    kind: usize,
    dataset: usize,
) -> usize {
    let mut spill = 0;
    let mut view = configured(w, h, kind, dataset);
    let mut state = ListState::default();
    for step in 0..18 {
        match step {
            1 => state.select_previous(),
            2 | 4 | 13 => state.select_next(),
            3 => state.select_first(),
            5 | 15 => state.scroll_down_by(2),
            6 => state.select_previous(),
            7 => *state.offset_mut() = 12,
            8 => {
                state.select(None);
                *state.offset_mut() = 3
            }
            9 => state.scroll_up_by(9),
            10 => state.select_last(),
            11 => state.scroll_up_by(2),
            12 => view = view.items(items((dataset + 1) % 3)),
            14 => state.select(None),
            16 => {
                view = view.items(items(dataset));
                *state.offset_mut() = 1
            }
            17 => state.select(Some(99)),
            _ => {}
        }
        let width = if step == 6 { w / 2 } else { w };
        let height = if matches!(step, 4 | 7) { h / 2 } else { h };
        let area = Rect::new(1, 1, width as u16, height as u16);
        let mut full = widget_cells::seeded(area);
        let mut prior = state;
        StatefulWidget::render(&view, area, &mut full, &mut state);
        let actual_state = input.next().unwrap();
        let actual = input.next().unwrap();
        assert_eq!(
            actual_state,
            format!("{} {}", state.offset(), index(state.selected())),
            "state w={w} h={h} options={kind} data={dataset} step={step}"
        );
        if actual != widget_cells::snapshot(&full) {
            let mut clipped = Buffer::empty(area);
            Fill::new(".")
                .green()
                .on_blue()
                .bold()
                .render(area, &mut clipped);
            StatefulWidget::render(&view, area, &mut clipped, &mut prior);
            let mut guard = widget_cells::seeded(area);
            assert!(
                (0..full.area.height).any(|y| (0..full.area.width)
                    .any(|x| !area.contains((x, y).into()) && full[(x, y)] != guard[(x, y)])),
                "unexpected in-bounds mismatch w={w} h={h} options={kind} data={dataset} step={step}\nactual {actual}\nreference {}",
                widget_cells::snapshot(&full)
            );
            for (pos, cell) in clipped.content.iter().enumerate() {
                let x = area.x + (pos % width) as u16;
                let y = area.y + (pos / width) as u16;
                guard[(x, y)] = cell.clone();
            }
            assert_eq!(
                actual,
                widget_cells::snapshot(&guard),
                "bounded result w={w} h={h} options={kind} data={dataset} step={step}"
            );
            assert_eq!(state, prior);
            spill += 1;
        }
    }
    spill
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    if std::env::args().any(|arg| arg == "--benchmark") {
        benchmark(&mut input);
        return;
    }
    let mut spill = 0;
    for w in [0, 1, 2, 5, 12] {
        for h in [0, 1, 2, 4, 6, 9] {
            for kind in 0..24 {
                for dataset in 0..3 {
                    spill += sequence(&mut input, w, h, kind, dataset);
                }
            }
        }
    }
    assert_eq!(spill, 1847);
    assert!(input.next().is_none());
    println!(
        "{} whole frames, {spill} clipped marker overflows, 38880 matching list states",
        38880 - spill
    );
}
