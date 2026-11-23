#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/frames.rs"]
mod frames;
#[path = "../src/widgets_scene.rs"]
mod widgets_scene;
fn main() {
    let mut scenes = (0..8)
        .map(|p| widgets_scene::Scene::new(p, false))
        .collect::<Vec<_>>();
    scenes.push(widgets_scene::Scene::new(0, true));
    frames::run(
        &["idle", "cell", "scroll", "resize", "repaint"],
        |buf, mode, n| {
            let index = if mode == "scroll" {
                n % 8
            } else if mode == "repaint" && n % 2 == 1 {
                8
            } else {
                0
            };
            scenes[index].draw(buf, mode == "cell" && n % 2 == 1);
        },
    );
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
