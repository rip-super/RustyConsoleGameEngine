//! # RustyConsoleGameEngine
//! A Rust port of the olcConsoleGameEngine.
//! Make simple retro-style console games directly in the terminal,
//! with an API closely matching the original C++ engine.

// region: Imports

use std::collections::HashMap;
use std::f32::consts::PI;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering::*},
    mpsc::{self, Sender},
};
use std::thread;
use std::time::Instant;

use windows::core::{BOOL, HSTRING, PCWSTR, PSTR, PWSTR};
use windows::Win32::{
    Foundation::*, Graphics::Gdi::*, Media::Audio::*, Media::MMSYSERR_NOERROR, System::Console::*,
    UI::Input::KeyboardAndMouse::GetAsyncKeyState, UI::WindowsAndMessaging::wsprintfW,
};

// endregion

// region: Constants

// ---------------------
// COLORS
// ---------------------

/// Black foreground color. Used in drawing functions like `draw_with`, `draw_string_with`, `fill_rect_with`, `clear`, etc.
pub const FG_BLACK: u16 = 0x0000;
/// Dark blue foreground color.
pub const FG_DARK_BLUE: u16 = 0x0001;
/// Dark green foreground color.
pub const FG_DARK_GREEN: u16 = 0x0002;
/// Dark cyan foreground color.
pub const FG_DARK_CYAN: u16 = 0x0003;
/// Dark red foreground color.
pub const FG_DARK_RED: u16 = 0x0004;
/// Dark magenta foreground color.
pub const FG_DARK_MAGENTA: u16 = 0x0005;
/// Dark yellow foreground color.
pub const FG_DARK_YELLOW: u16 = 0x0006;
/// Grey foreground color.
pub const FG_GREY: u16 = 0x0007;
/// Dark grey foreground color.
pub const FG_DARK_GREY: u16 = 0x0008;
/// Blue foreground color.
pub const FG_BLUE: u16 = 0x0009;
/// Green foreground color.
pub const FG_GREEN: u16 = 0x000A;
/// Cyan foreground color.
pub const FG_CYAN: u16 = 0x000B;
/// Red foreground color.
pub const FG_RED: u16 = 0x000C;
/// Magenta foreground color.
pub const FG_MAGENTA: u16 = 0x000D;
/// Yellow foreground color.
pub const FG_YELLOW: u16 = 0x000E;
/// White foreground color.
pub const FG_WHITE: u16 = 0x000F;

/// Black background color. Used in drawing functions like `draw_with`, `draw_string_with`, `fill_rect_with`, `clear`, etc.
pub const BG_BLACK: u16 = 0x0000;
/// Dark blue background color.
pub const BG_DARK_BLUE: u16 = 0x0010;
/// Dark green background color.
pub const BG_DARK_GREEN: u16 = 0x0020;
/// Dark cyan background color.
pub const BG_DARK_CYAN: u16 = 0x0030;
/// Dark red background color.
pub const BG_DARK_RED: u16 = 0x0040;
/// Dark magenta background color.
pub const BG_DARK_MAGENTA: u16 = 0x0050;
/// Dark yellow background color.
pub const BG_DARK_YELLOW: u16 = 0x0060;
/// Grey background color.
pub const BG_GREY: u16 = 0x0070;
/// Dark grey background color.
pub const BG_DARK_GREY: u16 = 0x0080;
/// Blue background color.
pub const BG_BLUE: u16 = 0x0090;
/// Green background color.
pub const BG_GREEN: u16 = 0x00A0;
/// Cyan background color.
pub const BG_CYAN: u16 = 0x00B0;
/// Red background color.
pub const BG_RED: u16 = 0x00C0;
/// Magenta background color.
pub const BG_MAGENTA: u16 = 0x00D0;
/// Yellow background color.
pub const BG_YELLOW: u16 = 0x00E0;
/// White background color.
pub const BG_WHITE: u16 = 0x00F0;

// ---------------------
// PIXELS
// ---------------------

/// Solid block pixel. Used in drawing functions like `draw_with`, `draw_string_with`, `fill_rect_with`, `clear`, etc.
pub const PIXEL_SOLID: u16 = 0x2588;
/// Three-quarter block pixel.
pub const PIXEL_THREEQUARTERS: u16 = 0x2593;
/// Half block pixel.
pub const PIXEL_HALF: u16 = 0x2592;
/// Quarter block pixel.
pub const PIXEL_QUARTER: u16 = 0x2591;
/// Empty space (transparent) pixel.
pub const PIXEL_EMPTY: u16 = 0x20;

// ---------------------
// MOUSE BUTTONS
// ---------------------

/// Left mouse button. Used with `mouse_pressed`, `mouse_released`, or `mouse_held`.
pub const M_LEFT: usize = 0;
/// Right mouse button.
pub const M_RIGHT: usize = 1;
/// Middle mouse button.
pub const M_MIDDLE: usize = 2;
/// X1 mouse button (side button 1).
pub const M_X1: usize = 3;
/// X2 mouse button (side button 2).
pub const M_X2: usize = 4;

// ---------------------
// KEYS
// ---------------------

/// Space key. Used with `key_pressed`, `key_released`, or `key_held`.
pub const K_SPACE: usize = 0x20;
/// Enter key.
pub const K_ENTER: usize = 0x0D;
/// Escape key.
pub const K_ESCAPE: usize = 0x1B;
/// Backspace key.
pub const K_BACKSPACE: usize = 0x08;
/// Tab key.
pub const K_TAB: usize = 0x09;
/// Shift key.
pub const K_SHIFT: usize = 0x10;
/// Control key.
pub const K_CONTROL: usize = 0x11;
/// Alt key.
pub const K_ALT: usize = 0x12;
/// Caps Lock key.
pub const K_CAPSLOCK: usize = 0x14;
/// Num Lock key.
pub const K_NUMLOCK: usize = 0x90;
/// Scroll Lock key.
pub const K_SCROLL_LOCK: usize = 0x91;

/// Up arrow key.
pub const K_UP: usize = 0x26;
/// Down arrow key.
pub const K_DOWN: usize = 0x28;
/// Left arrow key.
pub const K_LEFT: usize = 0x25;
/// Right arrow key.
pub const K_RIGHT: usize = 0x27;

/// F1 function key.
pub const K_F1: usize = 0x70;
/// F2 function key.
pub const K_F2: usize = 0x71;
/// F3 function key.
pub const K_F3: usize = 0x72;
/// F4 function key.
pub const K_F4: usize = 0x73;
/// F5 function key.
pub const K_F5: usize = 0x74;
/// F6 function key.
pub const K_F6: usize = 0x75;
/// F7 function key.
pub const K_F7: usize = 0x76;
/// F8 function key.
pub const K_F8: usize = 0x77;
/// F9 function key.
pub const K_F9: usize = 0x78;
/// F10 function key.
pub const K_F10: usize = 0x79;
/// F11 function key.
pub const K_F11: usize = 0x7A;
/// F12 function key.
pub const K_F12: usize = 0x7B;

/// Number 0 key.
pub const K_0: usize = 0x30;
/// Number 1 key.
pub const K_1: usize = 0x31;
/// Number 2 key.
pub const K_2: usize = 0x32;
/// Number 3 key.
pub const K_3: usize = 0x33;
/// Number 4 key.
pub const K_4: usize = 0x34;
/// Number 5 key.
pub const K_5: usize = 0x35;
/// Number 6 key.
pub const K_6: usize = 0x36;
/// Number 7 key.
pub const K_7: usize = 0x37;
/// Number 8 key.
pub const K_8: usize = 0x38;
/// Number 9 key.
pub const K_9: usize = 0x39;

/// Letter A key.
pub const K_A: usize = 0x41;
/// Letter B key.
pub const K_B: usize = 0x42;
/// Letter C key.
pub const K_C: usize = 0x43;
/// Letter D key.
pub const K_D: usize = 0x44;
/// Letter E key.
pub const K_E: usize = 0x45;
/// Letter F key.
pub const K_F: usize = 0x46;
/// Letter G key.
pub const K_G: usize = 0x47;
/// Letter H key.
pub const K_H: usize = 0x48;
/// Letter I key.
pub const K_I: usize = 0x49;
/// Letter J key.
pub const K_J: usize = 0x4A;
/// Letter K key.
pub const K_K: usize = 0x4B;
/// Letter L key.
pub const K_L: usize = 0x4C;
/// Letter M key.
pub const K_M: usize = 0x4D;
/// Letter N key.
pub const K_N: usize = 0x4E;
/// Letter O key.
pub const K_O: usize = 0x4F;
/// Letter P key.
pub const K_P: usize = 0x50;
/// Letter Q key.
pub const K_Q: usize = 0x51;
/// Letter R key.
pub const K_R: usize = 0x52;
/// Letter S key.
pub const K_S: usize = 0x53;
/// Letter T key.
pub const K_T: usize = 0x54;
/// Letter U key.
pub const K_U: usize = 0x55;
/// Letter V key.
pub const K_V: usize = 0x56;
/// Letter W key.
pub const K_W: usize = 0x57;
/// Letter X key.
pub const K_X: usize = 0x58;
/// Letter Y key.
pub const K_Y: usize = 0x59;
/// Letter Z key.
pub const K_Z: usize = 0x5A;

/// Numpad 0 key.
pub const K_NUMPAD_0: usize = 0x60;
/// Numpad 1 key.
pub const K_NUMPAD_1: usize = 0x61;
/// Numpad 2 key.
pub const K_NUMPAD_2: usize = 0x62;
/// Numpad 3 key.
pub const K_NUMPAD_3: usize = 0x63;
/// Numpad 4 key.
pub const K_NUMPAD_4: usize = 0x64;
/// Numpad 5 key.
pub const K_NUMPAD_5: usize = 0x65;
/// Numpad 6 key.
pub const K_NUMPAD_6: usize = 0x66;
/// Numpad 7 key.
pub const K_NUMPAD_7: usize = 0x67;
/// Numpad 8 key.
pub const K_NUMPAD_8: usize = 0x68;
/// Numpad 9 key.
pub const K_NUMPAD_9: usize = 0x69;
/// Numpad addition key (+).
pub const K_NUMPAD_ADD: usize = 0x6B;
/// Numpad subtraction key (-).
pub const K_NUMPAD_SUBTRACT: usize = 0x6D;
/// Numpad multiplication key (*).
pub const K_NUMPAD_MULTIPLY: usize = 0x6A;
/// Numpad division key (/).
pub const K_NUMPAD_DIVIDE: usize = 0x6F;
/// Numpad Enter key.
pub const K_NUMPAD_ENTER: usize = 0x0D;

/// Semicolon / Colon key.
/// Only works with US ANSI Keyboards
pub const K_SEMICOLON: usize = 0xBA;
/// Equals / Plus key.
/// Only works with US ANSI Keyboards
pub const K_EQUAL: usize = 0xBB;
/// Comma / Less Than key.
/// Only works with US ANSI Keyboards
pub const K_COMMA: usize = 0xBC;
/// Dash / Underscore key.
/// Only works with US ANSI Keyboards
pub const K_DASH: usize = 0xBD;
/// Period / Greater Than key.
/// Only works with US ANSI Keyboards
pub const K_PERIOD: usize = 0xBE;
/// Forward Slash / Question Mark key.
/// Only works with US ANSI Keyboards
pub const K_SLASH: usize = 0xBF;
/// Backtick / Tilde key.
/// Only works with US ANSI Keyboards
pub const K_BACKTICK: usize = 0xC0;
/// Left Brace / Left Curly Bracket key.
/// Only works with US ANSI Keyboards
pub const K_LEFT_BRACE: usize = 0xDB;
/// Backslash / Pipe key.
/// Only works with US ANSI Keyboards
pub const K_BACKSLASH: usize = 0xDC;
/// Right Brace / Right Curly Bracket key.
/// Only works with US ANSI Keyboards
pub const K_RIGHT_BRACE: usize = 0xDD;
/// Apostrophe / Double Quote key.
/// Only works with US ANSI Keyboards
pub const K_APOSTROPHE: usize = 0xDE;

// ---------------------
// NOTES
// ---------------------

/// A1 (55.00 Hz). Used with the `AudioEngine`'s `play_note` and `play_notes` functions
pub const A1: f32 = 55.00;
/// A2 (110.00 Hz)
pub const A2: f32 = 110.00;
/// A3 (220.00 Hz)
pub const A3: f32 = 220.00;
/// A4 (440.00 Hz)
pub const A4: f32 = 440.00;
/// A5 (880.00 Hz)
pub const A5: f32 = 880.00;
/// A6 (1760.00 Hz)
pub const A6: f32 = 1760.00;
/// A7 (3520.00 Hz)
pub const A7: f32 = 3520.00;
/// A8 (7040.00 Hz)
pub const A8: f32 = 7040.00;

/// A#1 / Bb1 (58.27 Hz)
pub const A_SHARP1: f32 = 58.27;
/// A#2 / Bb2 (116.54 Hz)
pub const A_SHARP2: f32 = 116.54;
/// A#3 / Bb3 (233.08 Hz)
pub const A_SHARP3: f32 = 233.08;
/// A#4 / Bb4 (466.16 Hz)
pub const A_SHARP4: f32 = 466.16;
/// A#5 / Bb5 (932.33 Hz)
pub const A_SHARP5: f32 = 932.33;
/// A#6 / Bb6 (1864.66 Hz)
pub const A_SHARP6: f32 = 1864.66;
/// A#7 / Bb7 (3729.31 Hz)
pub const A_SHARP7: f32 = 3729.31;
/// A#8 / Bb8 (7458.62 Hz)
pub const A_SHARP8: f32 = 7458.62;

/// B1 (61.74 Hz)
pub const B1: f32 = 61.74;
/// B2 (123.47 Hz)
pub const B2: f32 = 123.47;
/// B3 (246.94 Hz)
pub const B3: f32 = 246.94;
/// B4 (493.88 Hz)
pub const B4: f32 = 493.88;
/// B5 (987.77 Hz)
pub const B5: f32 = 987.77;
/// B6 (1975.53 Hz)
pub const B6: f32 = 1975.53;
/// B7 (3951.07 Hz)
pub const B7: f32 = 3951.07;
/// B8 (7902.13 Hz)
pub const B8: f32 = 7902.13;

/// C1 (32.70 Hz)
pub const C1: f32 = 32.70;
/// C2 (65.41 Hz)
pub const C2: f32 = 65.41;
/// C3 (130.81 Hz)
pub const C3: f32 = 130.81;
/// C4 (261.63 Hz)
pub const C4: f32 = 261.63;
/// C5 (523.25 Hz)
pub const C5: f32 = 523.25;
/// C6 (1046.50 Hz)
pub const C6: f32 = 1046.50;
/// C7 (2093.00 Hz)
pub const C7: f32 = 2093.00;
/// C8 (4186.01 Hz)
pub const C8: f32 = 4186.01;

/// C#1 / Db1 (34.65 Hz)
pub const C_SHARP1: f32 = 34.65;
/// C#2 / Db2 (69.30 Hz)
pub const C_SHARP2: f32 = 69.30;
/// C#3 / Db3 (138.59 Hz)
pub const C_SHARP3: f32 = 138.59;
/// C#4 / Db4 (277.18 Hz)
pub const C_SHARP4: f32 = 277.18;
/// C#5 / Db5 (554.37 Hz)
pub const C_SHARP5: f32 = 554.37;
/// C#6 / Db6 (1108.73 Hz)
pub const C_SHARP6: f32 = 1108.73;
/// C#7 / Db7 (2217.46 Hz)
pub const C_SHARP7: f32 = 2217.46;
/// C#8 / Db8 (4434.92 Hz)
pub const C_SHARP8: f32 = 4434.92;

/// D1 (36.71 Hz)
pub const D1: f32 = 36.71;
/// D2 (73.42 Hz)
pub const D2: f32 = 73.42;
/// D3 (146.83 Hz)
pub const D3: f32 = 146.83;
/// D4 (293.66 Hz)
pub const D4: f32 = 293.66;
/// D5 (587.33 Hz)
pub const D5: f32 = 587.33;
/// D6 (1174.66 Hz)
pub const D6: f32 = 1174.66;
/// D7 (2349.32 Hz)
pub const D7: f32 = 2349.32;
/// D8 (4698.63 Hz)
pub const D8: f32 = 4698.63;

/// D#1 / Eb1 (38.89 Hz)
pub const D_SHARP1: f32 = 38.89;
/// D#2 / Eb2 (77.78 Hz)
pub const D_SHARP2: f32 = 77.78;
/// D#3 / Eb3 (155.56 Hz)
pub const D_SHARP3: f32 = 155.56;
/// D#4 / Eb4 (311.13 Hz)
pub const D_SHARP4: f32 = 311.13;
/// D#5 / Eb5 (622.25 Hz)
pub const D_SHARP5: f32 = 622.25;
/// D#6 / Eb6 (1244.51 Hz)
pub const D_SHARP6: f32 = 1244.51;
/// D#7 / Eb7 (2489.02 Hz)
pub const D_SHARP7: f32 = 2489.02;
/// D#8 / Eb8 (4978.03 Hz)
pub const D_SHARP8: f32 = 4978.03;

/// E1 (41.20 Hz)
pub const E1: f32 = 41.20;
/// E2 (82.41 Hz)
pub const E2: f32 = 82.41;
/// E3 (164.81 Hz)
pub const E3: f32 = 164.81;
/// E4 (329.63 Hz)
pub const E4: f32 = 329.63;
/// E5 (659.25 Hz)
pub const E5: f32 = 659.25;
/// E6 (1318.51 Hz)
pub const E6: f32 = 1318.51;
/// E7 (2637.02 Hz)
pub const E7: f32 = 2637.02;
/// E8 (5274.04 Hz)
pub const E8: f32 = 5274.04;

/// F1 (43.65 Hz)
pub const F1: f32 = 43.65;
/// F2 (87.31 Hz)
pub const F2: f32 = 87.31;
/// F3 (174.61 Hz)
pub const F3: f32 = 174.61;
/// F4 (349.23 Hz)
pub const F4: f32 = 349.23;
/// F5 (698.46 Hz)
pub const F5: f32 = 698.46;
/// F6 (1396.91 Hz)
pub const F6: f32 = 1396.91;
/// F7 (2793.83 Hz)
pub const F7: f32 = 2793.83;
/// F8 (5587.65 Hz)
pub const F8: f32 = 5587.65;

/// F#1 / Gb1 (46.25 Hz)
pub const F_SHARP1: f32 = 46.25;
/// F#2 / Gb2 (92.50 Hz)
pub const F_SHARP2: f32 = 92.50;
/// F#3 / Gb3 (185.00 Hz)
pub const F_SHARP3: f32 = 185.00;
/// F#4 / Gb4 (369.99 Hz)
pub const F_SHARP4: f32 = 369.99;
/// F#5 / Gb5 (739.99 Hz)
pub const F_SHARP5: f32 = 739.99;
/// F#6 / Gb6 (1479.98 Hz)
pub const F_SHARP6: f32 = 1479.98;
/// F#7 / Gb7 (2959.96 Hz)
pub const F_SHARP7: f32 = 2959.96;
/// F#8 / Gb8 (5919.91 Hz)
pub const F_SHARP8: f32 = 5919.91;

/// G1 (49.00 Hz)
pub const G1: f32 = 49.00;
/// G2 (98.00 Hz)
pub const G2: f32 = 98.00;
/// G3 (196.00 Hz)
pub const G3: f32 = 196.00;
/// G4 (392.00 Hz)
pub const G4: f32 = 392.00;
/// G5 (783.99 Hz)
pub const G5: f32 = 783.99;
/// G6 (1567.98 Hz)
pub const G6: f32 = 1567.98;
/// G7 (3135.96 Hz)
pub const G7: f32 = 3135.96;
/// G8 (6271.93 Hz)
pub const G8: f32 = 6271.93;

/// G#1 / Ab1 (51.91 Hz)
pub const G_SHARP1: f32 = 51.91;
/// G#2 / Ab2 (103.83 Hz)
pub const G_SHARP2: f32 = 103.83;
/// G#3 / Ab3 (207.65 Hz)
pub const G_SHARP3: f32 = 207.65;
/// G#4 / Ab4 (415.30 Hz)
pub const G_SHARP4: f32 = 415.30;
/// G#5 / Ab5 (830.61 Hz)
pub const G_SHARP5: f32 = 830.61;
/// G#6 / Ab6 (1661.22 Hz)
pub const G_SHARP6: f32 = 1661.22;
/// G#7 / Ab7 (3322.44 Hz)
pub const G_SHARP7: f32 = 3322.44;
/// G#8 / Ab8 (6644.88 Hz)
pub const G_SHARP8: f32 = 6644.88;

// endregion

// region: Console State

#[derive(Clone)]
struct ConsoleState {
    screen_width: i16,
    screen_height: i16,
    window_rect: SMALL_RECT,
    font_cfi: CONSOLE_FONT_INFOEX,
    cursor_info: CONSOLE_CURSOR_INFO,
    console_mode: CONSOLE_MODE,
}

impl ConsoleState {
    fn save(output_handle: HANDLE, input_handle: HANDLE) -> Self {
        let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::default();
        unsafe {
            GetConsoleScreenBufferInfo(output_handle, &mut csbi)
                .expect("Failed to get console screen buffer info");
        }

        let mut font_cfi = CONSOLE_FONT_INFOEX {
            cbSize: std::mem::size_of::<CONSOLE_FONT_INFOEX>() as u32,
            ..Default::default()
        };
        unsafe {
            GetCurrentConsoleFontEx(output_handle, false, &mut font_cfi)
                .expect("Failed to get current font");
        }

        let mut cursor_info = CONSOLE_CURSOR_INFO::default();
        unsafe {
            GetConsoleCursorInfo(output_handle, &mut cursor_info)
                .expect("Failed to get cursor info");
        }

        let mut mode = CONSOLE_MODE(0);
        unsafe {
            GetConsoleMode(input_handle, &mut mode).expect("Failed to get console mode");
        }

        Self {
            screen_width: csbi.dwSize.X,
            screen_height: csbi.dwSize.Y,
            window_rect: csbi.srWindow,
            font_cfi,
            cursor_info,
            console_mode: mode,
        }
    }

    fn restore(&self, output_handle: HANDLE, input_handle: HANDLE) {
        unsafe {
            let mut chars_written = 0;
            let size = (self.screen_width as u32) * (self.screen_height as u32);
            FillConsoleOutputCharacterW(
                output_handle,
                b' ' as u16,
                size,
                COORD { X: 0, Y: 0 },
                &mut chars_written,
            )
            .ok();
            SetConsoleCursorPosition(output_handle, COORD { X: 0, Y: 0 }).ok();

            let coord = COORD {
                X: self.screen_width,
                Y: self.screen_height,
            };
            SetConsoleScreenBufferSize(output_handle, coord).ok();
            SetConsoleWindowInfo(output_handle, true, &self.window_rect).ok();
            SetCurrentConsoleFontEx(output_handle, false, &self.font_cfi).ok();
            SetConsoleCursorInfo(output_handle, &self.cursor_info).ok();
            SetConsoleMode(input_handle, self.console_mode).ok();
        }
    }
}

// endregion

// region: Sprite

/// A 2D sprite consisting of glyphs and color values.
///
/// Sprites can be drawn using `ConsoleGameEngine` methods like `draw_sprite` or
/// `draw_partial_sprite`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sprite {
    /// Width of the sprite in characters.
    pub width: usize,
    /// Height of the sprite in characters.
    pub height: usize,
    glyphs: Vec<u16>,
    colors: Vec<u16>,
}

impl Sprite {
    /// Creates a new sprite of the given width and height.
    /// All glyphs are initialized to `PIXEL_EMPTY` and all colors to `FG_BLACK`.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            glyphs: vec![PIXEL_EMPTY; width * height],
            colors: vec![FG_BLACK; width * height],
        }
    }

    /// Loads a sprite from a file (by convention ending in `.spr`).
    /// The file must contain width and height (u32 little-endian) followed by colors and glyphs.
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        if buf.len() < 8 {
            return Err("sprite file too small".into());
        }

        let width = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
        let height = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as usize;
        let count = width
            .checked_mul(height)
            .ok_or("sprite dimensions overflow")?;
        let expected = 8 + 2 * count * 2;
        if buf.len() < expected {
            return Err("sprite file truncated".into());
        }

        let mut offset = 8;
        let mut colors = Vec::with_capacity(count);
        for _ in 0..count {
            let v = u16::from_le_bytes(buf[offset..offset + 2].try_into().unwrap());
            offset += 2;
            colors.push(v);
        }

        let mut glyphs = Vec::with_capacity(count);
        for _ in 0..count {
            let v = u16::from_le_bytes(buf[offset..offset + 2].try_into().unwrap());
            offset += 2;
            glyphs.push(v);
        }

        Ok(Self {
            width,
            height,
            glyphs,
            colors,
        })
    }

    /// Saves the sprite to a `.spr` file in the same format as `from_file`.
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(path)?;
        file.write_all(&(self.width as u32).to_le_bytes())?;
        file.write_all(&(self.height as u32).to_le_bytes())?;

        for &c in &self.colors {
            file.write_all(&c.to_le_bytes())?;
        }
        for &g in &self.glyphs {
            file.write_all(&g.to_le_bytes())?;
        }

        Ok(())
    }

    /// Sets the glyph at `(x, y)` to `c`.
    pub fn set_glyph(&mut self, x: usize, y: usize, g: u16) {
        if x < self.width && y < self.height {
            self.glyphs[y * self.width + x] = g;
        }
    }

    /// Sets the color at `(x, y)` to `c`.
    pub fn set_color(&mut self, x: usize, y: usize, c: u16) {
        if x < self.width && y < self.height {
            self.colors[y * self.width + x] = c;
        }
    }

    /// Returns the glyph at `(x, y)`, or `PIXEL_EMPTY` if out of bounds.
    pub fn get_glyph(&self, x: usize, y: usize) -> u16 {
        if x < self.width && y < self.height {
            self.glyphs[y * self.width + x]
        } else {
            PIXEL_EMPTY
        }
    }

    /// Returns the color at `(x, y)`, or `FG_BLACK` if out of bounds.
    pub fn get_color(&self, x: usize, y: usize) -> u16 {
        if x < self.width && y < self.height {
            self.colors[y * self.width + x]
        } else {
            FG_BLACK
        }
    }

    fn wrapped_sample_index(&self, x: f32, y: f32) -> (usize, usize) {
        let fx = x - x.floor();
        let fy = y - y.floor();
        let sx =
            ((fx * self.width as f32).floor() as isize).rem_euclid(self.width as isize) as usize;
        let sy =
            ((fy * self.height as f32).floor() as isize).rem_euclid(self.height as isize) as usize;
        (sx, sy)
    }

    /// Samples the glyph at normalized coordinates `(x, y)` in `[0.0, 1.0)` space.
    /// Wrapping occurs for coordinates outside the [0,1) range.
    pub fn sample_glyph(&self, x: f32, y: f32) -> u16 {
        let (sx, sy) = self.wrapped_sample_index(x, y);
        self.get_glyph(sx, sy)
    }

    /// Samples the color at normalized coordinates `(x, y)` in `[0.0, 1.0)` space.
    /// Wrapping occurs for coordinates outside the [0,1) range.
    pub fn sample_color(&self, x: f32, y: f32) -> u16 {
        let (sx, sy) = self.wrapped_sample_index(x, y);
        self.get_color(sx, sy)
    }
}

// endregion

// region: Audio

const CHUNK_SIZE: usize = 512;
static NOTE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
enum AudioCommand {
    LoadSample(String),
    PlaySample(String),
    LoadSampleFromBuffer(String, Vec<i16>),
    NoteOn(f32),
    NoteOff(f32),
    Quit,
}

struct PlayingSound {
    data: Vec<i16>,
    cursor: usize,
}

struct PlayingNote {
    freq: f32,
    phase: f32,
    amplitude: f32,
    target_amp: f32,
    step: f32,
    active: bool,
}

/// Audio engine used through  the `ConsoleGameEngine`.
///
/// Handles asynchronous playback of WAV files and synthesized notes.
///
/// Users can interact with it via the audio field in the `ConsoleGameEngine`:
///
/// ```rust
/// engine.audio.load_sample("explosion.wav");
/// engine.audio.play_sample("explosion.wav");
/// engine.audio.play_note(A4, 500);
/// engine.audio.play_notes(&[A4, C_SHARP5, E5], 1000);
/// engine.audio.note_on(A4);
/// engine.audio.note_off(A4);
/// ```
#[derive(Clone)]
pub struct AudioEngine {
    tx: Sender<AudioCommand>,
}

impl AudioEngine {
    #[allow(clippy::new_without_default)]
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<AudioCommand>();

        thread::spawn(move || {
            let format = WAVEFORMATEX {
                wFormatTag: WAVE_FORMAT_PCM as u16,
                nChannels: 2,
                nSamplesPerSec: 44100,
                nAvgBytesPerSec: 44100 * 2 * 2,
                nBlockAlign: 4,
                wBitsPerSample: 16,
                cbSize: 0,
            };

            let mut h_waveout = HWAVEOUT::default();
            unsafe {
                let res = waveOutOpen(
                    Some(&mut h_waveout),
                    WAVE_MAPPER,
                    &format,
                    None,
                    Some(0),
                    CALLBACK_NULL,
                );

                if res != MMSYSERR_NOERROR {
                    eprintln!("Failed to open audio device: {}", res);
                    return;
                }
            }

            let mut samples = HashMap::new();
            let mut active_sounds = Vec::new();
            let mut active_notes = Vec::new();

            'audio_loop: loop {
                while let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        AudioCommand::LoadSample(path) => {
                            if let Ok(data) = AudioEngine::load_wav(&path) {
                                samples.insert(path, data);
                            }
                        }
                        AudioCommand::LoadSampleFromBuffer(key, buffer) => {
                            samples.insert(key, buffer);
                        }
                        AudioCommand::PlaySample(path) => {
                            if let Some(data) = samples.get(&path) {
                                active_sounds.push(PlayingSound {
                                    data: data.clone(),
                                    cursor: 0,
                                });
                            }
                        }
                        AudioCommand::NoteOn(freq) => {
                            let sample_rate = 44100.0;
                            let attack_samples = 100;
                            let mut buffer = vec![0i16; attack_samples * 2];
                            for i in 0..attack_samples {
                                let t = i as f32 / sample_rate;
                                let s = ((2.0 * PI * freq * t).sin() * i16::MAX as f32 * 0.1)
                                    .clamp(i16::MIN as f32, i16::MAX as f32);
                                buffer[i * 2] = s as i16;
                                buffer[i * 2 + 1] = s as i16;
                            }
                            AudioEngine::play_buffer(h_waveout, buffer);

                            let attack_ms = 50.0;
                            let step = 1.0 / (44100.0 * (attack_ms / 1000.0));
                            active_notes.push(PlayingNote {
                                freq,
                                phase: 0.0,
                                amplitude: 0.0,
                                target_amp: 1.0,
                                step,
                                active: true,
                            });
                        }
                        AudioCommand::NoteOff(freq) => {
                            let sample_rate = 44100.0;
                            let release_samples = 100;
                            let mut buffer = vec![0i16; release_samples * 2];

                            for i in 0..release_samples {
                                let t = i as f32 / sample_rate;
                                let s = ((2.0 * PI * freq * t).sin() * i16::MAX as f32 * 0.05)
                                    .clamp(i16::MIN as f32, i16::MAX as f32);
                                buffer[i * 2] = s as i16;
                                buffer[i * 2 + 1] = s as i16;
                            }
                            AudioEngine::play_buffer(h_waveout, buffer);

                            for note in active_notes.iter_mut() {
                                if (note.freq - freq).abs() < f32::EPSILON && note.active {
                                    let release_ms = 50.0;
                                    note.target_amp = 0.0;
                                    note.step = -(1.0 / (44100.0 * (release_ms / 1000.0)));
                                }
                            }
                        }
                        AudioCommand::Quit => break 'audio_loop,
                    }
                }

                let mut mix_buffer = vec![0i32; CHUNK_SIZE * 2];

                for sound in active_sounds.iter_mut() {
                    for i in 0..CHUNK_SIZE {
                        let idx = i * 2;
                        if sound.cursor + 1 < sound.data.len() {
                            mix_buffer[idx] += sound.data[sound.cursor] as i32;
                            mix_buffer[idx + 1] += sound.data[sound.cursor + 1] as i32;
                            sound.cursor += 2;
                        }
                    }
                }

                let sample_rate = 44100.0;
                let max_notes = active_notes.len().max(1) as f32;

                for note in active_notes.iter_mut().filter(|n| n.active) {
                    let step = 2.0 * PI * note.freq / sample_rate;

                    for i in 0..CHUNK_SIZE {
                        let idx = i * 2;

                        if (note.step > 0.0 && note.amplitude < note.target_amp)
                            || (note.step < 0.0 && note.amplitude > note.target_amp)
                        {
                            note.amplitude = (note.amplitude + note.step).clamp(0.0, 1.0);
                        } else if note.target_amp == 0.0 {
                            note.active = false;
                        }

                        let s = (note.phase).sin() * note.amplitude * (0.3 / max_notes);

                        note.phase += step;
                        if note.phase > PI * 2.0 {
                            note.phase -= PI * 2.0;
                        }

                        let si = (s * i16::MAX as f32) as i16;
                        mix_buffer[idx] += si as i32;
                        mix_buffer[idx + 1] += si as i32;
                    }
                }

                let final_buffer: Vec<i16> = mix_buffer
                    .into_iter()
                    .map(|s| s.clamp(i16::MIN as i32, i16::MAX as i32) as i16)
                    .collect();

                AudioEngine::play_buffer(h_waveout, final_buffer);

                active_sounds.retain(|s| s.cursor < s.data.len());
                active_notes.retain(|n| n.active);

                thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        Self { tx }
    }

    /// Loads a WAV file asynchronously.
    ///
    /// The sample can later be played using `play_sample`.
    /// The path is used as the key to identify the sample.
    /// Normally used in the `create` function when implementing the `ConsoleGame` trait.
    pub fn load_sample<P: AsRef<Path>>(&self, path: P) {
        let _ = self.tx.send(AudioCommand::LoadSample(
            path.as_ref().to_string_lossy().into(),
        ));
    }

    /// Plays a previously loaded sample asynchronously.
    ///
    /// Multiple instances of the same sample can play simultaneously.
    pub fn play_sample<P: AsRef<Path>>(&self, path: P) {
        let _ = self.tx.send(AudioCommand::PlaySample(
            path.as_ref().to_string_lossy().into(),
        ));
    }

    /// Generates and plays a single note of the given frequency (Hz) and duration (ms).
    ///
    /// Useful for procedural audio or simple effects.
    /// Normally used in conjunction with the note constants (A4, C_SHARP5, E5)
    pub fn play_note(&self, frequency: f32, duration_ms: u32) {
        let sample_rate = 44100;
        let sample_count = ((duration_ms as f32 / 1000.0) * sample_rate as f32) as usize;
        if sample_count == 0 {
            return;
        }

        let mut mono = vec![0f32; sample_count];
        for (n, v) in mono.iter_mut().enumerate() {
            let t = n as f32 / sample_rate as f32;
            *v = (t * frequency * 2.0 * PI).sin();
        }

        Self::apply_attack_release(&mut mono, sample_rate, duration_ms);

        let mut stereo = vec![0i16; sample_count * 2];
        let scale = 0.95;
        for (n, &v) in mono.iter().enumerate() {
            let s = (v * scale * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32);
            let si = s as i16;
            stereo[n * 2] = si;
            stereo[n * 2 + 1] = si;
        }

        let key = Self::generate_unique_key();
        let _ = self
            .tx
            .send(AudioCommand::LoadSampleFromBuffer(key.clone(), stereo));
        let _ = self.tx.send(AudioCommand::PlaySample(key));
    }

    /// Generates and plays multiple notes simultaneously (like a chord).
    ///
    /// Each frequency in `freqs` is mixed together, scaled to avoid clipping,
    /// and played for the given duration (ms).
    /// Normally used in conjunction with the note constants (A4, C_SHARP5, E5)
    pub fn play_notes(&self, freqs: &[f32], duration_ms: u32) {
        if freqs.is_empty() {
            return;
        }
        let sample_rate = 44100u32;
        let sample_count = ((duration_ms as f32 / 1000.0) * sample_rate as f32) as usize;
        if sample_count == 0 {
            return;
        }

        let mut mono = vec![0f32; sample_count];
        for &freq in freqs {
            for (n, v) in mono.iter_mut().enumerate() {
                let t = n as f32 / sample_rate as f32;
                *v += (t * freq * 2.0 * PI).sin();
            }
        }

        let max_possible = freqs.len() as f32;
        let scale = 0.9 / max_possible;
        for v in mono.iter_mut() {
            *v *= scale;
        }

        Self::apply_attack_release(&mut mono, sample_rate, duration_ms);

        let mut stereo = vec![0i16; sample_count * 2];
        for (n, &v) in mono.iter().enumerate() {
            let s = (v * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32);
            let si = s as i16;
            stereo[n * 2] = si;
            stereo[n * 2 + 1] = si;
        }

        let key = Self::generate_unique_key();
        let _ = self
            .tx
            .send(AudioCommand::LoadSampleFromBuffer(key.clone(), stereo));
        let _ = self.tx.send(AudioCommand::PlaySample(key));
    }

    /// Starts playing a note of the given frequency (Hz) immediately.
    ///
    /// Normally used in conjunction with the note constants (A4, C_SHARP5, E5)
    pub fn note_on(&self, freq: f32) {
        let _ = self.tx.send(AudioCommand::NoteOn(freq));
    }

    /// Stops a previously started note of the given frequency (Hz).
    ///
    /// Normally used in conjunction with the note constants (A4, C_SHARP5, E5)
    /// and with `note_on` to control sustained notes.
    pub fn note_off(&self, freq: f32) {
        let _ = self.tx.send(AudioCommand::NoteOff(freq));
    }

    fn apply_attack_release(buffer: &mut [f32], sample_rate: u32, duration_ms: u32) {
        let len = buffer.len();
        if len == 0 {
            return;
        }

        let ramp_pct = 0.10;
        let ramp_samps =
            ((duration_ms as f32 / 1000.0) * sample_rate as f32 * ramp_pct).round() as usize;
        let ramp_samps = ramp_samps.min(len / 2);

        for (i, v) in buffer.iter_mut().take(ramp_samps).enumerate() {
            *v *= i as f32 / ramp_samps.max(1) as f32;
        }

        for i in 0..ramp_samps {
            let idx = len - ramp_samps + i;
            buffer[idx] *= 1.0 - (i as f32 / ramp_samps as f32);
        }
    }

    fn generate_unique_key() -> String {
        let id = NOTE_COUNTER.fetch_add(1, Relaxed);
        format!("__temp_notes_{}", id)
    }

    fn load_wav(path: &str) -> std::io::Result<Vec<i16>> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let data_start = buf.windows(4).position(|w| w == b"data").unwrap() + 8;
        let samples: Vec<i16> = buf[data_start..]
            .chunks_exact(2)
            .map(|b| i16::from_le_bytes([b[0], b[1]]))
            .collect();

        Ok(samples)
    }

    fn play_buffer(h_waveout: HWAVEOUT, data: Vec<i16>) {
        let boxed_data = Box::new(data);
        let raw_data = Box::into_raw(boxed_data);

        let mut hdr = Box::new(WAVEHDR {
            lpData: PSTR(unsafe { (*raw_data).as_ptr() as *mut u8 }),
            dwBufferLength: (unsafe { (*raw_data).len() * 2 } as u32),
            dwFlags: 0,
            dwLoops: 0,
            dwUser: raw_data as usize,
            ..Default::default()
        });

        unsafe {
            waveOutPrepareHeader(h_waveout, &mut *hdr, std::mem::size_of::<WAVEHDR>() as u32);
            waveOutWrite(h_waveout, &mut *hdr, std::mem::size_of::<WAVEHDR>() as u32);
        }

        let _ = Box::into_raw(hdr);
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        let _ = self.tx.send(AudioCommand::Quit);
    }
}

// endregion

// region: Engine

static RUNNING: AtomicBool = AtomicBool::new(true);

unsafe extern "system" fn console_handler(ctrl_type: u32) -> BOOL {
    if ctrl_type == CTRL_CLOSE_EVENT {
        RUNNING.store(false, SeqCst);
    }
    BOOL(1)
}

/// Trait that defines the behavior of a game to be run by the `ConsoleGameEngine`.
///
/// To create a game, define a struct containing your game state and implement this trait
/// for it. The engine will call the provided methods during the game loop.
pub trait ConsoleGame: Sized {
    fn app_name(&self) -> &str {
        "Default"
    }

    /// Called once when the game starts.
    ///
    /// Use this method to initialize your game state, load sprites, set variables, etc.
    ///
    /// # Parameters
    /// * `engine` - A mutable reference to the running `ConsoleGameEngine`. You can use
    ///   this to query the screen, input, or draw anything immediately if needed.
    ///
    /// # Returns
    /// Return `true` to continue running the game, or `false` to immediately exit.
    fn create(&mut self, engine: &mut ConsoleGameEngine<Self>) -> bool;

    /// Called once per frame to update the game state and render.
    ///
    /// This is where the main game logic should live: moving objects, handling input,
    /// checking collisions, drawing, etc.
    ///
    /// # Parameters
    /// * `engine` - A mutable reference to the `ConsoleGameEngine`. Use it to draw shapes,
    ///   sprites, and query input.
    /// * `elapsed_time` - Time (in seconds) since the last frame. Useful for smooth movement
    ///   and animations.
    ///
    /// # Returns
    /// Return `true` to continue running the game, or `false` to exit.
    fn update(&mut self, engine: &mut ConsoleGameEngine<Self>, elapsed_time: f32) -> bool;

    /// Called once when the game exits or the engine is shutting down.
    ///
    /// Use this method to clean up resources, save game state, or free memory.
    ///
    /// # Parameters
    /// * `engine` - A mutable reference to the `ConsoleGameEngine`.
    ///
    /// # Returns
    /// Return `true` to allow normal shutdown, or `false` to prevent shutdown (rarely used).
    ///
    /// # Default Implementation
    /// The default implementation does nothing and returns `true`.
    #[allow(unused_variables)]
    fn destroy(&mut self, engine: &mut ConsoleGameEngine<Self>) -> bool {
        true
    }
}

/// The main engine that runs a game implementing `ConsoleGame`.
///
/// Handles console creation, input, rendering, and the main game loop.
#[derive(Clone)]
pub struct ConsoleGameEngine<G: ConsoleGame> {
    app_name: String,

    output_handle: HANDLE,
    input_handle: HANDLE,

    original_state: ConsoleState,

    key_new_state: [u16; 256],
    key_old_state: [u16; 256],
    key_pressed: [bool; 256],
    key_released: [bool; 256],
    key_held: [bool; 256],

    mouse_new_state: [bool; 5],
    mouse_old_state: [bool; 5],
    mouse_pressed: [bool; 5],
    mouse_released: [bool; 5],
    mouse_held: [bool; 5],

    mouse_x: i32,
    mouse_y: i32,

    console_in_focus: bool,

    rect: SMALL_RECT,

    screen_width: i16,
    screen_height: i16,

    window_buffer: Vec<CHAR_INFO>,

    pub audio: AudioEngine,

    game: Option<G>,
}

// region: Core

impl<G: ConsoleGame> ConsoleGameEngine<G> {
    /// Creates a new `ConsoleGameEngine` with the given game.
    ///
    /// # Parameters
    /// * `game` - The user-defined struct implementing `ConsoleGame`.
    pub fn new(game: G) -> Self {
        let app_name = game.app_name().to_string();
        let mouse_x = 0;
        let mouse_y = 0;
        let output_handle = unsafe {
            GetStdHandle(STD_OUTPUT_HANDLE).unwrap_or_else(|e| {
                eprintln!("Error getting stdout handle: {:?}", e);
                exit(1);
            })
        };
        let input_handle = unsafe {
            GetStdHandle(STD_INPUT_HANDLE).unwrap_or_else(|e| {
                eprintln!("Error getting stin handle: {:?}", e);
                exit(1);
            })
        };
        let original_state = ConsoleState::save(output_handle, input_handle);
        let rect = SMALL_RECT::default();
        let window_buffer = Vec::new();

        Self {
            app_name,
            output_handle,
            input_handle,
            original_state,
            key_new_state: [0; 256],
            key_old_state: [0; 256],
            key_pressed: [false; 256],
            key_released: [false; 256],
            key_held: [false; 256],
            mouse_new_state: [false; 5],
            mouse_old_state: [false; 5],
            mouse_pressed: [false; 5],
            mouse_released: [false; 5],
            mouse_held: [false; 5],
            mouse_x,
            mouse_y,
            console_in_focus: true,
            rect,
            screen_width: 80,
            screen_height: 80,
            window_buffer,
            audio: AudioEngine::new(),
            game: Some(game),
        }
    }

    /// Returns the width of the console in characters.
    pub fn screen_width(&self) -> i32 {
        self.screen_width as i32
    }

    /// Returns the height of the console in characters.
    pub fn screen_height(&self) -> i32 {
        self.screen_height as i32
    }

    /// Returns `true` if the specified key was pressed this frame.
    ///
    /// Normally used in conjection with key constants such as
    /// `K_W`, `K_0`, `K_UP`, etc.
    pub fn key_pressed(&self, key: usize) -> bool {
        self.key_pressed[key]
    }

    /// Returns `true` if the specified key was released this frame.
    ///
    /// Normally used in conjection with key constants such as
    /// `K_W`, `K_0`, `K_UP`, etc.
    pub fn key_released(&self, key: usize) -> bool {
        self.key_released[key]
    }

    /// Returns `true` if the specified key is currently held down.
    ///
    /// Normally used in conjection with key constants such as
    /// `K_W`, `K_0`, `K_UP`, etc.
    pub fn key_held(&self, key: usize) -> bool {
        self.key_held[key]
    }

    /// Returns `true` if the specified mouse button was pressed this frame.
    ///
    /// Normally used in conjection with mouse button constants
    /// such as `M_LEFT`, `M_MIDDLE`, `M_RIGHT`, etc.
    pub fn mouse_pressed(&self, button: usize) -> bool {
        self.mouse_pressed[button]
    }

    /// Returns `true` if the specified mouse button was released this frame.
    ///
    /// Normally used in conjection with mouse button constants
    /// such as `M_LEFT`, `M_MIDDLE`, `M_RIGHT`, etc.
    pub fn mouse_released(&self, button: usize) -> bool {
        self.mouse_released[button]
    }

    /// Returns `true` if the specified mouse button is currently held down.
    ///
    /// Normally used in conjection with mouse button constants
    /// such as `M_LEFT`, `M_MIDDLE`, `M_RIGHT`, etc.
    pub fn mouse_held(&self, button: usize) -> bool {
        self.mouse_held[button]
    }

    /// Returns the current X position of the mouse in console coordinates.
    pub fn mouse_x(&self) -> i32 {
        self.mouse_x
    }

    /// Returns the current Y position of the mouse in console coordinates.
    pub fn mouse_y(&self) -> i32 {
        self.mouse_y
    }

    /// Returns the current (X, Y) position of the mouse.
    pub fn mouse_pos(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
    }

    /// Returns `true` if the console currently has focus.
    pub fn console_focused(&self) -> bool {
        self.console_in_focus
    }

    /// Initializes the console with the given dimensions and font size.
    ///
    /// This function sets up the console window, screen buffer, font, and other
    /// properties. It now returns a `Result` to indicate success or failure.
    ///
    /// # Parameters
    /// - `width` - Console width in characters.
    /// - `height` - Console height in characters.
    /// - `fontw` - Font width in pixels.
    /// - `fonth` - Font height in pixels.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The console handle is invalid.
    /// - The requested console size exceeds the maximum allowed for the current display/font.
    /// - Any Windows API call fails (setting buffer size, window info, font, etc.)
    pub fn construct_console(
        &mut self,
        width: i16,
        height: i16,
        fontw: i16,
        fonth: i16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.output_handle == INVALID_HANDLE_VALUE {
            return Err("Bad Handle".into());
        }

        self.screen_width = width;
        self.screen_height = height;

        self.rect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 1,
            Bottom: 1,
        };

        self.set_console_window_info(self.output_handle, true, &self.rect)?;

        let coord = COORD {
            X: self.screen_width,
            Y: self.screen_height,
        };

        self.set_console_screen_buffer_size(self.output_handle, coord)?;

        self.set_console_active_screen_buffer(self.output_handle)?;

        let mut font_cfi = CONSOLE_FONT_INFOEX {
            cbSize: size_of::<CONSOLE_FONT_INFOEX>().try_into().unwrap(),
            nFont: 0,
            dwFontSize: COORD { X: fontw, Y: fonth },
            FontFamily: FF_DONTCARE.0 as u32,
            FontWeight: FW_NORMAL.0,
            ..Default::default()
        };

        self.set_face_name(&mut font_cfi.FaceName, "Consolas");

        self.set_current_console_font_ex(self.output_handle, false, &font_cfi)?;

        let max_size = unsafe { GetLargestConsoleWindowSize(self.output_handle) };

        if width > max_size.X || height > max_size.Y {
            return Err(format!(
                "Requested console size {}x{} exceeds maximum {}x{} for this display/font.",
                width, height, max_size.X, max_size.Y
            )
            .into());
        }

        let mut screen_buffer_csbi = CONSOLE_SCREEN_BUFFER_INFO::default();
        self.get_console_screen_buffer_info(self.output_handle, &mut screen_buffer_csbi)?;

        self.validate_window_size(&screen_buffer_csbi)?;

        self.rect = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: self.screen_width - 1,
            Bottom: self.screen_height - 1,
        };

        self.set_console_window_info(self.output_handle, true, &self.rect)?;

        self.window_buffer = vec![
            CHAR_INFO::default();
            (self.screen_width as i32 * self.screen_height as i32) as usize
        ];

        self.set_ctrl_handler(Some(console_handler), true)?;

        self.set_console_mode()?;

        self.set_console_cursor_info()?;

        Ok(())
    }

    fn update_keys(&mut self) {
        for i in 0..256 {
            self.key_pressed[i] = false;
            self.key_released[i] = false;

            self.key_new_state[i] = unsafe { GetAsyncKeyState(i as i32) as u16 };

            if self.key_new_state[i] != self.key_old_state[i] {
                if (self.key_new_state[i] & 0x8000) != 0 {
                    self.key_pressed[i] = !self.key_held[i];
                    self.key_held[i] = true;
                } else {
                    self.key_released[i] = true;
                    self.key_held[i] = false;
                }
            }

            self.key_old_state[i] = self.key_new_state[i];
        }
    }

    fn update_mouse(&mut self) {
        let mut events: u32 = 0;
        self.get_number_of_console_input_events(&mut events);
        if events == 0 {
            return;
        }

        let count = events.min(32);
        let mut in_buf = [INPUT_RECORD::default(); 32];
        let mut read = 0;
        self.read_console_input_w(count as usize, &mut in_buf, &mut read);

        for record in &in_buf[..read as usize] {
            match record.EventType as u32 {
                FOCUS_EVENT => unsafe {
                    self.console_in_focus = record.Event.FocusEvent.bSetFocus.as_bool();
                },
                MOUSE_EVENT => {
                    let me = unsafe { record.Event.MouseEvent };
                    match me.dwEventFlags {
                        0 => {
                            for m in 0..5 {
                                self.mouse_new_state[m] = (me.dwButtonState & (1 << m)) != 0;
                            }
                        }
                        MOUSE_MOVED => {
                            self.mouse_x = me.dwMousePosition.X as i32;
                            self.mouse_y = me.dwMousePosition.Y as i32;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        for m in 0..5 {
            self.mouse_pressed[m] = false;
            self.mouse_released[m] = false;

            if self.mouse_new_state[m] != self.mouse_old_state[m] {
                if self.mouse_new_state[m] {
                    self.mouse_pressed[m] = true;
                    self.mouse_held[m] = true;
                } else {
                    self.mouse_released[m] = true;
                    self.mouse_held[m] = false;
                }
            }

            self.mouse_old_state[m] = self.mouse_new_state[m];
        }
    }

    /// Starts the game loop and runs the game until it exits.
    ///
    /// Calls `create()`, `update()`, and `destroy()` on the user's game struct.
    pub fn start(mut self) {
        let mut game = self.game.take().unwrap();

        if !game.create(&mut self) {
            RUNNING.store(false, SeqCst);
        }

        let mut s: [u16; 256] = [0; 256];
        let s_ptr = s.as_mut_ptr();

        let mut tp_1 = Instant::now();

        while RUNNING.load(SeqCst) {
            while RUNNING.load(SeqCst) {
                let tp_2 = Instant::now();
                let elapsed = tp_2.duration_since(tp_1);
                tp_1 = tp_2;

                let elapsed_time = elapsed.as_secs_f32();

                let fps = if elapsed_time > 0.0 {
                    1.0 / elapsed_time
                } else {
                    0.0
                };

                self.update_keys();
                self.update_mouse();

                if !game.update(&mut self, elapsed_time) {
                    RUNNING.store(false, SeqCst);
                }

                unsafe {
                    let mut rect = self.rect;

                    let w_char =
                        format!("Console Game Engine - {} - FPS: {:.2}", self.app_name, fps);
                    let w_string = HSTRING::from(w_char);

                    wsprintfW(PWSTR(s_ptr), PCWSTR(w_string.as_ptr()));

                    self.set_console_title(PCWSTR(s.as_ptr()));

                    self.write_console_output(
                        self.output_handle,
                        self.window_buffer.as_ptr(),
                        COORD {
                            X: self.screen_width,
                            Y: self.screen_height,
                        },
                        COORD { X: 0, Y: 0 },
                        &mut rect,
                    );
                }
            }

            if !game.destroy(&mut self) {
                RUNNING.store(true, SeqCst);
            }
        }
    }
}

impl<G: ConsoleGame> Drop for ConsoleGameEngine<G> {
    fn drop(&mut self) {
        self.original_state
            .restore(self.output_handle, self.input_handle);
    }
}

// endregion

// region: Win API Wrappers

impl<G: ConsoleGame> ConsoleGameEngine<G> {
    fn set_console_window_info(
        &self,
        handle: HANDLE,
        absolute: bool,
        rect: *const SMALL_RECT,
    ) -> windows::core::Result<()> {
        unsafe {
            SetConsoleWindowInfo(handle, absolute, rect)?;
        }
        Ok(())
    }

    fn set_console_screen_buffer_size(
        &self,
        handle: HANDLE,
        size: COORD,
    ) -> windows::core::Result<()> {
        unsafe {
            SetConsoleScreenBufferSize(handle, size)?;
        }
        Ok(())
    }

    fn set_console_active_screen_buffer(&self, handle: HANDLE) -> windows::core::Result<()> {
        unsafe {
            SetConsoleActiveScreenBuffer(handle)?;
        }
        Ok(())
    }

    fn set_current_console_font_ex(
        &self,
        handle: HANDLE,
        max_window: bool,
        font: *const CONSOLE_FONT_INFOEX,
    ) -> windows::core::Result<()> {
        unsafe {
            SetCurrentConsoleFontEx(handle, max_window, font)?;
        }
        Ok(())
    }

    fn get_console_screen_buffer_info(
        &self,
        handle: HANDLE,
        buffer: *mut CONSOLE_SCREEN_BUFFER_INFO,
    ) -> windows::core::Result<()> {
        unsafe {
            GetConsoleScreenBufferInfo(handle, buffer)?;
        }
        Ok(())
    }

    fn set_ctrl_handler(&self, routine: PHANDLER_ROUTINE, add: bool) -> windows::core::Result<()> {
        unsafe {
            SetConsoleCtrlHandler(routine, add)?;
        }
        Ok(())
    }

    fn set_face_name(&self, face_name_field: &mut [u16], value: &str) {
        let wide: Vec<u16> = value.encode_utf16().chain(Some(0)).collect();
        let len = wide.len().min(face_name_field.len());
        face_name_field[..len].copy_from_slice(&wide[..len]);
    }

    fn validate_window_size(&self, buffer: &CONSOLE_SCREEN_BUFFER_INFO) -> Result<(), String> {
        if self.screen_height > buffer.dwMaximumWindowSize.Y {
            return Err("Screen height or font height too big".into());
        }
        if self.screen_width > buffer.dwMaximumWindowSize.X {
            return Err("Screen width or font width too big".into());
        }
        Ok(())
    }

    fn set_console_title(&self, title: PCWSTR) {
        unsafe {
            SetConsoleTitleW(title).unwrap_or_else(|e| {
                eprintln!("SetConsoleTitleW Failed: {:?}", e);
                exit(1);
            });
        }
    }

    fn write_console_output(
        &self,
        handle: HANDLE,
        buffer: *const CHAR_INFO,
        buffer_size: COORD,
        buffer_coord: COORD,
        write_region: *mut SMALL_RECT,
    ) {
        unsafe {
            WriteConsoleOutputW(handle, buffer, buffer_size, buffer_coord, write_region)
                .unwrap_or_else(|e| {
                    eprintln!("WriteConsoleOutputW Failed: {:?}", e);
                    exit(1);
                });
        }
    }

    fn set_console_mode(&self) -> windows::core::Result<()> {
        unsafe {
            let mut mode = CONSOLE_MODE(0);
            GetConsoleMode(self.input_handle, &mut mode)?;

            mode &= !ENABLE_QUICK_EDIT_MODE;
            mode |= ENABLE_EXTENDED_FLAGS | ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT;

            SetConsoleMode(self.input_handle, mode)?;
        }
        Ok(())
    }

    fn set_console_cursor_info(&self) -> windows::core::Result<()> {
        unsafe {
            let info = CONSOLE_CURSOR_INFO {
                dwSize: 1,
                bVisible: FALSE,
            };
            SetConsoleCursorInfo(self.output_handle, &info)?;
        }
        Ok(())
    }

    fn get_number_of_console_input_events(&self, num_events: &mut u32) {
        unsafe {
            GetNumberOfConsoleInputEvents(self.input_handle, num_events).unwrap_or_else(|e| {
                eprintln!("GetNumberOfConsoleInputEvents Failed: {:?}", e);
                exit(1);
            })
        };
    }

    fn read_console_input_w(
        &self,
        count: usize,
        buffer: &mut [INPUT_RECORD],
        num_events: &mut u32,
    ) {
        unsafe {
            ReadConsoleInputW(self.input_handle, &mut buffer[..count], num_events).unwrap_or_else(
                |e| {
                    eprintln!("ReadConsoleInputW Failed: {:?}", e);
                    exit(1);
                },
            );
        }
    }
}

// endregion

// region: Drawing

impl<G: ConsoleGame> ConsoleGameEngine<G> {
    /// Clamps `x` and `y` to be within the screen boundaries.
    pub fn clip(&self, x: &mut i32, y: &mut i32) {
        if *x < 0 {
            *x = 0
        };
        if *x >= self.screen_width() {
            *x = self.screen_width()
        };
        if *y < 0 {
            *y = 0
        };
        if *y >= self.screen_height() {
            *y = self.screen_height()
        };
    }

    /// Draws a single white pixel at `(x, y)`.
    pub fn draw(&mut self, x: i32, y: i32) {
        self.draw_with(x, y, PIXEL_SOLID, FG_WHITE);
    }

    /// Draws a single pixel at `(x, y)` with the specified glyph and color.
    pub fn draw_with(&mut self, x: i32, y: i32, c: u16, col: u16) {
        if x >= 0 && x < self.screen_width as i32 && y >= 0 && y < self.screen_height as i32 {
            let idx = (y * self.screen_width as i32 + x) as usize;
            self.window_buffer[idx].Char.UnicodeChar = c;
            self.window_buffer[idx].Attributes = col;
        }
    }

    /// Clears the entire screen with the given color.
    pub fn clear(&mut self, col: u16) {
        self.fill_rect_with(
            0,
            0,
            self.screen_width(),
            self.screen_height(),
            PIXEL_EMPTY,
            col,
        );
    }

    /// Draws a string of white text starting at `(x, y)`.
    pub fn draw_string(&mut self, x: i32, y: i32, text: &str) {
        self.draw_string_with(x, y, text, FG_WHITE);
    }

    /// Draws a string starting at `(x, y)` with the specified color.
    pub fn draw_string_with(&mut self, x: i32, y: i32, text: &str, col: u16) {
        for (i, ch) in text.encode_utf16().enumerate() {
            let idx = (y as usize) * self.screen_width as usize + (x as usize + i);
            self.window_buffer[idx].Char.UnicodeChar = ch;
            self.window_buffer[idx].Attributes = col;
        }
    }

    /// Draws a string at `(x, y)` ignoring spaces (transparent spaces).
    pub fn draw_string_alpha(&mut self, x: i32, y: i32, text: &str) {
        self.draw_string_alpha_with(x, y, text, FG_WHITE);
    }

    /// Draws a string at `(x, y)` ignoring spaces (transparent spaces), using the specified color.
    pub fn draw_string_alpha_with(&mut self, x: i32, y: i32, text: &str, col: u16) {
        for (i, ch) in text.encode_utf16().enumerate() {
            if ch != ' ' as u16 {
                let idx = (y as usize) * self.screen_width as usize + (x as usize + i);
                self.window_buffer[idx].Char.UnicodeChar = ch;
                self.window_buffer[idx].Attributes = col;
            }
        }
    }

    /// Draws a white line from `(x1, y1)` to `(x2, y2)`.
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.draw_line_with(x1, y1, x2, y2, PIXEL_SOLID, FG_WHITE);
    }

    /// Draws a line from `(x1, y1)` to `(x2, y2)` with the specified glyph and color.
    pub fn draw_line_with(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, c: u16, col: u16) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;

        if dy1 <= dx1 {
            let (mut x, mut y, xe) = if dx >= 0 { (x1, y1, x2) } else { (x2, y2, x1) };
            self.draw_with(x, y, c, col);

            while x < xe {
                x += 1;
                if px < 0 {
                    px += 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                    px += 2 * (dy1 - dx1);
                }
                self.draw_with(x, y, c, col);
            }
        } else {
            let (mut x, mut y, ye) = if dy >= 0 { (x1, y1, y2) } else { (x2, y2, y1) };
            self.draw_with(x, y, c, col);

            while y < ye {
                y += 1;
                if py <= 0 {
                    py += 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    py += 2 * (dx1 - dy1);
                }
                self.draw_with(x, y, c, col);
            }
        }
    }

    /// Draws a white triangle connecting three points.
    pub fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
        self.draw_triangle_with(x1, y1, x2, y2, x3, y3, PIXEL_SOLID, FG_WHITE);
    }

    /// Draws a triangle connecting three points with the specified glyph and color.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_triangle_with(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        c: u16,
        col: u16,
    ) {
        self.draw_line_with(x1, y1, x2, y2, c, col);
        self.draw_line_with(x2, y2, x3, y3, c, col);
        self.draw_line_with(x3, y3, x1, y1, c, col);
    }

    /// Fills a triangle connecting three points with white pixels.
    pub fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
        self.fill_triangle_with(x1, y1, x2, y2, x3, y3, PIXEL_SOLID, FG_WHITE);
    }

    /// Fills a triangle connecting three points with the specified glyph and color.
    #[allow(clippy::too_many_arguments)]
    pub fn fill_triangle_with(
        &mut self,
        mut x1: i32,
        mut y1: i32,
        mut x2: i32,
        mut y2: i32,
        mut x3: i32,
        mut y3: i32,
        c: u16,
        col: u16,
    ) {
        use std::mem::swap;

        let draw_line = |engine: &mut Self, sx: i32, ex: i32, y: i32| {
            for i in sx..=ex {
                engine.draw_with(i, y, c, col);
            }
        };

        if y1 > y2 {
            swap(&mut y1, &mut y2);
            swap(&mut x1, &mut x2);
        }
        if y1 > y3 {
            swap(&mut y1, &mut y3);
            swap(&mut x1, &mut x3);
        }
        if y2 > y3 {
            swap(&mut y2, &mut y3);
            swap(&mut x2, &mut x3);
        }

        let mut t1x = x1;
        let mut t2x = x1;
        let mut y = y1;

        let mut dx1 = x2 - x1;
        let mut dy1 = y2 - y1;
        let signx1 = if dx1 < 0 { -1 } else { 1 };
        dx1 = dx1.abs();

        let dx2 = x3 - x1;
        let dy2 = y3 - y1;
        let signx2 = if dx2 < 0 { -1 } else { 1 };
        let dx2 = dx2.abs();

        let changed1 = dy1 > dx1;
        let changed2 = dy2 > dx2;

        if y1 != y2 {
            let mut i = 0;
            while i < dx1 {
                let minx = t1x.min(t2x);
                let maxx = t1x.max(t2x);
                draw_line(self, minx, maxx, y);

                y += 1;
                i += 1;
                if changed1 {
                    t1x += signx1;
                }
                if changed2 {
                    t2x += signx2;
                }
            }
        }

        dx1 = (x3 - x2).abs();
        dy1 = y3 - y2;
        let signx1 = if x3 - x2 < 0 { -1 } else { 1 };
        t1x = x2;
        let changed1 = dy1 > dx1;

        while y <= y3 {
            let minx = t1x.min(t2x);
            let maxx = t1x.max(t2x);
            draw_line(self, minx, maxx, y);

            y += 1;
            if changed1 {
                t1x += signx1;
            }
            if changed2 {
                t2x += signx2;
            }

            if y > y3 {
                break;
            }
        }
    }

    /// Draws a white rectangle at `(x, y)` with width `w` and height `h`.
    pub fn draw_rectangle(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.draw_rectangle_with(x, y, w, h, PIXEL_SOLID, FG_WHITE);
    }

    /// Draws a rectangle at `(x, y)` with width `w` and height `h` using the specified glyph and color.
    pub fn draw_rectangle_with(&mut self, x: i32, y: i32, w: i32, h: i32, c: u16, col: u16) {
        if w <= 0 || h <= 0 {
            return;
        }

        self.draw_line_with(x, y, x + w - 1, y, c, col);
        self.draw_line_with(x, y + h - 1, x + w - 1, y + h - 1, c, col);
        self.draw_line_with(x, y, x, y + h - 1, c, col);
        self.draw_line_with(x + w - 1, y, x + w - 1, y + h - 1, c, col);
    }

    /// Fills a rectangle from `(x1, y1)` to `(x2, y2)` with white pixels.
    pub fn fill_rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.fill_rect_with(x1, y1, x2, y2, PIXEL_SOLID, FG_WHITE);
    }

    /// Fills a rectangle from `(x1, y1)` to `(x2, y2)` with the specified glyph and color.
    pub fn fill_rect_with(
        &mut self,
        mut x1: i32,
        mut y1: i32,
        mut x2: i32,
        mut y2: i32,
        c: u16,
        col: u16,
    ) {
        self.clip(&mut x1, &mut y1);
        self.clip(&mut x2, &mut y2);

        for x in x1..x2 {
            for y in y1..y2 {
                self.draw_with(x, y, c, col);
            }
        }
    }

    /// Draws a white circle centered at `(xc, yc)` with radius `r`.
    pub fn draw_circle(&mut self, xc: i32, yc: i32, r: i32) {
        self.draw_circle_with(xc, yc, r, PIXEL_SOLID, FG_WHITE);
    }

    /// Draws a circle centered at `(xc, yc)` with radius `r` using the specified glyph and color.
    pub fn draw_circle_with(&mut self, xc: i32, yc: i32, r: i32, c: u16, col: u16) {
        if r == 0 {
            return;
        }
        let mut x = 0;
        let mut y = r;
        let mut p = 3 - 2 * r;

        while y >= x {
            self.draw_with(xc - x, yc - y, c, col);
            self.draw_with(xc - y, yc - x, c, col);
            self.draw_with(xc + y, yc - x, c, col);
            self.draw_with(xc + x, yc - y, c, col);
            self.draw_with(xc - x, yc + y, c, col);
            self.draw_with(xc - y, yc + x, c, col);
            self.draw_with(xc + y, yc + x, c, col);
            self.draw_with(xc + x, yc + y, c, col);

            if p < 0 {
                p += 4 * x + 6;
            } else {
                p += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }

    /// Fills a circle centered at `(xc, yc)` with white pixels and radius `r`.
    pub fn fill_circle(&mut self, xc: i32, yc: i32, r: i32) {
        self.fill_circle_with(xc, yc, r, PIXEL_SOLID, FG_WHITE);
    }

    /// Fills a circle centered at `(xc, yc)` with radius `r` using the specified glyph and color.
    pub fn fill_circle_with(&mut self, xc: i32, yc: i32, r: i32, c: u16, col: u16) {
        if r == 0 {
            return;
        }
        let mut x = 0;
        let mut y = r;
        let mut p = 3 - 2 * r;

        let draw_line = |engine: &mut Self, sx: i32, ex: i32, ny: i32| {
            for i in sx..=ex {
                engine.draw_with(i, ny, c, col);
            }
        };

        while y >= x {
            draw_line(self, xc - x, xc + x, yc - y);
            draw_line(self, xc - y, xc + y, yc - x);
            draw_line(self, xc - x, xc + x, yc + y);
            draw_line(self, xc - y, xc + y, yc + x);

            if p < 0 {
                p += 4 * x + 6;
            } else {
                p += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }

    /// Draws a 2D wireframe model at a given position, rotation, and scale.
    ///
    /// # Parameters
    /// - `model_coords`: A slice of `(x, y)` coordinates representing the vertices of the model.
    /// - `x`, `y`: The position on the screen to draw the model (translation applied to all vertices).
    /// - `r`: Rotation in radians, applied around the origin of the model coordinates.
    /// - `s`: Scale factor applied to the model.
    /// - `col`: Color used to draw the lines.
    /// - `c`:  glyph used to draw the lines
    #[allow(clippy::too_many_arguments)]
    pub fn draw_wireframe_model(
        &mut self,
        model_coords: &[(f32, f32)],
        x: f32,
        y: f32,
        r: f32,
        s: f32,
        col: u16,
        c: u16,
    ) {
        let verts = model_coords.len();
        let mut transformed: Vec<(f32, f32)> = vec![(0.0, 0.0); verts];

        for i in 0..verts {
            let (px, py) = model_coords[i];
            transformed[i].0 = px * r.cos() - py * r.sin();
            transformed[i].1 = px * r.sin() + py * r.cos();
        }

        for t in &mut transformed {
            t.0 *= s;
            t.1 *= s;
        }

        for t in &mut transformed {
            t.0 += x;
            t.1 += y;
        }

        for i in 0..verts {
            let j = (i + 1) % verts;
            self.draw_line_with(
                transformed[i].0 as i32,
                transformed[i].1 as i32,
                transformed[j].0 as i32,
                transformed[j].1 as i32,
                c,
                col,
            );
        }
    }

    /// Draws a filled 2D model at a given position, rotation, and scale.
    /// Works for concave and convex polygons (even-odd fill rule).
    ///
    /// # Parameters
    /// - `model_coords`: A slice of `(x, y)` coordinates representing the vertices of the model.
    /// - `x`, `y`: The position on the screen to draw the model (translation applied to all vertices).
    /// - `r`: Rotation in radians, applied around the origin of the model coordinates.
    /// - `s`: Scale factor applied to the model.
    /// - `col`: Color used to draw the filled pixels.
    /// - `c`: Glyph used to draw the filled pixels.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_filled_model(
        &mut self,
        model_coords: &[(f32, f32)],
        x: f32,
        y: f32,
        r: f32,
        s: f32,
        col: u16,
        c: u16,
    ) {
        let verts = model_coords.len();
        if verts < 3 {
            return;
        }

        let cos_r = r.cos();
        let sin_r = r.sin();
        let mut transformed: Vec<(f32, f32)> = Vec::with_capacity(verts);
        for &(px, py) in model_coords {
            let tx = px * cos_r - py * sin_r;
            let ty = px * sin_r + py * cos_r;
            transformed.push((tx * s + x, ty * s + y));
        }

        let min_yf = transformed
            .iter()
            .map(|t| t.1)
            .fold(f32::INFINITY, |a, b| a.min(b));
        let max_yf = transformed
            .iter()
            .map(|t| t.1)
            .fold(f32::NEG_INFINITY, |a, b| a.max(b));
        let y_start = min_yf.floor() as i32;
        let y_end = max_yf.ceil() as i32;

        for y_scan in y_start..=y_end {
            let sample_y = y_scan as f32 + 0.5;
            let mut intersects: Vec<f32> = Vec::new();

            for i in 0..verts {
                let (x1, y1) = transformed[i];
                let (x2, y2) = transformed[(i + 1) % verts];

                if (y1 - y2).abs() < f32::EPSILON {
                    continue;
                }

                let (ymin, ymax, x_a, y_a, x_b, y_b) = if y1 < y2 {
                    (y1, y2, x1, y1, x2, y2)
                } else {
                    (y2, y1, x2, y2, x1, y1)
                };

                if sample_y >= ymin && sample_y < ymax {
                    let t = (sample_y - y_a) / (y_b - y_a);
                    let xi = x_a + t * (x_b - x_a);
                    intersects.push(xi);
                }
            }

            if intersects.is_empty() {
                continue;
            }

            intersects.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            let mut k = 0usize;
            while k + 1 < intersects.len() {
                let x_left_f = intersects[k];
                let x_right_f = intersects[k + 1];

                let x_start = x_left_f.ceil() as i32;
                let x_end = x_right_f.floor() as i32;

                if x_end >= x_start {
                    for xi in x_start..=x_end {
                        self.draw_with(xi, y_scan, c, col);
                    }
                }

                k += 2;
            }
        }
    }

    /// Draws a sprite at position `(x, y)`.
    pub fn draw_sprite(&mut self, x: i32, y: i32, sprite: &Sprite) {
        for i in 0..sprite.width {
            for j in 0..sprite.height {
                let glyph = sprite.get_glyph(i, j);
                if glyph != PIXEL_EMPTY {
                    let color = sprite.get_color(i, j);
                    self.draw_with(x + i as i32, y + j as i32, glyph, color);
                }
            }
        }
    }

    /// Draws a portion of a sprite at position `(x, y)` on the screen.
    ///
    /// # Parameters
    /// - `x`, `y`: The top-left coordinates on the screen where the sprite portion will be drawn.
    /// - `sprite`: The `Sprite` to draw.
    /// - `ox`, `oy`: The top-left coordinates inside the sprite to start copying from (offset within the sprite).
    /// - `w`, `h`: The width and height of the portion to draw (how much of the sprite to copy).
    #[allow(clippy::too_many_arguments)]
    pub fn draw_partial_sprite(
        &mut self,
        x: i32,
        y: i32,
        sprite: &Sprite,
        ox: usize,
        oy: usize,
        w: usize,
        h: usize,
    ) {
        for i in 0..w {
            for j in 0..h {
                let glyph = sprite.get_glyph(i + ox, j + oy);
                if glyph != PIXEL_EMPTY {
                    let color = sprite.get_color(i + ox, j + oy);
                    self.draw_with(x + i as i32, y + j as i32, glyph, color);
                }
            }
        }
    }
}

// endregion

// endregion
