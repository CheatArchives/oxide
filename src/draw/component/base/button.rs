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
    val: Arc<Mutex<bool>>,
    text: String,
    size: FontSize,
}

impl Button {
    pub fn new(
        text: &str,
        x: isize,
        y: isize,
        w: isize,
        h: isize,
        val: Arc<Mutex<bool>>,
        size: FontSize,
    ) -> Button {
        Button {
            x,
            y,
            w,
            h,
            val,
            text: text.to_owned(),
            size,
        }
    }
}

impl RawComponent for Button {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let x = self.x + root_x;
        let y = self.y + root_y;
        frame.filled_rect(x, y, self.w, self.h, CURSOR_TEXT, 255);
        frame.outlined_rect(x, y, self.w, self.h, CURSOR, 255);
        frame.text(
            &self.text,
            x + self.w / 2 - 1,
            y + self.h / 2 + 1,
            self.size.clone(),
            true,
            FOREGROUND,
            255,
        );
    }

    fn handle_event(&mut self, mut event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown => {
                if point_in_bounds(
                    draw!().cursor.0,
                    draw!().cursor.1,
                    self.x,
                    self.y,
                    self.w,
                    self.h,
                ) {
                    let mut val = self.val.lock().unwrap();
                    *val = !*val;
                    event.handled = true;
                }
            }
            _ => (),
        }
    }
}

impl Component for Button {}
