use ratatui::{
    buffer::Buffer,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Fill, Widget},
};

pub struct Program {
    cues: Text<'static>,
    other: Vec<Line<'static>>,
}

fn set(evening: bool) -> Vec<Line<'static>> {
    let label = if evening { "  Scenljus" } else { "  Ljudprov" };
    (1..24)
        .map(|i| {
            Line::from(vec![
                Span::styled(i.to_string(), Style::new().cyan().bold()),
                Span::raw(label),
                Span::styled("  ✓", Style::new().green()),
            ])
        })
        .collect()
}
impl Program {
    pub fn new() -> Self {
        Self {
            cues: Text::from(set(false)).centered(),
            other: set(true),
        }
    }
    pub fn draw(&mut self, buf: &mut Buffer, mode: &str, step: usize) {
        if mode == "scroll" && step > 0 {
            let first = self.cues.lines.remove(0);
            self.cues.lines.push(first);
        }
        if mode == "repaint" && step > 0 {
            std::mem::swap(&mut self.cues.lines, &mut self.other);
        }
        let style = if mode == "repaint" && step % 2 == 1 {
            Style::new().yellow().on_blue()
        } else {
            Style::new().cyan()
        };
        Fill::new(" ").style(style).render(buf.area, buf);
        (&self.cues).render(buf.area, buf);
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
