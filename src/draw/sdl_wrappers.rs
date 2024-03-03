use std::{f32::consts::PI, ptr::null};

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
    pub fn draw_text(&mut self, text: &str, x: isize, y: isize, size: FontSize, color: usize) {
        let glyph = self
            .fonts
            .get_glyph(size.clone(), text.chars().next().unwrap());

        let mut x_offset = -(glyph.metrics.vertBearingX >> 6) as isize;
        let mut y_offset = self.fonts.get_text_size(text, size.clone()).1 as isize;

        for (i, letter) in text.chars().enumerate() {
            if letter == ' ' {
                x_offset += unsafe {
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
                continue;
            }
            let glyph = self.fonts.get_glyph(size.clone(), letter);

            let x = x + x_offset + (glyph.metrics.vertBearingX >> 6) as isize;
            let y = y + y_offset - (glyph.metrics.horiBearingY >> 6) as isize;

            x_offset += (glyph.metrics.horiAdvance >> 6) as isize;
            let surface = Fonts::glyph_to_surface(glyph, color);
            let texture = unsafe { SDL_CreateTextureFromSurface(self.renderer, surface) };

            let mut dest = SDL_Rect {
                x: x as i32,
                y: y as i32,
                w: 0,
                h: 0,
            };

            unsafe {
                SDL_RenderCopy(self.renderer, texture, null(), &mut dest);
                SDL_DestroyTexture(texture);
            }
            unsafe { SDL_FreeSurface(surface) };
        }
    }
}
