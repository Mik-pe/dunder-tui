#[cfg(feature = "allocation-counts")]
#[path = "../src/allocation.rs"]
mod allocation;
#[path = "../src/chart_scene.rs"]
mod chart_scene;
#[path = "../src/frames.rs"]
mod frames;
fn main() {
    let data = chart_scene::Data::new();
    let mut scene = chart_scene::Scene::new(&data);
    frames::run(
        &["idle", "cell", "scroll", "resize", "repaint"],
        |buf, mode, n| scene.draw(buf, mode, n),
    );
    #[cfg(feature = "allocation-counts")]
    allocation::report();
}
