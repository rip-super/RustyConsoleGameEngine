use rusty_console_game_engine::*;
use std::collections::HashMap;

macro_rules! nofmt {
    ($($code:tt)*) => { $($code)* }
}

struct Piano {
    key_map: HashMap<usize, f32>,
}

impl Piano {
    fn new() -> Self {
        let mut key_map = HashMap::new();

        key_map.insert(K_Z, A3);
        key_map.insert(K_S, A_SHARP3);
        key_map.insert(K_X, B3);
        key_map.insert(K_C, C4);
        key_map.insert(K_F, C_SHARP4);
        key_map.insert(K_V, D4);
        key_map.insert(K_G, D_SHARP4);
        key_map.insert(K_B, E4);
        key_map.insert(K_N, F4);
        key_map.insert(K_J, F_SHARP4);
        key_map.insert(K_M, G4);
        key_map.insert(K_K, G_SHARP4);
        key_map.insert(K_COMMA, A4);

        Self { key_map }
    }
}

impl ConsoleGame for Piano {
    fn create(&mut self, _engine: &mut ConsoleGameEngine<Self>) -> bool {
        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, _elapsed_time: f32) -> bool {
        engine.clear(FG_BLACK);

        nofmt!(
            engine.draw_string(0, 0, "      A#          C#    D#          F#    G#     ");

            engine.draw_string(0, 3, " |   |   |   |   |   | |   |   |   |   | |   |   |");
            engine.draw_string(0, 4, " |   | S |   |   | F | | G |   |   | J | | K |   |");
            engine.draw_string(0, 5, " |   |___|   |   |___| |___|   |   |___| |___|   |");
            engine.draw_string(0, 6, " |     |     |     |     |     |     |     |     |");
            engine.draw_string(0, 7, " |  Z  |  X  |  C  |  V  |  B  |  N  |  M  |  ,  |");
            engine.draw_string(0, 8, " |_____|_____|_____|_____|_____|_____|_____|_____|");

            engine.draw_string(0, 11, "   A     B     C     D     E     F     G     A   ");
        );

        for (&key, &freq) in &self.key_map {
            if engine.key_pressed(key) {
                engine.audio.note_on(freq);
            }

            if engine.key_released(key) {
                engine.audio.note_off(freq);
            }
        }

        true
    }
}

fn main() {
    let mut game = ConsoleGameEngine::new(Piano::new());
    game.set_app_name("Rust Piano");
    game.construct_console(51, 15, 16, 16)
        .expect("Console Construction Failed");
    game.start();
}
