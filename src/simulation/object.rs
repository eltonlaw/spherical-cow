pub trait MovingPoint {
    // FIXME: see if can move getters out and use
    // enum somehow
    fn get_pos(&self) -> na::Vector3<f32>;
    fn get_velocity(&self) -> na::Vector3<f32>;
    fn get_acceleration(&self) -> na::Vector3<f32>;
    fn update(&self, time_elapsed: u32);
}
