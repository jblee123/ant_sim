use std::f64::consts::PI;

use rand::SeedableRng;
use rand_distr::{self, Distribution};

pub struct Readings {}

pub enum Action {
    // None,
    Move(glam::DVec2),
    // PickUp,
    // Drop,
    // Eat,
    // Feed,
    // LayScent,
}

#[derive(Debug)]
pub struct Ant {
    pub id: u64,
    rng: rand::rngs::StdRng,
    norm_dist: rand_distr::Normal<f64>,
}

impl Ant {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            rng: rand::rngs::StdRng::seed_from_u64(id),
            norm_dist: rand_distr::Normal::new(0., PI / 4.).unwrap(),
        }
    }

    pub fn get_actions(&mut self, _readings: &Readings) -> Vec<Action> {
        let dir = self.norm_dist.sample(&mut self.rng);
        // let dir = 0.0872664; // ~5 deg
        let move_vec = glam::DVec2::from_angle(dir);
        let move_rand = Action::Move(move_vec);

        vec![move_rand]
        // vec![Action::None]
    }
}
