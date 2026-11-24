#[path = "../src/widget_cells.rs"]
mod widget_cells;

use Color::{Black, Blue, Cyan, Green, Magenta, Red, Reset, Yellow};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line as TextLine, Span},
    widgets::canvas::{
        Canvas, Circle, Context, FilledLine, Line, Map, MapResolution, Painter, Points, Rectangle,
        Shape,
    },
    widgets::{Block, Widget},
};
use std::io::{self, BufRead};

fn marker(k: usize) -> Marker {
    [
        Marker::Dot,
        Marker::Block,
        Marker::Bar,
        Marker::Braille,
        Marker::HalfBlock,
        Marker::Quadrant,
        Marker::Sextant,
        Marker::Octant,
        Marker::Custom('*'),
    ][k]
}

struct Pixels {
    w: usize,
    h: usize,
    mask: Option<usize>,
    bands: bool,
}
impl Shape for Pixels {
    fn draw(&self, p: &mut Painter) {
        for y in 0..self.h {
            for x in 0..self.w {
                let color = if let Some(mask) = self.mask {
                    if mask & (1 << (y * self.w + x)) == 0 {
                        continue;
                    }
                    if (x + y) % 3 == 0 { Red } else { Cyan }
                } else if self.bands {
                    match (x + 3 * y) % 7 {
                        0..=1 => Cyan,
                        2..=3 => Red,
                        _ => continue,
                    }
                } else if (x + 3 * y) % 5 == 0 {
                    Blue
                } else if (x + y) % 3 == 1 {
                    Red
                } else {
                    continue;
                };
                p.paint(x, y, color);
            }
        }
    }
}

fn resolution(m: Marker, area: Rect) -> (usize, usize) {
    let (dx, dy) = match m {
        Marker::Braille | Marker::Octant => (2, 4),
        Marker::Sextant => (2, 3),
        Marker::Quadrant => (2, 2),
        Marker::HalfBlock => (1, 2),
        _ => (1, 1),
    };
    (dx * usize::from(area.width), dy * usize::from(area.height))
}

fn paint(ctx: &mut Context, scene: usize, xs: [f64; 2], ys: [f64; 2], area: Rect, m: Marker) {
    let (cx, cy) = ((xs[0] + xs[1]) / 2., (ys[0] + ys[1]) / 2.);
    match scene {
        0 => {}
        1 => {
            ctx.draw(&Points {
                coords: &[
                    (xs[0], ys[0]),
                    (xs[1], ys[1]),
                    (cx, cy),
                    (xs[0], ys[1]),
                    (xs[1], ys[0]),
                    (92.5, -50.),
                ],
                color: Yellow,
            });
            ctx.draw(&Points {
                coords: &[(cx, cy)],
                color: Red,
            });
        }
        2 => {
            for n in 0..18 {
                let x = f64::from(n * 19 % 31 - 15);
                let y = f64::from(n * 23 % 21 - 10);
                ctx.draw(&Line::new(x, y, -0.5 * x, 0.8 * y, Cyan));
                ctx.draw(&Line::new(0.5 * x, 0., x, y, Magenta));
            }
        }
        3 => {
            ctx.draw(&Rectangle::new(-8., -3., 12., 6., Yellow));
            ctx.draw(&Rectangle::new(4., 2., -2., -4., Red));
            ctx.draw(&Rectangle::new(cx, cy, 0., 0., Cyan));
        }
        4 => {
            ctx.draw(&FilledLine::new(-12., 5., 12., -4., cy, Green));
            ctx.draw(&FilledLine::new(-4., -3., 0., 4., -40., Red));
            ctx.draw(&FilledLine::new(3., -3., 3., 4., 8., Yellow));
        }
        5 => {
            ctx.draw(&Circle::new(0., 0., 4., Cyan));
            ctx.draw(&Circle::new(5., 2., 1.3, Red));
            ctx.draw(&Circle::new(cx, cy, 0., Magenta));
        }
        6 => {
            ctx.draw(&Line::new(xs[0], ys[0], xs[1], ys[1], Red));
            ctx.layer();
            ctx.layer();
            ctx.draw(&Line::new(xs[0], ys[1], xs[1], ys[0], Yellow));
            ctx.draw(&Points {
                coords: &[(cx, cy)],
                color: Reset,
            });
        }
        7 => {
            ctx.marker(Marker::HalfBlock);
            let (w, h) = resolution(Marker::HalfBlock, area);
            ctx.draw(&Pixels {
                w,
                h,
                mask: None,
                bands: true,
            });
            ctx.marker(Marker::Braille);
            ctx.draw(&Circle::new(cx, cy, 3., Yellow));
            ctx.marker(Marker::Custom('/'));
            ctx.draw(&Points {
                coords: &[(xs[0], ys[1])],
                color: Magenta,
            });
        }
        8 => {
            ctx.print(
                cx,
                cy,
                TextLine::from(vec![
                    Span::styled("Å", Style::new().green()),
                    Span::raw("界é"),
                ])
                .red()
                .italic(),
            );
            ctx.print(xs[1], ys[0], "end");
            ctx.print(xs[1] + 1., ys[1], "outside");
            ctx.draw(&Line::new(xs[1], ys[0], xs[0], ys[1], Cyan));
            ctx.layer();
            ctx.print(
                xs[0],
                ys[1],
                TextLine::from("abc").alignment(Alignment::Right),
            );
        }
        9 => {
            let (w, h) = resolution(m, area);
            ctx.draw(&Pixels {
                w,
                h,
                mask: None,
                bands: false,
            });
        }
        10 | 11 => ctx.draw(&Map {
            resolution: if scene == 10 {
                MapResolution::Low
            } else {
                MapResolution::High
            },
            color: if scene == 10 { Yellow } else { Cyan },
        }),
        _ => {
            ctx.draw(&Map {
                resolution: MapResolution::Low,
                color: Green,
            });
            ctx.marker(Marker::Dot);
            ctx.draw(&Line::new(-40., 60., 110., -40., Cyan));
            ctx.layer();
            ctx.print(0., 0., TextLine::from("+").magenta());
        }
    }
}

fn path_points(seed: i32) -> Vec<(f64, f64)> {
    if seed == 0 {
        return Vec::new();
    }
    if seed == 1 {
        return vec![(1., 0.)];
    }
    let mut points = vec![(-11., -7.), (9., 6.), (0., 2.), (0., 2.)];
    points.extend((0..17).map(|i| {
        (
            f64::from((seed * 7 + i * 11) % 23 - 11),
            f64::from((seed + 3 * i * i) % 17 - 8),
        )
    }));
    points
}

fn paths(mut input: impl Iterator<Item = String>) {
    let mut checked = 0;
    for seed in 0..11 {
        let coords = path_points(seed);
        for (w, h) in [(0, 0), (1, 1), (2, 5), (9, 1), (13, 8)] {
            let area = Rect::new(1, 1, w, h);
            for k in 0..9 {
                for baseline in [None, Some(-9.), Some(-1.5), Some(2.), Some(9.)] {
                    let mut buf = widget_cells::seeded(area);
                    Canvas::default()
                        .x_bounds([-4., 4.])
                        .y_bounds([-3., 3.])
                        .marker(marker(k))
                        .paint(|ctx| {
                            ctx.draw(&Points {
                                coords: &coords,
                                color: Cyan,
                            });
                            for pair in coords.windows(2) {
                                let (a, b) = (pair[0], pair[1]);
                                if let Some(y) = baseline {
                                    ctx.draw(&FilledLine::new(a.0, a.1, b.0, b.1, y, Cyan));
                                } else {
                                    ctx.draw(&Line::new(a.0, a.1, b.0, b.1, Cyan));
                                }
                            }
                            ctx.layer();
                            ctx.marker(Marker::HalfBlock);
                            ctx.draw(&Points {
                                coords: &[(1., 0.)],
                                color: Red,
                            });
                        })
                        .render(area, &mut buf);
                    assert_eq!(
                        input.next().unwrap(),
                        widget_cells::snapshot(&buf),
                        "seed={seed} size={w}x{h} marker={k} baseline={baseline:?}"
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(input.next().is_none());
    println!("{checked} polyline frames matched Ratatui");
}

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    if std::env::args().any(|a| a == "--paths") {
        paths(input);
        return;
    }
    if std::env::args().any(|a| a == "--coordinates") {
        let path = std::env::args()
            .nth(2)
            .expect("supply ratatui-widgets-0.3.2/src/canvas/world.rs");
        let source = std::fs::read_to_string(path).unwrap();
        let mut n = 0;
        for (name, size) in [
            ("WORLD_LOW_RESOLUTION", 1166),
            ("WORLD_HIGH_RESOLUTION", 5125),
        ] {
            let section = source
                .split_once(name)
                .unwrap()
                .1
                .split_once("= [")
                .unwrap()
                .1
                .split_once("];")
                .unwrap()
                .0;
            let mut count = 0;
            for line in section
                .lines()
                .map(str::trim)
                .filter(|s| s.starts_with('('))
            {
                let coords = line.trim_start_matches('(').trim_end_matches("),");
                let (xs, ys) = coords.split_once(',').unwrap();
                let (x, y) = (
                    xs.trim().parse::<f64>().unwrap(),
                    ys.trim().parse::<f64>().unwrap(),
                );
                assert_eq!(
                    input.next().unwrap(),
                    format!("{} {}", x.to_bits() as i64, y.to_bits() as i64),
                    "{name} point {count}"
                );
                count += 1;
            }
            assert_eq!(count, size);
            n += count;
        }
        assert!(input.next().is_none());
        println!("{n} map coordinates match");
        return;
    }
    if std::env::args().any(|a| a == "--patterns") {
        let rect = Rect::new(0, 0, 1, 1);
        for k in 0..9 {
            for mask in 0..256 {
                let mut buf = Buffer::empty(rect);
                let m = marker(k);
                let (w, h) = resolution(m, rect);
                Canvas::default()
                    .marker(m)
                    .x_bounds([0., 1.])
                    .y_bounds([0., 1.])
                    .paint(|ctx| {
                        ctx.draw(&Pixels {
                            w,
                            h,
                            mask: Some(mask),
                            bands: false,
                        })
                    })
                    .render(rect, &mut buf);
                assert_eq!(
                    input.next().unwrap(),
                    widget_cells::snapshot(&buf),
                    "marker {m:?} mask {mask}"
                );
            }
        }
        assert!(input.next().is_none());
        println!("2304 marker patterns match");
        return;
    }
    let mut count = 0;
    let mut errors = 0;
    for scene in 0..13 {
        for (xs, ys) in [
            ([-10., 10.], [-5., 5.]),
            ([-180., 180.], [-90., 90.]),
            ([15., 15.], [3., 3.]),
            ([-4.8, 9.1], [-3.7, 6.3]),
        ] {
            for k in 0..9 {
                for border in 0..2 {
                    for (w, h) in [
                        (0, 0),
                        (0, 3),
                        (1, 1),
                        (1, 5),
                        (6, 1),
                        (3, 2),
                        (13, 5),
                        (31, 11),
                        (72, 23),
                    ] {
                        let rect = Rect::new(1, 1, w, h);
                        let m = marker(k);
                        let area = if border == 0 {
                            rect
                        } else {
                            Block::bordered().inner(rect)
                        };
                        let mut buf = widget_cells::seeded(rect);
                        let mut view = Canvas::default()
                            .marker(m)
                            .x_bounds(xs)
                            .y_bounds(ys)
                            .background_color(if k % 2 == 0 { Black } else { Reset })
                            .paint(|ctx| paint(ctx, scene, xs, ys, area, m));
                        if border == 1 {
                            view = view.block(Block::bordered());
                        }
                        view.render(rect, &mut buf);
                        let actual = input.next().unwrap();
                        let expected = widget_cells::snapshot(&buf);
                        if actual != expected {
                            if errors < 25 {
                                let (i, (a, b)) = actual
                                    .split('|')
                                    .zip(expected.split('|'))
                                    .enumerate()
                                    .find(|(_, (a, b))| a != b)
                                    .unwrap();
                                eprintln!(
                                    "scene {scene} xs {xs:?} marker {m:?} border {border} {w}x{h} first diff {i} {a} / {b}"
                                );
                            }
                            errors += 1;
                        }
                        count += 1;
                    }
                }
            }
        }
    }
    assert!(input.next().is_none());
    println!("{count} canvas frames, {errors} differences");
    assert_eq!(errors, 0);
}
