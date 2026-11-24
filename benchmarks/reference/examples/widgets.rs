#[path = "../src/widget_cells.rs"]
mod widget_cells;
#[path = "../src/widgets_scene.rs"]
mod widgets_scene;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Direction, Offset, Rect},
    style::{Style, Stylize},
    symbols::{bar, border, merge::MergeStrategy},
    text::{Line, Span},
    widgets::{
        Bar, BarChart, BarGroup, Block, Borders, Fill, Gauge, LineGauge, Padding, RenderDirection,
        Scrollbar, ScrollbarOrientation, ScrollbarState, Shadow, Sparkline, SparklineBar,
        StatefulWidget, Tabs, Widget, dimmed,
    },
};
use std::io::{self, BufRead};
use widget_cells::{seeded, snapshot};

fn fixture() -> Block<'static> {
    Block::bordered()
        .border_set(border::ROUNDED)
        .title("A")
        .border_style(Style::new().yellow())
}
struct Check<I> {
    input: I,
    count: usize,
    failures: usize,
}
impl<I: Iterator<Item = io::Result<String>>> Check<I> {
    fn frame(&mut self, kind: &str, id: usize, area: Rect, buf: Buffer) {
        let actual = self.input.next().expect("missing Dunder frame").unwrap();
        let expected = snapshot(&buf);
        self.count += 1;
        if actual != expected {
            if self.failures < 12 {
                eprintln!("{kind} {id}, {area:?}\nD {actual}\nR {expected}");
            }
            self.failures += 1;
        }
    }
    fn widgets(&mut self, a: Rect) {
        for (r, ratio) in [0., 0.005, 0.125, 0.375, 0.5, 0.6667, 0.875, 0.999, 1.]
            .into_iter()
            .enumerate()
        {
            for k in 0..8 {
                let mut b = seeded(a);
                let mut g = Gauge::default()
                    .ratio(ratio)
                    .use_unicode(k % 2 == 1)
                    .style(Style::new().on_cyan());
                if k & 2 != 0 {
                    g = g.gauge_style(Style::new().red().on_magenta().italic());
                }
                if k & 4 != 0 {
                    g = g.block(fixture()).label("界x".yellow());
                }
                g.render(a, &mut b);
                self.frame("gauge", r * 8 + k, a, b);
            }
            for k in 0..4 {
                let mut b = seeded(a);
                let mut g = LineGauge::default()
                    .ratio(ratio)
                    .filled_style(Style::new().red())
                    .unfilled_style(Style::new().cyan().italic())
                    .style(Style::new().on_magenta());
                if k % 2 == 1 {
                    g = g.block(fixture());
                }
                if k >= 2 {
                    g = g
                        .label(Line::from(vec!["界".yellow(), Span::raw("ab")]))
                        .filled_symbol("═")
                        .unfilled_symbol("·");
                }
                g.render(a, &mut b);
                self.frame("line gauge", r * 4 + k, a, b);
            }
        }
        for k in 0..16 {
            let mut b = seeded(a);
            let mut s = Sparkline::default()
                .data([
                    SparklineBar::from(0),
                    None.into(),
                    SparklineBar::from(2).style(Style::new().red()),
                    7.into(),
                    11.into(),
                    5.into(),
                    8.into(),
                ])
                .style(Style::new().cyan().italic())
                .absent_value_style(Style::new().yellow())
                .absent_value_symbol("·");
            if k & 1 != 0 {
                s = s.direction(RenderDirection::RightToLeft);
            }
            if k & 2 != 0 {
                s = s.bar_set(bar::THREE_LEVELS);
            }
            if k & 4 != 0 {
                s = s.max(6);
            }
            if k & 8 != 0 {
                s = s.block(fixture());
            }
            s.render(a, &mut b);
            self.frame("sparkline", k, a, b);
        }
        for k in 0..12 {
            let mut b = seeded(a);
            let mut t = Tabs::new([
                Line::raw("A"),
                Line::from(vec!["界".red(), Span::raw("z")]),
                Line::raw("B"),
            ])
            .highlight_style(Style::new().on_yellow().reversed())
            .cyan();
            if k % 3 == 1 {
                t = t.select(1);
            } else if k % 3 == 2 {
                t = t.select(None);
            }
            if k & 1 != 0 {
                t = t.padding("", ">").divider("·".red());
            }
            if k >= 6 {
                t = t.block(fixture());
            }
            t.render(a, &mut b);
            self.frame("tabs", k, a, b);
        }
        for edge in [
            ScrollbarOrientation::VerticalLeft,
            ScrollbarOrientation::VerticalRight,
            ScrollbarOrientation::HorizontalTop,
            ScrollbarOrientation::HorizontalBottom,
        ] {
            for k in 0..8 {
                for pos in [0, 1, 4, 9, 25] {
                    let mut b = seeded(a);
                    let mut s = Scrollbar::new(edge.clone());
                    if k & 1 != 0 {
                        s = s.begin_symbol(None).end_symbol(None);
                    }
                    if k & 2 != 0 {
                        s = s
                            .track_symbol(None)
                            .thumb_symbol("■")
                            .thumb_style(Style::new().cyan());
                    }
                    let mut state = ScrollbarState::new(10).position(pos);
                    if k & 4 != 0 {
                        state = state.viewport_content_length(3);
                    }
                    s.render(a, &mut b, &mut state);
                    self.frame("scrollbar", k * 32 + pos, a, b);
                }
            }
        }
        for (i, set) in [
            border::PLAIN,
            border::ROUNDED,
            border::DOUBLE,
            border::THICK,
            border::LIGHT_DOUBLE_DASHED,
            border::HEAVY_DOUBLE_DASHED,
            border::LIGHT_TRIPLE_DASHED,
            border::HEAVY_TRIPLE_DASHED,
            border::LIGHT_QUADRUPLE_DASHED,
            border::HEAVY_QUADRUPLE_DASHED,
            border::QUADRANT_INSIDE,
            border::QUADRANT_OUTSIDE,
            border::ONE_EIGHTH_WIDE,
            border::ONE_EIGHTH_TALL,
            border::PROPORTIONAL_WIDE,
            border::PROPORTIONAL_TALL,
            border::FULL,
            border::EMPTY,
        ]
        .into_iter()
        .enumerate()
        {
            for mask in 0..16 {
                let mut b = seeded(a);
                let mut sides = Borders::NONE;
                for (n, side) in [Borders::LEFT, Borders::RIGHT, Borders::TOP, Borders::BOTTOM]
                    .iter()
                    .enumerate()
                {
                    if mask & (1 << n) != 0 {
                        sides |= *side;
                    }
                }
                Block::new()
                    .borders(sides)
                    .border_set(set.clone())
                    .border_style(Style::new().yellow())
                    .on_magenta()
                    .render(a, &mut b);
                self.frame("borders", 16 * i + mask, a, b);
            }
        }
        for k in 0..12 {
            use ratatui::widgets::TitlePosition;
            let mut b = seeded(a);
            let shadow = if k < 4 {
                Shadow::custom(dimmed())
            } else if k < 8 {
                Shadow::light_shade()
            } else {
                Shadow::overlay()
            };
            let block = Block::bordered()
                .title_top("Abcde")
                .title_top(Line::raw("x").centered())
                .title_top(Line::raw("Centrum").centered())
                .title_top(Line::raw("Z").right_aligned())
                .title("xyz")
                .title_position(TitlePosition::Bottom)
                .title_style(Style::new().red())
                .title_alignment(if k % 2 == 1 {
                    Alignment::Right
                } else {
                    Alignment::Left
                })
                .padding(Padding::new(k % 3, k % 3, k % 2, k % 2))
                .shadow(
                    shadow
                        .style(Style::new().cyan())
                        .offset(Offset::new(i32::from(k % 3) - 1, i32::from(k % 4) - 2)),
                );
            let inner = block.inner(a);
            block.render(a, &mut b);
            Fill::new("+").magenta().render(inner, &mut b);
            self.frame("titles/shadow", usize::from(k), a, b);
        }
        for k in 0..16 {
            let mut b = seeded(a);
            let bar = Bar::with_label("A", 14)
                .yellow()
                .value_style(Style::new().reversed())
                .text_value("fast");
            let group = BarGroup::new(vec![bar, Bar::with_label("BB", 0), Bar::default().value(6)])
                .label(Line::raw("G").centered());
            let mut chart = BarChart::grouped(vec![
                group,
                BarGroup::new(vec![Bar::default().value(10), Bar::with_label("C", 2)]),
            ])
            .value_style(Style::new().red())
            .bar_style(Style::new().cyan())
            .label_style(Style::new().magenta())
            .bar_width(1 + k % 3)
            .bar_gap(k % 2)
            .group_gap((k / 2) % 2);
            if k & 4 != 0 {
                chart = chart.max(9);
            }
            if k & 8 != 0 {
                chart = chart.direction(Direction::Horizontal);
            }
            chart.render(a, &mut b);
            self.frame("barchart", usize::from(k), a, b);
        }
    }
}
fn main() {
    let mut input = io::stdin().lock().lines();
    if std::env::args().any(|s| s == "--merge") {
        for strategy in [
            MergeStrategy::Replace,
            MergeStrategy::Exact,
            MergeStrategy::Fuzzy,
        ] {
            for a in 0x24ff..0x2581 {
                let prev = char::from_u32(a).unwrap().to_string();
                let mut expected = String::new();
                for b in 0x24ff..0x2581 {
                    let next = char::from_u32(b).unwrap().to_string();
                    expected.push_str(strategy.merge(&prev, &next));
                }
                assert_eq!(
                    input.next().unwrap().unwrap(),
                    expected,
                    "{strategy:?} on {prev}"
                );
            }
        }
        assert!(input.next().is_none());
        println!("50700 border merges match");
        return;
    }
    let mut check = Check {
        input,
        count: 0,
        failures: 0,
    };
    let horizontal = std::env::args().any(|arg| arg == "--horizontal");
    if !horizontal {
        for w in [0, 1, 2, 3, 5, 8, 13, 22] {
            for h in [0, 1, 2, 3, 6] {
                check.widgets(Rect::new(1, 1, w, h));
            }
        }
    }
    for w in [80, 96] {
        for phase in 0..9 {
            for marker in [false, true] {
                let area = Rect::new(0, 0, w, 24);
                let mut buf = Buffer::empty(area);
                widgets_scene::Scene::dashboard(
                    if phase == 8 { 0 } else { phase },
                    phase == 8,
                    horizontal,
                )
                .draw(&mut buf, marker);
                check.frame("benchmark", phase as usize, area, buf);
            }
        }
    }
    assert_eq!(check.count, if horizontal { 36 } else { 24516 });
    assert!(check.input.next().is_none());
    assert_eq!(check.failures, 0, "out of {} frames", check.count);
    println!("{} identical widget frames", check.count);
}
