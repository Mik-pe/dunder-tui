#[path = "../src/styled_grid.rs"]
mod styled_grid;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::{Line, Masked, Span, Text, ToLine, ToSpan, ToText},
    widgets::{Cell, List, ListItem, Paragraph, Row, Table, Widget},
};
use std::{
    collections::HashMap,
    fmt::{self, Display, Write},
    hash::{DefaultHasher, Hash, Hasher},
};

struct Gate(i32);
impl Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gate {}\r\n東", self.0)
    }
}
fn hex(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        write!(&mut out, "{b:02x}").unwrap()
    }
    out
}
fn hash<T: Hash>(v: &T) -> u64 {
    let mut h = DefaultHasher::new();
    v.hash(&mut h);
    h.finish()
}
fn classes<T: Eq + Hash>(values: &[T]) {
    let mut keys = HashMap::new();
    for (i, value) in values.iter().enumerate() {
        let first = values[..i]
            .iter()
            .position(|other| other == value)
            .unwrap_or(i);
        if first < i {
            assert_eq!(hash(value), hash(&values[first]));
        }
        let key = keys.entry(value).or_insert(i);
        assert_eq!(*key, first);
        println!("K {key}");
    }
}
fn inspect<T: Display>(value: &T) {
    let span = value.to_span();
    let line = value.to_line();
    let doc = value.to_text();
    println!(
        "V {} {} {} {}",
        hex(&value.to_string()),
        hex(&span.content),
        line.spans.len(),
        doc.height()
    );
    for part in line.spans {
        println!("L {}", hex(&part.content));
    }
    for row in doc.lines {
        println!("T {} {}", row.spans.len(), hex(&row.to_string()));
    }
}
fn text_values() {
    let (mut spans, mut lines, mut docs) = (Vec::new(), Vec::new(), Vec::new());
    for s in [
        "",
        "a",
        "\r",
        "\n",
        "\r\n",
        "a\nb",
        "a\r\nb",
        "ab",
        "Å",
        "A\u{30a}",
        "🦊",
        "👩🏽‍💻",
        "\0",
        "界\n",
    ] {
        for style in [
            Style::new(),
            Style::reset(),
            Style::new().cyan().underline_color(Color::Rgb(3, 4, 5)),
        ] {
            let span = Span::styled(s, style);
            let edited = Span::raw("old").style(style).content(s.to_owned());
            spans.extend([edited, span.clone()]);
            let first = Line::from(span.clone());
            let mut dirty = first.clone();
            dirty.spans[0].content = s.to_owned().into();
            for line in [
                first.clone(),
                dirty,
                Line::raw(s),
                first.clone().left_aligned(),
                Line::from(vec![Span::raw(""), span.clone()]),
                first.clone().style(style),
            ] {
                println!(
                    "R {} {}",
                    hex(&line.to_string()),
                    hex(&String::from(line.clone()))
                );
                lines.push(line.clone());
                let doc = Text::from(line.clone());
                let mut changed = doc.clone();
                changed.lines[0].alignment = line.alignment;
                docs.extend([
                    changed,
                    doc.clone(),
                    doc.clone().right_aligned(),
                    doc.style(style),
                ]);
            }
            inspect(&span);
        }
        inspect(&Text::raw(s));
    }
    for (a, b) in [("\r", "\n"), ("A\r", "\nB"), ("x\n", "\r"), ("", "\r\n")] {
        let line = Line::from(vec![Span::raw(a), Span::raw(b)]);
        println!(
            "R {} {}",
            hex(&line.to_string()),
            hex(&String::from(line.clone()))
        );
        inspect(&line);
    }
    inspect(&Text::default());
    classes(&spans);
    classes(&lines);
    classes(&docs);
    inspect(&Gate(42));
    inspect(&-17);
}
fn collection_keys() {
    let mut items = vec![];
    let mut cells = vec![Cell::default()];
    let mut rows = vec![Row::default()];
    for word in ["", "北", "e\u{301}"] {
        for style in [Style::new(), Style::reset(), Style::new().blue()] {
            let item = ListItem::new(word);
            items.extend([item.clone(), item.style(style)]);
            for span in [0, 1, 2] {
                let cell = Cell::new(word).column_span(span).style(style);
                cells.push(cell.clone());
                let row = Row::new([cell]);
                rows.extend([
                    row.clone(),
                    row.clone().top_margin(1),
                    row.clone().bottom_margin(1),
                    row.clone().height(2),
                    row.style(style),
                ]);
            }
        }
    }
    classes(&items);
    classes(&cells);
    classes(&rows);
}

fn masked_values() {
    let mut keys = vec![Masked::default()];
    for value in [
        "",
        "\n",
        "a\r\nb",
        "🚦b",
        "e\u{301}",
        "👩🏽‍💻🇸🇪",
        "Åa東b",
        "a\0",
    ] {
        for mask in ['*', '●', '界', '🦊', '\u{301}', '\r', '\n', '\0'] {
            let field = Masked::new(value, mask);
            keys.extend([field.clone(), field.clone()]);
            println!(
                "M {} {} {}",
                field.mask_char() as u32,
                hex(&format!("{field:?}")),
                hex(&field.value())
            );
            inspect(&field);
            for width in [1, 5, 11] {
                let mut frame = styled_grid::seed(Rect::new(3, 2, 15, 7), true);
                Paragraph::new(field.clone()).render(Rect::new(4, 3, width, 2), &mut frame);
                List::new([field.clone()]).render(Rect::new(4, 5, width, 2), &mut frame);
                Table::new([Row::new([field.clone()])], [Constraint::Fill(1)])
                    .render(Rect::new(4, 7, width, 1), &mut frame);
                println!("{}", serde_json::to_string(&frame).unwrap());
            }
        }
    }
    for n in [
        0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 127, 128, 129, 1023, 1024, 1025,
    ] {
        let text: String = ['a', 'ö', '界', '🦊'].into_iter().cycle().take(n).collect();
        for mask in ['*', 'ö', '界', '🦊'] {
            println!("P {n} {}", hex(&Masked::new(text.as_str(), mask).value()));
        }
    }
    classes(&keys);
}
fn main() {
    text_values();
    collection_keys();
    masked_values();
}
