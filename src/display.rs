use std::collections::HashMap;
use std::f64::consts::SQRT_2;

use glam;
use glam::Vec3Swizzles;
use piston_window as pw;
use pw::{Transformed, Graphics, DrawState};

use super::world_state;

const IDENT_TRANSFORM: [[f64; 3]; 2] = [[1., 0., 0.], [0., 1., 0.]];

// const ANT_POLY: [[f64; 2]; 24] = [
//     [0.5, 0.],
//     [1., -0.25],
//     [0.5, -0.5],
//     [0., -1.5],
//     [-1., -1.5],
//     [-1.5, -0.5],
//     [-2., -1.5],
//     [-3., -1.5],
//     [-3.5, -0.5],
//     [-4., -1.5],
//     [-6., -1.5],
//     [-6.5, -0.5],
//     //
//     [-6.5, 0.5],
//     [-6., 1.5],
//     [-4., 1.5],
//     [-3.5, 0.5],
//     [-3., 1.5],
//     [-2., 1.5],
//     [-1.5, 0.5],
//     [-1., 1.5],
//     [0., 1.5],
//     [0.5, 0.5],
//     [1., 0.25],
//     [0.5, 0.],
// ];

const ANT_TRIS: [[f64; 2]; 20 * 3] = [
    // top mandible
    [0.5, 0.],
    [1., -0.25],
    [0.5, -0.5],

    // bottom mandible
    [0.5, 0.5],
    [1., 0.25],
    [0.5, 0.],

    // head
    [0.5, -0.5],
    [0., -1.5],
    [-1., -1.5],

    [0.5, -0.5],
    [-1., -1.5],
    [-1.5, -0.5],

    [0.5, -0.5],
    [-1.5, -0.5],
    [-1.5, 0.5],

    [0.5, -0.5],
    [-1.5, 0.5],
    [-1., 1.5],

    [0.5, -0.5],
    [-1., 1.5],
    [0., 1.5],

    [0.5, -0.5],
    [0., 1.5],
    [0.5, 0.5],

    // thorax
    [-1.5, -0.5],
    [-2., -1.5],
    [-3., -1.5],

    [-1.5, -0.5],
    [-3., -1.5],
    [-3.5, -0.5],

    [-1.5, -0.5],
    [-3.5, -0.5],
    [-3.5, 0.5],

    [-1.5, -0.5],
    [-3.5, 0.5],
    [-3., 1.5],

    [-1.5, -0.5],
    [-3., 1.5],
    [-2., 1.5],

    [-1.5, -0.5],
    [-2., 1.5],
    [-1.5, 0.5],

    // abdomen
    [-3.5, -0.5],
    [-4., -1.5],
    [-6., -1.5],

    [-3.5, -0.5],
    [-6., -1.5],
    [-6.5, -0.5],

    [-3.5, -0.5],
    [-6.5, -0.5],
    [-6.5, 0.5],

    [-3.5, -0.5],
    [-6.5, 0.5],
    [-6., 1.5],

    [-3.5, -0.5],
    [-6., 1.5],
    [-4., 1.5],

    [-3.5, -0.5],
    [-4., 1.5],
    [-3.5, 0.5],
];

/// Transformed x coordinate as f32.
#[inline(always)]
fn tx(m: pw::types::Matrix2d, x: pw::types::Scalar, y: pw::types::Scalar) -> f32 {
    (m[0][0] * x + m[0][1] * y + m[0][2]) as f32
}

/// Transformed y coordinate as f32.
#[inline(always)]
fn ty(m: pw::types::Matrix2d, x: pw::types::Scalar, y: pw::types::Scalar) -> f32 {
    (m[1][0] * x + m[1][1] * y + m[1][2]) as f32
}

#[inline(always)]
fn to_f32_color(r: u8, g: u8, b: u8) -> [f32; 4] {
    [r as f32 / 256.0, g as f32 / 256.0, b as f32 / 256.0, 1.0]
}

pub struct Display {
    win_size: glam::DVec2,
    cam_pos: glam::DVec3,
    mouse_pos: glam::DVec2,
    left_mouse_down: bool,
    draw_grid: bool,
}

impl Display {
    pub fn new(win_size: [f64; 2]) -> Self {
        Self {
            win_size: glam::DVec2::from_array(win_size),
            cam_pos: glam::DVec3::new(0., 0., 1.),
            mouse_pos: glam::DVec2::new(0., 0.),
            left_mouse_down: false,
            draw_grid: false,
        }
    }

    #[inline(always)]
    fn get_scale(&self) -> f64 {
        // cam_pos.z should always be > 0. If that's not the case, then
        // something is very wrong and the app needs to die a horrible death.
        1. / self.cam_pos.z
    }

    #[inline(always)]
    fn get_scale_inv(&self) -> f64 {
        self.cam_pos.z
    }

    fn handle_resize(&mut self, args: &pw::ResizeArgs) {
        self.win_size = glam::DVec2::from_array(args.window_size);
    }

    fn handle_button_press(&mut self, button: pw::Button) {
        match button {
            pw::Button::Mouse(pw::MouseButton::Left) => {
                self.left_mouse_down = true;
            }
            pw::Button::Keyboard(pw::Key::G) => {
                self.draw_grid = !self.draw_grid;
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
            self.cam_pos.x -= params[0] * self.get_scale_inv();
            self.cam_pos.y += params[1] * self.get_scale_inv();
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

    fn draw_grid(
        &self,
        transform: [[f64; 3]; 2],
        graphics: &mut pw::G2d<'_>,
    ) {
        const GRID_RESOLUTION: f64 = 20.;
        const GRID_COLOR: [f32; 4] = [0., 0., 0., 1.];

        let line_radius = 0.5 / self.get_scale();

        let viewport_size = self.win_size * self.get_scale_inv();
        let viewport_bottom_left = self.cam_pos.xy() - (viewport_size * 0.5);
        let viewport_top_right = viewport_bottom_left + viewport_size;
        
        let get_start = |s: f64| {
            if s >= 0. {
                GRID_RESOLUTION - (s % GRID_RESOLUTION) + s
            } else {
                (-s % GRID_RESOLUTION) + s
            }
        };

        let mut line_x = get_start(viewport_bottom_left.x);
        while line_x <= viewport_top_right.x {
            pw::line(
                GRID_COLOR,
                line_radius,
                [line_x, viewport_bottom_left.y, line_x, viewport_top_right.y],
                transform,
                graphics);

            line_x += GRID_RESOLUTION;
        }

        let mut line_y = get_start(viewport_bottom_left.y);
        while line_y <= viewport_top_right.y {
            pw::line(
                GRID_COLOR,
                line_radius,
                [viewport_bottom_left.x, line_y, viewport_top_right.x, line_y],
                transform,
                graphics);

            line_y += GRID_RESOLUTION;
        }
    }

    fn draw_home_locs(
        &self,
        home_locs: &Vec<glam::DVec2>,
        transform: [[f64; 3]; 2],
        graphics: &mut pw::G2d<'_>,
    ) {
        for home_loc in home_locs {
            const RAD: f64 = world_state::WorldState::HOME_RADIUS;
            const HOME_COLOR: [f32; 4] = [0., 0., 0., 1.];
            let rect = pw::ellipse::centered([home_loc.x, home_loc.y, RAD, RAD]);
            pw::ellipse(HOME_COLOR, rect, transform, graphics);
        }
    }

    fn draw_ants(
        &self,
        ant_poses: &HashMap<u64, world_state::Pose>,
        transform: [[f64; 3]; 2],
        draw_state: &DrawState,
        graphics: &mut pw::G2d<'_>,
    ) {
        // The ants are concave polys, so we have to triangulate them ourselves.
        let mut ant_tris: Vec<[f32; 2]> = vec![];
        ant_tris.reserve(ANT_TRIS.len() * ant_poses.len());
        for ant_pose in ant_poses.values() {
            let ant_model_transform = IDENT_TRANSFORM
                .scale(2., 2.)
                .trans(ant_pose.pos.x, ant_pose.pos.y)
                .rot_rad(ant_pose.dir);

            let ant_transform =
                transform
                .append_transform(ant_model_transform);

            for vertex in ANT_TRIS {
                ant_tris.push([
                    tx(ant_transform, vertex[0], vertex[1]),
                    ty(ant_transform, vertex[0], vertex[1]),
                ]);
            }
        }

        let ant_color: [f32; 4] = to_f32_color(86, 101, 115);
        graphics.tri_list(draw_state, &ant_color, |f| f(&ant_tris[..]));
    }

    pub fn draw_env(
        &self,
        ws: &world_state::WorldState,
        context: &pw::Context,
        graphics: &mut pw::G2d<'_>,
    ) {
        let ground_bg_color: [f32; 4] = to_f32_color(218, 165, 32);
        pw::clear(ground_bg_color, graphics);

        let center = self.win_size * 0.5;

        let scale = self.get_scale();

        let cam_transform = IDENT_TRANSFORM
            .trans(center.x, center.y)
            .scale(scale, scale)
            .trans(-self.cam_pos.x, self.cam_pos.y)
            .flip_v();

        let transform_cam_only = context.transform.append_transform(cam_transform);

        if self.draw_grid {
            self.draw_grid(transform_cam_only, graphics);
        }

        self.draw_home_locs(&ws.home_locs, transform_cam_only, graphics);

        self.draw_ants(&ws.ant_poses, transform_cam_only, &context.draw_state, graphics);

        // Center dot for debug if needed.
        // pw::ellipse(
        //     [1.0, 0.0, 0.0, 1.0],
        //     pw::ellipse::centered(
        //         [center.x, center.y, 5., 5.]),
        //         context.transform,
        //         graphics);
    }
}
