use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Cell, Row, StatefulWidget, Table, TableState},
};
pub struct Scene {
    view: Table<'static>,
    count: usize,
    pub state: TableState,
}
impl Scene {
    pub fn new(count: usize) -> Self {
        let rows = (0..count).map(|i| {
            let mut lines = vec![Line::from(format!("Track {i}"))];
            if i % 3 == 0 {
                lines.push(Line::from("  茶 / é").cyan());
            }
            Row::new([
                Cell::new(i.to_string()),
                Cell::new(Text::from(lines)),
                Cell::new("OPEN"),
                Cell::new((i % 12 + 1).to_string()),
            ])
            .height(if i % 3 == 0 { 2 } else { 1 })
        });
        let view = Table::new(
            rows,
            [
                Constraint::Length(7),
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Length(5),
            ],
        )
        .header(Row::new(["ID", "Track", "Status", "Bay"]))
        .footer(Row::new(["", "Departures"]))
        .highlight_symbol("› ")
        .row_highlight_style(Style::new().on_blue().bold())
        .column_highlight_style(Style::new().yellow())
        .cell_highlight_style(Style::new().cyan());
        Self {
            view,
            count,
            state: TableState::default(),
        }
    }
    pub fn draw(&mut self, buf: &mut Buffer, mode: &str, n: usize) {
        if n == 0 {
            self.state = TableState::default();
        }
        let selected = if mode == "scroll" {
            40 + n
        } else if mode == "jump" && n % 2 == 1 {
            self.count - 6
        } else {
            40
        };
        self.state.select_cell(Some((selected, 1)));
        let area = buf.area;
        let bounds = Rect::new(0, 0, area.width, area.height - 1);
        let background = if mode == "repaint" && n % 2 == 1 {
            Style::new().on_magenta()
        } else {
            Style::new().on_black()
        };
        buf.set_style(bounds, background);
        StatefulWidget::render(&self.view, bounds, buf, &mut self.state);
        buf.set_string(
            0,
            area.height - 1,
            if mode == "cell" && n % 2 == 1 {
                "+"
            } else {
                "-"
            },
            Style::new(),
        );
    }
}
