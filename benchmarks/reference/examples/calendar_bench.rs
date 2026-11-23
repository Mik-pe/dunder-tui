#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/frames.rs"]
mod frames;
#[allow(dead_code)]
#[path = "../src/widget_cells.rs"]
mod widget_cells;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{
        Widget,
        calendar::{CalendarEventStore, Monthly},
    },
};
use std::io::{self, BufRead};
use time::{Date, Month};

fn main() {
    let months: Vec<_> = (1..=12)
        .map(|m| {
            let month = Month::try_from(m).unwrap();
            let date = |d| Date::from_calendar_date(2024, month, d).unwrap();
            let mut events = CalendarEventStore::default();
            for d in [5, 12, 19, 26] {
                events.add(date(d), Style::new().red().on_blue());
            }
            Monthly::new(date(1), events)
                .default_style(Style::new().cyan())
                .show_surrounding(Style::new().blue().dim())
                .show_month_header(Style::new().yellow().bold())
                .show_weekdays_header(Style::new().green())
        })
        .collect();
    let draw = |buf: &mut Buffer, mode: &str, n: usize| {
        let w = buf.area.width;
        let style = if mode == "repaint" && n % 2 == 1 {
            Style::new().green().on_magenta()
        } else {
            Style::new().green().on_black()
        };
        buf.set_style(buf.area, style);
        let start = if mode == "scroll" { n % 12 } else { 0 };
        for i in 0..6 {
            let col = i as u16 % 3;
            let left = col * w / 3;
            let area = Rect::new(left, (i as u16 / 3) * 11, (col + 1) * w / 3 - left, 10);
            (&months[(start + i) % 12]).render(area, buf);
        }
        buf[(0, 23)]
            .set_symbol(if mode == "cell" && n % 2 == 1 {
                "+"
            } else {
                "-"
            })
            .set_style(style);
    };
    let cases = ["idle", "cell", "scroll", "resize", "repaint"];
    if std::env::args().any(|a| a == "--verify") {
        let mut input = io::stdin().lock().lines();
        for mode in cases {
            for n in 0..220 {
                let w = if mode == "resize" && n % 2 == 0 {
                    80
                } else {
                    96
                };
                let mut buf = Buffer::empty(Rect::new(0, 0, w, 24));
                draw(&mut buf, mode, n);
                assert!(
                    widget_cells::snapshot(&buf) == input.next().unwrap().unwrap(),
                    "{mode} frame {n}"
                );
            }
        }
        assert!(input.next().is_none());
        println!("1100 calendar benchmark frames matched");
        return;
    }
    frames::run(&cases, draw);
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
