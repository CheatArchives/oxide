use std::mem::MaybeUninit;

use crate::{Draw, *};
use freetype_sys::*;

static NERD_FONT: &[u8; 2215536] = include_bytes!("./../../HackNerdFont-Regular.ttf");

#[derive(Debug, Clone)]
pub struct Fonts {
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

impl Fonts {
    pub fn init_face(free_type: *mut c_void, size: isize) -> FT_Face{
        unsafe {
            let mut face = MaybeUninit::zeroed().assume_init();
            FT_New_Memory_Face(free_type, NERD_FONT.as_ptr(), NERD_FONT.len() as i32, 0, &mut face);
            let size = ((size) << 6) as i32;
            FT_Set_Char_Size(face, size, size, 72, 72);
            face
        }
    }
    pub fn init() -> Fonts {
        unsafe {
            let mut free_type = MaybeUninit::zeroed().assume_init();

            FT_Init_FreeType(&mut free_type);

            let face_large = Fonts::init_face(free_type, FontSize::Large as isize);
            let face_medium = Fonts::init_face(free_type, FontSize::Medium as isize);
            let face_small = Fonts::init_face(free_type, FontSize::Small as isize);
            Fonts {
                free_type,
                face_large,
                face_medium,
                face_small,
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
    pub fn restore(&self) {
        unsafe {
            FT_Done_Face(self.face_small);
            FT_Done_Face(self.face_medium);
            FT_Done_Face(self.face_large);
            FT_Done_FreeType(self.free_type);
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
}
