use std::{isize, mem::MaybeUninit, ptr::null};

use crate::*;
use sdl2_sys::*;

#[derive(Debug, Clone)]
pub struct Checkbox {
    pub checked: bool,
    x: isize,
    y: isize,
    rooted_x: isize,
    rooted_y: isize,
    motion: (isize, isize),
    text: &'static str,
}
impl Checkbox {
    pub fn new(text: &'static str, x: isize, y: isize) -> Checkbox {
        Checkbox {
            checked: false,
            x,
            y,
            rooted_x: 0,
            rooted_y: 0,
            motion: (0, 0),
            text,
        }
    }
}
impl Component for Checkbox {
    fn draw(&mut self, draw: &mut Draw, root_x: isize, root_y: isize) {
        self.rooted_x = (root_x + self.x);
        self.rooted_y = (root_y + self.y);
        let rect = SDL_Rect {
            x: self.rooted_x as i32,
            y: self.rooted_y as i32,
            w: 12,
            h: 12,
        };
        draw.filled_rect(rect, DBLUE, 255);
        if self.checked {
            let rect = SDL_Rect {
                x: self.rooted_x as i32 + 1,
                y: self.rooted_y as i32 + 1,
                w: 10,
                h: 10,
            };
            draw.filled_rect(rect, LBLUE, 255);
        }
        draw.draw_text(
            self.text,
            self.rooted_x + 20,
            self.rooted_y,
            FontSize::Small,
            ORANGE,
        );
    }

    fn handle_event(&mut self, event: *mut SDL_Event) {
        unsafe {
            match transmute::<u32, SDL_EventType>((*event).type_) {
                SDL_EventType::SDL_MOUSEBUTTONDOWN => {
                    let (x, y) = self.motion;
                    if x as isize <= self.rooted_x + 10
                        && self.rooted_x <= x as isize
                        && y as isize <= self.rooted_y + 10
                        && self.rooted_y <= y as isize
                    {
                        self.checked = !self.checked;
                    }
                }
                SDL_EventType::SDL_MOUSEMOTION => {
                    let motion = (*event).motion;
                    self.motion = (motion.x as isize, motion.y as isize);
                }
                _ => (),
            };
        }
    }
}
