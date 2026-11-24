use ratatui::{
    buffer::{Buffer, Cell, CellDiffOption},
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
};
use serde_json::Value;
use std::io::{self, BufRead};
use std::num::NonZeroU16;
#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;

const WORDS: &[&str] = &[
    "",
    " ",
    "A",
    "ABCD",
    "茶",
    "éZ",
    "🧑‍🚀a",
    "\u{301}カﾞﾟ",
    "\tA\r\nB\u{85}C",
    "لا",
    "😶‍🌫",
    "X🇸🇪Y",
    "X\u{200b}Y",
    "࢏",
];

fn seed() -> Cell {
    let mut cell = Cell::from('.');
    #[allow(deprecated)]
    cell.set_fg(Color::Indexed(1))
        .set_bg(Color::Blue)
        .set_style(
            Style::new()
                .underline_color(Color::Rgb(8, 9, 10))
                .add_modifier(Modifier::ITALIC),
        )
        .set_diff_option(CellDiffOption::AlwaysUpdate)
        .set_skip(true);
    cell
}

fn styles() -> [Style; 6] {
    [
        Style::new(),
        Style::new().fg(Color::Red).bg(Color::Reset).add_modifier(Modifier::BOLD),
        Style::new().underline_color(Color::Indexed(13)).remove_modifier(Modifier::ITALIC).add_modifier(Modifier::SLOW_BLINK),
        Style::new().fg(Color::Indexed(1)).bg(Color::Rgb(3,5,7)).add_modifier(Modifier::ITALIC),
        serde_json::from_str(r#"{"fg":null,"bg":null,"underline_color":null,"add_modifier":"BOLD | 0x8000","sub_modifier":"BOLD | SLOW_BLINK"}"#).unwrap(),
        Style::reset(),
    ]
}

fn cell_edits(record: &mut impl FnMut(&Buffer, (u16, u16))) {
    let mut b = Buffer::filled(Rect::new(3, 7, 3, 1), seed());
    for old in ["", " ", "┘", "╭", "┏", "茶", "ab", "é"] {
        for new in ["", "┏", "┘", "╆", "茶", "ab", " "] {
            for strategy in [
                MergeStrategy::Replace,
                MergeStrategy::Exact,
                MergeStrategy::Fuzzy,
            ] {
                b[(3, 7)].reset();
                b[(4, 7)].set_symbol(old);
                b[(5, 7)].set_char('┤');
                for x in 3..6 {
                    b[(x, 7)].merge_symbol(new, strategy);
                }
                record(&b, (0, 0));
            }
        }
    }
    for rule in [
        CellDiffOption::None,
        CellDiffOption::Skip,
        CellDiffOption::AlwaysUpdate,
        CellDiffOption::ForcedWidth(NonZeroU16::MIN),
        CellDiffOption::ForcedWidth(NonZeroU16::MAX),
    ] {
        for skip in [false, true] {
            for s in styles() {
                #[allow(deprecated)]
                b[(3, 7)].set_skip(skip).set_diff_option(rule).set_style(s);
                assert_eq!(b[(3, 7)].symbol(), " ");
                let width = match rule {
                    CellDiffOption::ForcedWidth(n) => n.get(),
                    _ => 1,
                };
                record(&b, (width, 0));
            }
        }
    }
}

fn write_text(record: &mut impl FnMut(&Buffer, (u16, u16))) {
    let area = Rect::new(10, 6, 7, 2);
    for &value in WORDS {
        for x in [10, 12, 16, 17] {
            for limit in [0, 1, 2, 3, 7, 65535u16] {
                let mut b = Buffer::filled(area, seed());
                let style = Style::new()
                    .fg(Color::LightRed)
                    .remove_modifier(Modifier::ITALIC);
                let end = b.set_stringn(x, 6, value, usize::from(limit), style);
                record(&b, end);
                let span = Span::styled(value, style);
                let end = b.set_span(x, 7, &span, limit);
                record(&b, end);
                let line = Line::from(vec![span, Span::raw("a茶"), Span::raw("!")])
                    .style(Style::new().bg(Color::Green).add_modifier(Modifier::ITALIC))
                    .right_aligned();
                let end = b.set_line(x, 7, &line, limit);
                record(&b, end);
            }
        }
    }
    let mut b = Buffer::filled(area, seed());
    for y in 6..8 {
        b.set_string(11, y, "A茶Z", Color::Magenta);
    }
    record(&b, (0, 0));
    for region in [Rect::new(0, 5, 15, 2), area, Rect::new(17, 7, 2, 1)] {
        for s in styles() {
            b.set_style(region, s);
            record(&b, (0, 0));
        }
    }
    for next in [
        Rect::new(14, 7, 2, 7),
        Rect::new(1, 1, 3, 1),
        Rect::new(65530, 65533, 5, 2),
        Rect::new(4, 4, 0, 5),
    ] {
        b.resize(next);
        record(&b, (0, 0));
    }
}

fn line_buffers(record: &mut impl FnMut(&Buffer, (u16, u16))) {
    record(&Buffer::with_lines(Vec::<String>::new()), (0, 0));
    for &v in WORDS {
        let short = Span::styled(v, Color::Cyan);
        let row = Line::from(vec![short.clone(), Span::raw("!")])
            .style(Style::new().bg(Color::Blue))
            .centered();
        record(&Buffer::with_lines([v, "A\r\n茶\nz", "XY"]), (0, 0));
        record(&Buffer::with_lines([short, Span::raw("XY")]), (0, 0));
        let mut b = Buffer::with_lines([row, Line::raw("XY")]);
        record(&b, (0, 0));
        b.reset();
        record(&b, (0, 0));
    }
}

fn moving(timing: bool, check: bool) {
    let rows = ["A茶é🦀..", "B氷ä🌙!.", "C海ô🪼;.", "D山ů🦉?."].map(|s| s.repeat(12));
    let mut b = Buffer::empty(Rect::new(0, 0, 96, 24));
    let mut samples = Vec::with_capacity(if timing { 100 } else { 0 });
    let mut lines = io::stdin().lock().lines();
    for i in 0..if check { 4 } else { 110 } {
        let style = if i % 2 == 0 {
            Style::new()
                .fg(Color::Green)
                .add_modifier(Modifier::UNDERLINED)
        } else {
            Style::new().fg(Color::Cyan).add_modifier(Modifier::ITALIC)
        };
        let start = timing.then(std::time::Instant::now);
        for y in 0..24 {
            b.set_string(0, y, &rows[(i + usize::from(y)) % 4], style);
        }
        std::hint::black_box(&b);
        if let Some(t) = start
            && i >= 10
        {
            samples.push(t.elapsed().as_nanos());
        }
        if check {
            let actual: Value = serde_json::from_str(&lines.next().unwrap().unwrap()).unwrap();
            assert_eq!(
                actual,
                serde_json::to_value(&b).unwrap(),
                "motion frame {i}"
            );
        }
    }
    if check {
        assert!(lines.next().is_none());
        println!("9216 moving cells matched Ratatui");
        return;
    }
    assert_eq!(b[(0, 23)].symbol(), "A");
    assert_eq!(b[(95, 23)].fg, Color::Cyan);
    if timing {
        samples.sort_unstable();
        println!("moving {} {}", samples[49], samples[94]);
    }
}

fn main() {
    let bench = std::env::args().nth(1);
    if bench.as_deref() == Some("--check-moving") {
        moving(false, true);
        return;
    }
    if matches!(bench.as_deref(), Some("--scalars" | "--raw-widths")) {
        use ratatui::buffer::CellWidth;
        let raw = bench.as_deref() == Some("--raw-widths");
        let widths = io::read_to_string(io::stdin()).unwrap();
        let mut observed = widths.trim_end().bytes();
        let mut b = Buffer::empty(Rect::new(0, 0, 4, 1));
        let mut total = 0;
        for ch in char::MIN..=char::MAX {
            let mut bytes = [0; 4];
            let symbol = ch.encode_utf8(&mut bytes);
            let x = if raw {
                Cell::from(ch).cell_width()
            } else {
                b.set_stringn(0, 0, &*symbol, 4, Style::new()).0
            };
            if !raw && x > 0 {
                assert_eq!(b[(0, 0)].symbol(), symbol);
            }
            assert_eq!(observed.next(), Some(b'A' + x as u8), "U+{:04X}", ch as u32);
            total += 1;
        }
        assert!(observed.next().is_none());
        println!("{total} Unicode scalars matched Ratatui");
        return;
    }
    let timing = bench.as_deref() == Some("--bench");
    if timing || bench.as_deref() == Some("--allocations") {
        for (name, unit) in [
            ("ascii", "1234567."),
            ("unicode", "A茶é🦀.."),
            ("span", "A茶é🦀.."),
            ("style", ""),
        ] {
            let area = Rect::new(0, 0, 96, 24);
            let mut b = Buffer::empty(area);
            let row = unit.repeat(12);
            let mut span = Span::raw(row.as_str());
            let mut samples = Vec::with_capacity(if timing { 100 } else { 0 });
            for i in 0..110 {
                let s = if i % 2 == 0 {
                    Style::new()
                        .fg(Color::Green)
                        .add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::new().fg(Color::Cyan).add_modifier(Modifier::ITALIC)
                };
                let start = timing.then(std::time::Instant::now);
                if name == "style" {
                    b.set_style(area, s);
                } else if name == "span" {
                    span.style = s;
                    for y in 0..24 {
                        b.set_span(0, y, &span, 96);
                    }
                } else {
                    for y in 0..24 {
                        b.set_string(0, y, &row, s);
                    }
                }
                std::hint::black_box(&b);
                if let Some(t) = start
                    && i >= 10
                {
                    samples.push(t.elapsed().as_nanos());
                }
            }
            let last = &b[(95, 23)];
            assert_eq!(last.fg, Color::Cyan);
            assert!(last.modifier.contains(Modifier::ITALIC));
            assert_eq!(last.symbol(), if name == "style" { " " } else { "." });
            if timing {
                samples.sort_unstable();
                println!("{name} {} {}", samples[49], samples[94]);
            }
        }
        moving(timing, false);
        #[cfg(feature = "allocation-counts")]
        if !timing {
            allocation::report();
        }
        return;
    }
    let mut lines = io::stdin().lock().lines();
    let mut states = 0;
    let mut cells = 0;
    let mut check = |b: &Buffer, (x, y): (u16, u16)| {
        let line = lines.next().expect("missing state").unwrap();
        let (pos, actual) = line.split_once('\t').unwrap();
        assert_eq!(pos, format!("{x} {y}"), "cursor in state {states}");
        assert_eq!(
            serde_json::from_str::<Value>(actual).unwrap(),
            serde_json::to_value(b).unwrap(),
            "cells in state {states}"
        );
        states += 1;
        cells += b.content.len();
    };
    cell_edits(&mut check);
    write_text(&mut check);
    line_buffers(&mut check);
    for w in [0, 1, 4] {
        for h in [0, 1, 3] {
            for x in [0, 3, 9] {
                for y in [0, 2, 6] {
                    let mut base = Buffer::filled(Rect::new(3, 2, w, h), seed());
                    for (i, cell) in base.content.iter_mut().enumerate() {
                        cell.set_char(char::from(b'A' + i as u8));
                    }
                    let overlay = Buffer::filled(Rect::new(x, y, h, w), Cell::new(" "));
                    base.merge(&overlay);
                    check(&base, (0, 0));
                }
            }
        }
    }
    for path in std::env::args().skip(1) {
        for codepoints in std::fs::read_to_string(path).unwrap().lines() {
            let mut s = String::from("X");
            s.extend(
                codepoints
                    .split_whitespace()
                    .map(|cp| char::from_u32(u32::from_str_radix(cp, 16).unwrap()).unwrap()),
            );
            s.push('.');
            let mut b = Buffer::filled(Rect::new(1, 2, 4, 2), seed());
            let (x, y) = b.set_stringn(1, 2, &s, 4, Color::Cyan);
            assert_eq!(b.set_span(1, 3, &Span::styled(s, Color::Cyan), 4), (x, 3));
            check(&b, (x, y));
        }
    }
    assert!(lines.next().is_none(), "extra states");
    println!(
        "{states} buffer states and returned positions, {cells} physical cells matched Ratatui"
    );
}
