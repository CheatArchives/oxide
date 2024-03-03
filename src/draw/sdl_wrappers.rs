use std::f32::consts::PI;

use sdl2_sys::*;

use crate::*;

impl Frame {
    pub fn filled_rect(&self, x: isize, y: isize, w: isize, h: isize, color: usize, alpha: u8) {
        let rect = SDL_Rect {
            x: x as i32,
            y: y as i32,
            w: w as i32,
            h: h as i32,
        };

        self.set_color(color, alpha);
        unsafe {
            SDL_RenderFillRect(self.renderer, &rect);
        }
    }

    pub fn outlined_rect(&self, x: isize, y: isize, w: isize, h: isize, color: usize, alpha: u8) {
        self.set_color(color, alpha);

        let rect = SDL_Rect {
            x: x as i32,
            y: y as i32,
            w: w as i32,
            h: h as i32,
        };
        unsafe {
            SDL_RenderDrawRect(self.renderer, &rect);
        }
    }

    pub fn circle(&self, root_x: isize, root_y: isize, r: f32, color: usize, alpha: u8) {
        let mut points = Vec::new();

        let step = (1f32 - 1f32 / (r as f32)).acos();
        let mut angle = 0f32;
        while angle < 360f32 {
            let x = (r as f32 * (angle as f32 * PI / 180f32).cos()) as i32 + root_x as i32;
            let y = (r as f32 * (angle as f32 * PI / 180f32).sin()) as i32 + root_y as i32;
            points.push(SDL_Point { x, y });
            angle += step;
        }

        self.set_color(color, alpha);
        unsafe {
            SDL_RenderDrawPoints(self.renderer, points.as_ptr(), points.len() as i32);
        }
    }

    pub fn set_color(&self, color: usize, a: u8) {
        let (r, g, b) = hex_to_rgb!(color);
        unsafe {
            SDL_SetRenderDrawBlendMode(self.renderer, SDL_BlendMode::SDL_BLENDMODE_BLEND);
            SDL_SetRenderDrawColor(self.renderer, r, g, b, a);
        }
    }
}