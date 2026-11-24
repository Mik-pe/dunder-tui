#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/frames.rs"]
mod frames;
#[path = "../src/widget_cells.rs"]
mod widget_cells;

use ratatui::{
    buffer::Buffer,
    layout::{Layout, Rect},
    macros::{constraints, line, row, text, vertical},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Cell, Paragraph, Table, Widget},
};

struct Manifest {
    panels: Layout,
    body: Paragraph<'static>,
    footer: Table<'static>,
}

impl Manifest {
    fn new() -> Self {
        let line = line!["", "  SKÄRGÅRD".cyan(), "  •  ", "KAJ 03".yellow().bold()];
        let mut log = text![line;128];
        for (i, line) in log.lines.iter_mut().enumerate() {
            line.spans[0].content = format!("{i:03}").into();
        }
        Self {
            panels: vertical![==1,*=1,==1],
            body: Paragraph::new(log),
            footer: Table::new([row![Cell::new("ÖPPEN").green();4]], constraints![*=1;4]),
        }
    }

    fn draw(&mut self, buf: &mut Buffer, mode: &str, n: usize) {
        let area = buf.area;
        let [header, body, footer] = self.panels.areas(area);
        if mode == "repaint" {
            buf.set_style(
                area,
                Style::new().bg(if n % 2 == 0 {
                    Color::Blue
                } else {
                    Color::Magenta
                }),
            );
        }
        let title = if mode == "cell" && n % 2 == 1 {
            "TRAFIK +"
        } else {
            "TRAFIK -"
        };
        Line::styled(title, Style::new().light_cyan().bold()).render(header, buf);
        self.body = std::mem::take(&mut self.body).scroll((
            if mode == "scroll" {
                (n % 100) as u16
            } else {
                0
            },
            0,
        ));
        (&self.body).render(body, buf);
        (&self.footer).render(footer, buf);
    }
}

fn main() {
    let mut manifest = Manifest::new();
    let cases = ["idle", "cell", "scroll", "resize", "repaint"];
    if std::env::args().nth(1).as_deref() == Some("--check") {
        for mode in cases {
            for n in 0..220 {
                let w = if mode == "resize" && n % 2 == 0 {
                    80
                } else {
                    96
                };
                let mut buffer = Buffer::empty(Rect::new(0, 0, w, 24));
                manifest.draw(&mut buffer, mode, n);
                println!("{}", widget_cells::snapshot(&buffer));
            }
        }
    } else {
        frames::run(&cases, |buf, mode, n| manifest.draw(buf, mode, n));
        #[cfg(feature = "allocation-counts")]
        allocation::report();
    }
}
