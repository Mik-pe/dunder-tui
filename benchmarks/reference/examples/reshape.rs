use ratatui::{
    buffer::{Buffer, CellDiffOption},
    layout::Rect,
    style::Style,
};
use std::{hint::black_box, time::Instant};

fn main() {
    let mode = std::env::args().nth(1).unwrap();
    let (a, b) = match mode.as_str() {
        "plain" | "styled" => (Rect::new(0, 0, 80, 24), Rect::new(0, 0, 96, 20)),
        "grow" => (Rect::new(0, 0, 80, 24), Rect::new(0, 0, 96, 24)),
        "large" => (Rect::new(0, 0, 1024, 1024), Rect::new(0, 0, 2048, 512)),
        _ => panic!("unknown workload"),
    };
    let mut f = Buffer::empty(a);
    f.set_string(0, 0, "A", Style::default());
    if mode == "styled" {
        f.set_string(13, 2, "界❤️", Style::new().blue().bold());
        f[(11, 9)].set_diff_option(CellDiffOption::Skip);
    }
    let mut samples = Vec::with_capacity(200);
    let mut sum = 0;
    for sample in 0..220 {
        let start = Instant::now();
        for _ in 0..256 {
            black_box(&mut f).resize(black_box(b));
            sum += u32::from(f.area.width);
            black_box(&mut f).resize(black_box(a));
            sum += u32::from(f.area.width);
        }
        let elapsed = start.elapsed().as_nanos() / 512;
        if sample >= 20 {
            samples.push(elapsed);
        }
    }
    assert_eq!(sum, 56320 * u32::from(a.width + b.width));
    assert_eq!(f[(0, 0)].symbol(), "A");
    if mode == "styled" {
        assert_eq!(f[(11, 9)].diff_option, CellDiffOption::Skip);
    }
    samples.sort_unstable();
    println!("{mode} {} {} {sum}", samples[99], samples[189]);
}
