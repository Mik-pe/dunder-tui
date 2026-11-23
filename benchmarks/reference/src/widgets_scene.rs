use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::border,
    text::Line,
    widgets::{Bar, BarChart, Block, Gauge, Sparkline, SparklineBar, Tabs, Widget},
};
pub struct Scene {
    tabs: Tabs<'static>,
    bars: BarChart<'static>,
    signal: Sparkline<'static>,
    gauge: Gauge<'static>,
    base: Style,
}
fn panel(title: &'static str) -> Block<'static> {
    Block::bordered()
        .border_set(border::ROUNDED)
        .title(title)
        .border_style(Style::new().cyan())
}
impl Scene {
    pub fn new(phase: u64, alternate: bool) -> Self {
        let tabs = Tabs::new(["Signal", "Spectrum", "Level"])
            .highlight_style(Style::new().red().bold())
            .select(usize::from(alternate));
        let signal = Sparkline::default()
            .data((0..96).map(|i| {
                SparklineBar::from((i + phase) * 11 % 31).style(if i % 3 == 0 {
                    Style::new().magenta()
                } else {
                    Style::new().cyan()
                })
            }))
            .block(panel(" Trace "))
            .max(30);
        let bars = BarChart::new(
            (0..12)
                .map(|i| Bar::with_label(Line::raw((i + 1).to_string()), (i * 7 + 3) % 20))
                .collect::<Vec<_>>(),
        )
        .block(panel(" Harmonics "))
        .bar_style(if alternate {
            Style::new().yellow()
        } else {
            Style::new().magenta()
        })
        .value_style(Style::new().cyan())
        .bar_width(3);
        let gauge = Gauge::default()
            .ratio(if alternate { 0.8125 } else { 0.375 })
            .use_unicode(true)
            .label("level")
            .gauge_style(Style::new().yellow().on_blue());
        Self {
            tabs,
            bars,
            signal,
            gauge,
            base: if alternate {
                Style::new().on_blue()
            } else {
                Style::new().on_black()
            },
        }
    }
    pub fn draw(&self, buf: &mut Buffer, marker: bool) {
        let a = buf.area;
        buf.set_style(a, self.base);
        (&self.tabs).render(Rect::new(0, 0, a.width, 1), buf);
        (&self.signal).render(Rect::new(0, 1, a.width, 9), buf);
        (&self.bars).render(Rect::new(0, 10, a.width, a.height - 11), buf);
        (&self.gauge).render(Rect::new(0, a.height - 1, a.width, 1), buf);
        buf.set_string(
            a.width - 1,
            a.height - 1,
            if marker { "+" } else { "-" },
            Style::default(),
        );
    }
}
