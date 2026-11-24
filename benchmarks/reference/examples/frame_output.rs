#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use std::io::{self, Write};
use std::time::Instant;

const MODES: [&str; 8] = [
    "idle",
    "cell",
    "style",
    "last",
    "scattered",
    "scroll",
    "resize",
    "full",
];

fn picture(w: u16, offset: u16) -> Buffer {
    let mut frame = Buffer::empty(Rect::new(0, 0, w, 24));
    for p in frame.area.positions() {
        let row = p.y + offset;
        let ink = if row % 3 == 0 {
            Color::Red
        } else {
            Color::Cyan
        };
        frame[p]
            .set_char(char::from(b'A' + (row % 26) as u8))
            .set_fg(ink);
    }
    frame
}

fn revised(mode: &str) -> Buffer {
    let mut next = picture(
        if mode == "resize" { 80 } else { 96 },
        u16::from(mode == "scroll"),
    );
    match mode {
        "cell" => {
            next[(63, 12)].set_char('+');
        }
        "style" => {
            next[(63, 12)].set_fg(Color::Yellow);
        }
        "last" => {
            next[(95, 23)].set_char('+');
        }
        "scattered" => {
            for y in 0..24 {
                next[((y * 37) % 96, y)].set_char('#');
            }
        }
        "full" => {
            for cell in &mut next.content {
                cell.set_char('!').set_fg(Color::Green);
            }
        }
        _ => {}
    }
    next
}

fn cells(previous: &Buffer, current: &Buffer) {
    println!("{}", serde_json::to_string(current).unwrap());
    if previous.area.width == current.area.width {
        for (x, y, _) in previous.diff_iter(current) {
            println!("{x} {y}");
        }
    } else {
        for p in current.area.positions() {
            println!("{} {}", p.x, p.y);
        }
    }
}

fn encode(previous: &Buffer, current: &Buffer, output: &mut Vec<u8>) {
    let mut b = CrosstermBackend::new(output);
    if previous.area.width == current.area.width {
        b.draw(previous.diff_iter(current)).unwrap();
    } else {
        b.draw(current.area.positions().map(|p| (p.x, p.y, &current[p])))
            .unwrap();
    }
    Backend::flush(&mut b).unwrap();
}

fn measure(mode: &str, physical: bool, action: &str, inspect: bool) {
    let old = picture(96, 0);
    let next = revised(mode);
    if inspect {
        println!("{mode}");
        cells(&old, &next);
        cells(&next, &old);
        return;
    }
    let mut output = Vec::new();
    let mut sink = io::stdout().lock();
    if physical {
        encode(&Buffer::empty(Rect::ZERO), &old, &mut output);
        sink.write_all(&output).unwrap();
        sink.flush().unwrap();
    }
    if action == "once" {
        output.clear();
        encode(&old, &next, &mut output);
        sink.write_all(&output).unwrap();
        sink.flush().unwrap();
        return;
    }
    if action == "setup" {
        return;
    }
    let mut samples = Vec::with_capacity(200);
    let mut total = 0;
    for n in 0..220 {
        output.clear();
        let start = Instant::now();
        let (previous, current) = if n % 2 == 0 {
            (&old, &next)
        } else {
            (&next, &old)
        };
        encode(previous, current, &mut output);
        if physical {
            sink.write_all(&output).unwrap();
            sink.flush().unwrap();
        }
        let elapsed = start.elapsed().as_nanos();
        total += output.len();
        if n >= 20 {
            samples.push(elapsed);
        }
    }
    samples.sort_unstable();
    eprintln!("{mode} {} {} {total}", samples[99], samples[189]);
}

fn main() {
    ratatui::crossterm::style::force_color_output(true);
    let args: Vec<_> = std::env::args().skip(1).collect();
    let arg = args.first().map(String::as_str).unwrap_or("");
    assert!(matches!(arg, "" | "--terminal" | "--cells" | "--memory"));
    let action = args.get(2).map(String::as_str).unwrap_or("");
    assert!(
        args.len() <= 3
            && (matches!(action, "" | "setup") || (arg == "--terminal" && action == "once"))
    );
    if let Some(mode) = args.get(1) {
        assert!(MODES.contains(&mode.as_str()));
    }
    for mode in MODES {
        if args.get(1).is_some_and(|v| v != mode) {
            continue;
        }
        measure(mode, arg == "--terminal", action, arg == "--cells");
    }
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
