use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Wrap};

pub fn document() -> Paragraph<'static> {
    let lines = (0..48)
        .map(|i| {
            let title = if i % 2 == 0 {
                "Polarnatt"
            } else {
                "Efterglöd"
            };
            Line::from(vec![
                title.cyan().bold(),
                Span::raw("  界  Ljus över skogen 👩🏽‍💻 och staden  ·  "),
                "Koi 🇸🇪   Färg i rörelse och stilla vatten"
                    .magenta()
                    .italic(),
            ])
        })
        .collect::<Vec<_>>();
    Paragraph::new(Text::from(lines)).wrap(Wrap { trim: true })
}
