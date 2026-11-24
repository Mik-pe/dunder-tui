use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Margin, Offset, Position, Rect, Size, Spacing},
};
use serde_json::{from_str, json, to_string};
use std::io::{self, BufRead};

fn cases() {
    let mut sample = json!({"direction":"Horizontal","constraints":[{"Fill":1}],
        "margin":{"horizontal":0,"vertical":0},"flex":"Start","spacing":{"Space":0}});
    for (kind, values) in [
        (
            "D",
            vec![
                r#""Horizontal""#,
                r#""Vertical""#,
                r#"{"Horizontal":null}"#,
                r#"{"Horizontal":[]}"#,
                r#""horizontal""#,
            ],
        ),
        (
            "F",
            vec![r#""SpaceEvenly""#, r#"{"Legacy":null}"#, r#""Inherit""#],
        ),
        (
            "C",
            vec![
                r#"{"Length":65535}"#,
                r#"{"Length":[2]}"#,
                r#"{"Length":65536}"#,
                r#"{"Ratio":[4294967295,0]}"#,
                r#"{"Ratio":[1]}"#,
                r#"{"Ratio":[1,-1]}"#,
                r#"{"Ratio":[1,2,3]}"#,
                r#"{"Fill":-0}"#,
                r#"{"Fill":1,"Min":2}"#,
            ],
        ),
        (
            "S",
            vec![
                r#"{"Space":0}"#,
                r#"{"Overlap":0}"#,
                r#"{"Overlap":65535}"#,
                r#"{"Space":65536}"#,
                r#"{"Space":1.0}"#,
            ],
        ),
        (
            "M",
            vec![
                "[0,65535]",
                "[1]",
                "[1,2,3]",
                r#"{"vertical":1,"horizontal":0,"unused":2}"#,
                r#"{"vertical":1,"horizontal":2,"vertical":3}"#,
            ],
        ),
        (
            "R",
            vec![
                "[65535,65534,65535,65535]",
                "[0,1,2,3]",
                "[0,1,2]",
                "[0,1,2,3,4]",
                r#"{"x":1,"y":2,"width":3,"height":4,"future":5}"#,
                r#"{"x":1,"\u0078":0,"y":2,"width":0,"height":0}"#,
                "[0,-1,0,0]",
                "[0,0,0,65536]",
            ],
        ),
        (
            "L",
            vec![
                "{}",
                "[]",
                "[null,[],[0,0],null,null]",
                r#"["Vertical",[],[0,0],"End",{"Overlap":0}]"#,
                r#"{"direction":"Horizontal","direction":"Vertical","constraints":[],"margin":[0,0],"flex":"Start","spacing":{"Space":0}}"#,
            ],
        ),
    ] {
        for value in values {
            println!("{kind}\t{value}");
        }
        for value in ["null", "0", "true", r#""unknown""#] {
            println!("{kind}\t{value}");
        }
    }
    for kind in ["P", "Z", "O"] {
        for value in [
            "[0,0]",
            "[1,-1]",
            "[65535,65536]",
            "[-2147483648,2147483647]",
            "[0,2147483648]",
            "[1,2,3]",
            r#"{"x":1,"y":2,"x":3}"#,
            r#"{"x":1,"y":2,"extra":4}"#,
            r#"{"width":2,"height":3}"#,
        ] {
            println!("{kind}\t{value}");
        }
    }
    for x in [0, 3, 100, 65530, 65534, 65535] {
        for y in [0, 4, 65532, 65535] {
            for (width, height) in [(0, 0), (0, 1), (1, 0), (4, 8), (41, 9), (65535, 65535)] {
                println!("G\t{}", json!([x, y, width, height]));
            }
        }
    }
    for axis in ["Vertical", "Horizontal"] {
        for (h, v) in [
            (32767, 0),
            (32768, 0),
            (65535, 32767),
            (0, 32768),
            (0, 65535),
            (65535, 65535),
        ] {
            println!(
                "L\t{}",
                json!({"direction":axis,"constraints":[{"Length":6},{"Fill":1}],"margin":[h,v],
                "flex":"Center","spacing":{"Space":1}})
            );
        }
    }
    for axis in ["Horizontal", "Vertical"] {
        for flex in [
            "Start",
            "Center",
            "End",
            "SpaceBetween",
            "SpaceAround",
            "SpaceEvenly",
            "Legacy",
        ] {
            for gap in [0, 1, 3, 32767, 32768, 32769, 65535] {
                for variant in ["Space", "Overlap"] {
                    for (rules, margin) in [
                        (json!([]), json!([0, 0])),
                        (json!([{"Length":3},{"Fill":2},{"Max":9}]), json!([1, 0])),
                        (
                            json!([{"Ratio":[1,3]},{"Min":1},{"Percentage":42}]),
                            json!([0, 1]),
                        ),
                        (json!([{"Fill":1},{"Length":4}]), json!([2, 3])),
                    ] {
                        sample["direction"] = json!(axis);
                        sample["constraints"] = rules;
                        sample["margin"] = margin;
                        sample["flex"] = json!(flex);
                        sample["spacing"] = json!({(variant):gap});
                        println!("L\t{sample}");
                    }
                }
            }
        }
    }
}

fn render(layout: Layout) -> String {
    let normal = Rect::new(0, 0, 22, 5);
    let mut buf = Buffer::empty(normal);
    let mut fields = vec![to_string(&layout).unwrap()];
    for (i, rect) in [
        normal,
        Rect::new(3, 7, 31, 27),
        Rect {
            x: 65523,
            y: 65530,
            width: 35,
            height: 24,
        },
        Rect::new(0, 0, 65535, 65535),
        Rect {
            x: 65530,
            y: 3,
            width: 65535,
            height: 65535,
        },
    ]
    .into_iter()
    .enumerate()
    {
        let (regions, gaps) = layout.split_with_spacers(rect);
        fields.push(to_string(regions.as_ref()).unwrap());
        fields.push(to_string(gaps.as_ref()).unwrap());
        if i == 0 {
            for (j, r) in regions.iter().enumerate() {
                for y in r.top()..r.bottom() {
                    for x in r.left()..r.right() {
                        buf[(x, y)].set_char((b'A' + j as u8) as char);
                    }
                }
            }
        }
    }
    for row in buf.content.chunks(usize::from(normal.width)) {
        fields.push(row.iter().map(|cell| cell.symbol()).collect());
    }
    fields.join("\t")
}

fn geometry(rect: Rect) -> String {
    let p = rect.as_position();
    let size = rect.as_size();
    let shifted = p.offset(Offset::new(i32::MAX, i32::MIN));
    let other = Rect::new(65529, 2, 6, 45);
    let margin = Margin::new(2, 3);
    [
        to_string(&rect).unwrap(),
        to_string(&Rect::from((p, size))).unwrap(),
        to_string(&size.area()).unwrap(),
        to_string(&p).unwrap(),
        to_string(&shifted).unwrap(),
        to_string(&rect.right()).unwrap(),
        to_string(&rect.bottom()).unwrap(),
        to_string(&rect.inner(margin)).unwrap(),
        to_string(&rect.outer(margin)).unwrap(),
        to_string(&rect.union(other)).unwrap(),
        to_string(&rect.intersection(other)).unwrap(),
        to_string(&rect.rows().nth(2)).unwrap(),
        to_string(&rect.columns().nth(2)).unwrap(),
        to_string(&rect.positions().nth(2)).unwrap(),
    ]
    .join("\t")
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--cases") {
        cases();
        return;
    }
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let (kind, data) = line.split_once('\t').unwrap();
        macro_rules! canon {
            ($t:ty) => {
                from_str::<$t>(data).and_then(|v| to_string(&v))
            };
        }
        let value = match kind {
            "P" => canon!(Position),
            "Z" => canon!(Size),
            "O" => canon!(Offset),
            "G" => from_str::<Rect>(data).map(geometry),
            "D" => canon!(Direction),
            "F" => canon!(Flex),
            "C" => canon!(Constraint),
            "S" => canon!(Spacing),
            "M" => canon!(Margin),
            "R" => canon!(Rect),
            "L" => from_str::<Layout>(data).map(render),
            _ => panic!("invalid input kind"),
        };
        println!("{}", value.unwrap_or_else(|_| "error".into()));
    }
}
