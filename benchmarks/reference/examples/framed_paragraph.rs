#[path = "../src/styled_grid.rs"]
mod styled_grid;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap},
};

fn article(k: u16, align: u16, wrap: Option<Wrap>) -> Paragraph<'static> {
    let content = Text::from(vec![
        Line::from(vec![
            Span::raw(" Natt").yellow().bold(),
            Span::raw(" över 界"),
        ]),
        Line::raw(""),
        Line::raw("Drift 🌙"),
        Line::raw("aaa界").right_aligned().green(),
    ])
    .italic();
    let mut p = Paragraph::new(content).style(
        Style::new()
            .cyan()
            .on_blue()
            .remove_modifier(Modifier::BOLD),
    );
    if let Some(wrap) = wrap {
        p = p.wrap(wrap);
    }
    p = match align {
        0 => p.left_aligned(),
        1 => p.centered(),
        _ => p.right_aligned(),
    };
    if k == 0 {
        return p;
    }
    if k == 4 {
        let set = ratatui::symbols::border::Set {
            vertical_left: "界",
            vertical_right: "👩🏽‍💻",
            ..ratatui::symbols::border::ROUNDED
        };
        return p.block(Block::bordered().border_set(set).title("秋"));
    }
    let b = match k {
        2 => Block::bordered()
            .border_type(BorderType::Rounded)
            .title("N")
            .padding(Padding::new(1, 2, 0, 1)),
        3 => Block::new()
            .borders(Borders::LEFT | Borders::TOP)
            .title_bottom(Line::raw("S").right_aligned())
            .padding(Padding::new(0, 1, 1, 0)),
        _ => Block::new(),
    };
    p.block(
        b.style(Style::new().red().on_magenta().underlined())
            .title_style(Color::Green)
            .border_style(Color::Yellow),
    )
}

fn main() {
    for a in [Rect::new(0, 0, 16, 9), Rect::new(5, 3, 16, 9)] {
        for flags in [false, true] {
            for wrap in [None, Some(Wrap { trim: true }), Some(Wrap { trim: false })] {
                for k in 0..5 {
                    for align in 0..3 {
                        let mut buf = styled_grid::seed(a, flags);
                        for (step, (dx, dy, w, h)) in [
                            (1, 1, 13, 7),
                            (3, 2, 4, 5),
                            (3, 2, 4, 5),
                            (1, 1, 0, 4),
                            (2, 1, 10, 0),
                            (0, 0, 1, 1),
                            (12, 2, 10, 5),
                            (1, 1, 13, 7),
                        ]
                        .into_iter()
                        .enumerate()
                        {
                            let area = Rect::new(a.x + dx, a.y + dy, w, h);
                            article(k, align, wrap)
                                .scroll(((step % 3) as u16, 0))
                                .render(area, &mut buf);
                            println!("{}", serde_json::to_string(&buf).unwrap());
                        }
                    }
                }
            }
        }
    }
}
