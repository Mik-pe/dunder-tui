use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::Rect,
    style::{Color, Modifier, Stylize},
    widgets::{Fill, Widget},
};

pub fn snapshot(buf: &Buffer) -> String {
    let mut out = String::new();
    for y in 0..buf.area.height {
        let mut covered = 0;
        for x in 0..buf.area.width {
            if covered > 0 {
                out.push('~');
                covered -= 1;
                continue;
            }
            let cell = &buf[(x, y)];
            let code = |c| match c {
                Color::Reset => 'A',
                Color::Black => 'B',
                Color::Red => 'C',
                Color::Green => 'D',
                Color::Yellow => 'E',
                Color::Blue => 'F',
                Color::Magenta => 'G',
                Color::Cyan => 'H',
                _ => panic!("{c:?}"),
            };
            out.push(code(cell.fg));
            out.push(code(cell.bg));
            let mut flags = 0;
            for (n, m) in [
                Modifier::BOLD,
                Modifier::ITALIC,
                Modifier::REVERSED,
                Modifier::DIM,
            ]
            .iter()
            .enumerate()
            {
                if cell.modifier.contains(*m) {
                    flags |= 1 << n;
                }
            }
            out.push(char::from(65 + flags));
            out.push_str(cell.symbol());
            out.push('|');
            covered = cell.symbol().cell_width().saturating_sub(1);
        }
    }
    out
}
pub fn seeded(area: Rect) -> Buffer {
    let mut buf = Buffer::empty(Rect::new(0, 0, area.width + 2, area.height + 2));
    Fill::new(".")
        .green()
        .on_blue()
        .bold()
        .render(buf.area, &mut buf);
    buf
}
