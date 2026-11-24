use ratatui::{
    Terminal,
    backend::{Backend, ClearType, TestBackend},
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use serde_json::{from_str, json, to_string, to_value};
use std::io::{self, BufRead};

fn cases() {
    if to_value(ratatui::buffer::Cell::EMPTY)
        .unwrap()
        .get("underline_color")
        .is_none()
    {
        let cells = json!([
            ["茶", "Yellow", "Blue", "UNDERLINED", "None", false],
            [null, "Reset", "Reset", "", "None", false]
        ]);
        let row = json!([[0, 0, 2, 1], cells]);
        for seed in 0..6 {
            println!("{seed}\t{}", json!([row, row, true, [1, 0]]));
        }
    }
    for seed in 0..12 {
        println!("C\t{seed}");
    }
    let symbols = ["A", "茶", "🌊", "e\u{301}", "", " ", "☀️", "x"];
    for w in 0..8 {
        for h in 0..5 {
            for seed in 0..6 {
                let mut screen = Buffer::empty(Rect::new(0, 0, w, h));
                let mut history = Buffer::empty(Rect::new(0, 0, w, seed % 4));
                for (j, b) in [&mut screen, &mut history].into_iter().enumerate() {
                    for (i, c) in b.content.iter_mut().enumerate() {
                        let n = i + j + seed as usize;
                        if n % 4 == 0 {
                            continue;
                        }
                        c.set_symbol(symbols[n % symbols.len()]);
                        if n % 3 == 0 {
                            c.set_fg(Color::LightRed);
                            c.set_bg(Color::Rgb(8, 40, 90));
                        }
                        if n % 7 == 0 {
                            c.modifier = Modifier::ITALIC;
                        }
                    }
                }
                let pos = if seed % 3 == 0 {
                    [65535, 65535]
                } else {
                    [seed + 1, seed]
                };
                let mut state = json!({"buffer": screen,"scrollback": history,"cursor":seed % 2 == 0,"pos":pos});
                if seed % 2 == 0 {
                    state["extra"] = json!("ignored");
                } else {
                    state = json!([
                        state["buffer"],
                        state["scrollback"],
                        state["cursor"],
                        state["pos"]
                    ]);
                }
                println!("{seed}\t{state}");
            }
        }
    }
    for seed in 0..6 {
        for (screen, history) in [
            ("茶🌊A", "B茶🌊"),
            ("X茶X", "茶🌊"),
            ("🌊中", "中🌊"),
            ("🧑‍🚀  ", "  🧑‍🚀"),
        ] {
            let screen = Buffer::with_lines([screen; 3]);
            let history = Buffer::with_lines([history; 3]);
            assert_eq!(screen.area.width, history.area.width);
            for rule in [
                json!("None"),
                json!("Skip"),
                json!("AlwaysUpdate"),
                json!({"ForcedWidth":3}),
            ] {
                let mut b =
                    json!({"buffer":screen,"scrollback":history,"cursor":false,"pos":[0,0]});
                b["buffer"]["content"][2]["diff_option"] = rule.clone();
                b["scrollback"]["content"][3]["diff_option"] = rule;
                println!("{seed}\t{b}");
            }
        }
    }
    let base = to_value(TestBackend::new(1, 1)).unwrap();
    for key in ["buffer", "scrollback", "cursor", "pos"] {
        let mut missing = base.clone();
        missing.as_object_mut().unwrap().remove(key);
        println!("0\t{missing}");
        for value in [
            json!(null),
            json!([]),
            json!({}),
            json!("0"),
            json!(true),
            json!(65536),
        ] {
            let mut input = base.clone();
            input[key] = value;
            println!("0\t{input}");
        }
        let input = base.to_string().replace(
            &format!("\"{key}\":"),
            &format!("\"{key}\":{},\"{key}\":", base[key]),
        );
        println!("0\t{input}");
    }
    for pos in [
        "[65535,0]",
        "[0,65536]",
        "[-1,0]",
        "[0,0,0]",
        "[0.0,1]",
        "[1]",
        r#"{"x":1,"y":1}"#,
    ] {
        println!("0\t{}", base.to_string().replace("[0,0]", pos));
    }
    for key in ["buffer", "scrollback"] {
        for (field, val) in [
            ("x", 1),
            ("y", 1),
            ("width", 2),
            ("width", 65535),
            ("height", 65535),
        ] {
            let mut raw = base.clone();
            raw[key]["area"][field] = json!(val);
            println!("0\t{raw}");
        }
        for content in [
            json!([]),
            json!([null]),
            json!({}),
            json!([base["buffer"]["content"][0], base["buffer"]["content"][0]]),
        ] {
            let mut raw = base.clone();
            raw[key]["content"] = content;
            println!("0\t{raw}");
        }
    }
    let b = &base["buffer"];
    let s = &base["scrollback"];
    for v in [
        json!([b, s, true]),
        json!([b, s, true, [1, 2], false]),
        json!(null),
        json!(false),
        json!([]),
    ] {
        println!("0\t{v}");
    }
    for words in [
        vec![],
        vec![""],
        vec!["", ""],
        vec!["a"],
        vec!["茶", "e\u{301}", "A  "],
        vec!["🌊", "👨‍👩‍👧‍👦", "Z"],
        vec!["a\tb", "\u{200b}x", "ab\ncd"],
        vec!["🦀x", "A中B", ""],
    ] {
        println!("L\t{}", json!(words));
    }
}

fn runnable(b: &TestBackend) -> bool {
    let a = b.buffer().area;
    let h = b.scrollback().area;
    [b.buffer(), b.scrollback()].into_iter().all(|buf| {
        buf.area.x == 0
            && buf.area.y == 0
            && usize::from(buf.area.width) * usize::from(buf.area.height) == buf.content.len()
    }) && a.width == h.width
}

fn trace(mut b: TestBackend, seed: u16) -> String {
    let mut parts = vec![to_string(&b).unwrap()];
    if !runnable(&b) {
        parts.push("invalid area".into());
        return parts.join("\t");
    }
    let Rect {
        width: w,
        height: h,
        ..
    } = b.buffer().area;
    if w > 0 && h > 0 {
        b.set_cursor_position((seed % w, (seed / 2) % h)).unwrap();
        if seed % 2 == 0 {
            b.hide_cursor().unwrap();
        } else {
            b.show_cursor().unwrap();
        }
        b.clear_region(
            [
                ClearType::AfterCursor,
                ClearType::BeforeCursor,
                ClearType::CurrentLine,
                ClearType::UntilNewLine,
                ClearType::All,
            ][usize::from(seed % 5)],
        )
        .unwrap();
        parts.push(to_string(&b).unwrap());
    }
    if w > 0 {
        b.append_lines(seed + 1).unwrap();
        b.scroll_region_down(0..h.saturating_sub(1), 1).unwrap();
        b.scroll_region_up(0..h, 2).unwrap();
        parts.push(to_string(&b).unwrap());
    }
    b.resize(1 + seed % 4, 1 + seed % 3);
    parts.push(to_string(&b).unwrap());
    b.append_lines(3).unwrap();
    parts.push(to_string(&b).unwrap());
    let mut terminal = Terminal::new(b).unwrap();
    for (i, word) in ["A茶", "B!"].into_iter().enumerate() {
        terminal
            .draw(|f| {
                let a = f.area();
                f.buffer_mut().set_stringn(
                    0,
                    0,
                    word,
                    a.width as usize,
                    Style::new().fg(Color::Cyan),
                );
                if i == 0 {
                    f.set_cursor_position((0, 0));
                }
            })
            .unwrap();
        parts.push(to_string(terminal.backend()).unwrap());
    }
    parts.join("\t")
}

fn with_lines(data: &str) -> serde_json::Result<String> {
    let words: Vec<String> = from_str(data)?;
    let lines = words.into_iter().enumerate().map(|(i, word)| {
        if i % 2 == 0 {
            Line::from(word)
        } else {
            Line::from(vec![
                Span::styled(word, Style::new().fg(Color::Green)),
                Span::raw("!"),
            ])
            .style(Style::new().bg(Color::Blue).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Right)
        }
    });
    Ok(trace(TestBackend::with_lines(lines), 3))
}

fn history_samples(backend: &TestBackend) -> String {
    let buf = backend.scrollback();
    let height = usize::from(buf.area.height);
    let width = usize::from(buf.area.width);
    let rows = (0..4)
        .chain(height - 32..height)
        .map(|y| Buffer {
            area: Rect::new(0, 0, buf.area.width, 1),
            content: buf.content[y * width..(y + 1) * width].to_vec(),
        })
        .collect::<Vec<_>>();
    format!("{height}\t{}", to_string(&rows).unwrap())
}

fn rollover(seed: u16) -> String {
    let mut device = TestBackend::with_lines(["A茶", "BC ", "🦀x"]);
    device.append_lines(65534).unwrap();
    let strip = Buffer::with_lines(["🌊Z"]);
    device
        .draw(
            strip
                .content
                .iter()
                .enumerate()
                .map(|(i, c)| (i as u16, 0, c)),
        )
        .unwrap();
    device.scroll_region_up(0..3, 3 + seed).unwrap();
    let a = history_samples(&device);
    device.resize(1 + seed % 4, 3);
    let b = history_samples(&device);
    device.append_lines(65535).unwrap();
    format!("{a}\t{b}\t{}", history_samples(&device))
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--cases") {
        cases();
        return;
    }
    for line in io::stdin().lock().lines().map_while(Result::ok) {
        let (kind, data) = line.split_once('\t').unwrap();
        let result = if kind == "C" {
            Ok(rollover(data.parse().unwrap()))
        } else if kind == "L" {
            with_lines(data)
        } else {
            from_str::<TestBackend>(data).map(|b| trace(b, kind.parse().unwrap()))
        };
        println!("{}", result.unwrap_or_else(|_| "error".into()));
    }
}
