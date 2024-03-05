use std::{
    sync::{Arc, Mutex},
    usize,
};

use sdl2_sys::*;

use crate::*;

const SIZE: isize = FontSize::Small as isize + 4;

#[derive(Debug)]
pub struct TextInput {
    x: isize,
    y: isize,
    w: isize,
    rooted_x: isize,
    rooted_y: isize,
    cursor: (isize, isize),
    val: Arc<Mutex<String>>,
    focussed: bool,
}

impl TextInput {
    pub fn new(x: isize, y: isize, w: isize, val: Arc<Mutex<String>>) -> TextInput {
        TextInput {
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

impl RawComponent for TextInput {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let x = self.x + root_x;
        let y = self.y + root_y;
        self.rooted_x = x;
        self.rooted_y = y;
        frame.filled_rect(x, y, self.w, SIZE, BACKGROUND, 255);
        let outline = if self.focussed { BLUE } else { FOREGROUND };
        frame.outlined_rect(x, y, self.w, SIZE, outline, 255);

        let val = self.val.lock().unwrap();

        frame.text(
            &val,
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
                    let key = (*event).key.keysym.scancode;
                    if self.focussed {
                        let letter = match key {
                            SDL_Scancode::SDL_SCANCODE_A => Some('A'),
                            SDL_Scancode::SDL_SCANCODE_B => Some('B'),
                            SDL_Scancode::SDL_SCANCODE_C => Some('C'),
                            SDL_Scancode::SDL_SCANCODE_D => Some('D'),
                            SDL_Scancode::SDL_SCANCODE_E => Some('E'),
                            SDL_Scancode::SDL_SCANCODE_F => Some('F'),
                            SDL_Scancode::SDL_SCANCODE_G => Some('G'),
                            SDL_Scancode::SDL_SCANCODE_H => Some('H'),
                            SDL_Scancode::SDL_SCANCODE_I => Some('I'),
                            SDL_Scancode::SDL_SCANCODE_J => Some('J'),
                            SDL_Scancode::SDL_SCANCODE_K => Some('K'),
                            SDL_Scancode::SDL_SCANCODE_L => Some('L'),
                            SDL_Scancode::SDL_SCANCODE_M => Some('M'),
                            SDL_Scancode::SDL_SCANCODE_N => Some('N'),
                            SDL_Scancode::SDL_SCANCODE_O => Some('O'),
                            SDL_Scancode::SDL_SCANCODE_P => Some('P'),
                            SDL_Scancode::SDL_SCANCODE_Q => Some('Q'),
                            SDL_Scancode::SDL_SCANCODE_R => Some('R'),
                            SDL_Scancode::SDL_SCANCODE_S => Some('S'),
                            SDL_Scancode::SDL_SCANCODE_T => Some('T'),
                            SDL_Scancode::SDL_SCANCODE_U => Some('U'),
                            SDL_Scancode::SDL_SCANCODE_V => Some('V'),
                            SDL_Scancode::SDL_SCANCODE_W => Some('W'),
                            SDL_Scancode::SDL_SCANCODE_X => Some('X'),
                            SDL_Scancode::SDL_SCANCODE_Y => Some('Y'),
                            SDL_Scancode::SDL_SCANCODE_Z => Some('Z'),
                            SDL_Scancode::SDL_SCANCODE_1 => Some('1'),
                            SDL_Scancode::SDL_SCANCODE_2 => Some('2'),
                            SDL_Scancode::SDL_SCANCODE_3 => Some('3'),
                            SDL_Scancode::SDL_SCANCODE_4 => Some('4'),
                            SDL_Scancode::SDL_SCANCODE_5 => Some('5'),
                            SDL_Scancode::SDL_SCANCODE_6 => Some('6'),
                            SDL_Scancode::SDL_SCANCODE_7 => Some('7'),
                            SDL_Scancode::SDL_SCANCODE_8 => Some('8'),
                            SDL_Scancode::SDL_SCANCODE_9 => Some('9'),
                            SDL_Scancode::SDL_SCANCODE_0 => Some('0'),
                            SDL_Scancode::SDL_SCANCODE_PERIOD => Some('.'),
                            SDL_Scancode::SDL_SCANCODE_MINUS => Some('-'),
                            SDL_Scancode::SDL_SCANCODE_SPACE => Some(' '),
                            _ => Option::None,
                        };
                        if let Some(letter) = letter {
                            let mut val = self.val.lock().unwrap();
                            val.push(letter);
                        }
                        match key {
                            SDL_Scancode::SDL_SCANCODE_DELETE => {}
                            SDL_Scancode::SDL_SCANCODE_BACKSPACE => {
                                let mut val = self.val.lock().unwrap();
                                val.pop();
                            }
                            SDL_Scancode::SDL_SCANCODE_RETURN
                            | SDL_Scancode::SDL_SCANCODE_ESCAPE => self.focussed = false,
                            _ => {}
                        }
                    }
                }
                _ => (),
            };
        }
    }
}

impl Component for TextInput {}
