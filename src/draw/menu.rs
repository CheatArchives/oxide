use std::f32::consts::PI;

use sdl2_sys::{SDL_Event, SDL_EventType, SDL_Scancode, SDL_Window};

use crate::*;

#[derive(Debug, Clone)]
pub struct Menu {
    pub is_menu_visible: bool,
    pub aimbot_checkbox: Checkbox,
    pub third_person_checkbox: Checkbox,
    pub bhop_checkbox: Checkbox,
    pub x: isize,
    pub y: isize,
}
impl Menu {
    pub unsafe fn init() -> Menu {
        Menu {
            is_menu_visible: true,
            aimbot_checkbox: Checkbox::new("aimbot", 10, 10),
            third_person_checkbox: Checkbox::new("third person", 10, 30),
            bhop_checkbox: Checkbox::new("bhop", 10, 50),
            x: 100,
            y: 100,
        }
    }


    pub unsafe fn handle_event(&mut self, event: *mut SDL_Event) {
        self.aimbot_checkbox.handle_event(event);
        self.third_person_checkbox.handle_event(event);
        self.bhop_checkbox.handle_event(event);
        match transmute::<u32, SDL_EventType>((*event).type_) {
            SDL_EventType::SDL_KEYUP => {
                let key = (*event).key.keysym.scancode;
                match key {
                    SDL_Scancode::SDL_SCANCODE_INSERT => {
                        self.is_menu_visible = !self.is_menu_visible;
                    }
                    _ => (),
                }
            }
            SDL_EventType::SDL_MOUSEMOTION => {
                let motion = (*event).motion;
            }
            _ => (),
        };
    }
}
