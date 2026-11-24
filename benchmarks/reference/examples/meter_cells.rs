#[path = "../src/raw_cells.rs"]
mod raw_cells;
use ratatui::{
    buffer::Buffer,
    layout::{Direction, Rect},
    style::{Color, Style},
    symbols::bar,
    text::{Line, Span},
    widgets::{Bar, BarChart, Gauge, LineGauge, RenderDirection, Sparkline, SparklineBar, Widget},
};
use std::io::{self, BufRead};

fn symbols() -> bar::Set<'static> {
    bar::Set {
        empty: "",
        one_eighth: "_",
        one_quarter: "é",
        three_eighths: "界",
        half: "🦊",
        five_eighths: "ab",
        three_quarters: "́",
        seven_eighths: "⣿",
        full: "😶‍🌫️",
    }
}

fn paint(buf: &mut Buffer, area: Rect, widget: usize, k: usize) {
    let style = if k % 3 == 0 {
        Style::new()
    } else {
        Style::new()
            .yellow()
            .underline_color(Color::Indexed(13))
            .not_bold()
    };
    let bar = Style::new().bg(Color::Rgb(0x18, 0x32, 0x4e)).italic();
    let text = ["", "界", "é", "ab", "🦊", "--"];
    let ratio = k as f64 / 5.0;
    match widget {
        0 => {
            let mut data = vec![SparklineBar::from(None)];
            for i in [0, 1, 2, 3, 4, 5, 6, 7, 8, u64::MAX] {
                let sample = SparklineBar::from(i);
                data.push(if i == 4 { sample.style(bar) } else { sample });
            }
            let w = Sparkline::default()
                .data(data)
                .style(style)
                .absent_value_style(bar)
                .direction(if k % 2 == 1 {
                    RenderDirection::RightToLeft
                } else {
                    RenderDirection::LeftToRight
                })
                .max(if k == 5 { 0 } else { 8 });
            (if k < 2 {
                w.absent_value_symbol("界").bar_set(bar::Set {
                    full: "狐",
                    ..bar::NINE_LEVELS
                })
            } else {
                w.absent_value_symbol(text[k]).bar_set(symbols())
            })
            .render(area, buf);
        }
        1 => {
            let mut w = LineGauge::default()
                .ratio(ratio)
                .style(style)
                .filled_style(bar)
                .unfilled_style(Style::new().red());
            if k != 5 {
                w = w.label(Line::from(vec![
                    Span::styled(text[k], Style::new().green()),
                    Span::raw("x"),
                ]));
            }
            (if k == 0 {
                w.filled_symbol("界").unfilled_symbol("狐")
            } else {
                w.filled_symbol(text[k]).unfilled_symbol(text[5 - k])
            })
            .render(area, buf);
        }
        2 => {
            let mut w = Gauge::default()
                .ratio(ratio)
                .style(style)
                .gauge_style(bar)
                .use_unicode(k % 2 == 0);
            if k != 5 {
                w = w.label(Span::styled(text[k], Style::new().green()));
            }
            w.render(area, buf);
        }
        _ => {
            let bars: Vec<_> = (0..9)
                .map(|i| {
                    let b = Bar::default().value(i).text_value(String::new());
                    if i % 2 == 1 {
                        b.style(Style::new().red())
                    } else {
                        b
                    }
                })
                .collect();
            BarChart::new(bars)
                .max(8)
                .bar_width(1 + (k % 2) as u16)
                .bar_gap(0)
                .direction(if widget == 4 {
                    Direction::Horizontal
                } else {
                    Direction::Vertical
                })
                .bar_set(if k >= 2 {
                    symbols()
                } else {
                    bar::Set {
                        empty: "狐",
                        full: "界",
                        ..bar::NINE_LEVELS
                    }
                })
                .style(style)
                .bar_style(bar)
                .render(area, buf);
        }
    }
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let (mut frames, mut cells) = (0, 0);
    for a in [
        Rect::new(0, 0, 13, 5),
        Rect::new(5, 3, 13, 5),
        Rect::new(65522, 65530, 13, 5),
    ] {
        for (dx, dy, w, h) in [
            (0, 0, 13, 5),
            (0, 0, 13, 1),
            (2, 1, 9, 3),
            (1, 1, 1, 2),
            (5, 3, 6, 1),
            (0, 1, 0, 1),
        ] {
            let area = Rect::new(a.x + dx, a.y + dy, w, h);
            for widget in 0..5 {
                for overlay in [false, true] {
                    let mut buf = raw_cells::seeded(a);
                    for k in 0..6 {
                        let old = buf.clone();
                        if !overlay {
                            buf = raw_cells::seeded(a);
                        }
                        paint(&mut buf, area, widget, k);
                        raw_cells::compare(
                            &mut input,
                            &old,
                            &buf,
                            &format!("{area:?} widget={widget} phase={k}"),
                        );
                        frames += 1;
                        cells += a.area();
                    }
                }
            }
        }
    }
    assert!(input.next().is_none());
    println!("{frames} overlaid meter frames, {cells} raw cells and ordered diffs match");
}
