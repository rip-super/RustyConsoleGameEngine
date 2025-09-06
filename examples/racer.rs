use console_game_engine::*;
use std::collections::VecDeque;

#[derive(Default)]
struct Racer {
    car_pos: f32,
    distance: f32,
    speed: f32,

    curvature: f32,
    track_curvature: f32,
    player_curvature: f32,
    track_distance: f32,

    current_lap_time: f32,

    track: Vec<(f32, f32)>,
    lap_times: VecDeque<f32>,
}

impl Racer {
    fn display_time(t: f32) -> String {
        let total_ms = (t * 1000.0).round() as i32;
        let minutes = total_ms / 1000 / 60;
        let seconds = (total_ms / 1000) % 60;
        let milliseconds = total_ms % 1000;
        format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
    }
}

impl ConsoleGame for Racer {
    fn create(&mut self, _engine: &mut ConsoleGameEngine<Self>) -> bool {
        self.track.push((0.0, 10.0));
        self.track.push((0.0, 200.0));
        self.track.push((1.0, 200.0));
        self.track.push((0.0, 400.0));
        self.track.push((-1.0, 100.0));
        self.track.push((0.0, 200.0));
        self.track.push((-1.0, 200.0));
        self.track.push((1.0, 200.0));
        self.track.push((0.0, 200.0));
        self.track.push((0.2, 500.0));
        self.track.push((0.0, 200.0));

        self.track_distance = 0.0;
        for &(_, dist) in &self.track {
            self.track_distance += dist;
        }

        self.lap_times = VecDeque::from(vec![0.0_f32; 5]);

        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, elapsed_time: f32) -> bool {
        if engine.key_held(K_UP) {
            self.speed += 2.0 * elapsed_time;
        } else {
            self.speed -= 1.0 * elapsed_time;
        }

        let mut car_direction = 0_i32;
        if engine.key_held(K_LEFT) {
            car_direction = -1;
            self.player_curvature -= 0.7 * elapsed_time;
        }
        if engine.key_held(K_RIGHT) {
            car_direction = 1;
            self.player_curvature += 0.7 * elapsed_time;
        }

        if (self.player_curvature - self.track_curvature).abs() >= 0.8 {
            self.speed -= 4.0 * elapsed_time;
        }

        self.speed = self.speed.clamp(0.0, 1.0);

        self.distance += (70.0 * self.speed) * elapsed_time;

        self.current_lap_time += elapsed_time;
        if self.distance >= self.track_distance {
            self.distance -= self.track_distance;
            self.lap_times.push_front(self.current_lap_time);
            if self.lap_times.len() > 5 {
                self.lap_times.pop_back();
            }
            self.current_lap_time = 0.0;
        }

        let mut offset = 0.0_f32;
        let mut track_section = 0_usize;
        while track_section < self.track.len() && offset <= self.distance {
            offset += self.track[track_section].1;
            track_section += 1;
        }

        let target_curvature = if track_section == 0 {
            self.track[0].0
        } else {
            self.track[track_section - 1].0
        };

        let track_curve_diff = (target_curvature - self.curvature) * elapsed_time * self.speed;
        self.curvature += track_curve_diff;
        self.track_curvature += self.curvature * elapsed_time * self.speed;

        let sw = engine.screen_width() as usize;
        let sh = engine.screen_height() as usize;

        for y in 0..(sh / 2) {
            for x in 0..sw {
                let pix = if y < (sh / 4) {
                    PIXEL_HALF
                } else {
                    PIXEL_SOLID
                };
                engine.draw_with(x as i32, y as i32, pix, FG_DARK_BLUE);
            }
        }

        for x in 0..sw {
            let hill = (((x as f32) * 0.01 + self.track_curvature).sin() * 16.0).abs() as i32;
            let hill_top = (sh as i32 / 2) - hill;
            for y in hill_top..(sh as i32 / 2) {
                if y >= 0 && (y as usize) < sh {
                    engine.draw_with(x as i32, y, PIXEL_SOLID, FG_DARK_YELLOW);
                }
            }
        }

        for y in 0..(sh / 2) {
            let perspective = (y as f32) / (sh as f32 / 2.0);
            let middle = 0.5 + self.curvature * (1.0 - perspective).powf(3.0);

            let mut road_width = 0.1 + perspective * 0.8;
            let clip_width = road_width * 0.15;
            road_width *= 0.5;

            let left_grass = ((middle - road_width - clip_width) * sw as f32) as i32;
            let left_clip = ((middle - road_width) * sw as f32) as i32;
            let right_grass = ((middle + road_width + clip_width) * sw as f32) as i32;
            let right_clip = ((middle + road_width) * sw as f32) as i32;

            let row = (sh / 2) as i32 + y as i32;

            let grass_color =
                if (20.0 * (1.0 - perspective).powf(3.0) + self.distance * 0.1).sin() > 0.0 {
                    FG_GREEN
                } else {
                    FG_DARK_GREEN
                };
            let clip_color = if (80.0 * (1.0 - perspective).powf(2.0) + self.distance).sin() > 0.0 {
                FG_RED
            } else {
                FG_WHITE
            };

            let road_color = if track_section == 1 {
                FG_WHITE
            } else {
                FG_GREY
            };

            for x in 0..sw {
                let xi = x as i32;
                if xi >= 0 && xi < left_grass {
                    engine.draw_with(xi, row, PIXEL_SOLID, grass_color);
                } else if xi >= left_grass && xi < left_clip {
                    engine.draw_with(xi, row, PIXEL_SOLID, clip_color);
                } else if xi >= left_clip && xi < right_clip {
                    engine.draw_with(xi, row, PIXEL_SOLID, road_color);
                } else if xi >= right_clip && xi < right_grass {
                    engine.draw_with(xi, row, PIXEL_SOLID, clip_color);
                } else if xi >= right_grass && xi < sw as i32 {
                    engine.draw_with(xi, row, PIXEL_SOLID, grass_color);
                }
            }
        }

        self.car_pos = self.player_curvature - self.track_curvature;
        let sw = engine.screen_width();
        let car_pos_f = (sw as f32 / 2.0) + (sw as f32 * self.car_pos) / 2.0 - 7.0;
        let mut car_draw_x = car_pos_f.round() as i32;

        if car_draw_x < 0 {
            car_draw_x = (sw + (car_draw_x % sw)) % sw;
        } else if car_draw_x >= sw {
            car_draw_x %= sw;
        }

        let car_lines = match car_direction {
            0 => vec![
                "   ||####||   ",
                "      ##      ",
                "     ####     ",
                "     ####     ",
                "|||  ####  |||",
                "|||########|||",
                "|||  ####  |||",
            ],
            1 => vec![
                "      //####//",
                "         ##   ",
                "       ####   ",
                "      ####    ",
                "///  ####//// ",
                "//#######///O ",
                "/// #### //// ",
            ],
            -1 => vec![
                r#"\\####\\      "#,
                "   ##         ",
                "   ####       ",
                "    ####      ",
                r#" \\\\####  \\\"#,
                r#" O\\\#######\\"#,
                r#" \\\\ #### \\\"#,
            ],
            _ => vec![],
        };

        for (i, line) in car_lines.iter().enumerate() {
            let y = sh as i32 - 20 + i as i32;
            for (dx, ch) in line.chars().enumerate() {
                if ch == ' ' {
                    continue;
                }
                let mut x = car_draw_x + dx as i32;
                x = ((x % sw) + sw) % sw;
                engine.draw_string_alpha(x, y, &ch.to_string());
            }
        }

        engine.draw_string(0, 0, &format!("Distance: {:.2}", self.distance));
        engine.draw_string(0, 1, &format!("Target Curvature: {:.3}", self.curvature));
        engine.draw_string(
            0,
            2,
            &format!("Player Curvature: {:.3}", self.player_curvature),
        );
        engine.draw_string(0, 3, &format!("Player Speed    : {:.3}", self.speed));
        engine.draw_string(
            0,
            4,
            &format!("Track Curvature : {:.3}", self.track_curvature),
        );

        engine.draw_string(10, 8, &Racer::display_time(self.current_lap_time));

        let mut j = 10;
        for &l in &self.lap_times {
            engine.draw_string(10, j, &Racer::display_time(l));
            j += 1;
        }

        true
    }
}

fn main() {
    let mut engine = ConsoleGameEngine::new(Racer::default());
    engine.set_app_name("Racer");
    engine.construct_console(160, 100, 8, 8);
    engine.start();
}
