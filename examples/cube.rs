use rusty_console_game_engine::*;
use std::f32::consts::PI;

macro_rules! nofmt {
    ($($code:tt)*) => { $($code)* }
}

#[derive(Clone, Default)]
struct Vec3d {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Default)]
struct Triangle {
    p: [Vec3d; 3],
}

struct Mesh {
    tris: Vec<Triangle>,
}

#[derive(Default)]
struct Mat4x4 {
    m: [[f32; 4]; 4],
}

struct Engine3D {
    mesh_cube: Mesh,
    mat_proj: Mat4x4,
    theta: f32,
}

impl Engine3D {
    fn new() -> Self {
        Self {
            mesh_cube: Mesh { tris: Vec::new() },
            mat_proj: Mat4x4::default(),
            theta: 0.0,
        }
    }

    fn mul_mat_vec(&self, v1: &Vec3d, m: &Mat4x4) -> Vec3d {
        let mut v2 = Vec3d::default();
        let w = v1.x * m.m[0][3] + v1.y * m.m[1][3] + v1.z * m.m[2][3] + m.m[3][3];

        v2.x = v1.x * m.m[0][0] + v1.y * m.m[1][0] + v1.z * m.m[2][0] + m.m[3][0];
        v2.y = v1.x * m.m[0][1] + v1.y * m.m[1][1] + v1.z * m.m[2][1] + m.m[3][1];
        v2.z = v1.x * m.m[0][2] + v1.y * m.m[1][2] + v1.z * m.m[2][2] + m.m[3][2];

        if w != 0.0 {
            v2.x /= w;
            v2.y /= w;
            v2.z /= w;
        }
        v2
    }
}

impl ConsoleGame for Engine3D {
    fn app_name(&self) -> &str {
        "Cube"
    }

    fn create(&mut self, engine: &mut ConsoleGameEngine<Self>) -> bool {
        nofmt!(
            self.mesh_cube.tris = vec![
                Triangle { p: [Vec3d { x: 0.0, y: 0.0, z: 0.0 }, Vec3d { x: 0.0, y: 1.0, z: 0.0 }, Vec3d { x: 1.0, y: 1.0, z: 0.0 }] },
                Triangle { p: [Vec3d { x: 0.0, y: 0.0, z: 0.0 }, Vec3d { x: 1.0, y: 1.0, z: 0.0 }, Vec3d { x: 1.0, y: 0.0, z: 0.0 }] },

                Triangle { p: [Vec3d { x: 1.0, y: 0.0, z: 0.0 }, Vec3d { x: 1.0, y: 1.0, z: 0.0 }, Vec3d { x: 1.0, y: 1.0, z: 1.0 }] },
                Triangle { p: [Vec3d { x: 1.0, y: 0.0, z: 0.0 }, Vec3d { x: 1.0, y: 1.0, z: 1.0 }, Vec3d { x: 1.0, y: 0.0, z: 1.0 }] },

                Triangle { p: [Vec3d { x: 1.0, y: 0.0, z: 1.0 }, Vec3d { x: 1.0, y: 1.0, z: 1.0 }, Vec3d { x: 0.0, y: 1.0, z: 1.0 }] },
                Triangle { p: [Vec3d { x: 1.0, y: 0.0, z: 1.0 }, Vec3d { x: 0.0, y: 1.0, z: 1.0 }, Vec3d { x: 0.0, y: 0.0, z: 1.0 }] },

                Triangle { p: [Vec3d { x: 0.0, y: 0.0, z: 1.0 }, Vec3d { x: 0.0, y: 1.0, z: 1.0 }, Vec3d { x: 0.0, y: 1.0, z: 0.0 }] },
                Triangle { p: [Vec3d { x: 0.0, y: 0.0, z: 1.0 }, Vec3d { x: 0.0, y: 1.0, z: 0.0 }, Vec3d { x: 0.0, y: 0.0, z: 0.0 }] },

                Triangle { p: [Vec3d { x: 0.0, y: 1.0, z: 0.0 }, Vec3d { x: 0.0, y: 1.0, z: 1.0 }, Vec3d { x: 1.0, y: 1.0, z: 1.0 }] },
                Triangle { p: [Vec3d { x: 0.0, y: 1.0, z: 0.0 }, Vec3d { x: 1.0, y: 1.0, z: 1.0 }, Vec3d { x: 1.0, y: 1.0, z: 0.0 }] },

                Triangle { p: [Vec3d { x: 1.0, y: 0.0, z: 1.0 }, Vec3d { x: 0.0, y: 0.0, z: 1.0 }, Vec3d { x: 0.0, y: 0.0, z: 0.0 }] },
                Triangle { p: [Vec3d { x: 1.0, y: 0.0, z: 1.0 }, Vec3d { x: 0.0, y: 0.0, z: 0.0 }, Vec3d { x: 1.0, y: 0.0, z: 0.0 }] },
            ];
        );

        let near = 0.1f32;
        let far = 1000.0f32;
        let fov = 90.0f32;
        let aspect_ratio = engine.screen_height() as f32 / engine.screen_width() as f32;
        let fov_rad = 1.0 / (fov * 0.5 / 180.0 * PI).tan();

        self.mat_proj.m[0][0] = aspect_ratio * fov_rad;
        self.mat_proj.m[1][1] = fov_rad;
        self.mat_proj.m[2][2] = far / (far - near);
        self.mat_proj.m[3][2] = (-far * near) / (far - near);
        self.mat_proj.m[2][3] = 1.0;
        self.mat_proj.m[3][3] = 0.0;

        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, elapsed_time: f32) -> bool {
        engine.clear(FG_BLACK);

        let mut mat_rot_z = Mat4x4::default();
        let mut mat_rot_x = Mat4x4::default();

        self.theta += 1.0 * elapsed_time;

        mat_rot_z.m[0][0] = self.theta.cos();
        mat_rot_z.m[0][1] = self.theta.sin();
        mat_rot_z.m[1][0] = -self.theta.sin();
        mat_rot_z.m[1][1] = self.theta.cos();
        mat_rot_z.m[2][2] = 1.0;
        mat_rot_z.m[3][3] = 1.0;

        mat_rot_x.m[0][0] = 1.0;
        mat_rot_x.m[1][1] = (self.theta * 0.5).cos();
        mat_rot_x.m[1][2] = (self.theta * 0.5).sin();
        mat_rot_x.m[2][1] = -(self.theta * 0.5).sin();
        mat_rot_x.m[2][2] = (self.theta * 0.5).cos();
        mat_rot_x.m[3][3] = 1.0;

        for tri in &self.mesh_cube.tris {
            let mut tri_rot_z = Triangle::default();
            let mut tri_rot_zx = Triangle::default();
            let mut tri_projected = Triangle::default();

            for i in 0..3 {
                tri_rot_z.p[i] = self.mul_mat_vec(&tri.p[i], &mat_rot_z);
            }

            for i in 0..3 {
                tri_rot_zx.p[i] = self.mul_mat_vec(&tri_rot_z.p[i], &mat_rot_x);
            }

            let mut tri_translated = tri_rot_zx;
            for i in 0..3 {
                tri_translated.p[i].z += 3.0;
            }

            for i in 0..3 {
                tri_projected.p[i] = self.mul_mat_vec(&tri_translated.p[i], &self.mat_proj);

                tri_projected.p[i].x += 1.0;
                tri_projected.p[i].y += 1.0;
                tri_projected.p[i].x *= 0.5 * engine.screen_width() as f32;
                tri_projected.p[i].y *= 0.5 * engine.screen_height() as f32;
            }

            engine.draw_triangle(
                tri_projected.p[0].x as i32,
                tri_projected.p[0].y as i32,
                tri_projected.p[1].x as i32,
                tri_projected.p[1].y as i32,
                tri_projected.p[2].x as i32,
                tri_projected.p[2].y as i32,
            );
        }

        true
    }
}

fn main() {
    let mut engine = ConsoleGameEngine::new(Engine3D::new());
    engine
        .construct_console(256, 240, 4, 4)
        .expect("Console Construction Failed");
    engine.start();
}
