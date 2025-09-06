use console_game_engine::*;
use rand::random_range;

struct GameOfLife {
    output: Vec<i32>,
    state: Vec<i32>,

    tick_timer: f32,
    tick_rate: f32,
    paused: bool,
}

impl GameOfLife {
    fn new() -> Self {
        Self {
            output: Vec::new(),
            state: Vec::new(),
            tick_timer: 0.0,
            tick_rate: 0.05,
            paused: false,
        }
    }
}

impl ConsoleGame for GameOfLife {
    fn create(&mut self, engine: &mut ConsoleGameEngine<Self>) -> bool {
        let size = (engine.screen_width() * engine.screen_height()) as usize;
        self.output = vec![0; size];
        self.state = vec![0; size];

        for i in 0..size {
            self.state[i] = random_range(0..2);
        }

        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, elapsed_time: f32) -> bool {
        if engine.key_pressed(K_UP) {
            self.tick_rate = (self.tick_rate * 0.8).max(0.01);
        }
        if engine.key_pressed(K_DOWN) {
            self.tick_rate = (self.tick_rate * 1.2).min(1.0);
        }
        if engine.key_pressed(K_SPACE) {
            self.paused = !self.paused;
        }

        if !self.paused {
            self.tick_timer += elapsed_time;
            if self.tick_timer < self.tick_rate {
                return true;
            }
            self.tick_timer = 0.0;
        } else {
            return true;
        }

        let sw = engine.screen_width() as usize;
        let sh = engine.screen_height() as usize;

        let cell = |x: usize, y: usize, out: &Vec<i32>| -> i32 { out[(y % sh) * sw + (x % sw)] };

        self.output.clone_from(&self.state);

        for x in 0..sw {
            for y in 0..sh {
                let n_neighbours = cell(x.wrapping_sub(1), y.wrapping_sub(1), &self.output)
                    + cell(x, y.wrapping_sub(1), &self.output)
                    + cell(x + 1, y.wrapping_sub(1), &self.output)
                    + cell(x.wrapping_sub(1), y, &self.output)
                    + cell(x + 1, y, &self.output)
                    + cell(x.wrapping_sub(1), y + 1, &self.output)
                    + cell(x, y + 1, &self.output)
                    + cell(x + 1, y + 1, &self.output);

                if cell(x, y, &self.output) == 1 {
                    self.state[y * sw + x] = if n_neighbours == 2 || n_neighbours == 3 {
                        1
                    } else {
                        0
                    };
                } else {
                    self.state[y * sw + x] = if n_neighbours == 3 { 1 } else { 0 };
                }

                if cell(x, y, &self.output) == 1 {
                    engine.draw(x as i32, y as i32);
                } else {
                    engine.draw_with(x as i32, y as i32, PIXEL_SOLID, FG_BLACK);
                }
            }
        }

        true
    }
}

fn main() {
    let mut game = ConsoleGameEngine::new(GameOfLife::new());
    game.set_app_name("Game Of Life");
    game.construct_console(160, 100, 8, 8);
    game.start();
}
