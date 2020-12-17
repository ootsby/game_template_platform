pub struct Config {
    pub target_update_fps: u32,
    pub gravity_force: f32,
}

impl Default for Config {
    fn default() -> Self {
        let target_update_fps = 50;
        let gravity_force = 0.1;

        Self {
            target_update_fps,
            gravity_force,
        }
    }
}