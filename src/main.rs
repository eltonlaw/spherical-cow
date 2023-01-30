use spherical_cow::simulation;

fn main() {
    let mut engine = simulation::engine::Engine::new();
    let rocket = simulation::rocket::Rocket::new();
    engine.add_object(Box::new(rocket));
    engine.run_for(2);
}
