use std::collections::HashMap;

use glam;

#[derive(Default)]
pub struct Pose {
    pub pos: glam::DVec3,
    pub dir: f64,
}

pub struct WorldState {
    pub home_locs: Vec<glam::DVec2>,
    pub ant_poses: HashMap<u64, Pose>,
}

impl WorldState {
    pub const HOME_RADIUS: f64 = 10.0;

    pub fn new(home_loc: glam::DVec2) -> Self {
        Self {
            home_locs: vec![home_loc /*, glam::DVec2::new(30., 20.)*/],
            ant_poses: HashMap::new(),
        }
    }
}
