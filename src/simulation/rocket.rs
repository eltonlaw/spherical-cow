use crate::simulation::object;

pub struct Rocket {
    pos: na::Vector3<f32>,
    velocity: na::Vector3<f32>,
    acceleration: na::Vector3<f32>,
}

impl Rocket {
    pub fn new() -> Rocket {
        Rocket {
            pos: na::Vector3::new(0., 0., 0.),
            velocity: na::Vector3::new(0., 0., 0.),
            acceleration: na::Vector3::new(0., 0., 0.),
        }
    }
}

impl object::MovingPoint for Rocket {
    fn update(&self, _time_elapsed: u32) { }
    fn get_pos(&self) -> na::Vector3<f32> { self.pos.clone_owned() }
    fn get_velocity(&self) -> na::Vector3<f32> { self.velocity.clone_owned() }
    fn get_acceleration(&self) -> na::Vector3<f32> { self.acceleration.clone_owned() }
}
