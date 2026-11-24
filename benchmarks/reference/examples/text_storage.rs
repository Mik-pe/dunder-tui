#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[allow(dead_code)]
#[path = "../src/list_scene.rs"]
mod list_scene;
#[allow(dead_code)]
#[path = "../src/table_scene.rs"]
mod table_scene;

use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span, Text},
};
use std::{hint::black_box, time::Instant};

fn measure<T>(create: impl FnOnce() -> T) -> u128 {
    let start = Instant::now();
    let data = black_box(create());
    let elapsed = start.elapsed().as_nanos();
    drop(data);
    elapsed
}

fn main() {
    let Some(kind) = std::env::args().nth(1) else {
        std::process::exit(2);
    };
    let elapsed = match kind.as_str() {
        "list" => measure(|| list_scene::Scene::new(4096)),
        "table" => measure(|| table_scene::Scene::new(4096)),
        "plain" | "styled" => {
            let start = Instant::now();
            let docs: Vec<_> = (0..16384)
                .map(|_| {
                    if kind == "plain" {
                        Text::raw("OPEN")
                    } else {
                        let span = Span::styled("OPEN", Style::new().cyan());
                        let line = Line::from(vec![span]).bold();
                        Text::from(vec![line]).on_blue()
                    }
                })
                .collect();
            let elapsed = start.elapsed().as_nanos();
            assert_eq!(
                black_box(&docs).iter().map(Text::width).sum::<usize>(),
                65536
            );
            elapsed
        }
        _ => std::process::exit(2),
    };
    println!("{kind} {elapsed}");
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
