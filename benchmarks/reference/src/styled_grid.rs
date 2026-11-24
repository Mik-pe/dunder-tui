use ratatui::{
    buffer::{Buffer, Cell, CellDiffOption},
    layout::Rect,
    style::{Color, Modifier, Style},
};

pub fn seed(a: Rect, flags: bool) -> Buffer {
    let mut tile = Cell::new(".");
    tile.set_style(
        Style::new()
            .fg(Color::Green)
            .bg(Color::Blue)
            .underline_color(Color::Rgb(4, 5, 6))
            .add_modifier(Modifier::BOLD),
    );
    let mut buf = Buffer::filled(a, tile);
    for y in a.top()..a.bottom() {
        buf.set_string(a.x + 1, y, "界", Color::Red);
        buf.set_string(
            a.x + 4,
            y,
            "🦀",
            Style::new().add_modifier(Modifier::ITALIC),
        );
        if flags {
            #[allow(deprecated)]
            buf[(a.x + 2, y)].set_skip(true);
            buf[(a.x + 3, y)].set_diff_option(CellDiffOption::Skip);
            buf[(a.x + 4, y)].set_diff_option(CellDiffOption::AlwaysUpdate);
            buf[(a.x + 6, y)].set_diff_option(CellDiffOption::ForcedWidth(2.try_into().unwrap()));
        }
    }
    buf
}
