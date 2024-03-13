use std::sync::{Arc, Mutex};

use sdl2_sys::SDL_Scancode;

use crate::{am, amt};


#[derive(Debug, Clone)]
pub struct Settings {
    pub aimbot: AimbotSettings,
    pub visual: VisualSettings,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            aimbot: AimbotSettings::new(),
            visual: VisualSettings::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AimbotSettings {
    pub enabled: amt!(bool),
    pub draw_fov: amt!(bool),
    pub fov: amt!(f32),
    pub key: amt!(SDL_Scancode),
    pub multipoint: amt!(bool),
}

impl AimbotSettings {
    pub fn new() -> AimbotSettings {
        AimbotSettings {
            multipoint: am!(false),
            enabled: am!(false),
            draw_fov: am!(false),
            fov: am!(30f32),
            key: am!(SDL_Scancode::SDL_SCANCODE_LSHIFT),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VisualSettings {
    pub third_person: amt!(bool),
    pub fov: amt!(f32),
}

impl VisualSettings {
    pub fn new() -> VisualSettings {
        VisualSettings {
            third_person: am!(false),
            fov: am!(100f32),
        }
    }
}
