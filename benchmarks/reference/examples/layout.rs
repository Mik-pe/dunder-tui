#[path = "../src/layout_optimum.rs"]
mod layout_optimum;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Margin, Rect};
use std::io::{self, BufRead};
#[path = "../src/layout_scene.rs"]
mod layout_scene;
#[allow(dead_code)]
#[path = "../src/widget_cells.rs"]
mod widget_cells;
fn frames() {
    let scene = layout_scene::Scene::new();
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    for mode in ["idle", "cell", "scroll", "resize", "repaint"] {
        for n in 0..220 {
            let w = if mode == "resize" && n % 2 == 0 {
                80
            } else {
                96
            };
            let mut buf = ratatui::buffer::Buffer::empty(Rect::new(0, 0, w, 24));
            scene.draw(&mut buf, mode, n);
            assert_eq!(
                lines.next().unwrap(),
                widget_cells::snapshot(&buf),
                "{mode} frame {n}"
            );
        }
    }
    assert!(lines.next().is_none());
    println!("1100 layout benchmark frames identical");
}

fn geometry() {
    use ratatui::layout::{Offset, Position, Size};
    let mut count = 0;
    for l in io::stdin().lock().lines().map(Result::unwrap) {
        let mut it = l.split_whitespace().map(|n| n.parse::<i64>().unwrap());
        let (dx, dy, mx, my, ix, pos) = (
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        );
        let mut rect = || {
            let a = [
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            ];
            if a == [-1; 4] {
                None
            } else {
                Some(Rect::new(
                    coord(a[0]),
                    coord(a[1]),
                    coord(a[2]),
                    coord(a[3]),
                ))
            }
        };
        let (a, b) = (rect().unwrap(), rect().unwrap());
        let m = Margin::new(coord(mx), coord(my));
        for r in [
            a.inner(m),
            a.outer(m),
            a.offset(Offset {
                x: dx.try_into().unwrap(),
                y: dy.try_into().unwrap(),
            }),
            a.resize(Size::new(b.width, b.height)),
            a.union(b),
            a.intersection(b),
            a.clamp(b),
        ] {
            assert_eq!(rect(), Some(r));
        }
        assert_eq!(rect(), a.rows().nth(ix.try_into().unwrap()));
        assert_eq!(rect(), a.columns().nth(ix.try_into().unwrap()));
        let p = a
            .positions()
            .nth(pos.try_into().unwrap())
            .map_or((-1, -1), |p| (i64::from(p.x), i64::from(p.y)));
        assert_eq!((it.next().unwrap(), it.next().unwrap()), p);
        assert_eq!(
            it.collect::<Vec<_>>(),
            [
                i64::from(a.right()),
                i64::from(a.bottom()),
                i64::from(a.area()),
                i64::from(a.is_empty()),
                i64::from(a.intersects(b)),
                i64::from(a.contains(Position::new(b.x, b.y)))
            ]
        );
        count += 1;
    }
    assert_eq!(count, 1200);
    println!("{count} rectangle cases identical");
}

fn coord(n: i64) -> u16 {
    u16::try_from(n).expect("invalid terminal coordinate")
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("--geometry") {
        geometry();
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--frames") {
        frames();
        return;
    }
    assert_eq!(
        Layout::horizontal([Constraint::Min(65535), Constraint::Length(7)])
            .split(Rect::new(0, 0, 100, 3))[0]
            .width,
        93
    );
    let mut matches = 0;
    let mut differs = 0;
    let mut ties = 0;
    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let mut numbers = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap());
        let mut next = || numbers.next().unwrap();
        let direction = [Direction::Horizontal, Direction::Vertical][next() as usize];
        let flex = [
            Flex::Start,
            Flex::Center,
            Flex::End,
            Flex::SpaceBetween,
            Flex::SpaceAround,
            Flex::SpaceEvenly,
            Flex::Legacy,
        ][next() as usize];
        let gap = i32::try_from(next()).unwrap().clamp(-32768, 32767);
        let (mx, my) = (coord(next()), coord(next()));
        let area = Rect {
            x: coord(next()),
            y: coord(next()),
            width: coord(next()),
            height: coord(next()),
        };
        let count = next() as usize;
        let rules: Vec<_> = (0..count)
            .map(|_| {
                let (t, a, b) = (
                    next(),
                    u32::try_from(next()).unwrap(),
                    u32::try_from(next()).unwrap(),
                );
                match t {
                    0 => Constraint::Length(u16::try_from(a).unwrap()),
                    1 => Constraint::Percentage(u16::try_from(a).unwrap()),
                    2 => Constraint::Ratio(a, b),
                    3 => Constraint::Min(u16::try_from(a).unwrap()),
                    4 => Constraint::Max(u16::try_from(a).unwrap()),
                    5 => Constraint::Fill(u16::try_from(a).unwrap()),
                    _ => panic!(),
                }
            })
            .collect();
        let (parts, spacers) = Layout::new(direction, rules.clone())
            .horizontal_margin(mx)
            .vertical_margin(my)
            .flex(flex)
            .spacing(gap)
            .split_with_spacers(area);
        let expected = parts
            .iter()
            .chain(spacers.iter())
            .copied()
            .collect::<Vec<_>>();
        let actual = (0..(2 * count + 1))
            .map(|_| Rect {
                x: coord(next()),
                y: coord(next()),
                width: coord(next()),
                height: coord(next()),
            })
            .collect::<Vec<_>>();
        assert!(numbers.next().is_none());
        if actual == expected {
            matches += 1;
        } else if layout_optimum::same_optimum(
            &rules,
            flex,
            gap,
            area.inner(Margin::new(mx, my)),
            direction,
            &expected,
            &actual,
        ) {
            ties += 1;
        } else {
            differs += 1;
            eprintln!(
                "{direction:?} {area:?} {flex:?} {gap} {mx} {my} {rules:?}\nD {actual:?}\nR {expected:?}"
            );
        }
    }
    println!("{matches} identical, {ties} verified alternate optima, {differs} unresolved");
    assert_eq!(
        matches + ties + differs,
        if std::env::args().nth(1).as_deref() == Some("--limits") {
            980
        } else {
            14_096
        }
    );
    assert_eq!(differs, 0);
}
