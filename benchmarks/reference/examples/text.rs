#[path = "../src/paragraph_document.rs"]
mod paragraph_document;

use ratatui::buffer::{Buffer, CellWidth};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Widget, Wrap};
use std::fmt::Write;
use std::io::{self, BufRead};

fn sample(case: u8) -> Text<'static> {
    match case {
        0 => Text::raw("  alpha   beta \n\n gamma\n  \n\r\nZ\n"),
        1 => Text::raw("界x 👩🏽‍💻  y中🇸🇪\nKör अक्क 각\n1️⃣　✈️   z"),
        2 => Text::raw("a\u{a0}b c\u{200b}de\u{2003}f\tg\rz\nabcdefghijklmno   PQ"),
        _ => Text::from(vec![
            Line::from(vec![
                "  fle".red().bold(),
                "rfärg ".cyan(),
                "界  z".italic(),
            ])
            .on_blue(),
            Line::raw(""),
            Line::from(vec![Span::raw("  "), "tre ord".red(), Span::raw(" x")]).right_aligned(),
        ])
        .cyan()
        .centered(),
    }
}
fn color(c: Color) -> char {
    match c {
        Color::Reset => 'A',
        Color::Red => 'C',
        Color::Blue => 'F',
        Color::Magenta => 'G',
        Color::Cyan => 'H',
        _ => panic!("unexpected {c}"),
    }
}
fn snapshot(buf: &Buffer) -> String {
    let mut out = String::new();
    for y in buf.area.rows() {
        let mut covered = 0;
        for x in 0..buf.area.width {
            if covered > 0 {
                out.push('~');
                covered -= 1;
                continue;
            }
            let cell = &buf[(x, y.y)];
            let flags = u8::from(cell.modifier.contains(Modifier::BOLD))
                + 2 * u8::from(cell.modifier.contains(Modifier::ITALIC));
            write!(
                out,
                "{}{}{}{}|",
                color(cell.fg),
                color(cell.bg),
                char::from(65 + flags),
                cell.symbol()
            )
            .unwrap();
            covered = cell.symbol().cell_width().saturating_sub(1);
        }
    }
    out
}
fn overflow(p: &Paragraph, w: u16) -> Option<u16> {
    let mut probe = Buffer::empty(Rect::new(0, 0, w + 16, 96));
    for y in 0..96 {
        for x in w..w + 16 {
            probe[(x, y)].set_symbol("#");
        }
    }
    p.clone()
        .scroll((0, 0))
        .render(Rect::new(0, 0, w, 96), &mut probe);
    (0..96).find(|&y| {
        (w..w + 16).any(|x| probe[(x, y)].symbol() != "#")
            || (0..w).any(|x| x + probe[(x, y)].symbol().cell_width() > w)
    })
}
fn main() {
    let mut input = io::stdin().lock().lines();
    if std::env::args().nth(1).as_deref() == Some("--emoji") {
        let corpus =
            std::fs::read_to_string(std::env::args().nth(2).expect("path to emoji corpus"))
                .unwrap();
        let mut count = 0;
        let mut errors = 0;
        for row in corpus.lines() {
            let symbol: String = row
                .split_whitespace()
                .map(|s| char::from_u32(u32::from_str_radix(s, 16).unwrap()).unwrap())
                .collect();
            let glyphs = Span::raw(&symbol)
                .styled_graphemes(Style::default())
                .count();
            let span = Span::from(format!("{symbol}!")).red().bold();
            let widths = format!("{glyphs} {} {}", symbol.cell_width(), span.width());
            let mut buf = Buffer::empty(Rect::new(0, 0, 12, 8));
            for cell in &mut buf.content {
                cell.set_symbol(".").set_fg(Color::Cyan);
            }
            for (y, w) in [1, 2, 3, 10].into_iter().enumerate() {
                (&span).render(Rect::new(1, y as u16, w, 1), &mut buf);
                buf.set_stringn(
                    1,
                    y as u16 + 4,
                    format!("K{symbol}!"),
                    usize::from(w),
                    span.style,
                );
            }
            let actual = input.next().unwrap().unwrap();
            let frame = input.next().unwrap().unwrap();
            if actual != widths || frame != snapshot(&buf) {
                if errors < 12 {
                    eprintln!(
                        "{row}: Dunder {actual}, Ratatui {widths}, frame matches {}",
                        frame == snapshot(&buf)
                    );
                }
                errors += 1;
            }
            count += 1;
        }
        assert!(count > 0, "empty sequence corpus");
        assert!(input.next().is_none());
        assert_eq!(errors, 0);
        println!(
            "{count} emoji sequences match Ratatui in width, grapheme count and styled cells at four clipping widths"
        );
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--widths") {
        let encoded = input.next().unwrap().unwrap();
        let mut widths = encoded.bytes();
        let mut count = 0;
        let mut errors = 0;
        for c in (0..=0x10ffff)
            .filter_map(char::from_u32)
            .filter(|c| !c.is_control())
        {
            let mut utf = [0; 4];
            let w = c.encode_utf8(&mut utf).cell_width();
            let actual = widths.next().unwrap() - b'0';
            if u16::from(actual) != w {
                errors += 1;
                if errors < 60 {
                    eprintln!("U+{:04X} Dunder {actual}, Ratatui {w}", c as u32);
                }
            }
            count += 1;
        }
        assert!(widths.next().is_none());
        assert_eq!(errors, 0);
        println!("{count} scalar widths match Ratatui");
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--benchmark") {
        let mut doc = paragraph_document::document();
        for w in [44, 60] {
            for top in 0..48 {
                let mut buf = Buffer::empty(Rect::new(0, 0, w, 18));
                doc = doc.scroll((top, 0));
                (&doc).render(buf.area, &mut buf);
                assert_eq!(
                    input.next().unwrap().unwrap(),
                    snapshot(&buf),
                    "width {w}, scroll {top}"
                );
            }
        }
        assert!(input.next().is_none());
        println!("96 benchmark frames match");
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--inline") {
        let mut count = 0;
        for w in [1, 2, 3, 4, 5, 6, 9, 14] {
            for a in [
                None,
                Some(Alignment::Left),
                Some(Alignment::Center),
                Some(Alignment::Right),
            ] {
                for kind in [0, 1, 3] {
                    let mut buf = Buffer::empty(Rect::new(0, 0, w, 3));
                    let mut doc = sample(kind);
                    if let Some(a) = a {
                        doc = doc.alignment(a);
                    }
                    doc.render(buf.area, &mut buf);
                    assert_eq!(
                        input.next().unwrap().unwrap(),
                        snapshot(&buf),
                        "kind {kind}, width {w}, alignment {a:?}"
                    );
                    count += 1;
                }
            }
            for value in [
                "Koi界",
                "\u{301}a",
                "\u{200b}",
                "Kör",
                "👩🏽‍💻",
                "1️⃣",
                "q\u{ad}",
            ] {
                let mut buf = Buffer::empty(Rect::new(0, 0, w, 1));
                value.red().bold().render(buf.area, &mut buf);
                assert_eq!(
                    input.next().unwrap().unwrap(),
                    snapshot(&buf),
                    "span {value:?}, width {w}"
                );
                count += 1;
            }
        }
        assert!(input.next().is_none());
        println!("{count} independent rich-text and span frames match");
        return;
    }
    let mut cases = 0;
    let mut failures = 0;
    let mut panics = 0;
    let mut outside = 0;
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let message = info.payload().downcast_ref::<String>();
        if !message.is_some_and(|s| s.starts_with("index outside of buffer:")) {
            original_hook(info);
        }
    }));
    for kind in 0..4 {
        for w in [1, 2, 3, 4, 5, 6, 9, 14] {
            for a in [
                None,
                Some(Alignment::Left),
                Some(Alignment::Center),
                Some(Alignment::Right),
            ] {
                for wrap in [None, Some(true), Some(false)] {
                    for top in [0, 1, 3, 12] {
                        for left in [0, 1, 3] {
                            let mut buf = Buffer::empty(Rect::new(0, 0, w, 3));
                            let mut p = Paragraph::new(sample(kind))
                                .style(Style::new().bg(Color::Magenta))
                                .scroll((top, left));
                            if let Some(a) = a {
                                p = p.alignment(a);
                            }
                            if let Some(trim) = wrap {
                                p = p.wrap(Wrap { trim });
                            }
                            let actual = input.next().expect("missing Dunder frame").unwrap();
                            cases += 1;
                            let first_overflow = if wrap.is_some() {
                                overflow(&p, w)
                            } else {
                                None
                            };
                            if let Err(payload) =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                    p.render(buf.area, &mut buf)
                                }))
                            {
                                if !payload
                                    .downcast_ref::<String>()
                                    .is_some_and(|s| s.starts_with("index outside of buffer:"))
                                {
                                    std::panic::resume_unwind(payload);
                                }
                                assert!(first_overflow.is_some_and(|y| y < top + 3));
                                panics += 1;
                                continue;
                            }
                            let expected = snapshot(&buf);
                            if expected != actual {
                                if first_overflow.is_some_and(|y| y < top + 3) {
                                    outside += 1;
                                    continue;
                                }
                                failures += 1;
                                if failures <= 6 {
                                    eprintln!(
                                        "kind={kind} w={w} align={a:?} wrap={wrap:?} top={top} left={left}\n  D {actual}\n  R {expected}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(input.next().is_none());
    assert_eq!(failures, 0, "frames differ before any reference overflow");
    assert_eq!((cases, outside, panics), (4608, 432, 12));
    println!(
        "{} identical paragraph frames, {outside} differences after reference overflow, {panics} reference bounds panics",
        cases - outside - panics
    );
}
