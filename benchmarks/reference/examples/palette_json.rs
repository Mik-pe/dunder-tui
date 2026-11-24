use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect, VerticalAlignment},
    style::{
        Color, Modifier, Style,
        palette::{material, tailwind},
    },
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Widget},
};
use serde_json::{Value, from_str, json, to_string, to_value};
use std::io::{self, BufRead};

const BASE: [&str; 10] = [
    "c50", "c100", "c200", "c300", "c400", "c500", "c600", "c700", "c800", "c900",
];
const ACCENTS: [&str; 4] = ["a100", "a200", "a400", "a700"];
const BORDERS: [&str; 12] = [
    "Plain",
    "Rounded",
    "Double",
    "Thick",
    "LightDoubleDashed",
    "HeavyDoubleDashed",
    "LightTripleDashed",
    "HeavyTripleDashed",
    "LightQuadrupleDashed",
    "HeavyQuadrupleDashed",
    "QuadrantInside",
    "QuadrantOutside",
];

fn records(kind: char, keys: &[&str], seeds: &[Value]) {
    for offset in 0..seeds.len() {
        let vals: Vec<_> = (0..keys.len())
            .map(|i| seeds[(i + offset) % seeds.len()].clone())
            .collect();
        let mut obj = json!({});
        for (key, val) in keys.iter().zip(&vals) {
            obj[key] = val.clone();
        }
        println!("{kind}\t{obj}");
        println!("{kind}\t{}", json!(vals));
        obj["future"] = json!({"bad":null});
        println!("{kind}\t{obj}");
    }
    let mut base = json!({});
    for k in keys {
        base[k] = seeds[0].clone();
    }
    for key in keys {
        let mut partial = base.clone();
        partial.as_object_mut().unwrap().remove(*key);
        println!("{kind}\t{partial}");
        let k = to_string(key).unwrap();
        println!(
            "{kind}\t{}",
            base.to_string()
                .replace(&format!("{k}:"), &format!("{k}:{},{k}:", seeds[0]))
        );
        for v in [
            json!(null),
            json!(true),
            json!(-1),
            json!(65536),
            json!({}),
            json!([]),
            json!({"Rgb":[0,0,256]}),
            json!("???"),
        ] {
            let mut bad = base.clone();
            bad[key] = v;
            println!("{kind}\t{bad}");
        }
    }
    let vals = vec![seeds[0].clone(); keys.len() + 1];
    for n in 0..=vals.len() {
        println!("{kind}\t{}", json!(&vals[..n]));
    }
    for v in [json!(null), json!(""), json!(0), json!({}), json!(true)] {
        println!("{kind}\t{v}");
    }
}

fn cases() {
    let colors = vec![
        json!("#Ff00aA"),
        json!("Bright silver"),
        json!("blacK"),
        json!({"Rgb":[32,71,0]}),
        json!({"Indexed":7}),
        json!("255"),
        json!("reset"),
        json!("Blue"),
        json!("White"),
    ];
    for (kind, extra) in [('T', vec!["c950"]), ('M', vec![]), ('A', ACCENTS.to_vec())] {
        let mut keys = BASE.to_vec();
        keys.extend(extra);
        records(kind, &keys, &colors);
    }
    records(
        'P',
        &["left", "right", "top", "bottom"],
        &[
            json!(0),
            json!(1),
            json!(2),
            json!(4),
            json!(65534),
            json!(65535),
        ],
    );
    for (kind, n) in [('t', 22), ('m', 19), ('a', 16)] {
        for i in 0..n {
            println!("{kind}\t{i}");
        }
    }
    for (kind, n) in [('U', 14), ('N', 10)] {
        for high in [0, 0x12000000, 0xfe000000, 0xff000000] {
            let vals: Vec<u32> = (0..n)
                .map(|i| high | (((i + 1) * 0xbdaff) & 0xffffff))
                .collect();
            println!("{kind}\t{}", json!(vals));
        }
        for v in [
            json!([]),
            json!(vec![0; n as usize + 1]),
            json!(vec![-1; n as usize]),
            json!(vec![4294967296u64; n as usize]),
        ] {
            println!("{kind}\t{v}");
        }
    }
    for (kind, names) in [
        ('H', vec!["Left", "Center", "Right"]),
        ('V', vec!["Top", "Center", "Bottom"]),
        ('K', BORDERS.to_vec()),
    ] {
        for n in names {
            println!("{kind}\t{}", json!(n));
            println!("{kind}\t{}", json!({n:null}));
            println!("{kind}\t{}", json!({n:[]}));
            println!("{kind}\t{}", json!(n.to_lowercase()));
        }
        for v in [
            json!(null),
            json!("Inherit"),
            json!("Empty"),
            json!(true),
            json!(12),
            json!([]),
            json!({}),
        ] {
            println!("{kind}\t{v}");
        }
    }
    for i in 0..256 {
        println!("B\t{}", json!(format!("0x{i:02X}")));
    }
    for v in [
        "ALL",
        "NONE",
        "",
        " ",
        "TOP | RIGHT | BOTTOM | LEFT",
        "ALL | 0x80",
        "TOP | TOP",
        " | TOP",
        "LEFT |",
        "top",
        "0x+ff",
        "0x100",
        "0Xf",
        "TOP\u{a0}|\u{2003}RIGHT",
    ] {
        println!("B\t{}", json!(v));
    }
    for v in [json!(null), json!(0), json!(true), json!([]), json!({})] {
        println!("B\t{v}");
    }
    for prefix in [
        "",
        "light",
        "LIGHT",
        "bright",
        "Bri-ght",
        "dark",
        "brightLight",
        "lightbright",
    ] {
        for name in [
            "red",
            "g-r_e eN",
            "Blue",
            "cyan",
            "Magenta",
            "yellow",
            "BlacK",
            "blAck",
            "white",
            "gray",
            "grEy",
            "silver",
            "reSet",
            "",
            "orange",
            "blacK\u{307}",
        ] {
            for sep in ["", " _- ", "\t"] {
                println!("C\t{}", json!(format!("{prefix}{sep}{name}")));
            }
        }
    }
    for i in 0..256 {
        println!("C\t{}", json!(format!("+{i:03}")));
        println!("C\t{}", json!(format!("#{:02X}fF{:02x}", 255 - i, i)));
    }
}

fn swatches(saved: String, colors: &[Color]) -> String {
    let w = colors.len();
    let mut buf = Buffer::empty(Rect::new(1, 2, w as u16, 2));
    for y in 0..2 {
        for x in 0..w {
            buf[(1 + x as u16, 2 + y as u16)]
                .set_symbol(if y == 0 { "▀" } else { "·" })
                .set_style(
                    Style::new()
                        .fg(colors[(x + y) % w])
                        .bg(colors[(x + 3 + y) % w])
                        .underline_color(colors[(x + 7 + y) % w])
                        .add_modifier(Modifier::UNDERLINED),
                );
        }
    }
    format!("{saved}\t{}", to_string(&buf).unwrap())
}

fn align(saved: String, h: Alignment, v: VerticalAlignment) -> String {
    let mut output = saved;
    let flex = match v {
        VerticalAlignment::Top => Flex::Start,
        VerticalAlignment::Center => Flex::Center,
        VerticalAlignment::Bottom => Flex::End,
    };
    for (w, height) in [(2, 3), (7, 5), (10, 4)] {
        let area = Rect::new(1, 2, w, height);
        let row = Layout::vertical([Constraint::Length(1)])
            .flex(flex)
            .split(area)[0];
        let mut buf = Buffer::empty(area);
        Line::from("茶IS")
            .alignment(h)
            .style(Style::new().fg(Color::LightCyan))
            .render(row, &mut buf);
        output.push('\t');
        output.push_str(&to_string(&buf).unwrap());
    }
    output
}

fn block(saved: String, borders: Borders, kind: BorderType, padding: Padding) -> String {
    let area = Rect::new(2, 2, 7, 4);
    let b = Block::new()
        .borders(borders)
        .border_type(kind)
        .padding(padding)
        .border_style(Style::new().fg(Color::Cyan));
    let mut buf = Buffer::empty(Rect::new(1, 1, 9, 6));
    let inner = b.inner(area);
    let far = b.inner(Rect::new(65532, 65530, 2, 4));
    let zero = b.inner(Rect::new(3, 3, 0, 0));
    b.render(area, &mut buf);
    format!(
        "{saved}\t{}\t{}\t{}\t{}",
        to_string(&inner).unwrap(),
        to_string(&far).unwrap(),
        to_string(&zero).unwrap(),
        to_string(&buf).unwrap()
    )
}

fn check(kind: &str, input: &str) -> serde_json::Result<String> {
    let tw = [
        tailwind::SLATE,
        tailwind::GRAY,
        tailwind::ZINC,
        tailwind::NEUTRAL,
        tailwind::STONE,
        tailwind::RED,
        tailwind::ORANGE,
        tailwind::AMBER,
        tailwind::YELLOW,
        tailwind::LIME,
        tailwind::GREEN,
        tailwind::EMERALD,
        tailwind::TEAL,
        tailwind::CYAN,
        tailwind::SKY,
        tailwind::BLUE,
        tailwind::INDIGO,
        tailwind::VIOLET,
        tailwind::PURPLE,
        tailwind::FUCHSIA,
        tailwind::PINK,
        tailwind::ROSE,
    ];
    let ac = [
        material::RED,
        material::PINK,
        material::PURPLE,
        material::DEEP_PURPLE,
        material::INDIGO,
        material::BLUE,
        material::LIGHT_BLUE,
        material::CYAN,
        material::TEAL,
        material::GREEN,
        material::LIGHT_GREEN,
        material::LIME,
        material::YELLOW,
        material::AMBER,
        material::ORANGE,
        material::DEEP_ORANGE,
    ];
    let m = [material::BROWN, material::GRAY, material::BLUE_GRAY];
    let result = match kind {
        "C" => to_string(&from_str::<Color>(input)?)?,
        "H" => {
            let a = from_str::<Alignment>(input)?;
            align(to_string(&a)?, a, VerticalAlignment::Center)
        }
        "V" => {
            let a = from_str::<VerticalAlignment>(input)?;
            align(to_string(&a)?, Alignment::Center, a)
        }
        "B" => {
            let b = from_str::<Borders>(input)?;
            block(
                to_string(&b)?,
                b,
                BorderType::Rounded,
                Padding::new(1, 0, 0, 1),
            )
        }
        "K" => {
            let k = from_str::<BorderType>(input)?;
            block(to_string(&k)?, Borders::ALL, k, Padding::ZERO)
        }
        "P" => {
            let p = from_str::<Padding>(input)?;
            block(
                to_string(&p)?,
                Borders::TOP | Borders::RIGHT,
                BorderType::Double,
                p,
            )
        }
        "T" | "t" => {
            let p = if kind == "T" {
                from_str::<tailwind::Palette>(input)?
            } else {
                tw.into_iter().nth(from_str(input)?).unwrap()
            };
            swatches(
                to_string(&p)?,
                &[
                    p.c50, p.c100, p.c200, p.c300, p.c400, p.c500, p.c600, p.c700, p.c800, p.c900,
                    p.c950,
                ],
            )
        }
        "M" | "m" | "N" => {
            let p = match kind {
                "M" => from_str::<material::NonAccentedPalette>(input)?,
                "N" => material::NonAccentedPalette::from_variants(from_str(input)?),
                _ => {
                    let i = from_str::<usize>(input)?;
                    if i < 16 {
                        serde_json::from_value(to_value(ac[i])?)?
                    } else {
                        m[i - 16]
                    }
                }
            };
            swatches(
                to_string(&p)?,
                &[
                    p.c50, p.c100, p.c200, p.c300, p.c400, p.c500, p.c600, p.c700, p.c800, p.c900,
                ],
            )
        }
        _ => {
            let p = match kind {
                "A" => from_str::<material::AccentedPalette>(input)?,
                "U" => material::AccentedPalette::from_variants(from_str(input)?),
                _ => ac[from_str::<usize>(input)?],
            };
            swatches(
                to_string(&p)?,
                &[
                    p.c50, p.c100, p.c200, p.c300, p.c400, p.c500, p.c600, p.c700, p.c800, p.c900,
                    p.a100, p.a200, p.a400, p.a700,
                ],
            )
        }
    };
    Ok(result)
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("--cases") {
        cases();
        return;
    }
    for line in io::stdin().lock().lines().map_while(Result::ok) {
        let (k, v) = line.split_once('\t').unwrap();
        println!("{}", check(k, v).unwrap_or_else(|_| "error".into()));
    }
}
