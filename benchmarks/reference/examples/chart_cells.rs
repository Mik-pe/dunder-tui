#[path = "../src/styled_grid.rs"]
mod styled_grid;

use ratatui::{
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{Axis, Chart, Dataset, GraphType, LegendPosition, Widget},
};

fn view(k: usize, second: bool) -> Chart<'static> {
    let series = Dataset::default()
        .name(Line::raw("A界").cyan())
        .style(Style::new().yellow().remove_modifier(Modifier::BOLD))
        .data(&[(0., 0.), (1., 1.)])
        .graph_type("Line".parse::<GraphType>().unwrap())
        .marker(Marker::Custom('界'));
    let sets = if second { vec![] } else { vec![series] };
    let mut c = if k != 11 {
        Chart::new(sets)
    } else {
        Chart::default()
    };
    let mut x = Axis::default()
        .bounds([0., 1.])
        .style(Style::new().red().underline_color(Color::Indexed(13)));
    let mut y = Axis::default()
        .bounds([0., 1.])
        .style(Style::new().blue().italic());
    if k % 4 < 2 {
        x = x
            .labels([Line::raw("A界z").yellow(), Line::raw("0")])
            .labels_alignment(Alignment::Right);
    }
    if k % 3 == 0 {
        y = y
            .labels([Line::raw("界"), Line::raw("top").green()])
            .labels_alignment(Alignment::Center);
    }
    if k % 4 == 2 {
        y = y.title(Line::raw("界Y").cyan());
    }
    if k % 4 == 1 {
        x = x.title(Line::raw("X").red());
    }
    if k % 3 == 1 {
        c = c.style(Style::new().on_black().italic());
    }
    if k >= 4 {
        c = c.legend_position(Some(LegendPosition::Top));
    }
    if k >= 8 {
        c = c.hidden_legend_constraints((Constraint::Fill(1), Constraint::Fill(1)));
    }
    if second {
        x = Axis::default();
    }
    c.x_axis(x).y_axis(y)
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--types") {
        println!("{}", GraphType::default());
        for name in [
            "Scatter", "Line", "Bar", "Area", "scatter", "line", "BAR", " Area", "Area ", "",
        ] {
            match name.parse::<GraphType>() {
                Ok(t) => println!("{t}"),
                Err(_) => println!("-"),
            }
        }
        return;
    }
    for a in [Rect::new(0, 0, 13, 7), Rect::new(7, 4, 13, 7)] {
        for area in [
            a,
            Rect::new(a.x + 2, a.y + 1, 8, 5),
            Rect::new(a.x + 5, a.y + 2, 5, 3),
            Rect::new(a.x + 1, a.y, 1, 2),
            Rect::new(a.x + 2, a.y + 1, 0, 2),
        ] {
            for flags in [false, true] {
                for k in 0..13 {
                    let mut buf = styled_grid::seed(a, flags);
                    for second in [false, true] {
                        if k == 12 {
                            let mut inner = ratatui::buffer::Buffer::empty(area);
                            for pos in area.positions() {
                                inner[pos] = buf[pos].clone();
                            }
                            view(1, second)
                                .legend_position(None)
                                .render(area, &mut inner);
                            for pos in area.positions() {
                                buf[pos] = inner[pos].clone();
                            }
                        } else {
                            view(k, second).render(area, &mut buf);
                        }
                        println!("{}", serde_json::to_string(&buf).unwrap());
                    }
                }
            }
        }
    }
}
