use std::{
    sync::{Arc, Mutex},
    usize,
};

use sdl2_sys::*;

use crate::*;

#[derive(Debug)]
pub struct Button {
    x: isize,
    y: isize,
    w: isize,
    h: isize,
    cursor: (isize, isize),
    val: Arc<Mutex<bool>>,
}

impl Button {
    pub fn new(x: isize, y: isize, w: isize, h: isize, val: Arc<Mutex<bool>>) -> Button {
        Button {
            x,
            y,
            w,
            h,
            cursor: (0, 0),
            val,
        }
    }
}

impl RawComponent for Button {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        frame.filled_rect(self.x, self.y, self.w, self.h, CURSOR_TEXT, 255);
        frame.outlined_rect(self.x, self.y, self.w, self.h, CURSOR, 255);
        frame.text(
            "AIMBOT",
            self.x + self.w / 2,
            self.y + self.h / 2,
            FontSize::Medium,
            true,
            FOREGROUND,
            255,
        );
    }

    fn handle_event(&mut self, event: *mut sdl2_sys::SDL_Event) {
        unsafe {
            match transmute::<u32, SDL_EventType>((*event).type_) {
                SDL_EventType::SDL_MOUSEBUTTONDOWN => {
                    if self.x <= self.cursor.0
                        && self.cursor.0 <= self.x + self.w
                        && self.y <= self.cursor.1
                        && self.cursor.1 <= self.y + self.h
                    {
                        let mut val = self.val.lock().unwrap();
                        *val= !*val;
                    }
                }
                SDL_EventType::SDL_MOUSEMOTION => {
                    let motion = (*event).motion;
                    self.cursor = (motion.x as isize, motion.y as isize);
                }
                _ => (),
            };
        }
    }
}

impl Component for Button {}
