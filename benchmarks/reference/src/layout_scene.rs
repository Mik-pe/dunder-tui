use ratatui::{
    buffer::Buffer,
    layout::{Constraint::*, Layout, Rect},
    style::{Color, Style},
    widgets::Widget,
};

pub struct Scene {
    bands: Layout,
    columns: Layout,
    tracks: Layout,
}
impl Scene {
    pub fn new() -> Self {
        Self {
            bands: Layout::vertical([Length(1), Fill(1), Length(1)]),
            columns: Layout::horizontal([Length(13), Percentage(25), Fill(2), Fill(1)]).spacing(1),
            tracks: Layout::vertical([Ratio(1, 4); 4]),
        }
    }
    pub fn draw(&self, buf: &mut Buffer, mode: &str, n: usize) {
        let rows = self.bands.split(buf.area);
        let panes = self.columns.split(rows[1]);
        buf.set_string(
            0,
            0,
            if mode == "cell" && n % 2 == 1 {
                "+"
            } else {
                "-"
            },
            Style::default(),
        );
        let bg = if mode == "repaint" && n % 2 == 1 {
            Color::Magenta
        } else {
            Color::Black
        };
        let phase = if mode == "scroll" { n } else { 0 };
        for (j, &pane) in panes.iter().enumerate() {
            let tracks = self.tracks.split(pane);
            let style =
                Style::new()
                    .bg(bg)
                    .fg([Color::Cyan, Color::Yellow, Color::Blue, Color::Green][j]);
            for (k, &part) in tracks.iter().enumerate() {
                for y in 0..part.height {
                    let glyph = [".", "=", " ", "-"][(usize::from(y) + j + k + phase) % 4];
                    ratatui::widgets::Fill::new(glyph)
                        .style(style)
                        .render(Rect::new(part.x, part.y + y, part.width, 1), buf);
                }
            }
        }
        buf.set_string(rows[2].x, rows[2].y, "Tracks", Style::default());
    }
}
