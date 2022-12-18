use std::f64::consts::SQRT_2;

use glam;
use piston_window as pw;
use pw::Transformed;

use super::world_state;

fn to_f32_color(r: u8, g: u8, b: u8) -> [f32; 4] {
    [r as f32 / 256.0, g as f32 / 256.0, b as f32 / 256.0, 1.0]
}

pub struct Display {
    win_size: glam::DVec2,
    cam_pos: glam::DVec3,
    mouse_pos: glam::DVec2,
    left_mouse_down: bool,
}

impl Display {
    pub fn new(win_size: [f64; 2]) -> Self {
        Self {
            win_size: glam::DVec2::from_array(win_size),
            cam_pos: glam::DVec3::new(0., 0., 1.),
            mouse_pos: glam::DVec2::new(0., 0.),
            left_mouse_down: false,
        }
    }

    fn handle_resize(&mut self, args: &pw::ResizeArgs) {
        self.win_size = glam::DVec2::from_array(args.window_size);
    }

    fn handle_button_press(&mut self, button: pw::Button) {
        match button {
            pw::Button::Mouse(pw::MouseButton::Left) => {
                self.left_mouse_down = true;
            }
            _ => {}
        }
    }

    fn handle_button_release(&mut self, button: pw::Button) {
        match button {
            pw::Button::Mouse(pw::MouseButton::Left) => {
                self.left_mouse_down = false;
            }
            _ => {}
        }
    }

    fn handle_mouse_cursor(&mut self, params: [f64; 2]) {
        self.mouse_pos = glam::DVec2::new(params[0], params[1]);
    }

    fn handle_mouse_relative(&mut self, params: [f64; 2]) {
        if self.left_mouse_down {
            self.cam_pos.x -= params[0] * self.cam_pos.z;
            self.cam_pos.y += params[1] * self.cam_pos.z;
        }
    }

    fn handle_mouse_scroll(&mut self, params: [f64; 2]) {
        let z_start = self.cam_pos.z;

        const SCALE_FACTOR: f64 = SQRT_2;
        let zooming_in = params[1] > 0.0;
        let zooming_out = params[1] < 0.0;
        if zooming_in {
            self.cam_pos.z /= SCALE_FACTOR;
        } else if zooming_out {
            self.cam_pos.z *= SCALE_FACTOR;
        }

        const MAX_CAM_Z: f64 = 16.0; // 2^4
        const MIN_CAM_Z: f64 = 0.0625; // 1/(2^4)
        if self.cam_pos.z > MAX_CAM_Z {
            self.cam_pos.z = MAX_CAM_Z;
        }
        if self.cam_pos.z < MIN_CAM_Z {
            self.cam_pos.z = MIN_CAM_Z;
        }

        // We want the point under the mouse cursor to stay where it is, so that
        // means moving the camera. This will shift the camera to the mouse
        // cursor (taking the old scale into account) and then shift it back in
        // the opposite direction a distance based on the same number of pixels
        // but at the new scale.
        let center = self.win_size * 0.5;
        let mut pix_offset = self.mouse_pos - center;
        pix_offset.y = -pix_offset.y; // screen coords to space coords
        let cam_offset = pix_offset * (z_start - self.cam_pos.z);
        self.cam_pos.x += cam_offset.x;
        self.cam_pos.y += cam_offset.y;
    }

    pub fn handle_input(&mut self, event: &pw::Event) {
        use pw::mouse::*;
        use pw::PressEvent;
        use pw::ReleaseEvent;
        use pw::ResizeEvent;

        event.resize(|params| {
            self.handle_resize(params);
        });
        event.mouse_cursor(|params| {
            self.handle_mouse_cursor(params);
        });
        event.press(|params| {
            self.handle_button_press(params);
        });
        event.release(|params| {
            self.handle_button_release(params);
        });
        event.mouse_relative(|params| {
            self.handle_mouse_relative(params);
        });
        event.mouse_scroll(|params| {
            self.handle_mouse_scroll(params);
        });
    }

    pub fn draw_env(
        &self,
        ws: &world_state::WorldState,
        context: pw::Context,
        graphics: &mut pw::G2d<'_>,
    ) {
        let ground_bg_color: [f32; 4] = to_f32_color(218, 165, 32);
        pw::clear(ground_bg_color, graphics);

        // let win_size = &context.viewport.unwrap().window_size;
        let center = self.win_size * 0.5;

        let scale = 1. / self.cam_pos.z;
        let transform = context
            .transform
            .trans(center.x, center.y)
            .scale(scale, scale)
            .trans(-self.cam_pos.x, self.cam_pos.y);

        for home_loc in &ws.home_locs {
            const RAD: f64 = world_state::WorldState::HOME_RADIUS;
            const HOME_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            let rect = pw::ellipse::centered([home_loc.x, -home_loc.y, RAD, RAD]);
            pw::ellipse(HOME_COLOR, rect, transform, graphics);
        }

        // Center dot for debug if needed.
        // pw::ellipse(
        //     [1.0, 0.0, 0.0, 1.0],
        //     pw::ellipse::centered(
        //         [center.x, center.y, 5., 5.]),
        //         context.transform,
        //         graphics);
    }
}
