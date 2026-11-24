#[path = "../src/widget_cells.rs"]
mod widget_cells;
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::Widget,
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

fn main() {
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
