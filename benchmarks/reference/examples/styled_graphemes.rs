use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, StyledGrapheme};
use std::fmt::Write;
use std::io::{self, BufRead};

fn style(id: &str) -> Style {
    match id {
        "0" => Style::new(),
        "1" => Style::reset(),
        "2" => Style::new()
            .green()
            .bg(Color::Indexed(173))
            .underline_color(Color::Rgb(0x25, 0x4f, 0x91))
            .bold()
            .slow_blink()
            .hidden()
            .not_italic(),
        "3" => Style::new()
            .fg(Color::Rgb(0xab, 0x78, 0x03))
            .underline_color(Color::Cyan)
            .dim()
            .italic()
            .crossed_out()
            .not_bold(),
        "4" => Style::new()
            .on_blue()
            .underlined()
            .rapid_blink()
            .reversed()
            .not_hidden(),
        _ => panic!("unknown style {id}"),
    }
}

fn color(value: Option<Color>) -> i32 {
    match value {
        None => -2,
        Some(Color::Reset) => -1,
        Some(Color::Green) => 2,
        Some(Color::Blue) => 4,
        Some(Color::Cyan) => 6,
        Some(Color::Indexed(n)) => n.into(),
        Some(Color::Rgb(r, g, b)) => {
            0x1000000 + i32::from(r) * 65536 + i32::from(g) * 256 + i32::from(b)
        }
        _ => panic!("unexpected color {value:?}"),
    }
}

fn describe(g: StyledGrapheme, visitor: bool) -> String {
    let mut hex = String::new();
    for byte in g.symbol.bytes() {
        write!(hex, "{byte:02x}").unwrap();
    }
    let s = g.style;
    let item = if visitor {
        format!("v {hex}")
    } else {
        format!("g {hex} {}", u8::from(g.is_whitespace()))
    };
    format!(
        "{item} {} {} {} {} {}",
        color(s.fg),
        color(s.bg),
        color(s.underline_color),
        s.add_modifier.bits(),
        s.sub_modifier.bits()
    )
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let mut cases = 0;
    let mut symbols = 0;
    while let Some(header) = input.next() {
        let (base, row) = header
            .strip_prefix("case ")
            .unwrap()
            .split_once(' ')
            .unwrap();
        let mut spans = Vec::new();
        for _ in 0..4 {
            let fields = input.next().unwrap();
            let (id, hex) = fields
                .strip_prefix("span ")
                .unwrap()
                .split_once(' ')
                .unwrap();
            let bytes = (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
                .collect();
            spans.push(Span::styled(String::from_utf8(bytes).unwrap(), style(id)));
        }
        let line = Line::from(spans).style(style(row));
        for span in &line.spans {
            assert_eq!(input.next().as_deref(), Some("s"));
            for visitor in [false, true] {
                for g in span.styled_graphemes(style(base)) {
                    assert_eq!(
                        input.next().unwrap(),
                        describe(g, visitor),
                        "span in case {cases}"
                    );
                    symbols += 1;
                }
            }
        }
        assert_eq!(input.next().as_deref(), Some("l"));
        for visitor in [false, true] {
            for g in line.styled_graphemes(style(base)) {
                assert_eq!(
                    input.next().unwrap(),
                    describe(g, visitor),
                    "line in case {cases}"
                );
                symbols += 1;
            }
        }
        assert_eq!(input.next().as_deref(), Some("end"));
        cases += 1;
    }
    assert_eq!(cases, 6920);
    println!("{cases} cases, {symbols} styled graphemes identical");
}
