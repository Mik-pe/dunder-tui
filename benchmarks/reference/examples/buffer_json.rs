use std::io::{self, BufRead};

use ratatui::buffer::{Buffer, Cell, CellDiffOption};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use serde_json::{Value, from_str, json, to_string, to_value};

fn buffer(x: u16, y: u16, width: u16, height: u16, cells: &[Value]) -> Value {
    json!({"area": {"x":x, "y":y, "width":width, "height":height}, "content":cells})
}

fn cases() {
    let empty = to_value(Cell::EMPTY).unwrap();
    let symbols = [
        json!(null),
        json!(" "),
        json!(""),
        json!("A"),
        json!("界"),
        json!("♥️"),
        json!("e\u{301}"),
        json!("🧑🏽‍🚀"),
        json!("abc"),
        json!("\t"),
        json!("\n"),
        json!("\u{301}"),
        json!("\0"),
        json!("⣿"),
        json!("\u{7f}"),
    ];
    let options = [
        json!("None"),
        json!("Skip"),
        json!("AlwaysUpdate"),
        json!({"ForcedWidth":1}),
        json!({"ForcedWidth":2}),
        json!({"ForcedWidth":65535}),
    ];
    for option in &options {
        println!("O\t{option}");
    }
    for s in [
        r#"{"None":null}"#,
        r#"{"Skip":[]}"#,
        r#"{"ForcedWidth":0}"#,
        r#"{"ForcedWidth":65536}"#,
        r#"{"ForcedWidth":-1}"#,
        r#"{"ForcedWidth":1.0}"#,
        r#"{"None":null,"Skip":null}"#,
        r#""ForcedWidth""#,
        r#"null"#,
        r#"[]"#,
    ] {
        println!("O\t{s}");
    }
    for flags in 0..512u16 {
        let m = Modifier::from_bits_retain(flags);
        println!("M\t{}", json!(m));
        let mut cell = empty.clone();
        cell["modifier"] = json!(Modifier::from_bits_retain(flags.rotate_right(2)));
        cell["fg"] = json!({"Indexed":flags % 256});
        cell["bg"] = json!({"Rgb":[flags%256,17,92]});
        cell["underline_color"] = json!("LightBlue");
        cell["symbol"] = symbols[flags as usize % symbols.len()].clone();
        cell["diff_option"] = options[flags as usize % options.len()].clone();
        cell["skip"] = json!(flags % 2 == 1);
        println!("C\t{cell}");
        println!("B\t{}", buffer(2, 3, 1, 1, &[cell]));
    }
    for v in [
        json!(null),
        json!("0xffff"),
        json!(""),
        json!("BOLD||DIM"),
        json!(1),
    ] {
        println!("M\t{v}");
    }
    let keys = [
        "symbol",
        "fg",
        "bg",
        "underline_color",
        "modifier",
        "diff_option",
        "skip",
    ];
    let mut seq = Vec::new();
    for key in keys {
        let mut missing = empty.clone();
        missing.as_object_mut().unwrap().remove(key);
        println!("C\t{missing}");
        let mut invalid = empty.clone();
        invalid[key] = json!(17);
        println!("C\t{invalid}");
        seq.push(empty[key].clone());
        println!("C\t{}", json!(&seq));
    }
    seq.push(json!(false));
    println!("C\t{}", json!(&seq));
    let mut extra = empty.clone();
    extra["unknown"] = json!([4]);
    println!("C\t{extra}");
    println!(
        "C\t{}",
        to_string(&empty)
            .unwrap()
            .replace("\"skip\":", "\"skip\":true,\"skip\":")
    );
    for (x, y, w, h, n) in [
        (0, 0, 0, 0, 0),
        (65535, 65535, 0, 0, 0),
        (65534, 65534, 1, 1, 1),
        (65535, 4, 1, 1, 1),
        (0, 65535, 1, 1, 1),
        (0, 0, 1, 1, 0),
        (2, 1, 0, 0, 1),
    ] {
        println!("B\t{}", buffer(x, y, w, h, &vec![empty.clone(); n]));
    }
    for head in &symbols {
        for (idx, tail) in symbols.iter().enumerate() {
            let mut old = vec![empty.clone(); 8];
            old[1]["symbol"] = head.clone();
            old[1]["bg"] = json!("Blue");
            old[2]["symbol"] = tail.clone();
            for (n, option) in options.iter().enumerate() {
                let mut next = old.clone();
                next[1]["symbol"] = tail.clone();
                next[1]["diff_option"] = option.clone();
                next[1]["skip"] = json!(idx % 2 == 1);
                next[2]["symbol"] = head.clone();
                next[5]["diff_option"] = options[(n + 2) % options.len()].clone();
                next[7]["symbol"] = json!("X");
                let a = buffer(3, 5, 4, 2, &old);
                let b = buffer(3, 5, 4, 2, &next);
                println!("P\t{}", json!([a, b]));
                println!("P\t{}", json!([b, a]));
                if n == 0 {
                    println!("B\t{a}");
                }
            }
        }
    }
    for sample in [
        "",
        "a b",
        "界 abc",
        "x❤️y",
        "abcd🧑🏽‍🚀 🦀",
        "e\u{301}å",
        "0123456789012界",
    ] {
        println!("T\t{}", json!(sample));
    }
}

fn check(kind: &str, input: &str) -> serde_json::Result<String> {
    match kind {
        "O" => to_string(&from_str::<CellDiffOption>(input)?),
        "M" => to_string(&from_str::<Modifier>(input)?),
        "C" => to_string(&from_str::<Cell>(input)?),
        "B" => {
            let buf: Buffer = from_str(input)?;
            let a = buf.area;
            let valid = a.x.checked_add(a.width).is_some()
                && a.y.checked_add(a.height).is_some()
                && a.area() as usize == buf.content.len();
            Ok(format!(
                "{}\t{}",
                to_string(&buf)?,
                if valid { "ok" } else { "invalid area" }
            ))
        }
        "P" => {
            let [a, b]: [Buffer; 2] = from_str(input)?;
            to_string(
                &a.diff_iter(&b)
                    .flat_map(|(x, y, _)| [x, y])
                    .collect::<Vec<_>>(),
            )
        }
        _ => {
            let mut buf = Buffer::empty(Rect::new(3, 5, 14, 2));
            buf.set_string(
                3,
                5,
                from_str::<String>(input)?,
                Style::new()
                    .fg(Color::Green)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );
            let saved = to_string(&buf)?;
            let mut loaded: Buffer = from_str(&saved)?;
            loaded.set_string(3, 6, "  OK", Style::new().fg(Color::Red));
            Ok(format!("{saved}\t{}", to_string(&loaded)?))
        }
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
