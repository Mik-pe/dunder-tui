#[path = "../src/styled_grid.rs"]
mod styled_grid;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Direction, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Widget},
};

fn setup(k: usize, horizontal: bool) -> BarChart<'static> {
    let mut first = Bar::new(if k % 3 == 0 { 0 } else { (k + 1) as u64 });
    let text = ["0", "界X", "éX", "éZ", "👩🏽‍💻Z", "9", "", "0123456789", "XYZ"];
    if k != 8 {
        first = first.text_value(text[k]);
    }
    first = first
        .style(Style::new().cyan().on_blue())
        .value_style(Style::new().magenta().underline_color(Color::Rgb(2, 4, 7)));
    if k % 3 == 1 {
        first = first.label(Line::raw("Ha界").green());
    }
    let second = if k % 2 == 0 {
        Bar::new(u64::MAX)
    } else {
        Bar::with_label(Line::raw("B").blue(), u64::MAX)
    };
    let label = Line::raw(if k % 2 == 0 { "Gr茶pp" } else { "界GH界" })
        .alignment([Alignment::Left, Alignment::Center, Alignment::Right][k % 3]);
    let mut c = BarChart::default()
        .data(BarGroup::with_label(label, [first, second]))
        .data(BarGroup::default())
        .data(&[("Q", 8), ("AB", 1)])
        .direction(if horizontal {
            Direction::Horizontal
        } else {
            Direction::Vertical
        })
        .bar_width(1 + (k % 4) as u16)
        .bar_gap((k % 2) as u16)
        .group_gap(((k / 2) % 3) as u16);
    if k != 7 {
        c = c.max(if k == 0 { 0 } else { 12 });
    }
    if k % 2 == 1 {
        c = c.style(Style::new().on_black().remove_modifier(Modifier::BOLD));
    }
    c.bar_style(Style::new().yellow().italic())
        .label_style(Style::new().red().underline_color(Color::Indexed(7)))
        .value_style(Style::new().on_green().bold())
}

fn main() {
    if std::env::args().any(|arg| arg == "--caption") {
        for a in [Rect::new(0, 0, 16, 8), Rect::new(5, 3, 16, 8)] {
            let mut frame = Buffer::empty(a);
            let target = Rect::new(a.x + 3, a.y + 2, 2, 1);
            for clip in [false, true] {
                frame.set_string(target.x - 1, target.y, ".........", Style::new());
                let bars = BarChart::horizontal([Bar::with_label("NORD", 0)]);
                if clip {
                    let mut panel = Buffer::empty(target);
                    bars.render(target, &mut panel);
                    for p in target.positions() {
                        frame[p] = panel[p].clone();
                    }
                } else {
                    bars.render(target, &mut frame);
                }
                println!("{}", serde_json::to_string(&frame).unwrap());
            }
        }
        return;
    }
    for a in [Rect::new(0, 0, 16, 8), Rect::new(5, 3, 16, 8)] {
        for area in [
            a,
            Rect::new(a.x + 2, a.y + 1, 9, 5),
            Rect::new(a.x + 3, a.y + 1, 2, 6),
            Rect::new(a.x + 5, a.y + 2, 6, 1),
            Rect::new(a.x + 1, a.y + 2, 0, 3),
        ] {
            for flags in [false, true] {
                for horizontal in [false, true] {
                    for k in 0..9 {
                        let mut buf = styled_grid::seed(a, flags);
                        for phase in 0..2 {
                            let mut c = setup(k, horizontal);
                            if phase == 1 {
                                c = c.bar_gap(2).bar_width(2 + (k % 4) as u16).max(24);
                            }
                            c.render(area, &mut buf);
                            println!("{}", serde_json::to_string(&buf).unwrap());
                        }
                    }
                }
            }
        }
    }
}
