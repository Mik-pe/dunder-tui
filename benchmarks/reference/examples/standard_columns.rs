use ratatui::{buffer::CellWidth, text::Span};
use std::io::{self, BufRead};

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let mut samples: Vec<String> = ["", "\t", "a\nb", "\r\n", "لا", "ﾞ", "ﾟ", "ｶﾞ", "😶‍🌫️", "😶‍🌫"]
        .map(String::from)
        .into();
    for path in std::env::args().skip(1) {
        samples.extend(std::fs::read_to_string(path).unwrap().lines().map(|line| {
            line.split_whitespace()
                .map(|s| char::from_u32(u32::from_str_radix(s, 16).unwrap()).unwrap())
                .collect::<String>()
        }));
    }
    let (mut widths, mut controls, mut cell_errors, mut std_errors) = (0, 0, 0, 0);
    for s in &samples {
        let row = input.next().unwrap();
        let mut actual = row.split_whitespace().map(|n| n.parse::<i32>().unwrap());
        let segments = std::iter::once(s.as_str()).chain(
            s.char_indices()
                .skip(1)
                .flat_map(|(i, _)| [&s[..i], &s[i..]]),
        );
        for part in segments {
            let std_width = actual.next().unwrap();
            let cell_width = actual.next().unwrap();
            let span_width = actual.next().unwrap();
            assert_eq!(span_width, Span::raw(part).width() as i32, "span {part:?}");
            let has_control = part.chars().any(char::is_control);
            let want = if has_control {
                -1
            } else {
                i32::from(part.cell_width())
            };
            if std_width != want {
                if std_errors < 8 {
                    eprintln!(
                        "std {part:?} = {std_width} expected {want}, Span width {}",
                        Span::raw(part).width()
                    );
                }
                std_errors += 1;
            }
            if cell_width != i32::from(part.cell_width()) {
                if cell_errors < 8 {
                    eprintln!(
                        "TUI {part:?} = {cell_width}, expected {}",
                        part.cell_width()
                    );
                }
                cell_errors += 1;
            }
            widths += 1;
            controls += usize::from(has_control);
        }
        assert!(actual.next().is_none());
    }
    assert!(input.next().is_none());
    println!(
        "{} inputs, {widths} widths, {controls} control spans, {std_errors} std mismatches, {cell_errors} TUI mismatches",
        samples.len()
    );
    assert_eq!((std_errors, cell_errors), (0, 0));
}
