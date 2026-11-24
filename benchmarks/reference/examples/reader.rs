#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/frames.rs"]
mod frames;
#[path = "../src/paragraph_document.rs"]
mod paragraph_document;
use ratatui::{
    buffer::Buffer,
    style::{Color, Style},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

fn page(night: bool) -> Paragraph<'static> {
    paragraph_document::document()
        .style(Style::new().bg(if night { Color::Blue } else { Color::Black }))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" Journal ")
                .padding(Padding::horizontal(1))
                .border_style(Color::Cyan),
        )
}

fn draw(pages: &mut [Paragraph; 2], buf: &mut Buffer, mode: &str, n: usize) {
    let i = if mode == "repaint" { n % 2 } else { 0 };
    let top = if mode == "scroll" { n % 48 } else { 0 };
    pages[i] = std::mem::take(&mut pages[i]).scroll((top as u16, 0));
    (&pages[i]).render(buf.area, buf);
    buf.set_string(
        buf.area.width - 2,
        buf.area.height - 1,
        if mode == "cell" && n % 2 == 1 {
            "+"
        } else {
            "-"
        },
        Style::default(),
    );
}

fn main() {
    let mut pages = [page(false), page(true)];
    #[cfg(feature = "theme-oracle")]
    if std::env::args().any(|arg| arg == "--snapshots") {
        for w in [80, 96] {
            for mode in ["scroll", "cell", "repaint"] {
                for n in 0..if mode == "scroll" { 48 } else { 2 } {
                    let mut b = Buffer::empty(ratatui::layout::Rect::new(0, 0, w, 24));
                    draw(&mut pages, &mut b, mode, n);
                    println!("{}", serde_json::to_string(&b).unwrap());
                }
            }
        }
        return;
    }
    frames::run(
        &["idle", "cell", "scroll", "resize", "repaint"],
        |buf, mode, n| draw(&mut pages, buf, mode, n),
    );
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
