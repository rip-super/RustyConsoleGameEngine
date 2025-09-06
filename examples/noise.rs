use console_game_engine::*;
use rand::random;

struct PerlinNoise {
    output_width: usize,
    output_height: usize,
    noise_seed_2d: Vec<f32>,
    noise_2d: Vec<f32>,

    output_size: usize,
    noise_seed_1d: Vec<f32>,
    noise_1d: Vec<f32>,

    octave_count: usize,
    scaling_bias: f32,
    mode: usize,
}

impl PerlinNoise {
    fn new() -> Self {
        Self {
            output_width: 256,
            output_height: 256,
            noise_seed_2d: Vec::new(),
            noise_2d: Vec::new(),
            output_size: 256,
            noise_seed_1d: Vec::new(),
            noise_1d: Vec::new(),
            octave_count: 1,
            scaling_bias: 2.0,
            mode: 1,
        }
    }

    fn perlin_noise_1d(&mut self) {
        for x in 0..self.output_size {
            let mut noise = 0.0;
            let mut scale_acc = 0.0;
            let mut scale = 1.0;

            for o in 0..self.octave_count {
                let pitch = self.output_size >> o;
                let sample1 = (x / pitch) * pitch;
                let sample2 = (sample1 + pitch) % self.output_size;
                let blend = (x - sample1) as f32 / pitch as f32;
                let sample = (1.0 - blend) * self.noise_seed_1d[sample1]
                    + blend * self.noise_seed_1d[sample2];

                scale_acc += scale;
                noise += sample * scale;
                scale /= self.scaling_bias;
            }

            self.noise_1d[x] = noise / scale_acc;
        }
    }

    fn perlin_noise_2d(&mut self) {
        for x in 0..self.output_width {
            for y in 0..self.output_height {
                let mut noise = 0.0;
                let mut scale_acc = 0.0;
                let mut scale = 1.0;

                for o in 0..self.octave_count {
                    let pitch_x = self.output_width >> o;
                    let pitch_y = self.output_height >> o;

                    let sample_x1 = (x / pitch_x) * pitch_x;
                    let sample_y1 = (y / pitch_y) * pitch_y;

                    let sample_x2 = (sample_x1 + pitch_x) % self.output_width;
                    let sample_y2 = (sample_y1 + pitch_y) % self.output_height;

                    let blend_x = (x - sample_x1) as f32 / pitch_x as f32;
                    let blend_y = (y - sample_y1) as f32 / pitch_y as f32;

                    let sample_t = (1.0 - blend_x)
                        * self.noise_seed_2d[sample_y1 * self.output_width + sample_x1]
                        + blend_x * self.noise_seed_2d[sample_y1 * self.output_width + sample_x2];
                    let sample_b = (1.0 - blend_x)
                        * self.noise_seed_2d[sample_y2 * self.output_width + sample_x1]
                        + blend_x * self.noise_seed_2d[sample_y2 * self.output_width + sample_x2];

                    scale_acc += scale;
                    noise += (blend_y * (sample_b - sample_t) + sample_t) * scale;
                    scale /= self.scaling_bias;
                }

                self.noise_2d[y * self.output_width + x] = noise / scale_acc;
            }
        }
    }
}

impl ConsoleGame for PerlinNoise {
    fn create(&mut self, engine: &mut ConsoleGameEngine<Self>) -> bool {
        self.output_width = engine.screen_width() as usize;
        self.output_height = engine.screen_height() as usize;

        self.noise_seed_2d = (0..self.output_width * self.output_height)
            .map(|_| random::<f32>())
            .collect::<Vec<f32>>();

        self.noise_2d = vec![0.0; self.output_width * self.output_height];

        self.perlin_noise_2d();

        self.output_size = self.output_width;
        self.noise_seed_1d = (0..self.output_size)
            .map(|_| random::<f32>())
            .collect::<Vec<f32>>();

        self.noise_1d = vec![0.0; self.output_size];

        self.perlin_noise_1d();

        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, _elapsed_time: f32) -> bool {
        engine.clear(FG_BLACK);

        if engine.key_released(K_SPACE) {
            self.octave_count += 1;
        }
        if engine.key_released(K_1) {
            self.mode = 1;
        }
        if engine.key_released(K_2) {
            self.mode = 2;
        }
        if engine.key_released(K_3) {
            self.mode = 3;
        }
        if engine.key_released(K_Q) {
            self.scaling_bias += 0.2;
        }
        if engine.key_released(K_A) {
            self.scaling_bias -= 0.2;
        }

        if self.scaling_bias < 0.2 {
            self.scaling_bias = 0.2;
        }

        if self.octave_count == 9 {
            self.octave_count = 1;
        }

        if self.mode == 1 {
            if engine.key_released(K_Z) {
                for i in 0..self.output_size {
                    self.noise_seed_1d[i] = random::<f32>();
                }
            }

            if engine.key_released(K_X) {
                for i in 0..self.output_size {
                    self.noise_seed_1d[i] = 2.0 * random::<f32>() - 1.0;
                }
            }

            self.perlin_noise_1d();

            for x in 0..self.output_size {
                let y = -(self.noise_1d[x] * engine.screen_height() as f32 / 2.0)
                    + engine.screen_height() as f32 / 2.0;

                let mid = engine.screen_height() as f32 / 2.0;

                if y < mid {
                    for f in y as i32..mid as i32 {
                        engine.draw_with(x as i32, f, PIXEL_SOLID, FG_GREEN);
                    }
                } else {
                    let flipped_y = mid - (y - mid);
                    for f in flipped_y as i32..mid as i32 {
                        engine.draw_with(x as i32, f, PIXEL_SOLID, FG_GREEN);
                    }
                }
            }
        } else if self.mode == 2 {
            if engine.key_released(K_Z) {
                for i in 0..self.output_width * self.output_height {
                    self.noise_seed_2d[i] = random::<f32>();
                }
            }

            self.perlin_noise_2d();

            for x in 0..self.output_width {
                for y in 0..self.output_height {
                    let pixel_bw = (self.noise_2d[y * self.output_width + x] * 12.0) as usize;
                    let (bg_col, fg_col, sym) = match pixel_bw {
                        0 => (BG_BLACK, FG_BLACK, PIXEL_SOLID),
                        1 => (BG_BLACK, FG_DARK_GREY, PIXEL_QUARTER),
                        2 => (BG_BLACK, FG_DARK_GREY, PIXEL_HALF),
                        3 => (BG_BLACK, FG_DARK_GREY, PIXEL_THREEQUARTERS),
                        4 => (BG_BLACK, FG_DARK_GREY, PIXEL_SOLID),
                        5 => (BG_DARK_GREY, FG_GREY, PIXEL_QUARTER),
                        6 => (BG_DARK_GREY, FG_GREY, PIXEL_HALF),
                        7 => (BG_DARK_GREY, FG_GREY, PIXEL_THREEQUARTERS),
                        8 => (BG_DARK_GREY, FG_GREY, PIXEL_SOLID),
                        9 => (BG_GREY, FG_WHITE, PIXEL_QUARTER),
                        10 => (BG_GREY, FG_WHITE, PIXEL_HALF),
                        11 => (BG_GREY, FG_WHITE, PIXEL_THREEQUARTERS),
                        _ => (BG_GREY, FG_WHITE, PIXEL_SOLID),
                    };
                    engine.draw_with(x as i32, y as i32, sym, fg_col | bg_col);
                }
            }
        } else if self.mode == 3 {
            if engine.key_released(K_Z) {
                for i in 0..self.output_width * self.output_height {
                    self.noise_seed_2d[i] = random::<f32>();
                }
            }

            self.perlin_noise_2d();

            for x in 0..self.output_width {
                for y in 0..self.output_height {
                    let pixel_bw = (self.noise_2d[y * self.output_width + x] * 16.0) as usize;
                    let (bg_col, fg_col, sym) = match pixel_bw {
                        0 => (BG_DARK_BLUE, FG_DARK_BLUE, PIXEL_SOLID),
                        1 => (BG_DARK_BLUE, FG_BLUE, PIXEL_QUARTER),
                        2 => (BG_DARK_BLUE, FG_BLUE, PIXEL_HALF),
                        3 => (BG_DARK_BLUE, FG_BLUE, PIXEL_THREEQUARTERS),
                        4 => (BG_DARK_BLUE, FG_BLUE, PIXEL_SOLID),
                        5 => (BG_BLUE, FG_GREEN, PIXEL_QUARTER),
                        6 => (BG_BLUE, FG_GREEN, PIXEL_HALF),
                        7 => (BG_BLUE, FG_GREEN, PIXEL_THREEQUARTERS),
                        8 => (BG_BLUE, FG_GREEN, PIXEL_SOLID),
                        9 => (BG_GREEN, FG_DARK_GREY, PIXEL_QUARTER),
                        10 => (BG_GREEN, FG_DARK_GREY, PIXEL_HALF),
                        11 => (BG_GREEN, FG_DARK_GREY, PIXEL_THREEQUARTERS),
                        12 => (BG_GREEN, FG_DARK_GREY, PIXEL_SOLID),
                        13 => (BG_DARK_GREY, FG_WHITE, PIXEL_QUARTER),
                        14 => (BG_DARK_GREY, FG_WHITE, PIXEL_HALF),
                        15 => (BG_DARK_GREY, FG_WHITE, PIXEL_THREEQUARTERS),
                        _ => (BG_DARK_GREY, FG_WHITE, PIXEL_SOLID),
                    };
                    engine.draw_with(x as i32, y as i32, sym, fg_col | bg_col);
                }
            }
        }

        true
    }
}

fn main() {
    let mut engine = ConsoleGameEngine::new(PerlinNoise::new());
    engine.set_app_name("Perlin Noise");
    engine.construct_console(256, 256, 3, 3);
    engine.start();
}
