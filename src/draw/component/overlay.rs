use crate::*;

#[derive(Debug)]
pub struct Overlay {}

impl Component for Overlay {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let size = frame.window_size;
        frame.filled_rect(0, 0, size.0, 50, BACKGROUND, 220);
        frame.filled_rect(0, 50, 300, size.1, BACKGROUND, 220);
    }

    fn handle_event(&mut self, event: *mut sdl2_sys::SDL_Event) {}
}

impl ComponentDebug for Overlay {}
