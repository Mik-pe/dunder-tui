use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        HighlightSpacing, List, ListDirection, ListState, RenderDirection, Row, ScrollDirection,
        Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState,
    },
};
use serde_json::{from_str, json, to_string};
use std::io::{self, BufRead};

fn cases() {
    let indices = [
        0,
        1,
        2,
        3,
        4,
        5,
        21,
        65535,
        i32::MAX as u64,
        i32::MAX as u64 + 1,
        1 << 32,
        1 << 53,
        (1 << 53) + 1,
        i64::MAX as u64,
        1 << 63,
        u64::MAX - 1,
        u64::MAX,
    ];
    let words = ["Fyr", "茶", "Röd", "Is", "Te"];
    for (i, &offset) in indices.iter().enumerate() {
        for (j, selected) in std::iter::once(None)
            .chain(indices.iter().copied().map(Some))
            .enumerate()
        {
            println!("L\t{}", json!({"offset":offset,"selected":selected}));
            for col in [None, Some(indices[(i + j) % indices.len()])] {
                println!("T\t{}", json!([offset, selected, col]));
            }
        }
        for (j, &pos) in indices.iter().enumerate() {
            for viewport in [0, 1, 7, indices[(i + j) % indices.len()], u64::MAX] {
                println!("B\t{}", json!([offset, pos, viewport]));
            }
        }
    }
    for (kind, names) in [
        ('L', vec!["offset", "selected"]),
        ('T', vec!["offset", "selected", "selected_column"]),
        (
            'B',
            vec!["content_length", "position", "viewport_content_length"],
        ),
    ] {
        let mut object = json!({});
        for name in &names {
            object[name] = json!(1);
        }
        println!("{kind}\t{object}");
        let mut extended = object.clone();
        extended["future"] = json!(words);
        println!("{kind}\t{extended}");
        for name in &names {
            let mut partial = object.clone();
            partial.as_object_mut().unwrap().remove(*name);
            println!("{kind}\t{partial}");
            for value in [
                "null",
                "-1",
                "1.0",
                "18446744073709551616",
                "true",
                r#""1""#,
                "[]",
                "{}",
            ] {
                println!(
                    "{kind}\t{}",
                    object
                        .to_string()
                        .replace(&format!("\"{name}\":1"), &format!("\"{name}\":{value}"))
                );
            }
            println!(
                "{kind}\t{}",
                object.to_string().replace(
                    &format!("\"{name}\":"),
                    &format!("\"{name}\":2,\"{name}\":")
                )
            );
        }
        for raw in [
            "[]",
            "[0]",
            "[0,1]",
            "[1,2,3]",
            "[1,2,3,4]",
            "{}",
            "null",
            "true",
            "-1",
        ] {
            println!("{kind}\t{raw}");
        }
    }
    for (kind, names) in [
        ('D', vec!["TopToBottom", "BottomToTop"]),
        ('H', vec!["Always", "WhenSelected", "Never"]),
        ('F', vec!["LeftToRight", "RightToLeft"]),
        (
            'E',
            vec![
                "VerticalLeft",
                "VerticalRight",
                "HorizontalTop",
                "HorizontalBottom",
            ],
        ),
        ('R', vec!["Forward", "Backward"]),
    ] {
        for name in names {
            println!("{kind}\t{}", json!(name));
            println!("{kind}\t{}", json!({name:null}));
            println!("{kind}\t{}", json!({name:1}));
        }
        for raw in [
            r#""Left""#,
            r#""forward""#,
            "null",
            "[]",
            "{}",
            r#"{"Forward":null,"Forward":null}"#,
        ] {
            println!("{kind}\t{raw}");
        }
    }
}

fn lists(input: &str) -> serde_json::Result<String> {
    let mut state: ListState = from_str(input)?;
    let saved = to_string(&state)?;
    let view = List::new(["Fyr", "茶", "Röd", "Is", "Te"])
        .highlight_symbol(">")
        .highlight_style(Style::new().bg(Color::Blue));
    let mut buf = Buffer::empty(Rect::new(0, 0, 9, 3));
    StatefulWidget::render(&view, Rect::ZERO, &mut buf, &mut state);
    assert_eq!(to_string(&state)?, saved);
    let mut nav = state;
    nav.select_previous();
    nav.scroll_up_by(65535);
    let a = to_string(&nav)?;
    nav.scroll_down_by(65535);
    nav.select_next();
    let b = to_string(&nav)?;
    StatefulWidget::render(&view, buf.area, &mut buf, &mut state);
    let drawn = format!("{}\t{}", to_string(&state)?, to_string(&buf)?);
    state.select_previous();
    buf.reset();
    StatefulWidget::render(&view, buf.area, &mut buf, &mut state);
    Ok(format!(
        "{saved}\t{a}\t{b}\t{drawn}\t{}\t{}",
        to_string(&state)?,
        to_string(&buf)?
    ))
}

fn tables(input: &str) -> serde_json::Result<String> {
    let mut state: TableState = from_str(input)?;
    let saved = to_string(&state)?;
    let view = Table::new(
        ["Fyr", "茶", "Röd", "Is", "Te"].map(|word| Row::new([word, "茶"])),
        [Constraint::Length(3), Constraint::Fill(1)],
    )
    .highlight_symbol(">")
    .row_highlight_style(Style::new().fg(Color::Yellow))
    .column_highlight_style(Style::new().add_modifier(Modifier::UNDERLINED))
    .cell_highlight_style(Style::new().bg(Color::Blue));
    let mut buf = Buffer::empty(Rect::new(0, 0, 9, 3));
    StatefulWidget::render(&view, Rect::ZERO, &mut buf, &mut state);
    assert_eq!(to_string(&state)?, saved);
    let mut nav = state;
    nav.select_previous();
    nav.select_previous_column();
    nav.scroll_up_by(65535);
    nav.scroll_left_by(65535);
    let a = to_string(&nav)?;
    nav.scroll_down_by(65535);
    nav.scroll_right_by(65535);
    nav.select_next();
    nav.select_next_column();
    let b = to_string(&nav)?;
    StatefulWidget::render(&view, buf.area, &mut buf, &mut state);
    let drawn = format!("{}\t{}", to_string(&state)?, to_string(&buf)?);
    state.select_previous();
    state.select_previous_column();
    buf.reset();
    StatefulWidget::render(&view, buf.area, &mut buf, &mut state);
    Ok(format!(
        "{saved}\t{a}\t{b}\t{drawn}\t{}\t{}",
        to_string(&state)?,
        to_string(&buf)?
    ))
}

fn rails(state: &mut ScrollbarState) -> serde_json::Result<String> {
    use ScrollbarOrientation::*;
    let mut buf = Buffer::empty(Rect::new(0, 0, 11, 4));
    for edge in [VerticalLeft, VerticalRight, HorizontalTop, HorizontalBottom] {
        Scrollbar::new(edge).render(buf.area, &mut buf, state);
    }
    to_string(&buf)
}

fn scrollbar(input: &str) -> serde_json::Result<String> {
    let mut state: ScrollbarState = from_str(input)?;
    let saved = to_string(&state)?;
    let first = rails(&mut state)?;
    state.scroll(ScrollDirection::Backward);
    let a = to_string(&state)?;
    state.scroll(ScrollDirection::Forward);
    let b = to_string(&state)?;
    state.last();
    let last = to_string(&state)?;
    let end = rails(&mut state)?;
    state.first();
    Ok(format!(
        "{saved}\t{first}\t{a}\t{b}\t{last}\t{end}\t{}",
        to_string(&state)?
    ))
}

fn check(kind: &str, input: &str) -> serde_json::Result<String> {
    match kind {
        "L" => lists(input),
        "T" => tables(input),
        "B" => scrollbar(input),
        "D" => to_string(&from_str::<ListDirection>(input)?),
        "H" => to_string(&from_str::<HighlightSpacing>(input)?),
        "F" => to_string(&from_str::<RenderDirection>(input)?),
        "E" => to_string(&from_str::<ScrollbarOrientation>(input)?),
        _ => to_string(&from_str::<ScrollDirection>(input)?),
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--cases") {
        cases();
        return;
    }
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let (kind, input) = line.split_once('\t').unwrap();
        println!("{}", check(kind, input).unwrap_or_else(|_| "error".into()));
    }
}
