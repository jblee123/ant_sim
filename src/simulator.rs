use std::collections::HashMap;

use glam;

use crate::ant;
use crate::reading_source;
use crate::world_state;

pub struct Simulator {
    pub ws: world_state::WorldState,
    pub ants: HashMap<u64, ant::Ant>,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            ws: world_state::WorldState::new(glam::DVec2::new(0., 0.)),
            ants: HashMap::new(),
        }
    }

    pub fn add_ant(&mut self, id: u64, pose: world_state::Pose) -> Result<(), String> {
        if self.ants.contains_key(&id) {
            return Err(std::format!("ant ID {} already exists", id));
        }

        self.ants.insert(id, ant::Ant::new(id));
        self.ws.ant_poses.insert(id, pose);

        Ok(())
    }

    fn exec_action(action: ant::Action, ant: &mut ant::Ant, ws: &mut world_state::WorldState) {
        match action {
            ant::Action::Move(movement) => {
                let pose = ws.ant_poses.get_mut(&ant.id).unwrap();
                let ant_angle_vec = glam::DVec2::from_angle(pose.dir);
                let movement = ant_angle_vec.rotate(movement);
                pose.pos.x += movement.x;
                pose.pos.y += movement.y;
                pose.dir = glam::DVec2::new(1., 0.).angle_between(movement);
            } // _ => {}
        };
    }

    pub fn step(&mut self) {
        for ant in self.ants.values_mut() {
            let readings = reading_source::get_readings(ant, &mut self.ws);
            let actions = ant.get_actions(&readings);
            for action in actions {
                Self::exec_action(action, ant, &mut self.ws);
            }
        }
    }
}
