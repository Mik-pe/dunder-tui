use std::num::NonZeroU16;
use std::panic::{self, AssertUnwindSafe};

use ratatui::backend::TestBackend;
use ratatui::buffer::{Buffer, Cell, CellDiffOption};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier};
use serde_json::{Value, json};

fn backend(screen: &Buffer, history: &Buffer) -> TestBackend {
    serde_json::from_value(json!({
        "buffer": screen, "scrollback": history, "cursor": false, "pos": [2, 1]
    }))
    .unwrap()
}

fn emit(device: &TestBackend, mode: &str, expected: Value, diagnostic: &str) {
    let mut device = device.clone();
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(AssertUnwindSafe(|| match mode {
        "screen" | "history" => {
            let buffer: Buffer = serde_json::from_value(expected.clone()).unwrap();
            if mode == "screen" {
                device.assert_buffer(&buffer);
            } else {
                device.assert_scrollback(&buffer);
            }
        }
        "lines" | "history-lines" => {
            let lines: Vec<String> = serde_json::from_value(expected.clone()).unwrap();
            if mode == "lines" {
                device.assert_buffer_lines(lines);
            } else {
                device.assert_scrollback_lines(lines);
            }
        }
        "empty" => device.assert_scrollback_empty(),
        "cursor" => {
            let [x, y]: [u16; 2] = serde_json::from_value(expected.clone()).unwrap();
            device.assert_cursor_position((x, y));
        }
        _ => unreachable!(),
    }));
    panic::set_hook(hook);
    let outcome = if result.is_ok() { "pass" } else { "reject" };
    assert_eq!(result.is_ok(), diagnostic.is_empty(), "{mode} {expected}");
    let marker = if diagnostic.is_empty() {
        "-"
    } else {
        diagnostic
    };
    let case = format!("{}\t{expected}\t{outcome}\t{marker}", json!(device));
    println!("{mode}\t{case}");
    if matches!(mode, "screen" | "history") {
        println!("raw-{mode}\t{case}");
    }
}

fn variants() -> Vec<Cell> {
    let mut styled = Cell::new(" ");
    styled.modifier = Modifier::BOLD;
    let mut underlined = Cell::EMPTY;
    underlined.underline_color = Color::LightCyan;
    let mut skipped = Cell::EMPTY;
    #[allow(deprecated)]
    {
        skipped.skip = true;
    }
    let mut cells = vec![
        Cell::EMPTY,
        Cell::new(" "),
        Cell::new(""),
        Cell::new("茶"),
        Cell::new("e\u{301}"),
        Cell::new("🧑‍🚀"),
        styled,
        underlined,
        skipped,
    ];
    for color in [Color::Red, Color::Indexed(1), Color::Rgb(1, 2, 3)] {
        let mut fg = Cell::EMPTY;
        fg.set_fg(color);
        cells.push(fg);
        let mut bg = Cell::EMPTY;
        bg.set_bg(color);
        cells.push(bg);
    }
    for rule in [
        CellDiffOption::AlwaysUpdate,
        CellDiffOption::Skip,
        CellDiffOption::ForcedWidth(NonZeroU16::new(2).unwrap()),
    ] {
        let mut cell = Cell::EMPTY;
        cell.set_diff_option(rule);
        cells.push(cell);
    }
    cells
}

fn main() {
    let cells = variants();
    for i in 0..cells.len() {
        for mode in ["screen", "history"] {
            let mut base = Buffer::with_lines(["head", "茶 q"]);
            base[(1, 1)] = cells[i].clone();
            let device = backend(&base, &base);
            let name = if mode == "screen" {
                "screen"
            } else {
                "scrollback"
            };
            for j in [i, (i + 1) % cells.len(), (i + 7) % cells.len(), 0] {
                let mut other = base.clone();
                other[(1, 1)] = cells[j].clone();
                let diagnostic = if cells[i] == cells[j] {
                    String::new()
                } else {
                    format!("{name} differs at (1, 1)")
                };
                emit(&device, mode, json!(other), &diagnostic);
            }
        }
    }
    let lines = ["a茶", "x y"];
    let screen = Buffer::with_lines(lines);
    let empty = Buffer::empty(Rect::new(0, 0, 3, 0));
    let dev = backend(&screen, &screen);
    for (mode, name) in [("lines", "screen"), ("history-lines", "scrollback")] {
        emit(&dev, mode, json!(lines), "");
        emit(
            &dev,
            mode,
            json!(["a茶", "x!y"]),
            &format!("{name} differs at (1, 1)"),
        );
        emit(&dev, mode, json!(["a"]), &format!("{name} area differs"));
        emit(&dev, mode, json!([]), &format!("{name} area differs"));
    }
    for (mode, name) in [("screen", "screen"), ("history", "scrollback")] {
        let mut shifted = screen.clone();
        shifted.area.x += 1;
        emit(&dev, mode, json!(shifted), &format!("{name} area differs"));
    }
    emit(&dev, "empty", Value::Null, "expected empty scrollback");
    emit(&backend(&screen, &empty), "empty", Value::Null, "");
    emit(&backend(&screen, &empty), "history", json!(empty), "");
    emit(&dev, "cursor", json!([2, 1]), "");
    emit(&dev, "cursor", json!([2, 0]), "expected cursor");
    for lines in [
        vec![],
        vec!["", ""],
        vec!["\r\nA\nB", "C D"],
        vec!["e\u{301}🦀", "a\"b"],
    ] {
        let screen = Buffer::with_lines(lines.iter().copied());
        let dev = backend(&screen, &screen);
        for mode in ["lines", "history-lines"] {
            emit(&dev, mode, json!(lines), "");
        }
    }
}
