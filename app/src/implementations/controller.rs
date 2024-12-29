use chip8_lib::interfaces::Controller;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;

const KEYS: [Scancode; 16] = [
    Scancode::Num1,
    Scancode::Num2,
    Scancode::Num3,
    Scancode::Num4,
    Scancode::Q,
    Scancode::W,
    Scancode::E,
    Scancode::R,
    Scancode::A,
    Scancode::S,
    Scancode::D,
    Scancode::F,
    Scancode::Z,
    Scancode::X,
    Scancode::C,
    Scancode::V,
];

pub struct SdlController {
    key_down: Vec<bool>,
}

impl SdlController {
    pub fn new() -> Self {
        Self {
            key_down: vec![false; 16],
        }
    }

    pub fn set_keys_state(&mut self, keyboard_state: KeyboardState) {
        for i in 0..16 {
            self.key_down[i] = keyboard_state.is_scancode_pressed(KEYS[i]);
        }
    }
}

impl Controller for SdlController {
    fn is_key_down(&self, key_index: usize) -> bool {
        self.key_down[key_index & 0xF]
    }
}
