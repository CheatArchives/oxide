use std::{
    io::Cursor,
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

    fn handle_event(&mut self, mut event: &mut Event) {
        if !*self.visible.lock().unwrap() {
            return;
        }

        self.components.handle_event(event);
        match event.r#type {
            EventType::CursorMove(pos) => {
                if self.dragging {
                    self.x += pos.0 as isize - self.last_cursor.0;
                    self.y += pos.1 as isize - self.last_cursor.1;
                }
            }
            EventType::MouseButtonDown => {
                if point_in_bounds(
                    draw!().cursor.0,
                    draw!().cursor.1,
                    self.x,
                    self.y,
                    self.w,
                    HEADER_HEIGHT,
                ) {
                    self.dragging = true;
                }
                if point_in_bounds(
                    draw!().cursor.0,
                    draw!().cursor.1,
                    self.x,
                    self.y,
                    self.w,
                    self.h,
                ) {
                    event.handled = true;
                }
            }
            EventType::MouseButtonUp => {
                self.dragging = false;
            }
            _ => (),
        }
        self.last_cursor = draw!().cursor;
    }
}

impl Component for Window {}
