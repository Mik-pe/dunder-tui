use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{List, ListItem, ListState, StatefulWidget},
};
pub struct Scene {
    view: List<'static>,
    pub state: ListState,
}
impl Scene {
    pub fn new(count: usize) -> Self {
        let view = List::new((0..count).map(|i| {
            let mut lines = vec![Line::from(format!("Track {i}"))];
            if i % 3 == 0 {
                lines.push(Line::from("  茶 / é").cyan());
            }
            ListItem::new(Text::from(lines))
        }))
        .highlight_symbol("› ")
        .highlight_style(Style::new().on_blue().bold())
        .scroll_padding(2);
        Self {
            view,
            state: ListState::default(),
        }
    }
    pub fn draw(&mut self, buf: &mut Buffer, mode: &str, n: usize) {
        if n == 0 {
            self.state = ListState::default();
        }
        let selected = if mode == "scroll" {
            40 + n
        } else if mode == "jump" && n % 2 == 1 {
            self.view.len() - 6
        } else {
            40
        };
        self.state.select(Some(selected));
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
