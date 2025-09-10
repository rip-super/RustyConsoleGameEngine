![Cube](https://github.com/user-attachments/assets/a96058d7-3939-4c1e-9974-fa99a06e2763)

# The Rusty Console Game Engine

A Rust port of the [olcConsoleGameEngine](https://github.com/OneLoneCoder/Javidx9/blob/master/ConsoleGameEngine/olcConsoleGameEngine.h). Make simple retro-style console games directly in the terminal, with an API closely matching the original C++ engine.

⚠️ Currently works only on Windows 10/11. ⚠️

⚠️ Make sure to use `conhost.exe` to run any console games. ⚠️

## ✨ Features
- Basic Console Rendering (text, colors, and shapes)
- Sprites (.spr format)
- Keyboard & mouse input
- Audio support (.wav files and arbitrary frequencies)

## 🛠 Roadmap
- [x] Publish to crates.io (available [here](https://crates.io/crates/rusty_console_game_engine))
- [x] Documentation
- [x] Audio support
- [ ] Image → sprite converter
- [ ] Cross-platform support
- [ ] Clean up code (Convert constants to enums)

## 🚀 Quickstart

Add the engine to your `Cargo.toml`:

```toml
[dependencies]
rusty_console_game_engine = "0.3.0"
```

Then create a game:
```rust
use rusty_console_game_engine::*;

struct Demo;

impl ConsoleGame for Demo {
    fn create(&mut self, _engine: &mut ConsoleGameEngine<Self>) -> bool {
        true
    }

    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, _elapsed_time: f32) -> bool {
        engine.clear(FG_BLACK);
        engine.fill_circle(engine.mouse_x(), engine.mouse_y(), 5);

        true
    }
}

fn main() {
    let mut engine = ConsoleGameEngine::new(Demo);
    engine.set_app_name("Example");
    engine.construct_console(150, 150, 4, 4).expect("Console Construction Failed");
    engine.start();
}
```
To see more typical use cases of the engine, check out the `examples`!

## 🎮 Examples

Open `conhost.exe` (in the repo root) before running an example.

[Platformer](https://github.com/rip-super/RustyConsoleGameEngine/blob/main/examples/jario.rs) – Mario-style scrolling platformer

[Mode7](https://github.com/rip-super/RustyConsoleGameEngine/blob/main/examples/mode7.rs) – Pseudo 3D flying effect

[Mazes](https://github.com/rip-super/RustyConsoleGameEngine/blob/main/examples/mazes.rs) – Maze generator and renderer

[Raycaster](https://github.com/rip-super/RustyConsoleGameEngine/blob/main/examples/raycaster.rs) - Simple raycasted world to explore

[Piano](https://github.com/rip-super/RustyConsoleGameEngine/blob/main/examples/piano.rs) - Piano to play different notes