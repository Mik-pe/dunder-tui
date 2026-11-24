use ratatui::buffer::Buffer;
use std::io::{self, BufRead};

fn main() {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let mut old = Buffer::default();
    let mut frames = 0;
    let mut changed = 0;
    while let Some(line) = input.next() {
        if line == "seed" {
            old = serde_json::from_str(&input.next().unwrap()).unwrap();
            continue;
        }
        let next: Buffer = serde_json::from_str(&line).unwrap();
        assert_eq!(old.area.x, next.area.x);
        assert_eq!(old.area.y, next.area.y);
        assert_eq!(old.area.width, next.area.width);
        let actual: Vec<(u16, u16)> = serde_json::from_str(&input.next().unwrap()).unwrap();
        let expected: Vec<_> = old.diff_iter(&next).map(|(x, y, _)| (x, y)).collect();
        assert_eq!(actual, expected, "frame {frames}, area {:?}", next.area);
        changed += actual.len();
        old = next;
        frames += 1;
    }
    assert_eq!(frames, 960);
    println!("{frames} frame diffs, {changed} changed positions in Ratatui order");
}
