#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/frames.rs"]
mod frames;
#[path = "../src/table_scene.rs"]
mod table_scene;
fn main() {
    let mut scene = table_scene::Scene::new(4096);
    frames::run(
        &["idle", "cell", "scroll", "resize", "repaint", "jump"],
        |buf, mode, n| scene.draw(buf, mode, n),
    );
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
