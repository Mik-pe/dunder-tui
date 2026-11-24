#[path = "../src/chart_scene.rs"]
mod chart_scene;
#[path = "../src/widget_cells.rs"]
mod widget_cells;

use ratatui::{
    layout::{Alignment, Constraint, Rect},
    style::{Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Chart, Dataset, GraphType, LegendPosition, Widget},
};
use std::io::{self, BufRead};

fn align(n: usize) -> Alignment {
    [Alignment::Left, Alignment::Center, Alignment::Right][n % 3]
}
fn labels(count: usize, horizontal: bool) -> Vec<Line<'static>> {
    (0..count)
        .map(|i| {
            if i == 0 {
                Line::from(if horizontal { "start" } else { "low" }).red()
            } else if i + 1 == count {
                Line::from(if horizontal { "end" } else { "max" }).cyan()
            } else {
                Line::from(i.to_string())
            }
        })
        .collect()
}

fn view(k: usize, series: usize, legend: usize) -> Chart<'static> {
    let sets = (0..series)
        .map(|n| {
            let points: &'static [(f64, f64)] = match n {
                0 if k % 4 == 2 => &[],
                1 => &[(-4., 1.), (4., 1.)],
                2 => &[(0., 0.)],
                _ => &[(-7., 4.), (-2., -3.), (0., 2.), (2., -2.), (6., 3.)],
            };
            let mut d = Dataset::default()
                .data(points)
                .graph_type(
                    [
                        GraphType::Scatter,
                        GraphType::Line,
                        GraphType::Bar,
                        GraphType::Area,
                    ][(k + n) % 4],
                )
                .marker(
                    [
                        Marker::Braille,
                        Marker::Dot,
                        Marker::Quadrant,
                        Marker::Octant,
                        Marker::HalfBlock,
                        Marker::Sextant,
                    ][(k + n) % 6],
                )
                .fill_to_y(if k % 2 == 0 { -1.5 } else { 2.0 })
                .style(if n == 0 {
                    Style::new().magenta().on_blue()
                } else {
                    Style::new().yellow().italic()
                });
            if n == 0 && k % 3 != 1 {
                d = d.name(
                    Line::from(vec![Span::from("A").cyan(), Span::raw("bc")])
                        .green()
                        .bold(),
                );
            } else if n == 1 {
                d = d.name(Line::from(if k % 3 == 0 { "" } else { "Echo" }).right_aligned());
            }
            d
        })
        .collect::<Vec<_>>();
    let mut x = Axis::default()
        .bounds([-5., 5.])
        .labels(labels(k % 5, true))
        .labels_alignment(align(k))
        .yellow();
    let mut y = Axis::default()
        .bounds([-3., 3.])
        .labels(labels((k / 3) % 5, false))
        .labels_alignment(align(k / 2))
        .cyan();
    if k % 3 != 1 {
        x = x.title(Line::from("time").red());
    }
    if k % 4 != 1 {
        y = y.title(Line::from("Y").green());
    }
    let mut c = Chart::new(sets).x_axis(x).y_axis(y);
    if k % 2 == 0 {
        c = c.on_black();
    }
    if k % 4 == 0 {
        c = c.block(Block::bordered());
    }
    if k % 3 == 0 {
        c = c.hidden_legend_constraints((Constraint::Fill(1), Constraint::Fill(1)));
    }
    if k % 3 == 1 {
        c = c.hidden_legend_constraints((Constraint::Length(10), Constraint::Percentage(70)));
    }
    use LegendPosition::*;
    c.legend_position(
        [
            None,
            Some(TopRight),
            Some(Top),
            Some(TopLeft),
            Some(Left),
            Some(Right),
            Some(BottomLeft),
            Some(Bottom),
            Some(BottomRight),
        ][legend],
    )
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    if std::env::args().any(|v| v == "--benchmark") {
        let data = chart_scene::Data::new();
        let mut scene = chart_scene::Scene::new(&data);
        for mode in ["idle", "cell", "scroll", "resize", "repaint"] {
            for n in 0..220 {
                let w = if mode == "resize" && n % 2 == 0 {
                    80
                } else {
                    96
                };
                let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, w, 24));
                scene.draw(&mut buf, mode, n);
                assert_eq!(
                    input.next().unwrap(),
                    widget_cells::snapshot(&buf),
                    "{mode} frame {n}"
                );
            }
        }
        assert!(input.next().is_none());
        println!("1100 chart benchmark frames match");
        return;
    }
    let mut count = 0;
    for k in 0..15 {
        for series in 0..4 {
            for legend in 0..9 {
                let chart = view(k, series, legend);
                for (w, h) in [
                    (0, 0),
                    (1, 1),
                    (3, 2),
                    (6, 5),
                    (9, 6),
                    (13, 7),
                    (21, 11),
                    (42, 17),
                ] {
                    let area = Rect::new(1, 1, w, h);
                    let mut buf = widget_cells::seeded(area);
                    (&chart).render(area, &mut buf);
                    let actual = input.next().unwrap();
                    assert_eq!(
                        actual,
                        widget_cells::snapshot(&buf),
                        "chart {k}, series {series}, legend {legend}, {w}x{h}"
                    );
                    count += 1;
                }
            }
        }
    }
    assert!(input.next().is_none());
    println!("{count} chart frames match");
}
