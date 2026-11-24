use ratatui::buffer::{Buffer, CellDiffOption};
use std::fmt::Write;

pub fn compare(
    input: &mut impl Iterator<Item = String>,
    previous: &Buffer,
    buf: &Buffer,
    label: &str,
) {
    let a = buf.area;
    for y in a.top()..a.bottom() {
        let line = input.next().expect("missing Dunder row");
        let mut row = line.split_terminator('|');
        for x in a.left()..a.right() {
            let c = &buf[(x, y)];
            let flag = match c.diff_option {
                CellDiffOption::None => 0,
                CellDiffOption::Skip => -1,
                CellDiffOption::AlwaysUpdate => -2,
                CellDiffOption::ForcedWidth(n) => i32::from(n.get()),
            };
            let expected = format!(
                "{}:{}/{}/{}/{}/{}",
                c.symbol(),
                c.fg,
                c.bg,
                c.underline_color,
                c.modifier.bits(),
                flag
            );
            assert_eq!(
                row.next(),
                Some(expected.as_str()),
                "buffer {a:?} area {label} cell ({x},{y})"
            );
        }
        assert!(row.next().is_none());
    }
    let mut coords = String::new();
    for (x, y, _) in previous.diff_iter(&buf) {
        write!(coords, "{x},{y} ").unwrap();
        if x + 1 == a.right() && y + 1 == a.bottom() {
            break;
        }
    }
    assert_eq!(
        input.next().as_deref(),
        Some(coords.as_str()),
        "diff {a:?} {label}"
    );
}
