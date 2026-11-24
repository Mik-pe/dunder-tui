use ratatui::{
    buffer::Buffer,
    layout::{Margin, Offset},
    style::{Color, Style},
    widgets::{Fill, Shadow, Widget, dimmed},
};

pub struct Panels([Shadow; 3]);

impl Panels {
    pub fn new() -> Self {
        Self([
            Shadow::overlay().style(Style::new().cyan().on_blue()),
            Shadow::light_shade()
                .style(Style::new().magenta())
                .offset(Offset::new(2, 1)),
            Shadow::custom(dimmed()),
        ])
    }

    pub fn draw(&self, buf: &mut Buffer, mode: &str, step: usize) {
        let a = buf.area;
        let alt = mode == "repaint" && step % 2 == 1;
        let dx = if mode == "scroll" {
            (step % 5) as i32 - 2
        } else {
            0
        };
        let background = if alt { Color::White } else { Color::Black };
        Fill::new(".")
            .style(Style::new().bg(background))
            .render(a, buf);
        for (index, shadow) in self.0.iter().enumerate() {
            let i = index as u16;
            let area = a
                .inner(Margin::new(4 + 6 * i, 2 + 2 * i))
                .offset(Offset::new(dx, 0));
            let surface = if alt {
                Color::Yellow
            } else {
                Color::Rgb(0x35, 0x5a, 0x8f)
            };
            Fill::new(" ")
                .style(Style::new().bg(surface))
                .render(area, buf);
            shadow.render(area, buf);
        }
        buf.set_string(
            0,
            23,
            if mode == "cell" && step % 2 == 1 {
                "+"
            } else {
                "-"
            },
            Style::new(),
        );
    }
}
