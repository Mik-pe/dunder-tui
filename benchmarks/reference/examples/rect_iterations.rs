use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

fn axes(area: Rect, columns: bool, mode: usize) {
    fn consume(mut it: impl DoubleEndedIterator<Item = Rect>, mut left: usize, mode: usize) {
        for n in 0..(left + 2).min(32) {
            let back = mode == 1 || (mode == 2 && n % 2 == 0) || (mode == 3 && n % 3 == 2);
            let rect = if back { it.next_back() } else { it.next() };
            let (hint, upper) = it.size_hint();
            assert_eq!(upper, Some(hint));
            match rect {
                Some(r) => {
                    left -= 1;
                    println!("{} {} {} {} {left} {hint}", r.x, r.y, r.width, r.height)
                }
                None => println!("- {left} {hint}"),
            }
        }
    }
    if columns {
        consume(
            area.columns(),
            usize::from(area.right() - area.left()),
            mode,
        );
    } else {
        consume(area.rows(), usize::from(area.bottom() - area.top()), mode);
    }
}

fn points(area: Rect, limit: usize) {
    let mut checksum = 0u64;
    let mut left = u64::from(area.right() - area.left()) * u64::from(area.bottom() - area.top());
    let mut it = area.positions();
    println!("{left} {}", it.size_hint().0);
    for n in 0..limit {
        let point = it.next();
        let (hint, upper) = it.size_hint();
        assert_eq!(upper, Some(hint));
        match point {
            Some(p) => {
                left -= 1;
                checksum =
                    (checksum * 31 + u64::from(p.x) * 65536 + u64::from(p.y) + left + hint as u64)
                        % 2147483647;
                if limit < 80 || n % 65535 >= 65533 || n % 65535 <= 1 || n >= limit - 3 {
                    println!("{n} {} {} {left} {hint}", p.x, p.y);
                }
            }
            None => {
                println!("- {left} {hint}");
                assert!(it.next().is_none());
                break;
            }
        }
    }
    println!("{checksum}");
}

fn grid(area: Rect) -> Buffer {
    let mut b = Buffer::empty(area);
    for p in area.positions() {
        let code = ((p.x - area.x) + 3 * (p.y - area.y)) % 26;
        b[p].set_char(char::from(b'A' + code as u8));
    }
    let mut rows = area.rows();
    for n in 0..area.height {
        let row = if n % 2 == 0 {
            rows.next()
        } else {
            rows.next_back()
        };
        b.set_style(
            row.unwrap(),
            Style::new().fg(Color::Indexed(20 + (n % 16) as u8)),
        );
    }
    for col in area.columns() {
        b.set_style(
            col,
            Style::new().bg(Color::Indexed(52 + ((col.x - area.x) % 16) as u8)),
        );
    }
    b
}

fn main() {
    for x in [0, 7, 65532, 65535] {
        for y in [0, 9, 65533, 65535] {
            for width in [0, 1, 3, 5, 65535] {
                for height in [0, 1, 2, 4, 65535] {
                    let area = Rect {
                        x,
                        y,
                        width,
                        height,
                    };
                    for mode in 0..4 {
                        axes(area, false, mode);
                        axes(area, true, mode);
                    }
                    points(area, 32);
                }
            }
        }
    }
    for area in [
        Rect::new(0, 0, 65535, 2),
        Rect::new(0, 0, 3, 65535),
        Rect::new(0, 0, 65535, 65535),
        Rect {
            x: 65532,
            y: 0,
            width: 65535,
            height: 32768,
        },
    ] {
        points(area, 196608);
    }
    for x in [0, 17, 65531] {
        for y in [0, 13, 65532] {
            for w in 0..10 {
                for h in 0..7 {
                    println!(
                        "{}",
                        serde_json::to_string(&grid(Rect::new(x, y, w, h))).unwrap()
                    );
                }
            }
        }
    }
}
