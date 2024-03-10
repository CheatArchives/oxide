use std::f32::consts::PI;

use sdl2_sys::*;

use crate::hex_to_rgb;

use super::{
    fonts::{FontSize, Fonts},
    frame::Frame,
};

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

    pub fn line(&self, x1: isize, y1: isize, x2: isize, y2: isize, color: usize, alpha: u8) {
        self.set_color(color, alpha);

        unsafe {
            SDL_RenderDrawLine(self.renderer, x1 as i32, y1 as i32, x2 as i32, y2 as i32);
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
    pub fn text(
        &mut self,
        text: &str,
        x: isize,
        y: isize,
        size: FontSize,
        center_horizontaly: bool,
        color: usize,
        alpha: u8,
    ) {
        if text.len() == 0 {
            return;
        }
        let glyph = self
            .fonts
            .get_glyph(size.clone(), text.chars().next().unwrap());
        let calculated_size = self.fonts.get_text_size(text, size.clone());

        let mut x_offset = -(glyph.metrics.vertBearingX >> 6) as isize;
        if center_horizontaly {
            x_offset -= calculated_size.0 / 2;
        }
        let y_offset = calculated_size.1 / 2 - calculated_size.2 / 2;

        let max_advance = unsafe {
            (self
                .fonts
                .get_face(&size)
                .read()
                .size
                .read()
                .metrics
                .max_advance
                >> 6) as isize
        };

        for letter in text.chars() {
            if letter == ' ' {
                x_offset += max_advance;
                continue;
            }
            let glyph = self.fonts.get_glyph(size.clone(), letter);

            let x = x + x_offset + (glyph.metrics.vertBearingX >> 6) as isize;
            let y = y + y_offset - (glyph.metrics.horiBearingY >> 6) as isize;

            x_offset += (glyph.metrics.horiAdvance >> 6) as isize;
            Fonts::draw_glyph(glyph, x, y, color, alpha);
        }
    }
}
