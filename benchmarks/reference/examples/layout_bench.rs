#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/frames.rs"]
mod frames;
#[path = "../src/layout_scene.rs"]
mod layout_scene;
fn splits() {
    use ratatui::layout::{Constraint::*, Layout, Rect};
    use std::{num::NonZeroUsize, time::Instant};
    Layout::init_cache(NonZeroUsize::new(1).unwrap());
    for mode in ["cached", "changing", "constrained", "overlap"] {
        let rules = if mode == "constrained" {
            vec![Min(30), Fill(2), Length(8)]
        } else if mode == "overlap" {
            vec![Min(20), Fill(2), Length(23), Min(13)]
        } else {
            vec![Length(8), Percentage(20), Fill(1), Fill(2)]
        };
        let layout = Layout::horizontal(rules).spacing(1);
        let layout = if mode == "overlap" {
            layout.flex(ratatui::layout::Flex::SpaceAround).spacing(-2)
        } else {
            layout
        };
        let mut samples = Vec::new();
        let mut checksum = 0u64;
        for batch in 0..35 {
            let start = Instant::now();
            for i in 0..64 {
                let w = if mode == "cached" {
                    48
                } else {
                    40 + (i + batch) % 24
                };
                let (areas, spacers) = layout.split_with_spacers(Rect::new(0, 0, w, 8));
                for r in areas.iter().chain(spacers.iter()) {
                    checksum += u64::from(r.x + 3 * r.width);
                }
            }
            let elapsed = start.elapsed().as_nanos() / 64;
            if batch >= 5 {
                samples.push(elapsed);
            }
        }
        samples.sort_unstable();
        println!("{mode} {} {} {checksum}", samples[14], samples[28]);
    }
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("--splits") {
        splits();
        return;
    }
    let scene = layout_scene::Scene::new();
    frames::run(
        &["idle", "cell", "scroll", "resize", "repaint"],
        |buf, mode, n| scene.draw(buf, mode, n),
    );
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
