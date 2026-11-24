use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
};
use serde_json::{from_str, json, to_string};
use std::io::{self, BufRead};

fn cases() {
    if serde_json::to_value(ratatui::buffer::Cell::EMPTY)
        .unwrap()
        .get("underline_color")
        .is_none()
    {
        for s in [
            r#"{"underline_color":{"invalid":17},"underline_color":42,"add_modifier":"UNDERLINED"}"#,
            r#"{"underline_color":false,"under\u006cine_color":[],"fg":"Red"}"#,
            r#"["Yellow","Blue","UNDERLINED","BOLD"]"#,
            r#"[null,null,"UNDERLINED"]"#,
            r#"["Red","Blue",null,"BOLD","DIM"]"#,
        ] {
            println!("S\t{s}");
        }
    }
    for color in [
        r##""#FA00ff""##,
        r##""#+F+2+9""##,
        r#""Bright-Silver""#,
        r#""+01""#,
        r#""White""#,
        r#""Reset""#,
        "null",
        "2",
        "[]",
        "{}",
        r#""256""#,
        r#"{"Rgb":[256,0,1]}"#,
        r#"{"Rgb":[1.0,2,3]}"#,
        r#"{"Rgb":[1,2,3,4]}"#,
        r#"{"Indexed":-1}"#,
        r#"{"Indexed":-0}"#,
        r#"{"indexed":7}"#,
        r#"{"Indexed":1,"Rgb":[0,0,0]}"#,
        r#"{"Indexed":1,"Indexed":2}"#,
    ] {
        println!("C\t{color}");
    }
    for i in 0..=255u8 {
        println!("C\t{}", json!(i.to_string()));
        println!("C\t{}", json!({"Indexed": i}));
        println!("C\t{}", json!({"Rgb": [i, !i, i.rotate_left(3)]}));
    }
    for style in [
        "{}",
        "null",
        "[]",
        "[null]",
        "[null,null]",
        "[null,null,null]",
        r#"["Red",null,"Green","DIM"]"#,
        "[null,null,null,null,null,null]",
        r#"{"fg":null,"add_modifier":null}"#,
        r#"{"future":{"x":0},"future":1}"#,
        r#"{"fg":null,"f\u0067":"Red"}"#,
        r#"{"sub_modifier":null,"sub_modifier":null}"#,
        r#"{"fg":"bad"}"#,
        r#"{"add_modifier":[]}"#,
        r#"{"sub_modifier":"unknown"}"#,
        r#"{"add_modifier":"0xFFFF","sub_modifier":"0x2201"}"#,
        r#"{"add_modifier":"0x+2 | BOLD"}"#,
        r#"{"add_modifier":"0X02"}"#,
        r#"{"add_modifier":"\u2003ITALIC\u0085|\tDIM\u000b"}"#,
        r#"{"add_modifier":" \n"}"#,
        r#"{"add_modifier":"BOLD|"}"#,
        r#"{"add_modifier":"0x10000"}"#,
    ] {
        println!("S\t{style}");
    }
    for i in 0..512u16 {
        let add = Modifier::from_bits_retain(i);
        let sub = Modifier::from_bits_retain(i.rotate_left(3));
        println!(
            "S\t{}",
            json!({"fg": {"Indexed": i % 256}, "bg": "bright-grey",
            "underline_color": {"Rgb":[111,23,53]}, "add_modifier":add, "sub_modifier":sub})
        );
        println!(
            "S\t{}",
            json!({"add_modifier":format!("0x{:x}", i.rotate_right(3))})
        );
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
        if kind == "C" {
            println!(
                "{}",
                from_str::<Color>(input)
                    .map(|c| to_string(&c).unwrap())
                    .unwrap_or_else(|_| "error".into())
            );
        } else if let Ok(style) = from_str::<Style>(input) {
            let mut buf = Buffer::empty(Rect::new(0, 0, 2, 1));
            let seed = Style::new()
                .fg(Color::Blue)
                .bg(Color::Indexed(7))
                .add_modifier(Modifier::BOLD | Modifier::DIM);
            buf[(1, 0)].set_style(seed);
            for cell in &mut buf.content {
                cell.set_symbol("x").set_style(style);
            }
            println!(
                "{}\t{}\t{}\t{}",
                to_string(&style).unwrap(),
                to_string(&seed.patch(style)).unwrap(),
                to_string(&buf[(0, 0)].style()).unwrap(),
                to_string(&buf[(1, 0)].style()).unwrap()
            );
        } else {
            println!("error");
        }
    }
}
