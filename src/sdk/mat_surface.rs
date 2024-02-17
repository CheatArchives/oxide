
use crate::*;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub enum FontDrawType {
    FONT_DRAW_DEFAULT = 0,
    FONT_DRAW_NONADDITIVE,
    FONT_DRAW_TYPE_COUNT = 2,
    FONT_DRAW_ADDITIVE,
}

pub type MatSurface = WithVmt<VMTMatSurface>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTMatSurface {
    _pad1: [u32; 10],
    pub SetColor: cfn!(c_void, *const MatSurface, c_int, c_int, c_int, c_int),
    _pad2: [u32; 1],
    pub DrawFilledRect: cfn!(c_void, *const MatSurface, c_int, c_int, c_int, c_int),
    _pad3: [u32; 1],
    pub DrawRect: cfn!(c_void, *const MatSurface, c_int, c_int, c_int, c_int),
    pub DrawLine: cfn!(c_void, *const MatSurface, c_int, c_int, c_int, c_int),
    _pad4: [u32; 1],
    pub SetTextFont: cfn!(c_void, *const MatSurface, HFont),
    pub SetTextColor: cfn!(c_void, *const MatSurface, c_int, c_int, c_int, c_int),
    _pad5: [u32; 1],
    pub SetTextPos: cfn!(c_void, *const MatSurface, c_int, c_int),
    pub GetTextPos: cfn!(c_void, *const MatSurface, c_int, c_int),
    pub PrintText: cfn!(c_void, *const MatSurface, *const wchar_t, c_int, FontDrawType),
    _pad6: [u32; 29],
    pub SetCursorAlwaysVisible: cfn!(c_void, *const MatSurface, bool),
    _pad7: [u32; 13],
    pub CreateFont: cfn!(HFont, *const MatSurface),
    pub SetFontGlyphSet: cfn!(bool, *const MatSurface, HFont,*const c_char,c_int,c_int,c_int,c_int,c_int,c_int,c_int),
    _pad8: [u32; 7],
    pub GetTextSize: cfn!(c_void, *const MatSurface, HFont,*const wchar_t,*mut c_int,*mut c_int),
    _pad9: [u32; 23],
    pub DrawCircle: cfn!(c_void, *const MatSurface, c_int,c_int,c_int,c_int),
    _pad10: [u32; 11],
    pub OnScreenSizeChanged: cfn!(c_void, *const MatSurface, c_int,c_int),
}
