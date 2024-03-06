use std::{
    isize,
    mem::MaybeUninit,
    ptr::null,
    sync::{Arc, Mutex},
};

const SIZE: isize = 12;

use crate::*;
use sdl2_sys::*;

#[derive(Debug, Clone)]
pub struct Checkbox {
    pub checked: Arc<Mutex<bool>>,
    x: isize,
    y: isize,
    rooted_x: isize,
    rooted_y: isize,
    motion: (isize, isize),
    text: &'static str,
}
impl Checkbox {
    pub fn new(text: &'static str, checked: Arc<Mutex<bool>>, x: isize, y: isize) -> Checkbox {
        Checkbox {
            checked,
            x,
            y,
            rooted_x: 0,
            rooted_y: 0,
            motion: (0, 0),
            text,
        }
    }
}
impl RawComponent for Checkbox {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        self.rooted_x = (root_x + self.x);
        self.rooted_y = (root_y + self.y);
        frame.filled_rect(self.rooted_x, self.rooted_y, SIZE, SIZE, FOREGROUND, 255);
        if !*self.checked.lock().unwrap() {
            frame.filled_rect(
                self.rooted_x + 1,
                self.rooted_y + 1,
                SIZE - 2,
                SIZE - 2,
                BACKGROUND,
                255,
            );
        }
        frame.text(
            self.text,
            self.rooted_x + SIZE + 10,
            self.rooted_y + SIZE / 2,
            FontSize::Small,
            false,
            FOREGROUND,
            255,
        );
    }

    fn handle_event(&mut self, mut event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown => {
                if draw!().cursor.0 as isize <= self.rooted_x + SIZE
                    && self.rooted_x <= draw!().cursor.0 as isize
                    && draw!().cursor.1 as isize <= self.rooted_y + SIZE
                    && self.rooted_y <= draw!().cursor.1 as isize
                {
                    let mut checked = self.checked.lock().unwrap();
                    *checked = !*checked;
                    event.handled = true;
                }
            }
            _ => (),
        }
    }
}
impl Component for Checkbox {}
