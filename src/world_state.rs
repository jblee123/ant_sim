use glam;
pub struct WorldState {
    pub home_locs: Vec<glam::DVec2>,
}

impl WorldState {
    pub const HOME_RADIUS: f64 = 10.0;

    pub fn new(home_x: f64, home_y: f64) -> Self {
        Self {
            home_locs: vec![glam::DVec2::new(home_x, home_y)],
        }
    }
}
