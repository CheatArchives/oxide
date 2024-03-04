use sdl2_sys::*;

use crate::*;

#[derive(Debug)]
pub struct Overlay {
    pub visible: bool,
}

impl Overlay {
    pub fn new() -> Overlay {
        Overlay { visible: true }
    }
}

impl Component for Overlay {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        if !self.visible {
            return;
        }
        let size = frame.window_size;
        frame.filled_rect(0, 0, size.0, 50, BACKGROUND, 220);
        frame.filled_rect(0, 50, 300, size.1, BACKGROUND, 220);
        frame.text(NAME,  size.0/2, 25, FontSize::Large, FOREGROUND, 255)
    }

    fn handle_event(&mut self, event: *mut sdl2_sys::SDL_Event) {
        unsafe {
            match transmute::<u32, SDL_EventType>((*event).type_) {
                SDL_EventType::SDL_KEYUP => {
                    let key = (*event).key.keysym.scancode;
                    match key {
                        SDL_Scancode::SDL_SCANCODE_INSERT => {
                            self.visible = !self.visible;
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
}

impl ComponentDebug for Overlay {}
