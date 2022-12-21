use piston_window as pw;

mod ant;
mod display;
mod reading_source;
mod simulator;
mod world_state;

fn get_updates_per_step(sim_speed: i32) -> u64 {
    match sim_speed {
        1 => 240,
        2 => 120,
        3 => 60,
        4 => 30,
        5 => 15,
        6 => 8,
        7 => 4,
        8 => 2,
        _ => 120,
    }
}

fn handle_input(
    event: &pw::Event,
    sim_speed: &mut i32,
    updates_per_step: &mut u64,
    paused: &mut bool,
) {
    use pw::PressEvent;
    event.press(|params| {
        println!("press: {:?}", params);
        match params {
            pw::Button::Keyboard(pw::Key::Space) => {
                *paused = !*paused;
            }
            pw::Button::Keyboard(pw::Key::Minus) => {
                *sim_speed = std::cmp::max(1, (*sim_speed) - 1);
                *updates_per_step = get_updates_per_step(*sim_speed);
            }
            pw::Button::Keyboard(pw::Key::Equals) => {
                *sim_speed = std::cmp::min(8, (*sim_speed) + 1);
                *updates_per_step = get_updates_per_step(*sim_speed);
            }
            _ => {}
        }
    });
}

fn main() {
    let mut sim = simulator::Simulator::new();

    const NUM_ANTS: u64 = 100;
    (1..(NUM_ANTS + 1)).for_each(|id| {
        let add_ant_result = sim.add_ant(
            id,
            world_state::Pose {
                pos: glam::DVec3::new(10., 15., 0.),
                dir: 0.78539816,
            },
        );
        if let Err(err) = add_ant_result {
            panic!("could not add an initial ant: {}", err);
        }
    });

    let mut display = display::Display::new(DISP_SIZE);

    const DISP_SIZE: [f64; 2] = [800., 600.];
    let mut window: pw::PistonWindow = pw::WindowSettings::new("Ant Sim", DISP_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();

    const START_SPEED: i32 = 3;
    let mut sim_speed = START_SPEED;

    let mut paused = false;

    let mut updates_per_step: u64 = get_updates_per_step(sim_speed);
    let mut steps_since_update = u64::MAX;

    while let Some(event) = window.next() {
        handle_input(&event, &mut sim_speed, &mut updates_per_step, &mut paused);

        display.handle_input(&event);

        use pw::UpdateEvent;
        event.update(|_| {
            if steps_since_update >= updates_per_step {
                if !paused {
                    sim.step();
                }
                steps_since_update = 0;
            } else {
                steps_since_update += 1;
            }
        });

        window.draw_2d(&event, |context, graphics, _device| {
            display.draw_env(&sim.ws, context, graphics);
        });
    }
}
