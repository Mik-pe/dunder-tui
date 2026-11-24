use ratatui::{
    Terminal, TerminalOptions, Viewport,
    backend::{Backend, TestBackend},
    buffer::{Buffer, CellDiffOption},
    layout::Rect,
    style::{Color, Style},
};
use std::io::{self, BufRead};
#[path = "../src/raw_cells.rs"]
mod raw_cells;

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let mut frames = 0;
    for height in [1, 3, 5] {
        for start in [0, 2, 4] {
            let mut device = TestBackend::new(10, 5);
            device.set_cursor_position((1, start)).unwrap();
            let mut term = Terminal::with_options(
                device,
                TerminalOptions {
                    viewport: Viewport::Inline(height),
                },
            )
            .unwrap();
            term.draw(|f| {
                for c in &mut f.buffer_mut().content {
                    c.set_symbol("v").set_fg(Color::Green);
                }
            })
            .unwrap();
            for count in [1, 0, 2, 7, 4] {
                term.insert_before(count, |b| {
                    for y in 0..count {
                        b.set_string(
                            0,
                            y,
                            "A界é",
                            Style::new()
                                .red()
                                .italic()
                                .underlined()
                                .underline_color(Color::Indexed(127)),
                        );
                        b[(4, y)]
                            .set_symbol("S")
                            .set_fg(Color::Blue)
                            .set_diff_option(CellDiffOption::Skip);
                        b[(5, y)]
                            .set_symbol(&y.to_string())
                            .set_fg(Color::Yellow)
                            .set_diff_option(CellDiffOption::AlwaysUpdate);
                        b[(6, y)]
                            .set_symbol("X")
                            .set_fg(Color::Cyan)
                            .set_diff_option(CellDiffOption::ForcedWidth(2.try_into().unwrap()));
                        b[(7, y)].set_symbol("z").set_fg(Color::Magenta);
                    }
                })
                .unwrap();
                let d = term.backend();
                let label = format!("inline={height} start={start} rows={count}");
                raw_cells::compare(
                    &mut input,
                    &Buffer::empty(d.buffer().area),
                    d.buffer(),
                    &label,
                );
                let blank = Buffer::empty(Rect::new(0, 0, 10, 1));
                for y in 0..d.scrollback().area.height {
                    let mut row = blank.clone();
                    for x in 0..10 {
                        row[(x, 0)] = d.scrollback()[(x, y)].clone();
                    }
                    raw_cells::compare(&mut input, &blank, &row, &format!("{label} history {y}"));
                }
                frames += 1;
            }
        }
    }
    assert!(input.next().is_none());
    println!("{frames} complete inline screens, scrollback rows and ordered cell diffs match");
}
