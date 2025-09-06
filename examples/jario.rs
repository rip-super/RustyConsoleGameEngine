use console_game_engine::*;

struct Platformer {
    level: String,
    level_width: i32,
    level_height: i32,

    player_x: f32,
    player_y: f32,
    vel_x: f32,
    vel_y: f32,
    on_ground: bool,

    cam_x: f32,
    cam_y: f32,

    tile_sprite: Sprite,
    player_sprite: Sprite,

    dir_mod_x: i32,
    dir_mod_y: i32,
}

impl Platformer {
    fn new() -> Self {
        let mut level = String::new();

        level += "................................................................";
        level += "................................................................";
        level += ".......ooooo....................................................";
        level += "........ooo.....................................................";
        level += ".......................########.................................";
        level += ".....BB?BBBB?BB.......###..............#.#......................";
        level += "....................###................#.#......................";
        level += "...................####.........................................";
        level += "GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG.##############.....########";
        level += "...................................#.#...............###........";
        level += "........................############.#............###...........";
        level += "........................#............#.........###..............";
        level += "........................#.############......###.................";
        level += "........................#................###....................";
        level += "........................#################.......................";
        level += "................................................................";

        Self {
            level,
            level_width: 64,
            level_height: 16,

            player_x: 1.0,
            player_y: 1.0,
            vel_x: 0.0,
            vel_y: 0.0,
            on_ground: false,

            cam_x: 0.0,
            cam_y: 0.0,

            tile_sprite: Sprite::from_file("examples/sprites/level.spr").unwrap(),
            player_sprite: Sprite::from_file("examples/sprites/jario.spr").unwrap(),

            dir_mod_x: 0,
            dir_mod_y: 0,
        }
    }

    fn get_tile(&self, x: f32, y: f32) -> char {
        if x >= 0.0 && x < self.level_width as f32 && y >= 0.0 && y < self.level_height as f32 {
            self.level
                .chars()
                .nth((y as i32 * self.level_width + x as i32) as usize)
                .unwrap()
        } else {
            ' '
        }
    }

    fn set_tile(&mut self, x: f32, y: f32, c: char) {
        if x >= 0.0 && x < self.level_width as f32 && y >= 0.0 && y < self.level_height as f32 {
            let idx = (y as i32 * self.level_width + x as i32) as usize;
            let mut chars: Vec<char> = self.level.chars().collect();
            chars[idx] = c;
            self.level = chars.into_iter().collect();
        }
    }
}

impl ConsoleGame for Platformer {
    fn create(&mut self, _engine: &mut ConsoleGameEngine<Self>) -> bool {
        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, elapsed_time: f32) -> bool {
        if engine.console_focused() {
            if engine.key_held(K_UP) {
                self.vel_y = -6.0;
            }
            if engine.key_held(K_DOWN) {
                self.vel_y = 6.0;
            }
            if engine.key_held(K_LEFT) {
                self.vel_x += (if self.on_ground { -25.0 } else { -15.0 }) * elapsed_time;
                self.dir_mod_y = 1;
            }
            if engine.key_held(K_RIGHT) {
                self.vel_x += (if self.on_ground { 25.0 } else { 15.0 }) * elapsed_time;
                self.dir_mod_y = 0;
            }
            if engine.key_pressed(K_SPACE) && self.vel_y == 0.0 {
                self.vel_y = -12.0;
                self.dir_mod_x = 1;
            }
        }

        self.vel_y += 20.0 * elapsed_time;

        if self.on_ground {
            self.vel_x += -3.0 * self.vel_x * elapsed_time;
            if self.vel_x.abs() < 0.01 {
                self.vel_x = 0.0;
            }
        }

        self.vel_x = self.vel_x.clamp(-10.0, 10.0);
        self.vel_y = self.vel_y.clamp(-100.0, 100.0);

        let mut new_player_x = self.player_x + self.vel_x * elapsed_time;
        let mut new_player_y = self.player_y + self.vel_y * elapsed_time;

        for (ox, oy) in [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)] {
            let tx = new_player_x + ox;
            let ty = new_player_y + oy;
            if self.get_tile(tx, ty) == 'o' {
                self.set_tile(tx, ty, '.');
            }
        }

        if self.vel_x < 0.0 {
            if self.get_tile(new_player_x, self.player_y) != '.'
                || self.get_tile(new_player_x, self.player_y + 0.9) != '.'
            {
                new_player_x = new_player_x.floor() + 1.0;
                self.vel_x = 0.0;
            }
        } else if self.vel_x > 0.0
            && (self.get_tile(new_player_x + 1.0, self.player_y) != '.'
                || self.get_tile(new_player_x + 1.0, self.player_y + 0.9) != '.')
        {
            new_player_x = new_player_x.floor();
            self.vel_x = 0.0;
        }

        self.on_ground = false;
        if self.vel_y < 0.0 {
            if self.get_tile(new_player_x, new_player_y) != '.'
                || self.get_tile(new_player_x + 0.9, new_player_y) != '.'
            {
                new_player_y = new_player_y.floor() + 1.0;
                self.vel_y = 0.0;
            }
        } else if self.vel_y > 0.0 && self.get_tile(new_player_x, new_player_y + 1.0) != '.'
            || self.get_tile(new_player_x + 0.9, new_player_y + 1.0) != '.'
        {
            new_player_y = new_player_y.floor();
            self.vel_y = 0.0;
            self.on_ground = true;
            self.dir_mod_x = 0;
        }

        self.player_x = new_player_x;
        self.player_y = new_player_y;

        self.cam_x = self.player_x;
        self.cam_y = self.player_y;

        let tile_width = 16;
        let tile_height = 16;
        let visible_tiles_x = engine.screen_width() / tile_width;
        let visible_tiles_y = engine.screen_height() / tile_height;

        let mut offset_x = self.cam_x - visible_tiles_x as f32 / 2.0;
        let mut offset_y = self.cam_y - visible_tiles_y as f32 / 2.0;

        if offset_x < 0.0 {
            offset_x = 0.0;
        }
        if offset_y < 0.0 {
            offset_y = 0.0;
        }
        if offset_x > (self.level_width - visible_tiles_x) as f32 {
            offset_x = (self.level_width - visible_tiles_x) as f32;
        }
        if offset_y > (self.level_height - visible_tiles_y) as f32 {
            offset_y = (self.level_height - visible_tiles_y) as f32;
        }

        let tile_offset_x = offset_x.fract() * tile_width as f32;
        let tile_offset_y = offset_y.fract() * tile_height as f32;

        for x in -1..=visible_tiles_x {
            for y in -1..=visible_tiles_y {
                let tile_id = self.get_tile(x as f32 + offset_x, y as f32 + offset_y);

                match tile_id {
                    '.' => {
                        engine.fill_rect_with(
                            x * tile_width - tile_offset_x as i32,
                            y * tile_height - tile_offset_y as i32,
                            (x + 1) * tile_width - tile_offset_x as i32,
                            (y + 1) * tile_height - tile_offset_y as i32,
                            PIXEL_SOLID,
                            FG_CYAN,
                        );
                    }
                    '#' => {
                        engine.draw_partial_sprite(
                            x * tile_width - tile_offset_x as i32,
                            y * tile_height - tile_offset_y as i32,
                            &self.tile_sprite,
                            (2 * tile_width) as usize,
                            0,
                            (tile_width) as usize,
                            (tile_height) as usize,
                        );
                    }
                    'G' => {
                        engine.draw_partial_sprite(
                            x * tile_width - tile_offset_x as i32,
                            y * tile_height - tile_offset_y as i32,
                            &self.tile_sprite,
                            0,
                            0,
                            (tile_width) as usize,
                            (tile_height) as usize,
                        );
                    }
                    'B' => {
                        engine.draw_partial_sprite(
                            x * tile_width - tile_offset_x as i32,
                            y * tile_height - tile_offset_y as i32,
                            &self.tile_sprite,
                            0,
                            tile_height as usize,
                            (tile_width) as usize,
                            (tile_height) as usize,
                        );
                    }
                    '?' => {
                        engine.draw_partial_sprite(
                            x * tile_width - tile_offset_x as i32,
                            y * tile_height - tile_offset_y as i32,
                            &self.tile_sprite,
                            tile_width as usize,
                            tile_height as usize,
                            (tile_width) as usize,
                            (tile_height) as usize,
                        );
                    }
                    'o' => {
                        engine.fill_rect_with(
                            x * tile_width - tile_offset_x as i32,
                            y * tile_height - tile_offset_y as i32,
                            (x + 1) * tile_width - tile_offset_x as i32,
                            (y + 1) * tile_height - tile_offset_y as i32,
                            PIXEL_SOLID,
                            FG_CYAN,
                        );
                        engine.draw_partial_sprite(
                            x * tile_width - tile_offset_x as i32,
                            y * tile_height - tile_offset_y as i32,
                            &self.tile_sprite,
                            (3 * tile_width) as usize,
                            0,
                            (tile_width) as usize,
                            (tile_height) as usize,
                        );
                    }
                    _ => {
                        engine.fill_rect_with(
                            x * tile_width - tile_offset_x as i32,
                            y * tile_height - tile_offset_y as i32,
                            (x + 1) * tile_width - tile_offset_x as i32,
                            (y + 1) * tile_height - tile_offset_y as i32,
                            PIXEL_SOLID,
                            FG_BLACK,
                        );
                    }
                }
            }
        }

        engine.draw_partial_sprite(
            ((self.player_x - offset_x) * tile_width as f32) as i32,
            ((self.player_y - offset_y) * tile_height as f32) as i32,
            &self.player_sprite,
            (self.dir_mod_x * tile_width) as usize,
            (self.dir_mod_y * tile_height) as usize,
            tile_width as usize,
            tile_height as usize,
        );

        true
    }
}

fn main() {
    let mut engine = ConsoleGameEngine::new(Platformer::new());
    engine.set_app_name("2D Scrolling Platformer");
    engine.construct_console(256, 240, 4, 4);
    engine.start();
}
