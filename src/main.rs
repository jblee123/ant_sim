use piston_window as pw;

mod display;
mod world_state;

fn main() {
    const DISP_SIZE: [f64; 2] = [800., 600.];
    let ws = world_state::WorldState::new(0.0, 0.0);

    let mut display = display::Display::new(DISP_SIZE);

    let mut window: pw::PistonWindow = pw::WindowSettings::new("Ant Sim", DISP_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        display.handle_input(&event);

        window.draw_2d(&event, |context, graphics, _device| {
            display.draw_env(&ws, context, graphics);
        });
    }
}
