use sdl2_sys::SDL_Scancode;

use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub aimbot: bool,
    pub aimbot_draw_fov: bool,
    pub aimbot_fov: f32,
    pub aimbot_key: SDL_Scancode,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            aimbot: false,
            aimbot_draw_fov: false,
            aimbot_fov: 30f32,
            aimbot_key: SDL_Scancode::SDL_SCANCODE_LSHIFT,
        }
    }
}
