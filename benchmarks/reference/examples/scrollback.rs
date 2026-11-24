#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
use ratatui::{
    backend::{Backend, TestBackend},
    buffer::Buffer,
};
use std::{hint::black_box, time::Instant};

fn run(which: usize, saved: &str) -> u16 {
    if which == 3 {
        let backend: TestBackend = serde_json::from_str(saved).unwrap();
        assert_eq!(backend.scrollback()[(0, 0)].symbol(), "A");
        assert_eq!(backend.scrollback()[(1, 0)].symbol(), "茶");
        return backend.scrollback().area.height;
    }
    let mut backend = if which == 0 {
        TestBackend::new(8, 2)
    } else {
        TestBackend::with_lines(["A茶🌊123", "B       "])
    };
    if which < 2 {
        backend.append_lines(65535).unwrap();
        backend.append_lines(2).unwrap();
        if which == 1 {
            assert_eq!(backend.scrollback()[(0, 0)].symbol(), "B");
        }
    } else {
        let strip = Buffer::with_lines(["A茶🌊123"]);
        for _ in 0..256 {
            backend
                .draw(
                    strip
                        .content
                        .iter()
                        .enumerate()
                        .map(|(x, c)| (x as u16, 0, c)),
                )
                .unwrap();
            backend.scroll_region_up(0..2, 1).unwrap();
        }
        assert_eq!(backend.scrollback()[(1, 255)].symbol(), "茶");
    }
    black_box(&backend).scrollback().area.height
}

fn snapshot() -> String {
    let mut sample = TestBackend::with_lines(["A茶🌊123", "B       "]);
    sample.append_lines(129).unwrap();
    serde_json::to_string(&sample).unwrap()
}

fn main() {
    let names = ["blank-limit", "mixed-limit", "painted", "restore"];
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() == Some("--resources") {
        let name = args.next().unwrap();
        let which = names.iter().position(|&n| n == name).unwrap();
        let saved = if which == 3 {
            snapshot()
        } else {
            String::new()
        };
        println!("{}", run(which, &saved));
        #[cfg(feature = "allocation-counts")]
        allocation::report();
        return;
    }
    let saved = snapshot();
    for (which, name) in names.into_iter().enumerate() {
        let mut times = Vec::new();
        let mut total = 0u32;
        let batch = if which < 2 { 4 } else { 1 };
        for i in 0..23 {
            let mut result = 0u32;
            let start = Instant::now();
            for _ in 0..batch {
                result += u32::from(run(which, black_box(&saved)));
            }
            let ns = start.elapsed().as_nanos() / batch;
            if i >= 3 {
                times.push(ns);
                total += result;
            }
        }
        times.sort_unstable();
        eprintln!("{name} {} {} {total}", times[9], times[18]);
    }
}
