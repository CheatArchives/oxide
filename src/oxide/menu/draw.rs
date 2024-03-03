use sdl2_sys::SDL_RendererInfo;

use crate::*;
use freetype_sys::*;
use sdl2_sys::*;
use std::{f32::consts::PI, intrinsics::offset, isize, mem::MaybeUninit, ptr::null, usize};

#[derive(Debug, Clone)]
pub struct Draw {
    renderer: *mut SDL_Renderer,
    pub free_type: FT_Library,
    pub face_large: FT_Face,
    pub face_medium: FT_Face,
    pub face_small: FT_Face,
}

#[derive(Debug, Clone)]
pub enum FontSize {
    Small = 16,
    Medium = 24,
    Large = 36,
}

impl Draw {
    pub unsafe fn init(window: *mut SDL_Window, renderer: *mut SDL_Renderer) -> Draw {
        let mut free_type = MaybeUninit::zeroed().assume_init();
        FT_Init_FreeType(&mut free_type);

        let font_file = include_bytes!("./../../../HackNerdFont-Regular.ttf");

        let mut face_large = MaybeUninit::zeroed().assume_init();
        FT_New_Memory_Face(
            free_type,
            font_file as *const u8,
            2215536,
            0,
            &mut face_large,
        );
        let size = ((FontSize::Large as isize) << 6) as i32;
        FT_Set_Char_Size(face_large, size, size, 72, 72);

        let mut face_medium = MaybeUninit::zeroed().assume_init();
        FT_New_Memory_Face(
            free_type,
            font_file as *const u8,
            2215536,
            0,
            &mut face_medium,
        );
        let size = ((FontSize::Medium as isize) << 6) as i32;
        FT_Set_Char_Size(face_medium, size, size, 72, 72);

        let mut face_small = MaybeUninit::zeroed().assume_init();
        FT_New_Memory_Face(
            free_type,
            font_file as *const u8,
            2215536,
            0,
            &mut face_small,
        );
        let size = ((FontSize::Small as isize) << 6) as i32;
        FT_Set_Char_Size(face_small, size, size, 72, 73);

        Draw {
            renderer,
            free_type,
            face_large,
            face_medium,
            face_small,
        }
    }

    pub fn draw_text(&mut self, text: &str, x: isize, y: isize, size: FontSize, color: usize) {
        unsafe {
            let face = self.get_face(&size);

            FT_Load_Char(face, text.chars().next().unwrap() as u32, FT_LOAD_RENDER);
            let glyph = (*face).glyph.read_volatile();

            let mut x_offset = -(glyph.metrics.vertBearingX >> 6) as isize;
            let mut y_offset = self.get_text_size(text, size).1 as isize;

            for (i, letter) in text.chars().enumerate() {
                if letter == ' ' {
                    x_offset += (face.read().size.read().metrics.max_advance >> 6) as isize;
                    continue;
                }
                FT_Load_Char(face, letter as u32, FT_LOAD_RENDER);
                let glyph = (*face).glyph.read_volatile();

                let x = x + x_offset + (glyph.metrics.vertBearingX >> 6) as isize;
                let y = y + y_offset - (glyph.metrics.horiBearingY >> 6) as isize;

                x_offset += (glyph.metrics.horiAdvance >> 6) as isize;
                self.draw_bitmap(glyph.bitmap, x, y, color);
            }
        }
    }
    pub fn get_face(&mut self, size: &FontSize) -> *mut FT_FaceRec {
        match size {
            FontSize::Small => self.face_small,
            FontSize::Medium => self.face_medium,
            FontSize::Large => self.face_large,
        }
    }
    pub unsafe fn get_text_size(&mut self, text: &str, size: FontSize) -> (isize, isize, isize) {
        let face = self.get_face(&size);

        let mut w = 0;
        let mut h_min = 0;
        let mut h_max = 0;

        for (i, letter) in text.chars().enumerate() {
            FT_Load_Char(face, letter as u32, FT_LOAD_RENDER);

            let glyph = (*face).glyph.read_volatile();
            w += (glyph.metrics.horiAdvance >> 6) as isize;

            h_min = std::cmp::max((glyph.metrics.horiBearingY >> 6) as isize, h_min);
            h_max = std::cmp::max((glyph.metrics.horiBearingX >> 6) as isize, h_max);
        }
        (w, h_min, h_max)
    }

    pub unsafe fn draw_bitmap(&mut self, bitmap: FT_Bitmap, x: isize, y: isize, color: usize) {
        let r = self.renderer;

        let len = (bitmap.width * bitmap.rows * 4) as usize;
        let mut rgba = vec![0u8; len];

        let buffer = std::slice::from_raw_parts(bitmap.buffer, len);
        for i in (0..len).step_by(4) {
            let val = buffer[i / 4];
            (rgba[i], rgba[i + 1], rgba[i + 2]) = hex_to_rgb!(color);
            rgba[i + 3] = val;
        }

        let glyph = SDL_CreateRGBSurfaceFrom(
            rgba.as_ptr() as *mut c_void,
            bitmap.width,
            bitmap.rows,
            32,
            bitmap.width * 4,
            0x000000ff,
            0x0000ff00,
            0x00ff0000,
            0xff000000,
        );
        SDL_SetSurfaceBlendMode(glyph, SDL_BlendMode::SDL_BLENDMODE_BLEND);

        let texture = SDL_CreateTextureFromSurface(r, glyph);
        let mut dest = SDL_Rect {
            x: x as i32,
            y: y as i32,
            w: bitmap.width,
            h: bitmap.rows,
        };

        SDL_RenderCopy(r, texture, null(), &mut dest);

        SDL_DestroyTexture(texture);
        SDL_FreeSurface(glyph);
    }

    pub fn filled_rect(&self, rect: SDL_Rect, color: usize, alpah: u8) -> SDL_Rect {
        let r = self.renderer;
        let (red, g, b) = hex_to_rgb!(color);
        unsafe {
            SDL_SetRenderDrawBlendMode(r, SDL_BlendMode::SDL_BLENDMODE_BLEND);
            SDL_SetRenderDrawColor(r, red, g, b, alpah);
            SDL_RenderFillRect(r, &rect);
        }
        return rect;
    }
    pub fn outlined_rect(&self, rect: SDL_Rect, color: usize, alpah: u8) -> SDL_Rect {
        let r = self.renderer;
        let (red, g, b) = hex_to_rgb!(color);
        unsafe {
            SDL_SetRenderDrawBlendMode(r, SDL_BlendMode::SDL_BLENDMODE_BLEND);
            SDL_SetRenderDrawColor(r, red, g, b, alpah);
            SDL_RenderDrawRect(r, &rect);
        }
        return rect;
    }

    pub fn circle(&self, root_x: i32, root_y: i32, r: f32, color: usize) {
        let mut points = Vec::new();

        let step = (1f32 - 1f32 / (r as f32)).acos();
        let mut angle = 0f32;
        while angle < 360f32 {
            let x = (r as f32 * (angle as f32 * PI / 180f32).cos()) as i32 + root_x;
            let y = (r as f32 * (angle as f32 * PI / 180f32).sin()) as i32 + root_y;
            points.push(SDL_Point { x, y });
            angle += step;
        }

        let (red, g, b) = hex_to_rgb!(color);
        unsafe {
            let r = self.renderer;
            SDL_SetRenderDrawColor(r, red, g, b, 255);
            SDL_RenderDrawPoints(r, points.as_ptr(), points.len() as i32);
        }
    }
    pub fn restore(&self) {
        unsafe {
            FT_Done_Face(self.face_small);
            FT_Done_Face(self.face_medium);
            FT_Done_Face(self.face_large);
            FT_Done_FreeType(self.free_type);
        }
    }
    pub fn get_window_size(window: *mut SDL_Window) -> (i32, i32) {
        let mut w = 0i32;
        let mut h = 0i32;

        unsafe {
            SDL_GetWindowSize(window, &mut w, &mut h);
        }
        return (w, h);
    }
}
