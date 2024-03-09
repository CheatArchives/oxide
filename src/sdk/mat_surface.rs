use crate::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub enum FontDrawType {
    Default,
    NonAdditive,
    TypeCount,
    Additive,
}

impl Default for FontDrawType {
    fn default() -> Self {
        Self::Additive
    }
}

pub type MatSurface = WithVmt<VMTMatSurface>;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTMatSurface {
    _pad1: [u32; 10],
    pub set_color: cfn!(c_void, &'static MatSurface, isize, isize, isize, isize),
    _pad2: [u32; 1],
    pub draw_filled_rect: cfn!(c_void, &'static MatSurface, isize, isize, isize, isize),
    _pad3: [u32; 1],
    pub draw_rect: cfn!(c_void, &'static MatSurface, isize, isize, isize, isize),
    pub draw_line: cfn!(c_void, &'static MatSurface, isize, isize, isize, isize),
    _pad4: [u32; 1],
    pub set_text_font: cfn!(c_void, &'static MatSurface, HFont),
    pub set_text_color: cfn!(c_void, &'static MatSurface, isize, isize, isize, isize),
    _pad5: [u32; 1],
    pub set_text_pos: cfn!(c_void, &'static MatSurface, isize, isize),
    pub get_text_pos: cfn!(c_void, &'static MatSurface, isize, isize),
    pub print_text: cfn!(
        c_void,
        &'static MatSurface,
        &'static wchar_t,
        isize,
        FontDrawType
    ),
    _pad6: [u32; 29],
    pub set_cursor_always_visible: cfn!(c_void, &'static MatSurface, bool),
    _pad7: [u32; 13],
    pub create_font: cfn!(HFont, &'static MatSurface),
    pub set_font_glyph_set: cfn!(
        bool,
        &'static MatSurface,
        HFont,
        &CStr,
        isize,
        isize,
        isize,
        isize,
        isize,
        isize,
        isize
    ),
    _pad8: [u32; 7],
    pub get_text_size: cfn!(
        c_void,
        &'static MatSurface,
        HFont,
        &'static wchar_t,
        &'static mut isize,
        &'static mut isize
    ),
    _pad9: [u32; 23],
    pub draw_circle: cfn!(c_void, &'static MatSurface, isize, isize, isize, isize),
    _pad10: [u32; 11],
    pub on_screen_size_changed: cfn!(c_void, &'static MatSurface, isize, isize),
}
