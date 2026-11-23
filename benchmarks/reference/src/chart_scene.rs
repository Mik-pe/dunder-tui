use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    symbols::Marker,
    widgets::{Axis, Chart, Dataset, GraphType, Widget},
};

pub struct Data {
    series: [Vec<(f64, f64)>; 3],
}
impl Data {
    pub fn new() -> Self {
        Self {
            series: std::array::from_fn(|k| {
                (0i32..96)
                    .map(|i| {
                        let v = match k {
                            0 => (i * 3 % 22 - 11).abs() - 5,
                            1 => ((i + 3) % 14 - 7).abs() - 2,
                            _ => i * 7 % 13 - 6,
                        };
                        (f64::from(i), f64::from(v))
                    })
                    .collect()
            }),
        }
    }
}
pub struct Scene<'a> {
    view: Option<Chart<'a>>,
}
impl<'a> Scene<'a> {
    pub fn new(data: &'a Data) -> Self {
        let sets = (0..3)
            .map(|k| {
                Dataset::default()
                    .data(&data.series[k])
                    .name(["Area", "Line", "Dots"][k])
                    .graph_type([GraphType::Area, GraphType::Line, GraphType::Scatter][k])
                    .marker([Marker::Sextant, Marker::Braille, Marker::Dot][k])
                    .style(
                        [
                            Style::new().blue(),
                            Style::new().cyan(),
                            Style::new().yellow(),
                        ][k],
                    )
                    .fill_to_y(-7.)
            })
            .collect();
        Self {
            view: Some(
                Chart::new(sets)
                    .y_axis(
                        Axis::default()
                            .bounds([-7., 7.])
                            .labels(["-7", "0", "7"])
                            .green(),
                    )
                    .hidden_legend_constraints((Constraint::Max(14), Constraint::Length(7))),
            ),
        }
    }

    pub fn draw(&mut self, buf: &mut Buffer, mode: &str, n: usize) {
        let offset = if mode == "scroll" { n % 32 } else { 8 };
        let view = self
            .view
            .take()
            .unwrap()
            .x_axis(Axis::default().bounds([offset as f64, (offset + 60) as f64]))
            .style(if mode == "repaint" && n % 2 == 1 {
                Style::new().on_magenta()
            } else {
                Style::new().on_black()
            });
        let area = Rect::new(0, 0, buf.area.width, buf.area.height - 1);
        (&view).render(area, buf);
        self.view = Some(view);
        buf.set_string(
            0,
            buf.area.height - 1,
            if mode == "cell" && n % 2 == 1 {
                "+"
            } else {
                "-"
            },
            Style::new(),
        );
    }
}
