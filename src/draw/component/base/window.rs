use std::{
    sync::{Arc, Mutex},
    usize,
};

use sdl2_sys::*;

use crate::*;

const HEADER_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct Window {
    x: isize,
    y: isize,
    w: isize,
    h: isize,
    title: String,
    last_cursor: (isize, isize),
    pub visible: Arc<Mutex<bool>>,
    dragging: bool,
    components: Components,
}

impl Window {
    pub fn new(
        x: isize,
        y: isize,
        title: String,
        visible: Arc<Mutex<bool>>,
        components: Components,
    ) -> Window {
        Window {
            x,
            y,
            w: 500,
            h: 500,
            title,
            last_cursor: (0, 0),
            visible,
            dragging: false,
            components,
        }
    }
}

impl RawComponent for Window {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        if *self.visible.lock().unwrap() {
            frame.filled_rect(self.x, self.y, self.w, HEADER_HEIGHT, BACKGROUND, 255);
            frame.filled_rect(
                self.x,
                self.y + HEADER_HEIGHT,
                self.w,
                self.h - HEADER_HEIGHT,
                BACKGROUND,
                220,
            );

            frame.text(
                &self.title,
                self.x + self.w / 2,
                self.y + HEADER_HEIGHT / 2,
                FontSize::Medium,
                true,
                FOREGROUND,
                255,
            );

            frame.filled_rect(self.x, self.y + HEADER_HEIGHT, self.w, 1, CURSOR, 100);
            frame.outlined_rect(self.x, self.y, self.w, self.h, CURSOR, 255);

            self.components.draw(frame, self.x, self.y + HEADER_HEIGHT);
        }
    }

    fn handle_event(&mut self, event: *mut sdl2_sys::SDL_Event) {
        self.components.handle_event(event);
        unsafe {
            match transmute::<u32, SDL_EventType>((*event).type_) {
                SDL_EventType::SDL_MOUSEBUTTONDOWN => {
                    if self.x <= self.last_cursor.0
                        && self.last_cursor.0 <= self.x + self.w
                        && self.y <= self.last_cursor.1
                        && self.last_cursor.1 <= self.y + self.h
                        && *self.visible.lock().unwrap()
                    {
                        self.dragging = true;
                        (*event).type_ = 0;
                    }
                }
                SDL_EventType::SDL_MOUSEBUTTONUP => {
                    self.dragging = false;
                }
                SDL_EventType::SDL_MOUSEMOTION => {
                    let motion = (*event).motion;
                    if self.dragging {
                        self.x += motion.x as isize - self.last_cursor.0;
                        self.y += motion.y as isize - self.last_cursor.1;
                    }
                    self.last_cursor = (motion.x as isize, motion.y as isize);
                }
                _ => (),
            };
        }
    }
}

impl Component for Window {}
