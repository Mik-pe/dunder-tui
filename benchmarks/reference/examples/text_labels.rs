#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
use ratatui::{
    style::Style,
    text::{Line, Masked, Span, Text},
};
use std::{collections::HashMap, hint::black_box, time::Instant};
fn label(i: usize, plain: bool) -> Text<'static> {
    if plain {
        return Text::raw(format!("Gate {i}"));
    }
    Text::from(vec![
        Line::from(vec![
            Span::styled(format!("Gate {i}\r\n東"), Style::new().cyan()),
            Span::raw(" 👩🏽‍💻"),
        ]),
        Line::raw("café\n終"),
    ])
}
fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    if !matches!(mode.as_str(), "short" | "mixed" | "mask" | "lookup") {
        std::process::exit(2)
    }
    let mut docs = Vec::new();
    let mut fields = Vec::new();
    let mut keys = HashMap::new();
    for i in 0..128 {
        if mode != "mask" {
            docs.push(label(i, mode == "short"));
        }
        if mode == "lookup" {
            keys.insert(label(i, false), i);
        }
        if mode == "mask" {
            fields.push(format!("🚦Gate {i} 👩🏽‍💻"));
        }
    }
    let mut samples = Vec::new();
    let mut checksum = 0;
    for trial in 0..65 {
        let start = Instant::now();
        let sum = match mode.as_str() {
            "lookup" => docs.iter().map(|key| keys[key]).sum::<usize>(),
            "mask" => fields
                .iter()
                .map(|s| black_box(Masked::new(s.as_str(), '●').value()).len())
                .sum(),
            _ => docs
                .iter()
                .map(|doc| black_box(doc.to_string()).len())
                .sum(),
        };
        let elapsed = start.elapsed().as_nanos();
        if trial > 0 {
            assert_eq!(checksum, sum);
        }
        checksum = black_box(sum);
        if trial >= 5 {
            samples.push(elapsed);
        }
    }
    samples.sort_unstable();
    println!("{mode} {} {} {checksum}", samples[29], samples[56]);
    if std::env::args().nth(2).as_deref() == Some("--resident") {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        black_box((&docs, &fields, &keys));
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
