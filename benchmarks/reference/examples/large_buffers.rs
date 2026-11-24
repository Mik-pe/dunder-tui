use ratatui::{
    buffer::{Buffer, CellDiffOption},
    layout::Rect,
    style::{Color, Modifier, Style},
};
use std::io::{self, BufRead, BufWriter, Write};

#[path = "../src/raw_cells.rs"]
mod raw_cells;

fn extent(a: Rect, out: &mut impl Write) -> io::Result<()> {
    let blank = Buffer::empty(a);
    let mut f = blank.clone();
    let cells = f.content.len();
    writeln!(out, "A {} {} {} {} {cells}", a.x, a.y, a.width, a.height)?;
    assert!(f.cell((a.right(), a.y)).is_none() && f.cell((a.x, a.bottom())).is_none());
    if cells == 0 {
        return Ok(());
    }
    for n in [
        0,
        cells / 2,
        cells - 1,
        4095,
        4096,
        65534,
        65535,
        65536,
        1048575,
        1048576,
    ] {
        if n < cells {
            let (x, y) = f.pos_of(n);
            assert_eq!(f.index_of(x, y), n);
            f[(x, y)]
                .set_symbol("#")
                .set_style(Style::new().yellow().bold());
            writeln!(out, "I {n} {x} {y}")?;
        }
    }
    rows(&f, out)?;
    for (x, y, c) in blank.diff_iter(&f) {
        assert!(c.modifier.contains(Modifier::BOLD) && c.fg == Color::Yellow);
        writeln!(out, "D {x} {y} {}", c.symbol())?;
    }
    Ok(())
}

fn rows(f: &Buffer, out: &mut impl Write) -> io::Result<()> {
    for row in f.content.chunks(usize::from(f.area.width)) {
        write!(out, "R")?;
        for c in row {
            write!(out, "{}", c.symbol())?;
        }
        writeln!(out, "|")?;
    }
    Ok(())
}

fn reshape() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let mut frames = 0;
    for w in 1..10 {
        for h in [0, 1, 2, 4, 9] {
            let mut f = Buffer::empty(Rect::new(9, 7, 8, 3));
            for y in 7..10 {
                f.set_string(
                    9,
                    y,
                    "a界b❤️cd",
                    Style::new()
                        .cyan()
                        .underlined()
                        .underline_color(Color::Indexed(181)),
                );
                f[(11, y)].set_diff_option(CellDiffOption::Skip);
                f[(16, y)].set_diff_option(CellDiffOption::AlwaysUpdate);
            }
            for area in [Rect::new(65000, 53000, w, h), Rect::new(9, 7, 8, 3)] {
                f.resize(area);
                raw_cells::compare(&mut input, &Buffer::empty(area), &f, "reshape");
                frames += 1;
            }
        }
    }
    assert!(input.next().is_none());
    println!("{frames} reshaped buffers, every raw cell and ordered diff match");
}

fn merge(x: u16, out: &mut impl Write) -> io::Result<()> {
    let mut left = Buffer::empty(Rect::new(x, 65531, 3, 2));
    left.set_string(x, 65531, "ABC", Style::default());
    left.set_string(x, 65532, "DEF", Style::default());
    let mut right = Buffer::empty(Rect::new(6, 65531, 2, 1));
    right.set_string(6, 65531, "XY", Style::default());
    let a = left.area.union(right.area);
    let mut placed = Buffer::empty(a);
    for source in [&left, &right] {
        for (i, cell) in source.content.iter().enumerate() {
            placed[source.pos_of(i)] = cell.clone();
        }
    }
    if x == 25003 {
        left.merge(&right);
        assert_eq!(left, placed);
    } else if !cfg!(debug_assertions) {
        left.merge(&right);
        assert_eq!(left[(58467, 65531)].symbol(), "D");
        assert_eq!(left[(x, 65532)].symbol(), " ");
        assert_eq!(placed[(x, 65532)].symbol(), "D");
    }
    writeln!(out, "M {} {} {} {}", a.x, a.y, a.width, a.height)?;
    rows(&placed, out)
}

fn main() -> io::Result<()> {
    let mut out = BufWriter::new(io::stdout().lock());
    match std::env::args().nth(1).as_deref() {
        Some("--reshape") => reshape(),
        Some("--merge") => {
            merge(25003, &mut out)?;
            merge(62003, &mut out)?;
        }
        _ => {
            for a in [
                Rect::new(0, 0, 65535, 1),
                Rect::new(0, 0, 1, 65535),
                Rect::new(17, 21, 8193, 3),
                Rect::new(8, 27, 3, 8193),
                Rect::new(0, 0, 1025, 1025),
                Rect::new(7, 19, 257, 4097),
                Rect::new(65531, 65532, 4000, 4000),
                Rect::new(19, 65535, 20000, 20),
                Rect::new(65535, 15, 10, 65000),
            ] {
                extent(a, &mut out)?;
            }
        }
    }
    Ok(())
}
