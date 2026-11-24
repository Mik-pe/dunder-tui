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
struct LayoutCase {
    direction: Direction,
    flex: Flex,
    gap: i32,
    margin: Margin,
    area: Rect,
    rules: Vec<Constraint>,
    actual: Vec<Rect>,
}

impl LayoutCase {
    fn parse(line: &str) -> Self {
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
        let actual = (0..(2 * count + 1))
            .map(|_| Rect {
                x: coord(next()),
                y: coord(next()),
                width: coord(next()),
                height: coord(next()),
            })
            .collect::<Vec<_>>();
        assert!(numbers.next().is_none());
        Self {
            direction,
            flex,
            gap,
            margin: Margin::new(mx, my),
            area,
            rules,
            actual,
        }
    }
    fn reference(&self) -> Option<Vec<Rect>> {
        let layout = Layout::new(self.direction, self.rules.clone())
            .horizontal_margin(self.margin.horizontal)
            .vertical_margin(self.margin.vertical)
            .flex(self.flex)
            .spacing(self.gap);
        let (parts, spacers) =
            match std::panic::catch_unwind(|| layout.split_with_spacers(self.area)) {
                Ok(regions) => regions,
                Err(error) => {
                    let message = error
                        .downcast_ref::<String>()
                        .map(String::as_str)
                        .or_else(|| error.downcast_ref::<&str>().copied());
                    if message == Some("failed to split: InternalSolverError(ObjectiveUnbounded)") {
                        return None;
                    }
                    std::panic::resume_unwind(error)
                }
            };
        Some(parts.iter().chain(spacers.iter()).copied().collect())
    }
    fn compare(&self, expected: &[Rect]) -> u8 {
        if self.actual == expected {
            return b'=';
        }
        let verdict = mismatch(
            &self.rules,
            self.flex,
            self.gap,
            self.area.inner(self.margin),
            self.direction,
            expected,
            &self.actual,
        );
        if verdict != b'~' {
            eprintln!(
                "{} {:?} {:?} {:?} {:?} {} {:?}\nD {:?}\nR {expected:?}",
                char::from(verdict),
                self.direction,
                self.area,
                self.margin,
                self.flex,
                self.gap,
                self.rules,
                self.actual
            );
        }
        verdict
    }
    fn compare_record(&self, input: &str, record: &str) -> u8 {
        let (parameters, _) = record.split_once('\t').expect("missing rectangle list");
        assert!(
            input
                .split_ascii_whitespace()
                .take(10 + 3 * self.rules.len())
                .eq(parameters.split_ascii_whitespace()),
            "recorded parameters differ from the input"
        );
        self.compare(&Self::parse(record).actual)
    }

    fn optimal(&self) -> bool {
        layout_optimum::optimal_projections(
            &self.rules,
            self.flex,
            self.gap,
            self.area.inner(self.margin),
            self.direction,
            &self.actual,
            &self.actual,
        )
        .0
    }
}

fn compare(line: &str) -> u8 {
    let case = LayoutCase::parse(line);
    case.reference()
        .map_or(b'U', |expected| case.compare(&expected))
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
            if flex == Flex::SpaceEvenly
                && rules.first().is_some_and(|first| {
                    matches!(first, Constraint::Length(_)) && rules.iter().all(|r| r == first)
                })
                && panels.iter().any(|r| r.is_empty())
                && actual[..rules.len()].iter().all(|r| !r.is_empty()) =>
        {
            b'N'
        }
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

#[test]
fn observed_empty_panels_cannot_validate_an_unequal_dunder_split() {
    let area = Rect::new(2, 3, 4, 79);
    let place = |heights: &[u16]| {
        let mut y = area.y;
        let mut panels = Vec::new();
        let mut spacers = Vec::new();
        for h in heights {
            spacers.push(Rect::new(area.x, y, area.width, 2));
            y += 2;
            panels.push(Rect::new(area.x, y, area.width, *h));
            y += h;
        }
        spacers.push(Rect::new(area.x, y, area.width, 2));
        assert_eq!(y + 2, area.bottom());
        panels.extend(spacers);
        panels
    };
    let observed = place(&[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 5, 6, 6,
    ]);
    let uniform = place(&[
        1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1,
    ]);
    let classify = |r: &[Rect], d: &[Rect]| {
        mismatch(
            &[Constraint::Length(6); 24],
            Flex::SpaceEvenly,
            2,
            area,
            Direction::Vertical,
            r,
            d,
        )
    };
    assert_eq!(classify(&observed, &uniform), b'N');
    assert_eq!(classify(&uniform, &observed), b'!');
}

fn finish_child(mut child: std::process::Child, limit: Duration) -> (std::process::Output, bool) {
    let deadline = Instant::now() + limit;
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
    (child.wait_with_output().unwrap(), timed_out)
}

fn mixed_results(lines: &[String], attempts: usize, deadline: Duration) -> Vec<u8> {
    let executable = std::env::current_exe().unwrap();
    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let case = LayoutCase::parse(line);
            for _ in 0..attempts {
                let child = Command::new(&executable)
                    .arg("--reference-regions")
                    .arg(line)
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let (output, timed_out) = finish_child(child, deadline);
                if timed_out || (output.status.code() == Some(2) && output.stdout.is_empty()) {
                    continue;
                }
                assert!(
                    output.status.success(),
                    "reference failed at case {}",
                    index + 1
                );
                return case.compare_record(line, std::str::from_utf8(&output.stdout).unwrap());
            }
            eprintln!(
                "No Ratatui split for case {}. Checking the exact objective.",
                index + 1
            );
            if case.optimal() { b'O' } else { b'!' }
        })
        .collect()
}

fn checked_batches(lines: &[String], batch_size: usize, isolated_attempts: usize) -> Vec<u8> {
    let executable = std::env::current_exe().unwrap();
    let mut results = Vec::with_capacity(lines.len());
    let mut stalled = 0;
    while results.len() < lines.len() {
        let start = results.len();
        let end = (start + if stalled == 0 { batch_size } else { 1 }).min(lines.len());
        let child = Command::new(&executable)
            .arg("--layout-batch")
            .args(&lines[start..end])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let (mut output, timed_out) = finish_child(child, Duration::from_secs(5));
        assert!(
            timed_out || output.status.success(),
            "reference exited {} in cases {}–{}",
            output.status,
            start + 1,
            end
        );
        assert!(output.stdout.len() <= end - start);
        let interruption = match output.stdout.last() {
            Some(b'U') => Some("ObjectiveUnbounded"),
            Some(b'N') => Some("nonoptimal equal-length split"),
            _ => None,
        };
        if interruption.is_some() {
            output.stdout.pop();
        }
        results.extend(output.stdout);
        if (timed_out || interruption.is_some()) && results.len() < end {
            let index = results.len();
            eprintln!(
                "Ratatui layout {} at case {}: {}",
                interruption.unwrap_or("deadline"),
                index + 1,
                lines[index]
            );
            stalled = if index == start { stalled + 1 } else { 1 };
            assert!(
                stalled <= isolated_attempts,
                "reference did not return case {} in isolation",
                index + 1
            );
        } else {
            assert_eq!(results.len(), end, "missing reference results");
            stalled = 0;
        }
    }
    results
}

fn main() {
    let mode = std::env::args().nth(1);
    match mode.as_deref() {
        Some("--geometry") => return geometry(),
        Some("--frames") => return frames(),
        Some("--rules") => return rule_values(),
        Some("--reference-regions") => {
            let input = std::env::args().nth(2).unwrap();
            let case = LayoutCase::parse(&input);
            let Some(regions) = case.reference() else {
                std::process::exit(2);
            };
            let parameters = input
                .split_ascii_whitespace()
                .take(10 + 3 * case.rules.len())
                .collect::<Vec<_>>()
                .join(" ");
            let mut out = io::stdout().lock();
            write!(out, "{parameters}\t").unwrap();
            for (i, r) in regions.iter().enumerate() {
                if i > 0 {
                    write!(out, " ").unwrap();
                }
                write!(out, "{} {} {} {}", r.x, r.y, r.width, r.height).unwrap();
            }
            writeln!(out).unwrap();
            return;
        }
        Some("--exact-layout") => {
            assert!(LayoutCase::parse(&std::env::args().nth(2).unwrap()).optimal());
            return;
        }
        Some("--layout-batch") => {
            let mut out = io::stdout().lock();
            for line in std::env::args().skip(2) {
                let verdict = compare(&line);
                out.write_all(&[verdict]).unwrap();
                out.flush().unwrap();
                if matches!(verdict, b'U' | b'N') {
                    break;
                }
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
        Some("--many-panels") => 1284,
        Some("--mixed-panels" | "--recorded-panels") => 84,
        Some("--benchmark-panels") => 15,
        Some("--minimum-levels") => 2880,
        _ => 14_096,
    };
    assert_eq!(lines.len(), expected);
    let results = match mode.as_deref() {
        Some("--minimums") | Some("--minimum-levels") => checked_batches(&lines, 64, 1),
        Some("--many-panels") => checked_batches(&lines, 8, 4),
        Some("--recorded-panels") => {
            let records = include_str!("../fixtures/mixed-panels-0.30.2.tsv");
            assert_eq!(records.lines().count(), lines.len());
            lines
                .iter()
                .zip(records.lines())
                .map(|(input, record)| LayoutCase::parse(input).compare_record(input, record))
                .collect()
        }
        Some("--mixed-panels") => {
            let mut args = std::env::args().skip(2);
            let attempts = args.next().map(|v| v.parse().unwrap()).unwrap_or(2);
            let seconds = args.next().map(|v| v.parse().unwrap()).unwrap_or(5);
            assert!(attempts > 0 && seconds > 0 && args.next().is_none());
            mixed_results(&lines, attempts, Duration::from_secs(seconds))
        }
        _ => lines.iter().map(|line| compare(line)).collect(),
    };
    let matches = results.iter().filter(|&&v| v == b'=').count();
    let ties = results.iter().filter(|&&v| v == b'~').count();
    let invalid_reference = results.iter().filter(|&&v| v == b'R').count();
    let differs = results.iter().filter(|&&v| v == b'!').count();
    let exact_only = results.iter().filter(|&&v| v == b'O').count();
    assert_eq!(
        matches + ties + invalid_reference + differs + exact_only,
        expected
    );
    let summary = format!(
        "{matches} identical, {ties} verified alternate optima, {invalid_reference} overlapping nonoptimal reference layouts"
    );
    if mode.as_deref() == Some("--mixed-panels") {
        println!(
            "{summary}, {exact_only} exact optima without a live reference answer, {differs} incorrect or unclassified"
        );
    } else {
        println!("{summary}, {differs} unresolved");
    }
    assert_eq!(differs, 0);
    if exact_only > 0 {
        std::process::exit(2);
    }
}

#[allow(deprecated)]
fn rule_values() {
    use std::fmt::Write;
    let values = [0, 1, 3, 17, 53, 99, 100, 101, 32768, 65535];
    let pairs = [
        (0, 0),
        (1, 0),
        (0, 9),
        (2, 3),
        (7, 11),
        (16777215, 16777217),
        (4294967294, 4294967295),
        (4294967295, 1),
    ];
    for rules in [
        vec![Constraint::default()],
        Constraint::from_lengths([]),
        Constraint::from_lengths(values),
        Constraint::from_percentages(values),
        Constraint::from_ratios(pairs),
        Constraint::from_mins(values),
        Constraint::from_maxes(values),
        Constraint::from_fills(values),
    ] {
        println!("{}", rules.len());
        for rule in rules {
            println!("{rule}");
            let mut row = String::with_capacity(262144);
            for extent in 0..=u16::MAX {
                write!(row, "{:04x}", rule.apply(extent)).unwrap();
            }
            println!("{row}");
        }
    }
    let mut layout = Layout::default();
    let mut direction = Direction::default();
    for count in [0, 3, 8] {
        layout = layout.constraints(Constraint::from_ratios(pairs[..count].iter().copied()));
        for step in 0..4 {
            if step != 0 {
                direction = direction.perpendicular();
                layout = layout.direction(direction);
            }
            for area in [
                Rect::new(2, 3, 13, 7),
                Rect::new(5, 1, 11, 11),
                Rect::new(2, 3, 13, 7),
            ] {
                let (parts, spacers) = layout.split_with_spacers(area);
                let mut row = String::new();
                for r in parts.iter().chain(spacers.iter()) {
                    write!(row, " {} {} {} {}", r.x, r.y, r.width, r.height).unwrap();
                }
                println!("{row}");
            }
        }
    }
}
