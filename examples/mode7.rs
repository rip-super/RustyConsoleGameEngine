use rusty_console_game_engine::prelude::*;
use rusty_console_game_engine::{
    color::FG_CYAN,
    key::{Q, X, Z},
};
use std::f32::consts::PI;

pub struct Mode7 {
    world_x: f32,
    world_y: f32,
    world_a: f32,
    near: f32,
    far: f32,
    fov_half: f32,

    ground_sprite: Sprite,
    sky_sprite: Sprite,
}

impl Mode7 {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            world_x: 1000.0,
            world_y: 1000.0,
            world_a: 0.1,
            near: 0.005,
            far: 0.03,
            fov_half: PI / 4.0,

            ground_sprite: Sprite::from_file("examples/sprites/world.spr").unwrap(),
            sky_sprite: Sprite::from_file("examples/sprites/sky.spr").unwrap(),
        }
    }
}

impl ConsoleGame for Mode7 {
    fn app_name(&self) -> &str {
        "Mode7"
    }

    fn create(&mut self, _engine: &mut ConsoleGameEngine<Self>) -> bool {
        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, elapsed_time: f32) -> bool {
        if engine.key_held(Q) {
            self.near += 0.1 * elapsed_time;
        }
        if engine.key_held(A) {
            self.near -= 0.1 * elapsed_time;
        }

        if engine.key_held(W) {
            self.far += 0.1 * elapsed_time;
        }
        if engine.key_held(S) {
            self.far -= 0.1 * elapsed_time;
        }

        if engine.key_held(Z) {
            self.fov_half += 0.1 * elapsed_time;
        }
        if engine.key_held(X) {
            self.fov_half -= 0.1 * elapsed_time;
        }

        let far_x1 = self.world_x + (self.world_a - self.fov_half).cos() * self.far;
        let far_y1 = self.world_y + (self.world_a - self.fov_half).sin() * self.far;

        let near_x1 = self.world_x + (self.world_a - self.fov_half).cos() * self.near;
        let near_y1 = self.world_y + (self.world_a - self.fov_half).sin() * self.near;

        let far_x2 = self.world_x + (self.world_a + self.fov_half).cos() * self.far;
        let far_y2 = self.world_y + (self.world_a + self.fov_half).sin() * self.far;

        let near_x2 = self.world_x + (self.world_a + self.fov_half).cos() * self.near;
        let near_y2 = self.world_y + (self.world_a + self.fov_half).sin() * self.near;

        for y in 0..(engine.screen_height() / 2) {
            let sample_depth = y as f32 / (engine.screen_height() as f32 / 2.0);

            let start_x = (far_x1 - near_x1) / sample_depth + near_x1;
            let start_y = (far_y1 - near_y1) / sample_depth + near_y1;
            let end_x = (far_x2 - near_x2) / sample_depth + near_x2;
            let end_y = (far_y2 - near_y2) / sample_depth + near_y2;

            for x in 0..engine.screen_width() {
                let sample_width = x as f32 / engine.screen_width() as f32;
                let sample_x = (end_x - start_x) * sample_width + start_x;
                let sample_y = (end_y - start_y) * sample_width + start_y;

                let sample_x = sample_x % 1.0;
                let sample_y = sample_y % 1.0;

                let glyph = self.ground_sprite.sample_glyph(sample_x, sample_y);
                let color = self.ground_sprite.sample_color(sample_x, sample_y);
                engine.draw_with(x, y + (engine.screen_height() / 2), glyph, color);

                let glyph = self.sky_sprite.sample_glyph(sample_x, sample_y);
                let color = self.sky_sprite.sample_color(sample_x, sample_y);
                engine.draw_with(x, (engine.screen_height() / 2) - y, glyph, color);
            }
        }

        engine.draw_line_with(
            0,
            engine.screen_height() / 2,
            engine.screen_width(),
            engine.screen_height() / 2,
            SOLID,
            FG_CYAN,
        );

        if engine.key_held(LEFT) {
            self.world_a -= 1.0 * elapsed_time;
        }
        if engine.key_held(RIGHT) {
            self.world_a += 1.0 * elapsed_time;
        }

        if engine.key_held(ARROW_UP) {
            self.world_x += self.world_a.cos() * 0.2 * elapsed_time;
            self.world_y += self.world_a.sin() * 0.2 * elapsed_time;
        }

        if engine.key_held(ARROW_DOWN) {
            self.world_x -= self.world_a.cos() * 0.2 * elapsed_time;
            self.world_y -= self.world_a.sin() * 0.2 * elapsed_time;
        }

        true
    }
}

fn main() {
    let mut engine = ConsoleGameEngine::new(Mode7::new());
    engine
        .construct_console(320, 240, 4, 4)
        .expect("Console Construction Failed");
    engine.start();
}
