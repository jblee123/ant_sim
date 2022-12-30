use std::f64::consts::SQRT_2;

use gl;
use glam;
use piston_window as pw;
use pw::{Transformed, Graphics};

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

    ant_vert_shader: gl::types::GLuint,
}

impl Display {
    pub fn new(win_size: [f64; 2]) -> Self {
        Self {
            win_size: glam::DVec2::from_array(win_size),
            cam_pos: glam::DVec3::new(0., 0., 1.),
            mouse_pos: glam::DVec2::new(0., 0.),
            left_mouse_down: false,
            ant_vert_shader: 0,
        }
    }

    fn shader_from_source(&self, glapi: &gfx_gl::Gl, source: &str) -> Result<gl::types::GLuint, String> {
        use shader_version::Shaders;
        Ok(0)
    }

    pub fn init(&mut self, device: &mut gfx_device_gl::Device) {
        let vertex_shader_source = "\
            #version 320 core \
            layout (location = 0) in vec2 aPos; \
            void main() { \
               gl_Position = vec4(aPos.x, aPos.y, 1.0, 1.0); \
            }";

        unsafe {
            device.with_gl(|glapi| {
                self.ant_vert_shader = glapi.CreateShader(gl::VERTEX_SHADER);

                self.shader_from_source(glapi, vertex_shader_source);
            });
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

    fn draw_ants(
        &self,
        ws: &world_state::WorldState,
        cam_transform: &[[f64; 3]; 2],
        context: &pw::Context,
        graphics: &mut pw::G2d<'_>,
    ) {
        // use std::time::Instant;
        // use std::time::Duration;
        // static mut avg_duration: Duration = Duration::new(0, 0);

        // let now = Instant::now();

        // Draw the ants. They are concave polys, so we have to triangulate
        // them ourselves.
        let mut ant_tris: Vec<[f32; 2]> = vec![];
        ant_tris.reserve(ANT_TRIS.len() * ws.ant_poses.len());
        for ant_pose in ws.ant_poses.values() {
            let ant_model_transform = IDENT_TRANSFORM
                .scale(2., 2.)
                .trans(ant_pose.pos.x, ant_pose.pos.y)
                .rot_rad(ant_pose.dir);

            let ant_transform = context
                .transform
                .append_transform(*cam_transform)
                .append_transform(ant_model_transform);

            for vertex in ANT_TRIS {
                ant_tris.push([
                    tx(ant_transform, vertex[0], vertex[1]),
                    ty(ant_transform, vertex[0], vertex[1]),
                ]);
            }
        }

        let ant_color: [f32; 4] = to_f32_color(86, 101, 115);
        graphics.tri_list(
            &context.draw_state,
            &ant_color,
            |f| f(&ant_tris[..]),
        );


        // gl::ShaderSource();
        // context.

        // let elapsed = now.elapsed();
        // unsafe {
        //     avg_duration = avg_duration.mul_f64(0.99) + elapsed.mul_f64(0.01);
        //     println!("Elapsed: {:.2?}", avg_duration);
        // }
    }

    pub fn draw_env(
        &self,
        ws: &world_state::WorldState,
        context: pw::Context,
        graphics: &mut pw::G2d<'_>,
        device: &mut gfx_device_gl::Device,
    ) {
        unsafe {
            device.with_gl(|glapi| {
                glapi.ClearColor(0., 1., 0., 0.);
                glapi.Clear(gl::COLOR_BUFFER_BIT);
            });
        }

        // let ground_bg_color: [f32; 4] = to_f32_color(218, 165, 32);
        // pw::clear(ground_bg_color, graphics);

        let center = self.win_size * 0.5;

        let scale = 1. / self.cam_pos.z;

        let cam_transform = IDENT_TRANSFORM
            .trans(center.x, center.y)
            .scale(scale, scale)
            .trans(-self.cam_pos.x, self.cam_pos.y)
            .flip_v();

        let home_transform = context.transform.append_transform(cam_transform);
        for home_loc in &ws.home_locs {
            const RAD: f64 = world_state::WorldState::HOME_RADIUS;
            const HOME_COLOR: [f32; 4] = [0., 0., 0., 1.];
            let rect = pw::ellipse::centered([home_loc.x, home_loc.y, RAD, RAD]);
            pw::ellipse(HOME_COLOR, rect, home_transform, graphics);
        }

        // self.draw_ants(ws, &cam_transform, &context, graphics);

        // Draw the ants. They are concave polys, so we have to triangulate
        // them ourselves.
        // let mut ant_tris: Vec<[f32; 2]> = vec![];
        // ant_tris.reserve(ANT_TRIS.len() * ws.ant_poses.len());
        // for ant_pose in ws.ant_poses.values() {
        //     let ant_model_transform = IDENT_TRANSFORM
        //         .scale(2., 2.)
        //         .trans(ant_pose.pos.x, ant_pose.pos.y)
        //         .rot_rad(ant_pose.dir);

        //     let ant_transform = context
        //         .transform
        //         .append_transform(cam_transform)
        //         .append_transform(ant_model_transform);

        //     for vertex in ANT_TRIS {
        //         ant_tris.push([
        //             tx(ant_transform, vertex[0], vertex[1]),
        //             ty(ant_transform, vertex[0], vertex[1]),
        //         ]);
        //     }
        // }

        // let ant_color: [f32; 4] = to_f32_color(86, 101, 115);
        // graphics.tri_list(
        //     &context.draw_state,
        //     &ant_color,
        //     |f| f(&ant_tris[..]),
        // );

        // Center dot for debug if needed.
        // pw::ellipse(
        //     [1.0, 0.0, 0.0, 1.0],
        //     pw::ellipse::centered(
        //         [center.x, center.y, 5., 5.]),
        //         context.transform,
        //         graphics);
    }
}
