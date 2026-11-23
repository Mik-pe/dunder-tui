#[path = "../src/widget_cells.rs"]
mod widget_cells;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{
        Block, Padding, Widget,
        calendar::{CalendarEventStore, DateStyler, Monthly},
    },
};
use std::io::{self, BufRead};
use time::{Date, Duration, Month, Weekday};

struct Events {
    store: CalendarEventStore,
    weekends: bool,
}
impl DateStyler for Events {
    fn get_style(&self, date: Date) -> Style {
        let base = if self.weekends && matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
        {
            Style::new().cyan().dim()
        } else {
            Style::new()
        };
        base.patch(self.store.get_style(date))
    }
}

fn dates(mut input: impl Iterator<Item = io::Result<String>>) {
    let events = CalendarEventStore::default();
    for year in 1600..2000 {
        for m in 1..=12 {
            let month = Month::try_from(m).unwrap();
            let first = Date::from_calendar_date(year, month, 1).unwrap();
            let prior = first.previous_day().unwrap();
            let end = first.replace_day(month.length(year)).unwrap();
            let next = end.next_day().unwrap();
            let grid = Monthly::new(first, &events);
            let expected = format!(
                "{year} {m} {} {} {} {} {} {} {} {} {}",
                end.day(),
                first.weekday().number_days_from_sunday(),
                grid.width(),
                grid.height(),
                prior.year(),
                u8::from(prior.month()),
                prior.day(),
                end.weekday().number_days_from_sunday(),
                end.day().min(next.month().length(next.year()))
            );
            assert_eq!(input.next().unwrap().unwrap(), expected);
        }
    }
    assert!(input.next().is_none());
    println!("4800 months matched over a complete Gregorian cycle");
}

fn main() {
    let mut input = io::stdin().lock().lines();
    if std::env::args().any(|a| a == "--dates") {
        return dates(input);
    }
    let mut count = 0;
    let mut overflow = 0;
    for index in 0..36 {
        let year = [-44, 0, 1899, 1900, 1999, 2000, 2015, 2024, 2099][index % 9];
        let m = (1 + (index * 5 + index / 12) % 12) as u8;
        let date = Date::from_calendar_date(year, Month::try_from(m).unwrap(), 13).unwrap();
        for mask in 0..16 {
            let mut events = Events {
                store: CalendarEventStore::default(),
                weekends: index % 2 == 1,
            };
            for offset in [-1, 0, 1, 14, 27, 30, 31] {
                let d = date.replace_day(1).unwrap() + Duration::days(offset);
                events.store.add(d, Style::new().yellow());
                events
                    .store
                    .add(d, Style::new().red().on_magenta().not_bold());
            }
            let mut month =
                Monthly::new(date, events).default_style(Style::new().yellow().on_cyan().italic());
            if mask & 1 != 0 {
                month = month.show_surrounding(Style::new().blue().dim());
            }
            if mask & 2 != 0 {
                month = month.show_month_header(Style::new().magenta().not_bold());
            }
            if mask & 4 != 0 {
                month = month.show_weekdays_header(Style::new().red().reversed());
            }
            let block = if mask & 8 != 0 {
                let block = Block::bordered()
                    .title("M")
                    .padding(Padding::new(2, 1, 0, 1))
                    .border_style(Style::new().yellow());
                month = month.block(block.clone());
                Some(block)
            } else {
                None
            };
            for (w, h) in [
                (0, 0),
                (1, 1),
                (3, 2),
                (8, 3),
                (16, 5),
                (20, 7),
                (21, 8),
                (25, 12),
                (31, 12),
            ] {
                let area = Rect::new(1, 1, w, h);
                let mut actual = widget_cells::seeded(area);
                let mut base = actual.clone();
                let inner = if let Some(block) = &block {
                    block.render(area, &mut base);
                    block.inner(area)
                } else {
                    area
                };
                (&month).render(area, &mut actual);
                assert_eq!(
                    input.next().unwrap().unwrap(),
                    format!("{} {}", month.width(), month.height())
                );
                let dunder = input.next().expect("missing frame").unwrap();
                if dunder != widget_cells::snapshot(&actual) {
                    let mut corrected = actual.clone();
                    for pos in actual.area.positions().filter(|p| !inner.contains(*p)) {
                        corrected[pos] = base[pos].clone();
                    }
                    assert_eq!(
                        dunder,
                        widget_cells::snapshot(&corrected),
                        "index {index}, mask {mask}, area {area:?}"
                    );
                    overflow += 1;
                }
                count += 1;
            }
        }
    }
    assert!(input.next().is_none());
    println!(
        "{count} calendar frames, {} exact, {overflow} contained reference writes outside the inner area",
        count - overflow
    );
}
