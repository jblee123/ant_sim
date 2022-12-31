use std::f64::consts::PI;

use rand::SeedableRng;
use rand_distr::{self, Distribution};

pub struct Readings {}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    // None,
    Move(glam::DVec2),
    // PickUp,
    // Drop,
    // Eat,
    // Feed,
    // LayScent,
}

const WANDER_PERSIST: u32 = 5;

#[derive(Debug)]
pub struct Ant {
    pub id: u64,
    rng: rand::rngs::StdRng,
    norm_dist: rand_distr::Normal<f64>,
    wander_moves_remaining: u32,
}

impl Ant {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            rng: rand::rngs::StdRng::seed_from_u64(id),
            norm_dist: rand_distr::Normal::new(0., PI / 4.).unwrap(),
            wander_moves_remaining: rand::random::<u32>() % (WANDER_PERSIST + 1),
        }
    }

    pub fn get_actions(&mut self, _readings: &Readings) -> Vec<Action> {
        let move_rand = self.wander();

        vec![move_rand]
        // vec![Action::None]
    }

    fn wander(&mut self) -> Action {
        let dir = if self.wander_moves_remaining == 0 {
            self.wander_moves_remaining = rand::random::<u32>() % (WANDER_PERSIST + 1);

            self.norm_dist.sample(&mut self.rng)
            // 0.0872664; // ~5 deg (for debug)
        } else {
            self.wander_moves_remaining -= 1;

            0.
        };

        let move_vec = glam::DVec2::from_angle(dir);
        Action::Move(move_vec)
    }
}
