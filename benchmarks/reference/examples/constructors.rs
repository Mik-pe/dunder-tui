#[path = "../src/styled_grid.rs"]
mod styled_grid;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    macros::{constraint, constraints, horizontal, line, row, span, text, vertical},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Cell, Row, Table, Widget},
};
use serde_json::to_string;

fn span_for(kind: usize, n: usize) -> Span<'static> {
    let value = n as i32 - 7;
    let name = "Sjö";
    let style = Style::new()
        .cyan()
        .bold()
        .underlined()
        .underline_color(Color::Rgb(20, 80, 160));
    match kind {
        0 => Span::default(),
        1 => span!("A{{B}}"),
        2 => span!("{} {}", value, name),
        3 => span!("{name}/{value:04}"),
        4 => span!("{:4}", value),
        5 => span!(value),
        6 => span!(style; "tea 茶"),
        7 => span!(style; "{name}/{value:04}"),
        8 => span!(Color::Yellow; "{:4}",value),
        9 => span!(Modifier::ITALIC; value),
        10 => span!(style; "{a} {b}",a=name,b=name),
        11 => span!("{n:04X}"),
        _ => span!("e\u{301}\n茶"),
    }
}

fn line_for(kind: usize, n: usize) -> Line<'static> {
    match kind {
        0 => line![],
        1 => line!["A\nB", "", "茶",],
        2 => line![span_for(2, n), "◆".magenta(), span_for(4, n)],
        3 => line!["茶".blue();n],
        _ => {
            let mut value = line!["🌊".cyan();n];
            if let Some(span) = value.spans.first_mut() {
                span.content = "X".into();
            }
            value.right_aligned()
        }
    }
}

fn document(kind: usize, n: usize) -> Text<'static> {
    match kind {
        0 => text![],
        1 => text!["A\r\nB", "", "茶",],
        2 => text!["NORD", line_for(2, n), span_for(9, n)],
        3 => text![line_for(3,2).centered();n],
        _ => {
            let mut value = text![line_for(3,2);n];
            if let Some(line) = value.lines.first_mut() {
                line.spans[0].content = "IS".into();
            }
            value
        }
    }
}

fn row_for(kind: usize, n: usize) -> Row<'static> {
    match kind {
        0 => row![],
        1 => row!["";0],
        2 => row!["A\nB", "茶", "",],
        3 => row!["F", line_for(2, n), document(2, n), span_for(9, n)],
        4 => row![document(2,2);n],
        _ => {
            let original = document(4, 2);
            let cell_style = Style::new().yellow().on_blue();
            if n == 0 {
                return row![Cell::new(original).style(cell_style);0].height(2);
            }
            let mut cells = vec![Cell::new(original.clone()).style(cell_style); n];
            let mut first = original;
            first.lines[0].spans[0].content = "Ö".into();
            cells[0] = Cell::new(first).style(cell_style);
            Row::new(cells).height(2)
        }
    }
}

fn split(vertical: bool, kind: usize, n: usize) -> Layout {
    let number = n as u16 + 1;
    let min = n as u16;
    let pct = 17 + min;
    let num = n as u32;
    let denom = 9;
    if kind == 0 {
        let rules = constraints![== number, >= min, <= 2*min, == pct %, == num / denom, *= number];
        assert_eq!(rules[5], constraint!(*=number));
        assert_eq!(rules[0], constraint!(== number));
        assert_eq!(rules[1], constraint!(>= min));
        assert_eq!(rules[2], constraint!(<= 2*min));
        assert_eq!(rules[3], constraint!(== pct %));
        assert_eq!(rules[4], constraint!(== num / denom));
        if vertical {
            vertical![== number, >= min, <= 2*min, == pct %, == num / denom, *= number]
        } else {
            horizontal![== number, >= min, <= 2*min, == pct %, == num / denom, *= number]
        }
    } else {
        let rules: Vec<Constraint> = match kind {
            1 => constraints![== num / denom;0].to_vec(),
            2 => constraints![== num / denom;1].to_vec(),
            _ => constraints![== num / denom;5].to_vec(),
        };
        if vertical {
            Layout::vertical(rules)
        } else {
            Layout::horizontal(rules)
        }
    }
}

fn emit(key: String, shape: String, area: Rect, bounds: Rect, widget: impl Widget) {
    let mut buf = Buffer::empty(bounds);
    widget.render(area, &mut buf);
    println!("{key}\t{shape}\t{}", to_string(&buf).unwrap());
}

fn layered_text() {
    let bounds = Rect::new(3, 2, 11, 4);
    for area in [bounds, Rect::new(5, 2, 7, 3), Rect::new(8, 3, 1, 2)] {
        for flags in [false, true] {
            for n in [0, 3, 8] {
                for kind in 0..5 {
                    let mut buf = styled_grid::seed(bounds, flags);
                    document(kind, n)
                        .style(Style::new().underline_color(Color::Indexed(7)))
                        .render(area, &mut buf);
                    println!("{}", to_string(&buf).unwrap());
                    line_for(kind, n)
                        .style(Style::new().red())
                        .render(area, &mut buf);
                    println!("{}", to_string(&buf).unwrap());
                }
            }
        }
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--layered-text") {
        layered_text();
        return;
    }
    for n in [0, 1, 3, 8] {
        for (w, h) in [(0, 0), (1, 2), (6, 4), (19, 7), (33, 5)] {
            let area = Rect::new(2, 1, w, h);
            let bounds = Rect::new(0, 0, w + 4, h + 2);
            for kind in 0..13 {
                let s = span_for(kind, n);
                emit(
                    format!("S{n}/{kind}"),
                    to_string(&s.to_string()).unwrap(),
                    area,
                    bounds,
                    s,
                );
            }
            for kind in 0..5 {
                let line = line_for(kind, n);
                let text = document(kind, n);
                emit(
                    format!("L{n}/{kind}"),
                    format!(
                        "{}/{} {}",
                        line.spans.len(),
                        line.width(),
                        to_string(&line.to_string()).unwrap()
                    ),
                    area,
                    bounds,
                    line,
                );
                emit(
                    format!("T{n}/{kind}"),
                    format!(
                        "{}/{} {}",
                        text.height(),
                        text.width(),
                        to_string(&text.to_string()).unwrap()
                    ),
                    area,
                    bounds,
                    text,
                );
                let table = Table::new(
                    [row!["FÖRE"], row_for(kind, n), row!["EFTER"]],
                    constraints![==5;5],
                );
                emit(format!("R{n}/{kind}"), String::new(), area, bounds, table);
            }
            let table = Table::new(
                [row!["FÖRE"], row_for(5, n), row!["EFTER"]],
                constraints![==5;5],
            );
            emit(format!("R{n}/5"), String::new(), area, bounds, table);
            for kind in 0..4 {
                for vertical in [false, true] {
                    let (areas, spacers) = split(vertical, kind, n).split_with_spacers(area);
                    let mut buf = Buffer::empty(bounds);
                    for (i, rect) in areas.iter().enumerate() {
                        if !rect.is_empty() {
                            buf.set_stringn(
                                rect.x,
                                rect.y,
                                i.to_string(),
                                rect.width.into(),
                                Style::new().yellow(),
                            );
                        }
                    }
                    println!(
                        "A{n}/{kind}/{vertical}\t{} {}\t{}",
                        to_string(areas.as_ref()).unwrap(),
                        to_string(spacers.as_ref()).unwrap(),
                        to_string(&buf).unwrap()
                    );
                }
            }
        }
    }
}
