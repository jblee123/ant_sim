use std::collections::HashMap;
use std::collections::HashSet;

use glam;

#[derive(Default)]
pub struct Pose {
    pub pos: glam::DVec3,
    pub dir: f64,
}

pub struct WorldState {
    pub home_locs: Vec<glam::DVec2>,
    pub ant_poses: HashMap<u64, Pose>,
    pub food_locs: HashSet<glam::IVec2>,
}

impl WorldState {
    pub const HOME_RADIUS: f64 = 10.0;
    pub const GRID_SIZE: f64 = 10.0;

    pub fn new(home_loc: glam::DVec2) -> Self {
        Self {
            home_locs: vec![home_loc /*, glam::DVec2::new(30., 20.)*/],
            ant_poses: HashMap::new(),
            food_locs: HashSet::new(),
        }
    }

    pub fn snap_to_grid_center(loc: glam::DVec2) -> glam::DVec2 {
        let mut x = (loc.x / Self::GRID_SIZE).floor() * Self::GRID_SIZE;
        let mut y = (loc.y / Self::GRID_SIZE).floor() * Self::GRID_SIZE;
        let half_grid = Self::GRID_SIZE * 0.5;
        x += half_grid * if loc.x >= 0. { 1. } else { -1. };
        y += half_grid * if loc.y >= 0. { 1. } else { -1. };
        glam::DVec2::new(x, y)
    }

    pub fn add_food_group(&mut self, loc: glam::DVec2, radius: f64) {
        let loc = Self::snap_to_grid_center(loc);
        let radius = (radius / Self::GRID_SIZE).floor() * Self::GRID_SIZE;
        let radius_sq = radius * radius;
        let mut pos_x = loc.x - radius;
        let start_pos_y = loc.y - radius;
        let max_x = loc.x + radius;
        let max_y = loc.y + radius;
        while pos_x <= max_x {
            let dx = pos_x - loc.x;
            let mut pos_y = start_pos_y;
            while pos_y <= max_y {
                let dy = pos_y - loc.y;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq <= radius_sq {
                    let pos = glam::IVec2::new(pos_x.round() as i32, pos_y.round() as i32);
                    self.food_locs.insert(pos);
                }

                pos_y += Self::GRID_SIZE;
            }
            pos_x += Self::GRID_SIZE;
        }
    }
}
