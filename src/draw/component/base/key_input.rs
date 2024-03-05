use std::{
    sync::{Arc, Mutex},
    usize,
};

use sdl2_sys::*;

use crate::*;

const SIZE: isize = FontSize::Small as isize + 4;

#[derive(Debug)]
pub struct KeyInput {
    x: isize,
    y: isize,
    w: isize,
    rooted_x: isize,
    rooted_y: isize,
    cursor: (isize, isize),
    val: Arc<Mutex<SDL_Scancode>>,
    focussed: bool,
}

impl KeyInput {
    pub fn new(x: isize, y: isize, w: isize, val: Arc<Mutex<SDL_Scancode>>) -> KeyInput {
        KeyInput {
            x,
            y,
            w,
            rooted_x: 0,
            rooted_y: 0,
            cursor: (0, 0),
            val,
            focussed: false,
        }
    }
}

impl RawComponent for KeyInput {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let x = self.x + root_x;
        let y = self.y + root_y;
        self.rooted_x = x;
        self.rooted_y = y;
        frame.filled_rect(x, y, self.w, SIZE, BACKGROUND, 255);
        let outline = if self.focussed { BLUE } else { FOREGROUND };
        frame.outlined_rect(x, y, self.w, SIZE, outline, 255);

        let val = *self.val.lock().unwrap();

        frame.text(
            &sdl_scancode_to_string(val),
            x + self.w / 2,
            y + SIZE / 2,
            FontSize::Small,
            true,
            FOREGROUND,
            255,
        );
    }

    fn handle_event(&mut self, event: *mut sdl2_sys::SDL_Event) {
        unsafe {
            match transmute::<u32, SDL_EventType>((*event).type_) {
                SDL_EventType::SDL_MOUSEBUTTONDOWN => {
                    if self.rooted_x <= self.cursor.0
                        && self.cursor.0 <= self.rooted_x + self.w
                        && self.rooted_y <= self.cursor.1
                        && self.cursor.1 <= self.rooted_y + SIZE
                    {
                        self.focussed = true;
                        (*event).type_ = 0;
                    } else {
                        self.focussed = false;
                    }
                }
                SDL_EventType::SDL_MOUSEMOTION => {
                    let motion = (*event).motion;
                    self.cursor = (motion.x as isize, motion.y as isize);
                }
                SDL_EventType::SDL_KEYDOWN => {
                    if !self.focussed {
                        return;
                    }
                    let key = (*event).key.keysym.scancode;
                     *self.val.lock().unwrap() = key;
                    self.focussed = false;
                    (*event).type_ = 0;
                }
                _ => (),
            };
        }
    }
}

impl Component for KeyInput {}
