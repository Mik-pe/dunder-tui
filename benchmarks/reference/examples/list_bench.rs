#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/frames.rs"]
mod frames;
#[path = "../src/list_scene.rs"]
mod list_scene;
fn main() {
    let mut scene = list_scene::Scene::new(16384);
    frames::run(
        &["idle", "cell", "scroll", "resize", "repaint", "jump"],
        |buf, mode, n| scene.draw(buf, mode, n),
    );
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
