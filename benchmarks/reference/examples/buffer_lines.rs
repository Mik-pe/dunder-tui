use ratatui::{
    backend::TestBackend,
    buffer::{Buffer, Cell},
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use std::panic::{AssertUnwindSafe, catch_unwind};

fn dump(b: &Buffer) {
    println!("{}", serde_json::to_string(&b.area).unwrap());
    let mut i = 0;
    while i < b.content.len() {
        let first = i;
        let c = &b.content[i];
        i += 1;
        while i < b.content.len()
            && c == &b.content[i]
            && (c.symbol() != " "
                || serde_json::to_value(c).unwrap()["symbol"]
                    == serde_json::to_value(&b.content[i]).unwrap()["symbol"])
        {
            i += 1;
        }
        println!("{} {}", i - first, serde_json::to_string(c).unwrap());
    }
}

fn check<'a, T: Clone + Into<Line<'a>>>(rows: &[T]) {
    let b = Buffer::with_lines(rows.iter().cloned());
    dump(&b);
    let backend = TestBackend::with_lines(rows.iter().cloned());
    dump(backend.buffer());
    backend.assert_buffer_lines(rows.iter().cloned());
}

fn text_rows() {
    let cases = [
        "",
        " ",
        "XY",
        "A\r\n\nB\n",
        "\rX\tY",
        "カﾞﾟZ",
        "لا",
        "界",
        "A\u{301}茶",
        "🧑🏽‍🚀",
        "A\u{200d}B",
        "\u{200b}Z",
        "❤︎️",
    ];
    for a in cases {
        for b in cases {
            check(&[a, b, ""]);
            check(&[
                Span::styled(
                    a,
                    Style::new().fg(Color::Red).add_modifier(Modifier::ITALIC),
                ),
                Span::raw(b),
            ]);
            let first = Line::from(vec![Span::styled(a, Color::Indexed(45)), Span::raw(b)])
                .style(
                    Style::new()
                        .bg(Color::Blue)
                        .add_modifier(Modifier::UNDERLINED),
                )
                .right_aligned();
            check(&[first, Line::raw("🦀.")]);
        }
    }
}

fn boundaries() {
    for n in [65535, 65536, 65537, 131075] {
        check(&["A".repeat(n), "wxyz".to_string()]);
        let mut rows = vec![""; n];
        if n % 65536 > 0 {
            rows[0] = "OPEN";
            if n > 65535 {
                rows[n % 65536] = "\u{301}\t";
            }
            rows[n - 1] = "END";
        }
        check(&rows);
    }
    for n in [32767, 32768, 32769] {
        check(&["界".repeat(n), "!".to_string()]);
    }
}

fn coordinates() {
    for (x, y, width, height) in [
        (0, 0, 0, 0),
        (5, 7, 4, 3),
        (65532, 65533, 5, 4),
        (0, 0, 65535, 65535),
        (65535, 0, 1, 3),
        (3, 8, 0, 2),
    ] {
        for count in [0, 1, 3, 16] {
            let b = Buffer {
                area: Rect {
                    x,
                    y,
                    width,
                    height,
                },
                content: (0..count)
                    .map(|i| Cell::from(char::from(b'A' + i as u8)))
                    .collect(),
            };
            for x in [0, 1, 3, 4, 5, 7, 8, 9, 65532, 65533, 65534, 65535] {
                for y in [0, 1, 5, 6, 7, 8, 9, 10, 65532, 65533, 65534, 65535] {
                    let index = if b.area.contains((x, y).into()) {
                        b.index_of(x, y) as i64
                    } else {
                        -1
                    };
                    println!("{} {}", index, b.cell((x, y)).map_or("-", Cell::symbol));
                }
            }
            for i in -1..count + 1 {
                let pos = (i >= 0 && i < count && width != 0)
                    .then(|| catch_unwind(AssertUnwindSafe(|| b.pos_of(i as usize))).ok())
                    .flatten();
                if let Some((x, y)) = pos {
                    println!("{x} {y}");
                } else {
                    println!("-");
                }
            }
        }
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--outside") {
        let mut rows = vec![""; 65537];
        rows[1] = " ";
        if std::env::args().nth(2).as_deref() == Some("buffer") {
            dump(&Buffer::with_lines(rows));
        } else {
            dump(TestBackend::with_lines(rows).buffer());
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--unicode") {
        for path in std::env::args().skip(2) {
            for row in std::fs::read_to_string(path)
                .unwrap()
                .lines()
                .filter(|s| !s.is_empty())
            {
                let value: String = row
                    .split_whitespace()
                    .map(|cp| char::from_u32(u32::from_str_radix(cp, 16).unwrap()).unwrap())
                    .collect();
                check(&[value]);
            }
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--bounds") {
        boundaries();
        return;
    }
    text_rows();
    std::panic::set_hook(Box::new(|_| {}));
    coordinates();
}
