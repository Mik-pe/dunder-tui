#[path = "../src/styled_grid.rs"]
mod styled_grid;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Cell, HighlightSpacing, List, ListDirection, ListItem, ListState, Row, StatefulWidget,
        Table, TableState,
    },
};

fn marker(k: usize) -> Line<'static> {
    if k == 0 {
        return Line::raw("NORD");
    }
    Line::from(vec![Span::raw("界").yellow().italic(), Span::raw("→")]).cyan()
}

fn menu(k: usize) -> List<'static> {
    List::new([
        ListItem::from("höst\n雨").on_magenta(),
        ListItem::from("🔭"),
        ListItem::from("slut\n水"),
    ])
    .highlight_symbol(marker(k))
    .highlight_spacing(HighlightSpacing::Always)
    .repeat_highlight_symbol(k == 1)
    .direction(if k == 2 {
        ListDirection::BottomToTop
    } else {
        ListDirection::TopToBottom
    })
    .highlight_style(Style::new().light_green().remove_modifier(Modifier::ITALIC))
}

fn grid(k: usize) -> Table<'static> {
    let word = Cell::from("aveny\n茶").column_span(2).red().underlined();
    Table::new(
        [
            Row::new([Cell::default(), word]).height(2).on_magenta(),
            Row::new(["x", "y"]),
        ],
        [Constraint::Length(0), Constraint::Fill(1)],
    )
    .column_spacing(4)
    .highlight_symbol(Text::from(vec![marker(k), Line::raw("↓")]))
    .highlight_spacing(HighlightSpacing::Always)
    .row_highlight_style(Style::new().light_green().remove_modifier(Modifier::ITALIC))
    .column_highlight_style(Style::new().underlined())
}

fn main() {
    for a in [Rect::new(0, 0, 16, 9), Rect::new(5, 3, 16, 9)] {
        for flags in [false, true] {
            for k in 0..3 {
                for table_first in [false, true] {
                    let mut frame = styled_grid::seed(a, flags);
                    let items = menu(k);
                    let rows = grid(k);
                    let mut selected = ListState::default();
                    let mut cell = TableState::new();
                    for (step, (dx, dy, w, h)) in [
                        (1, 1, 2, 5),
                        (2, 1, 3, 5),
                        (0, 2, 13, 5),
                        (3, 2, 1, 3),
                        (4, 1, 0, 5),
                        (2, 1, 5, 0),
                        (12, 1, 3, 4),
                        (1, 1, 2, 5),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        let area = Rect::new(a.x + dx, a.y + dy, w, h);
                        if step % 4 == 0 {
                            selected.select(None);
                            cell.select_cell(None);
                        } else {
                            selected.select(Some(step % 3));
                            cell.select_cell(Some((step % 2, step % 2)));
                        }
                        if (step % 2 == 0) == table_first {
                            StatefulWidget::render(&rows, area, &mut frame, &mut cell);
                        } else {
                            StatefulWidget::render(&items, area, &mut frame, &mut selected);
                        }
                        println!("{}", serde_json::to_string(&frame).unwrap());
                    }
                }
            }
        }
    }
}
