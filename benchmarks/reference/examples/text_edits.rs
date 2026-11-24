#[path = "../src/widget_cells.rs"]
mod widget_cells;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget, Wrap},
};

fn hex(value: &str) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    for b in value.bytes() {
        write!(out, "{b:02x}").unwrap();
    }
    out
}

fn inspect(doc: &Text) {
    println!(
        "T {} {} {}",
        doc.width(),
        doc.height(),
        hex(&doc.to_string())
    );
    for line in doc {
        println!(
            "L {} {} {}",
            line.width(),
            line.spans.len(),
            hex(&line.to_string())
        );
        for span in line {
            println!(
                "S {} {} {}",
                span.width(),
                hex(&span.content),
                hex(&span.to_string())
            );
        }
    }
    for w in [0, 1, 2, 4, 7, 13] {
        for h in [1, 4] {
            let area = Rect::new(1, 1, w, h);
            let mut buf = widget_cells::seeded(area);
            doc.render(area, &mut buf);
            println!("{}", widget_cells::snapshot(&buf));
        }
    }
}

fn collection(step: usize) -> Text<'static> {
    let mut doc = Text::from(vec![
        Line::raw("längsta gamla raden").right_aligned(),
        Line::from(vec![
            Span::raw("秋 ").cyan(),
            Span::raw("a\u{301}"),
            Span::raw(""),
        ])
        .centered(),
        Line::raw("👩🏽‍💻"),
    ])
    .yellow()
    .italic()
    .left_aligned();
    if step >= 1 {
        doc.lines.reverse();
    }
    if step >= 2 {
        assert_eq!(doc.lines.remove(2).to_string(), "längsta gamla raden");
        doc.lines.push(Line::default());
    }
    if step >= 3 {
        let spans = &mut doc.lines[1].spans;
        spans.swap(0, 2);
        for span in &mut *spans {
            span.style = span.style.bold();
        }
        spans[0].content = "ny ".into();
    }
    if step >= 4 {
        let old = std::mem::replace(
            &mut doc.lines[1].spans,
            vec![Span::raw(""), Span::raw("🦊").red()],
        );
        doc.lines[0] = Line::from(old).on_magenta();
        doc.lines[0].alignment = Some(Alignment::Right);
        doc.alignment = Some(Alignment::Center);
    }
    if step >= 5 {
        let old = std::mem::replace(&mut doc.lines, vec![Line::raw("mellan")]);
        doc.lines.extend(old);
        doc.lines.swap(0, 3);
    }
    if step >= 6 {
        let mut spans = doc.lines.remove(1).spans;
        spans.remove(0);
        spans.reverse();
        doc.lines[1].spans = spans;
    }
    if step >= 7 {
        doc.lines[1].spans = vec![];
        doc.lines[1].spans.push(Span::raw("🇸🇪"));
        doc.lines[1].spans.clear();
    }
    if step >= 8 {
        doc.lines = Text::raw("rad\n\nslut").lines;
        doc.lines.swap(0, 2);
    }
    if step >= 9 {
        doc.lines = vec![];
    }
    if step >= 10 {
        doc.push_span("nytt");
        doc.lines[0].alignment = None;
    }
    doc
}

fn collection_frames() {
    for step in 0..11 {
        let doc = collection(step);
        inspect(&doc);
        for wrap in [None, Some(Wrap { trim: false }), Some(Wrap { trim: true })] {
            for w in [1, 4, 9, 2, 12, 0] {
                let area = Rect::new(1, 1, w, 3);
                for top in [0, 1, 4] {
                    let mut para = Paragraph::new(doc.clone()).scroll((top, 0));
                    if let Some(wrap) = wrap {
                        para = para.wrap(wrap);
                    }
                    let mut buf = widget_cells::seeded(area);
                    para.render(area, &mut buf);
                    let expected = widget_cells::snapshot(&buf);
                    println!("{expected}\n{expected}");
                }
            }
        }
    }
}

fn main() {
    collection_frames();
    let mut doc = Text::default().right_aligned();
    inspect(&doc);
    doc.push_span(Span::raw("秋").red());
    inspect(&doc);
    doc.push_span("  👩🏽‍💻!");
    doc.push_line(Line::raw("a\r\nb\n").centered());
    doc.lines[1].style = Style::new().on_magenta();
    doc = doc.patch_style(Style::new().yellow().italic());
    inspect(&doc);
    doc.lines[0].spans[1].content = "".into();
    let span = &mut doc.lines[0].spans[0];
    span.style = span.style.patch(Style::new().bold());
    doc.lines[1].style = doc.lines[1].style.patch(Style::new().cyan());
    inspect(&doc);
    doc.lines[1].spans[0].content = "Kör".into();
    doc.lines[1].spans[1].style = Style::new().yellow();
    doc.lines[1].push_span("!");
    doc.lines[0].spans[0].style = Style::reset();
    doc.style = Style::new().on_red();
    inspect(&doc);
    doc.lines[1].style = Style::reset();
    doc = doc.reset_style();
    doc.push_line(Line::raw(""));
    doc.push_span("x\r\ny\tz");
    inspect(&doc);
    for value in ["", "\n", "\n\n", "\r", "\r\n", "\n界\r\nb\n\n", "لا", "🇸🇪!"] {
        inspect(&Text::raw(value));
        inspect(&Text::from(Line::raw(value)));
        inspect(&Text::from(Span::raw(value)));
    }
    for empty in [true, false] {
        for dirty in [true, false] {
            let mut joined = Text::default().red().bold().right_aligned();
            if !empty {
                joined.push_line("左");
                joined.push_line("a");
            }
            let mut suffix = Text::from(vec![
                Line::raw("👩🏽‍💻").left_aligned(),
                Line::from(vec![Span::raw("秋").yellow(), Span::raw("B")]).on_magenta(),
            ])
            .cyan()
            .italic()
            .centered();
            if dirty {
                suffix.lines[1].spans[1].content = "e\u{301}◌".into();
            }
            joined += suffix;
            inspect(&joined);
            joined += Text::default();
            joined.lines[0].spans[0].content = "!".into();
            joined.push_span("C");
            inspect(&joined);
        }
    }
}
