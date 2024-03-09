use std::sync::{Arc, Mutex};

use crate::*;

#[derive(Debug)]
pub struct FloatInput {
    text_input: TextInput,
    text_val: amt!(String),
    float_val: amt!(f32),
}

impl FloatInput {
    pub fn new(label: &'static str, x: isize, y: isize, w: isize, val: amt!(f32)) -> FloatInput {
        let text_val = am!(val.lock().unwrap().to_string());

        FloatInput {
            text_input: TextInput::new(label, x, y, w, text_val.clone()),
            text_val,
            float_val: val,
        }
    }
}

impl RawComponent for FloatInput {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let mut float_val = self.float_val.lock().unwrap();
        let text_val = self.text_val.lock().unwrap();
        if let Ok(val) = text_val.parse() {
            *float_val = val
        }
        drop(text_val);
        self.text_input.draw(frame, root_x, root_y)
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.text_input.handle_event(event)
    }
}

impl Component for FloatInput {}
