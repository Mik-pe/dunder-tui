#[path = "../src/layout_optimum.rs"]
mod layout_optimum;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Margin, Rect};
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
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
                Some(Rect {
                    x: coord(a[0]),
                    y: coord(a[1]),
                    width: coord(a[2]),
                    height: coord(a[3]),
                })
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
    assert_eq!(count, 2460);
    println!("{count} rectangle cases identical");
}

fn coord(n: i64) -> u16 {
    u16::try_from(n).expect("invalid terminal coordinate")
}
fn compare(line: &str) -> u8 {
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
        return b'=';
    }
    let verdict = mismatch(
        &rules,
        flex,
        gap,
        area.inner(Margin::new(mx, my)),
        direction,
        &expected,
        &actual,
    );
    if verdict != b'~' {
        eprintln!(
            "{} {direction:?} {area:?} {flex:?} {gap} {mx} {my} {rules:?}\nD {actual:?}\nR {expected:?}",
            char::from(verdict)
        );
    }
    verdict
}

fn mismatch(
    rules: &[Constraint],
    flex: Flex,
    gap: i32,
    inner: Rect,
    direction: Direction,
    reference: &[Rect],
    actual: &[Rect],
) -> u8 {
    let panels = &reference[..rules.len()];
    match layout_optimum::optimal_projections(rules, flex, gap, inner, direction, reference, actual)
    {
        (true, true) => b'~',
        (false, true)
            if flex == Flex::SpaceAround
                && gap == 0
                && panels.iter().enumerate().any(|(i, a)| {
                    panels[i + 1..]
                        .iter()
                        .any(|b| !a.intersection(*b).is_empty())
                }) =>
        {
            b'R'
        }
        _ => b'!',
    }
}

#[test]
fn overlapping_space_around_reference_requires_an_optimal_dunder_layout() {
    use Constraint::{Fill, Length, Max, Min, Percentage, Ratio};
    let rules = [
        Length(3),
        Min(32769),
        Fill(2),
        Max(7),
        Percentage(40),
        Min(1),
        Ratio(1, 5),
        Fill(1),
        Min(32769),
    ];
    let cells = |pairs: &[(u16, u16)]| {
        pairs
            .iter()
            .map(|&(x, w)| Rect::new(x, 0, w, 4))
            .collect::<Vec<_>>()
    };
    let actual = cells(&[
        (0, 3),
        (3, 4367),
        (4370, 8735),
        (13105, 7),
        (13112, 26214),
        (39326, 4367),
        (43693, 13107),
        (56800, 4368),
        (61168, 4367),
        (0, 0),
        (3, 0),
        (4370, 0),
        (13105, 0),
        (13112, 0),
        (39326, 0),
        (43693, 0),
        (56800, 0),
        (61168, 0),
        (65535, 0),
    ]);
    let observed = cells(&[
        (0, 3),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 65535),
        (65535, 0),
        (0, 27206),
        (0, 28109),
        (0, 65535),
        (0, 0),
        (3, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (65535, 0),
        (0, 0),
        (27206, 0),
        (28109, 0),
        (65535, 0),
    ]);
    let classify = |r: &[Rect], d: &[Rect]| {
        mismatch(
            &rules,
            Flex::SpaceAround,
            0,
            Rect::new(0, 0, 65535, 4),
            Direction::Horizontal,
            r,
            d,
        )
    };
    assert_eq!(classify(&observed, &actual), b'R');
    assert_eq!(classify(&actual, &observed), b'!');
    assert_eq!(classify(&observed, &observed), b'!');
}

fn minimums(lines: &[String]) -> Vec<u8> {
    let executable = std::env::current_exe().unwrap();
    let mut results = Vec::with_capacity(lines.len());
    let mut retry = None;
    while results.len() < lines.len() {
        let start = results.len();
        let end = (start + 64).min(lines.len());
        let mut child = Command::new(&executable)
            .arg("--minimum-batch")
            .args(&lines[start..end])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        let timed_out = loop {
            if child.try_wait().unwrap().is_some() {
                break false;
            }
            if Instant::now() >= deadline {
                break true;
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        if timed_out && let Err(error) = child.kill() {
            assert!(
                child.try_wait().unwrap().is_some(),
                "could not stop the reference: {error}"
            );
        }
        let output = child.wait_with_output().unwrap();
        assert!(
            timed_out || output.status.success(),
            "reference exited {} in cases {}–{}",
            output.status,
            start + 1,
            end
        );
        assert!(output.stdout.len() <= end - start);
        results.extend(output.stdout);
        if timed_out && results.len() < end {
            let index = results.len();
            eprintln!(
                "Ratatui layout deadline at case {}: {}",
                index + 1,
                lines[index]
            );
            assert_ne!(
                retry,
                Some(index),
                "reference did not finish the same case twice"
            );
            retry = Some(index);
        } else {
            assert_eq!(results.len(), end, "missing reference results");
        }
    }
    results
}

fn main() {
    let mode = std::env::args().nth(1);
    match mode.as_deref() {
        Some("--geometry") => return geometry(),
        Some("--frames") => return frames(),
        Some("--minimum-batch") => {
            let mut out = io::stdout().lock();
            for line in std::env::args().skip(2) {
                out.write_all(&[compare(&line)]).unwrap();
                out.flush().unwrap();
            }
            return;
        }
        _ => {}
    }
    assert_eq!(
        Layout::horizontal([Constraint::Min(65535), Constraint::Length(7)])
            .split(Rect::new(0, 0, 100, 3))[0]
            .width,
        93
    );
    let lines: Vec<_> = io::stdin().lock().lines().map(Result::unwrap).collect();
    let expected = match mode.as_deref() {
        Some("--limits") => 980,
        Some("--minimums") => 6048,
        _ => 14_096,
    };
    assert_eq!(lines.len(), expected);
    let results = if mode.as_deref() == Some("--minimums") {
        minimums(&lines)
    } else {
        lines.iter().map(|line| compare(line)).collect()
    };
    let matches = results.iter().filter(|&&v| v == b'=').count();
    let ties = results.iter().filter(|&&v| v == b'~').count();
    let invalid_reference = results.iter().filter(|&&v| v == b'R').count();
    let differs = results.iter().filter(|&&v| v == b'!').count();
    assert_eq!(matches + ties + invalid_reference + differs, expected);
    println!(
        "{matches} identical, {ties} verified alternate optima, {invalid_reference} overlapping nonoptimal reference layouts, {differs} unresolved"
    );
    assert_eq!(differs, 0);
}
