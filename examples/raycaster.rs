use rusty_console_game_engine::*;
use std::f32::consts::PI;

struct Raycaster {
    map: String,
    map_width: i32,
    map_height: i32,

    player_x: f32,
    player_y: f32,
    player_a: f32,
    fov: f32,
    depth: f32,
    speed: f32,
}

impl Raycaster {
    fn new() -> Self {
        let mut map = String::new();
        map += "#########.......";
        map += "#...............";
        map += "#.......########";
        map += "#..............#";
        map += "#......##......#";
        map += "#......##......#";
        map += "#..............#";
        map += "###............#";
        map += "##.............#";
        map += "#......####..###";
        map += "#......#.......#";
        map += "#......#.......#";
        map += "#..............#";
        map += "#......#########";
        map += "#..............#";
        map += "################";

        Self {
            map,
            map_width: 16,
            map_height: 16,
            player_x: 14.7,
            player_y: 5.09,
            player_a: 0.0,
            fov: PI / 4.0,
            depth: 16.0,
            speed: 5.0,
        }
    }
}

impl ConsoleGame for Raycaster {
    fn create(&mut self, _engine: &mut ConsoleGameEngine<Self>) -> bool {
        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, elapsed_time: f32) -> bool {
        if engine.key_held(K_A) {
            self.player_a -= (self.speed * 0.75) * elapsed_time;
        }
        if engine.key_held(K_D) {
            self.player_a += (self.speed * 0.75) * elapsed_time;
        }

        let move_step = self.speed * elapsed_time;
        if engine.key_held(K_W) {
            let next_x = self.player_x + self.player_a.sin() * move_step;
            let next_y = self.player_y + self.player_a.cos() * move_step;
            if next_x >= 0.0
                && next_x < self.map_width as f32
                && next_y >= 0.0
                && next_y < self.map_height as f32
                && self
                    .map
                    .chars()
                    .nth((next_x as i32 * self.map_width + next_y as i32) as usize)
                    != Some('#')
            {
                self.player_x = next_x;
                self.player_y = next_y;
            }
        }
        if engine.key_held(K_S) {
            let next_x = self.player_x - self.player_a.sin() * move_step;
            let next_y = self.player_y - self.player_a.cos() * move_step;
            if next_x >= 0.0
                && next_x < self.map_width as f32
                && next_y >= 0.0
                && next_y < self.map_height as f32
                && self
                    .map
                    .chars()
                    .nth((next_x as i32 * self.map_width + next_y as i32) as usize)
                    != Some('#')
            {
                self.player_x = next_x;
                self.player_y = next_y;
            }
        }

        let sw = engine.screen_width();
        let sh = engine.screen_height();

        for x in 0..sw {
            let ray_angle = (self.player_a - self.fov / 2.0) + (x as f32 / sw as f32) * self.fov;

            let eye_x = ray_angle.sin();
            let eye_y = ray_angle.cos();

            let mut distance_to_wall = 0.0;
            let mut hit_wall = false;
            let mut boundary = false;

            while !hit_wall && distance_to_wall < self.depth {
                distance_to_wall += 0.1;

                let test_x = (self.player_x + eye_x * distance_to_wall) as i32;
                let test_y = (self.player_y + eye_y * distance_to_wall) as i32;

                if test_x < 0 || test_x >= self.map_width || test_y < 0 || test_y >= self.map_height
                {
                    hit_wall = true;
                    distance_to_wall = self.depth;
                } else if self
                    .map
                    .chars()
                    .nth((test_x * self.map_width + test_y) as usize)
                    == Some('#')
                {
                    hit_wall = true;

                    let mut p = Vec::new();
                    for tx in 0..2 {
                        for ty in 0..2 {
                            let vx = test_x as f32 + tx as f32 - self.player_x;
                            let vy = test_y as f32 + ty as f32 - self.player_y;
                            let d = (vx * vx + vy * vy).sqrt();
                            let dot = (eye_x * vx / d) + (eye_y * vy / d);
                            p.push((d, dot));
                        }
                    }
                    p.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                    let bound = 0.01;
                    if p.len() >= 3
                        && (p[0].1.acos() < bound || p[1].1.acos() < bound || p[2].1.acos() < bound)
                    {
                        boundary = true;
                    }
                }
            }

            let ceiling = (sh as f32 / 2.0 - sh as f32 / distance_to_wall) as i32;
            let floor = sh - ceiling;

            let mut shade = ' ' as u16;
            if distance_to_wall <= self.depth / 4.0 {
                shade = PIXEL_SOLID;
            } else if distance_to_wall < self.depth / 3.0 {
                shade = PIXEL_THREEQUARTERS;
            } else if distance_to_wall < self.depth / 2.0 {
                shade = PIXEL_HALF;
            } else if distance_to_wall < self.depth {
                shade = PIXEL_QUARTER;
            }

            if boundary {
                shade = PIXEL_EMPTY;
            }

            for y in 0..sh {
                if y <= ceiling {
                    engine.draw_with(x, y, PIXEL_SOLID, FG_BLACK);
                } else if (y) > ceiling && (y) <= floor {
                    engine.draw_with(x, y, shade, FG_WHITE);
                } else {
                    let b = 1.0 - ((y as f32 - sh as f32 / 2.0) / (sh as f32 / 2.0));
                    let floor_shade: u16 = if b < 0.25 {
                        PIXEL_SOLID
                    } else if b < 0.5 {
                        PIXEL_THREEQUARTERS
                    } else if b < 0.75 {
                        PIXEL_HALF
                    } else if b < 0.9 {
                        PIXEL_QUARTER
                    } else {
                        PIXEL_EMPTY
                    };
                    engine.draw_with(x, y, floor_shade, FG_WHITE);
                }
            }
        }

        engine.draw_string(
            0,
            0,
            &format!(
                "X={:.2}, Y={:.2}, A={:.2}, FPS={:.2}",
                self.player_x,
                self.player_y,
                self.player_a,
                1.0 / elapsed_time
            ),
        );

        for nx in 0..self.map_width {
            for ny in 0..self.map_height {
                engine.draw_string(
                    nx,
                    ny + 1,
                    &self
                        .map
                        .chars()
                        .nth((ny * self.map_width + nx) as usize)
                        .unwrap()
                        .to_string(),
                );
            }
        }
        engine.draw_string(self.player_y as i32, self.player_x as i32 + 1, "P");

        true
    }
}

fn main() {
    let mut engine = ConsoleGameEngine::new(Raycaster::new());
    engine.set_app_name("Raycaster");
    engine
        .construct_console(200, 100, 8, 8)
        .expect("Console Construction Failed");
    engine.start();
}
