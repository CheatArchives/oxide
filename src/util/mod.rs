use std::{error::Error, ffi::{c_char, CStr, CString}, mem::{transmute, MaybeUninit}, usize};

use elf::{dynamic::Elf64_Dyn, segment::Elf64_Phdr};
use libc::{c_void, dlclose, dlerror, dlopen, Elf64_Addr, RTLD_LAZY, RTLD_NOLOAD};
use sdl2_sys::SDL_Scancode;


use crate::{c, error::OxideError, i, math::vector::{Vector2, Vector3}};

pub mod macros;
pub mod sigscanner;

pub fn vmt_size(vmt: *const c_void) -> usize {
    unsafe {
        let mut funcs = transmute::<_, *const *const c_void>(vmt);
        let size = std::mem::size_of::<*const c_void>();

        let mut i = 0;
        while !(*funcs).is_null() {
            i += 1;
            funcs = (funcs as usize + size) as *const *const c_void;
        }

        i * size
    }
}

pub fn get_handle(name: &str) -> Result<*mut c_void, std::boxed::Box<dyn Error>> {
    unsafe {
        let handle = dlopen(CString::new(name)?.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
        if handle.is_null() {
            let error = CStr::from_ptr(dlerror()).to_str()?;
            return Err(std::boxed::Box::new(OxideError::new(&format!(
                "{} handle not found\n {}",
                name, error
            ))));
        }
        dlclose(handle);
        Ok(handle)
    }
}

#[allow(unused)]
struct LinkMap {
    addr: Elf64_Addr,
    name: *const c_char,
    ld: *const Elf64_Dyn,
    next: *const LinkMap,
    prev: *const LinkMap,
    real: *const LinkMap,
    ns: i32,
    module_name: *const u8,
    info: *const u8,
    phdr: *const Elf64_Phdr,
}

pub fn sdl_scancode_name_to_string(scan_code: SDL_Scancode) -> String {
    match scan_code {
        SDL_Scancode::SDL_SCANCODE_UNKNOWN => "UNKNOWN",
        SDL_Scancode::SDL_SCANCODE_A => "A",
        SDL_Scancode::SDL_SCANCODE_B => "B",
        SDL_Scancode::SDL_SCANCODE_C => "C",
        SDL_Scancode::SDL_SCANCODE_D => "D",
        SDL_Scancode::SDL_SCANCODE_E => "E",
        SDL_Scancode::SDL_SCANCODE_F => "F",
        SDL_Scancode::SDL_SCANCODE_G => "G",
        SDL_Scancode::SDL_SCANCODE_H => "H",
        SDL_Scancode::SDL_SCANCODE_I => "I",
        SDL_Scancode::SDL_SCANCODE_J => "J",
        SDL_Scancode::SDL_SCANCODE_K => "K",
        SDL_Scancode::SDL_SCANCODE_L => "L",
        SDL_Scancode::SDL_SCANCODE_M => "M",
        SDL_Scancode::SDL_SCANCODE_N => "N",
        SDL_Scancode::SDL_SCANCODE_O => "O",
        SDL_Scancode::SDL_SCANCODE_P => "P",
        SDL_Scancode::SDL_SCANCODE_Q => "Q",
        SDL_Scancode::SDL_SCANCODE_R => "R",
        SDL_Scancode::SDL_SCANCODE_S => "S",
        SDL_Scancode::SDL_SCANCODE_T => "T",
        SDL_Scancode::SDL_SCANCODE_U => "U",
        SDL_Scancode::SDL_SCANCODE_V => "V",
        SDL_Scancode::SDL_SCANCODE_W => "W",
        SDL_Scancode::SDL_SCANCODE_X => "X",
        SDL_Scancode::SDL_SCANCODE_Y => "Y",
        SDL_Scancode::SDL_SCANCODE_Z => "Z",
        SDL_Scancode::SDL_SCANCODE_1 => "1",
        SDL_Scancode::SDL_SCANCODE_2 => "2",
        SDL_Scancode::SDL_SCANCODE_3 => "3",
        SDL_Scancode::SDL_SCANCODE_4 => "4",
        SDL_Scancode::SDL_SCANCODE_5 => "5",
        SDL_Scancode::SDL_SCANCODE_6 => "6",
        SDL_Scancode::SDL_SCANCODE_7 => "7",
        SDL_Scancode::SDL_SCANCODE_8 => "8",
        SDL_Scancode::SDL_SCANCODE_9 => "9",
        SDL_Scancode::SDL_SCANCODE_0 => "0",
        SDL_Scancode::SDL_SCANCODE_RETURN => "RETURN",
        SDL_Scancode::SDL_SCANCODE_ESCAPE => "ESCAPE",
        SDL_Scancode::SDL_SCANCODE_BACKSPACE => "BACKSPACE",
        SDL_Scancode::SDL_SCANCODE_TAB => "TAB",
        SDL_Scancode::SDL_SCANCODE_SPACE => "SPACE",
        SDL_Scancode::SDL_SCANCODE_MINUS => "MINUS",
        SDL_Scancode::SDL_SCANCODE_EQUALS => "EQUALS",
        SDL_Scancode::SDL_SCANCODE_LEFTBRACKET => "LEFTBRACKET",
        SDL_Scancode::SDL_SCANCODE_RIGHTBRACKET => "RIGHTBRACKET",
        SDL_Scancode::SDL_SCANCODE_BACKSLASH => "BACKSLASH",
        SDL_Scancode::SDL_SCANCODE_NONUSHASH => "NONUSHASH",
        SDL_Scancode::SDL_SCANCODE_SEMICOLON => "SEMICOLON",
        SDL_Scancode::SDL_SCANCODE_APOSTROPHE => "APOSTROPHE",
        SDL_Scancode::SDL_SCANCODE_GRAVE => "GRAVE",
        SDL_Scancode::SDL_SCANCODE_COMMA => "COMMA",
        SDL_Scancode::SDL_SCANCODE_PERIOD => "PERIOD",
        SDL_Scancode::SDL_SCANCODE_SLASH => "SLASH",
        SDL_Scancode::SDL_SCANCODE_CAPSLOCK => "CAPSLOCK",
        SDL_Scancode::SDL_SCANCODE_F1 => "F1",
        SDL_Scancode::SDL_SCANCODE_F2 => "F2",
        SDL_Scancode::SDL_SCANCODE_F3 => "F3",
        SDL_Scancode::SDL_SCANCODE_F4 => "F4",
        SDL_Scancode::SDL_SCANCODE_F5 => "F5",
        SDL_Scancode::SDL_SCANCODE_F6 => "F6",
        SDL_Scancode::SDL_SCANCODE_F7 => "F7",
        SDL_Scancode::SDL_SCANCODE_F8 => "F8",
        SDL_Scancode::SDL_SCANCODE_F9 => "F9",
        SDL_Scancode::SDL_SCANCODE_F10 => "F10",
        SDL_Scancode::SDL_SCANCODE_F11 => "F11",
        SDL_Scancode::SDL_SCANCODE_F12 => "F12",
        SDL_Scancode::SDL_SCANCODE_PRINTSCREEN => "PRINTSCREEN",
        SDL_Scancode::SDL_SCANCODE_SCROLLLOCK => "SCROLLLOCK",
        SDL_Scancode::SDL_SCANCODE_PAUSE => "PAUSE",
        SDL_Scancode::SDL_SCANCODE_INSERT => "INSERT",
        SDL_Scancode::SDL_SCANCODE_HOME => "HOME",
        SDL_Scancode::SDL_SCANCODE_PAGEUP => "PAGEUP",
        SDL_Scancode::SDL_SCANCODE_DELETE => "DELETE",
        SDL_Scancode::SDL_SCANCODE_END => "END",
        SDL_Scancode::SDL_SCANCODE_PAGEDOWN => "PAGEDOWN",
        SDL_Scancode::SDL_SCANCODE_RIGHT => "RIGHT",
        SDL_Scancode::SDL_SCANCODE_LEFT => "LEFT",
        SDL_Scancode::SDL_SCANCODE_DOWN => "DOWN",
        SDL_Scancode::SDL_SCANCODE_UP => "UP",
        SDL_Scancode::SDL_SCANCODE_NUMLOCKCLEAR => "NUMLOCKCLEAR",
        SDL_Scancode::SDL_SCANCODE_KP_DIVIDE => "KP_DIVIDE",
        SDL_Scancode::SDL_SCANCODE_KP_MULTIPLY => "KP_MULTIPLY",
        SDL_Scancode::SDL_SCANCODE_KP_MINUS => "KP_MINUS",
        SDL_Scancode::SDL_SCANCODE_KP_PLUS => "KP_PLUS",
        SDL_Scancode::SDL_SCANCODE_KP_ENTER => "KP_ENTER",
        SDL_Scancode::SDL_SCANCODE_KP_1 => "KP_1",
        SDL_Scancode::SDL_SCANCODE_KP_2 => "KP_2",
        SDL_Scancode::SDL_SCANCODE_KP_3 => "KP_3",
        SDL_Scancode::SDL_SCANCODE_KP_4 => "KP_4",
        SDL_Scancode::SDL_SCANCODE_KP_5 => "KP_5",
        SDL_Scancode::SDL_SCANCODE_KP_6 => "KP_6",
        SDL_Scancode::SDL_SCANCODE_KP_7 => "KP_7",
        SDL_Scancode::SDL_SCANCODE_KP_8 => "KP_8",
        SDL_Scancode::SDL_SCANCODE_KP_9 => "KP_9",
        SDL_Scancode::SDL_SCANCODE_KP_0 => "KP_0",
        SDL_Scancode::SDL_SCANCODE_KP_PERIOD => "KP_PERIOD",
        SDL_Scancode::SDL_SCANCODE_NONUSBACKSLASH => "NONUSBACKSLASH",
        SDL_Scancode::SDL_SCANCODE_APPLICATION => "APPLICATION",
        SDL_Scancode::SDL_SCANCODE_POWER => "POWER",
        SDL_Scancode::SDL_SCANCODE_KP_EQUALS => "KP_EQUALS",
        SDL_Scancode::SDL_SCANCODE_F13 => "F13",
        SDL_Scancode::SDL_SCANCODE_F14 => "F14",
        SDL_Scancode::SDL_SCANCODE_F15 => "F15",
        SDL_Scancode::SDL_SCANCODE_F16 => "F16",
        SDL_Scancode::SDL_SCANCODE_F17 => "F17",
        SDL_Scancode::SDL_SCANCODE_F18 => "F18",
        SDL_Scancode::SDL_SCANCODE_F19 => "F19",
        SDL_Scancode::SDL_SCANCODE_F20 => "F20",
        SDL_Scancode::SDL_SCANCODE_F21 => "F21",
        SDL_Scancode::SDL_SCANCODE_F22 => "F22",
        SDL_Scancode::SDL_SCANCODE_F23 => "F23",
        SDL_Scancode::SDL_SCANCODE_F24 => "F24",
        SDL_Scancode::SDL_SCANCODE_EXECUTE => "EXECUTE",
        SDL_Scancode::SDL_SCANCODE_HELP => "HELP",
        SDL_Scancode::SDL_SCANCODE_MENU => "MENU",
        SDL_Scancode::SDL_SCANCODE_SELECT => "SELECT",
        SDL_Scancode::SDL_SCANCODE_STOP => "STOP",
        SDL_Scancode::SDL_SCANCODE_AGAIN => "AGAIN",
        SDL_Scancode::SDL_SCANCODE_UNDO => "UNDO",
        SDL_Scancode::SDL_SCANCODE_CUT => "CUT",
        SDL_Scancode::SDL_SCANCODE_COPY => "COPY",
        SDL_Scancode::SDL_SCANCODE_PASTE => "PASTE",
        SDL_Scancode::SDL_SCANCODE_FIND => "FIND",
        SDL_Scancode::SDL_SCANCODE_MUTE => "MUTE",
        SDL_Scancode::SDL_SCANCODE_VOLUMEUP => "VOLUMEUP",
        SDL_Scancode::SDL_SCANCODE_VOLUMEDOWN => "VOLUMEDOWN",
        SDL_Scancode::SDL_SCANCODE_KP_COMMA => "KP_COMMA",
        SDL_Scancode::SDL_SCANCODE_KP_EQUALSAS400 => "KP_EQUALSAS400",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL1 => "INTERNATIONAL1",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL2 => "INTERNATIONAL2",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL3 => "INTERNATIONAL3",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL4 => "INTERNATIONAL4",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL5 => "INTERNATIONAL5",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL6 => "INTERNATIONAL6",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL7 => "INTERNATIONAL7",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL8 => "INTERNATIONAL8",
        SDL_Scancode::SDL_SCANCODE_INTERNATIONAL9 => "INTERNATIONAL9",
        SDL_Scancode::SDL_SCANCODE_LANG1 => "LANG1",
        SDL_Scancode::SDL_SCANCODE_LANG2 => "LANG2",
        SDL_Scancode::SDL_SCANCODE_LANG3 => "LANG3",
        SDL_Scancode::SDL_SCANCODE_LANG4 => "LANG4",
        SDL_Scancode::SDL_SCANCODE_LANG5 => "LANG5",
        SDL_Scancode::SDL_SCANCODE_LANG6 => "LANG6",
        SDL_Scancode::SDL_SCANCODE_LANG7 => "LANG7",
        SDL_Scancode::SDL_SCANCODE_LANG8 => "LANG8",
        SDL_Scancode::SDL_SCANCODE_LANG9 => "LANG9",
        SDL_Scancode::SDL_SCANCODE_ALTERASE => "ALTERASE",
        SDL_Scancode::SDL_SCANCODE_SYSREQ => "SYSREQ",
        SDL_Scancode::SDL_SCANCODE_CANCEL => "CANCEL",
        SDL_Scancode::SDL_SCANCODE_CLEAR => "CLEAR",
        SDL_Scancode::SDL_SCANCODE_PRIOR => "PRIOR",
        SDL_Scancode::SDL_SCANCODE_RETURN2 => "RETURN2",
        SDL_Scancode::SDL_SCANCODE_SEPARATOR => "SEPARATOR",
        SDL_Scancode::SDL_SCANCODE_OUT => "OUT",
        SDL_Scancode::SDL_SCANCODE_OPER => "OPER",
        SDL_Scancode::SDL_SCANCODE_CLEARAGAIN => "CLEARAGAIN",
        SDL_Scancode::SDL_SCANCODE_CRSEL => "CRSEL",
        SDL_Scancode::SDL_SCANCODE_EXSEL => "EXSEL",
        SDL_Scancode::SDL_SCANCODE_KP_00 => "KP_00",
        SDL_Scancode::SDL_SCANCODE_KP_000 => "KP_000",
        SDL_Scancode::SDL_SCANCODE_THOUSANDSSEPARATOR => "THOUSANDSSEPARATOR",
        SDL_Scancode::SDL_SCANCODE_DECIMALSEPARATOR => "DECIMALSEPARATOR",
        SDL_Scancode::SDL_SCANCODE_CURRENCYUNIT => "CURRENCYUNIT",
        SDL_Scancode::SDL_SCANCODE_CURRENCYSUBUNIT => "CURRENCYSUBUNIT",
        SDL_Scancode::SDL_SCANCODE_KP_LEFTPAREN => "KP_LEFTPAREN",
        SDL_Scancode::SDL_SCANCODE_KP_RIGHTPAREN => "KP_RIGHTPAREN",
        SDL_Scancode::SDL_SCANCODE_KP_LEFTBRACE => "KP_LEFTBRACE",
        SDL_Scancode::SDL_SCANCODE_KP_RIGHTBRACE => "KP_RIGHTBRACE",
        SDL_Scancode::SDL_SCANCODE_KP_TAB => "KP_TAB",
        SDL_Scancode::SDL_SCANCODE_KP_BACKSPACE => "KP_BACKSPACE",
        SDL_Scancode::SDL_SCANCODE_KP_A => "KP_A",
        SDL_Scancode::SDL_SCANCODE_KP_B => "KP_B",
        SDL_Scancode::SDL_SCANCODE_KP_C => "KP_C",
        SDL_Scancode::SDL_SCANCODE_KP_D => "KP_D",
        SDL_Scancode::SDL_SCANCODE_KP_E => "KP_E",
        SDL_Scancode::SDL_SCANCODE_KP_F => "KP_F",
        SDL_Scancode::SDL_SCANCODE_KP_XOR => "KP_XOR",
        SDL_Scancode::SDL_SCANCODE_KP_POWER => "KP_POWER",
        SDL_Scancode::SDL_SCANCODE_KP_PERCENT => "KP_PERCENT",
        SDL_Scancode::SDL_SCANCODE_KP_LESS => "KP_LESS",
        SDL_Scancode::SDL_SCANCODE_KP_GREATER => "KP_GREATER",
        SDL_Scancode::SDL_SCANCODE_KP_AMPERSAND => "KP_AMPERSAND",
        SDL_Scancode::SDL_SCANCODE_KP_DBLAMPERSAND => "KP_DBLAMPERSAND",
        SDL_Scancode::SDL_SCANCODE_KP_VERTICALBAR => "KP_VERTICALBAR",
        SDL_Scancode::SDL_SCANCODE_KP_DBLVERTICALBAR => "KP_DBLVERTICALBAR",
        SDL_Scancode::SDL_SCANCODE_KP_COLON => "KP_COLON",
        SDL_Scancode::SDL_SCANCODE_KP_HASH => "KP_HASH",
        SDL_Scancode::SDL_SCANCODE_KP_SPACE => "KP_SPACE",
        SDL_Scancode::SDL_SCANCODE_KP_AT => "KP_AT",
        SDL_Scancode::SDL_SCANCODE_KP_EXCLAM => "KP_EXCLAM",
        SDL_Scancode::SDL_SCANCODE_KP_MEMSTORE => "KP_MEMSTORE",
        SDL_Scancode::SDL_SCANCODE_KP_MEMRECALL => "KP_MEMRECALL",
        SDL_Scancode::SDL_SCANCODE_KP_MEMCLEAR => "KP_MEMCLEAR",
        SDL_Scancode::SDL_SCANCODE_KP_MEMADD => "KP_MEMADD",
        SDL_Scancode::SDL_SCANCODE_KP_MEMSUBTRACT => "KP_MEMSUBTRACT",
        SDL_Scancode::SDL_SCANCODE_KP_MEMMULTIPLY => "KP_MEMMULTIPLY",
        SDL_Scancode::SDL_SCANCODE_KP_MEMDIVIDE => "KP_MEMDIVIDE",
        SDL_Scancode::SDL_SCANCODE_KP_PLUSMINUS => "KP_PLUSMINUS",
        SDL_Scancode::SDL_SCANCODE_KP_CLEAR => "KP_CLEAR",
        SDL_Scancode::SDL_SCANCODE_KP_CLEARENTRY => "KP_CLEARENTRY",
        SDL_Scancode::SDL_SCANCODE_KP_BINARY => "KP_BINARY",
        SDL_Scancode::SDL_SCANCODE_KP_OCTAL => "KP_OCTAL",
        SDL_Scancode::SDL_SCANCODE_KP_DECIMAL => "KP_DECIMAL",
        SDL_Scancode::SDL_SCANCODE_KP_HEXADECIMAL => "KP_HEXADECIMAL",
        SDL_Scancode::SDL_SCANCODE_LCTRL => "LCTRL",
        SDL_Scancode::SDL_SCANCODE_LSHIFT => "LSHIFT",
        SDL_Scancode::SDL_SCANCODE_LALT => "LALT",
        SDL_Scancode::SDL_SCANCODE_LGUI => "LGUI",
        SDL_Scancode::SDL_SCANCODE_RCTRL => "RCTRL",
        SDL_Scancode::SDL_SCANCODE_RSHIFT => "RSHIFT",
        SDL_Scancode::SDL_SCANCODE_RALT => "RALT",
        SDL_Scancode::SDL_SCANCODE_RGUI => "RGUI",
        SDL_Scancode::SDL_SCANCODE_MODE => "MODE",
        SDL_Scancode::SDL_SCANCODE_AUDIONEXT => "AUDIONEXT",
        SDL_Scancode::SDL_SCANCODE_AUDIOPREV => "AUDIOPREV",
        SDL_Scancode::SDL_SCANCODE_AUDIOSTOP => "AUDIOSTOP",
        SDL_Scancode::SDL_SCANCODE_AUDIOPLAY => "AUDIOPLAY",
        SDL_Scancode::SDL_SCANCODE_AUDIOMUTE => "AUDIOMUTE",
        SDL_Scancode::SDL_SCANCODE_MEDIASELECT => "MEDIASELECT",
        SDL_Scancode::SDL_SCANCODE_WWW => "WWW",
        SDL_Scancode::SDL_SCANCODE_MAIL => "MAIL",
        SDL_Scancode::SDL_SCANCODE_CALCULATOR => "CALCULATOR",
        SDL_Scancode::SDL_SCANCODE_COMPUTER => "COMPUTER",
        SDL_Scancode::SDL_SCANCODE_AC_SEARCH => "AC_SEARCH",
        SDL_Scancode::SDL_SCANCODE_AC_HOME => "AC_HOME",
        SDL_Scancode::SDL_SCANCODE_AC_BACK => "AC_BACK",
        SDL_Scancode::SDL_SCANCODE_AC_FORWARD => "AC_FORWARD",
        SDL_Scancode::SDL_SCANCODE_AC_STOP => "AC_STOP",
        SDL_Scancode::SDL_SCANCODE_AC_REFRESH => "AC_REFRESH",
        SDL_Scancode::SDL_SCANCODE_AC_BOOKMARKS => "AC_BOOKMARKS",
        SDL_Scancode::SDL_SCANCODE_BRIGHTNESSDOWN => "BRIGHTNESSDOWN",
        SDL_Scancode::SDL_SCANCODE_BRIGHTNESSUP => "BRIGHTNESSUP",
        SDL_Scancode::SDL_SCANCODE_DISPLAYSWITCH => "DISPLAYSWITCH",
        SDL_Scancode::SDL_SCANCODE_KBDILLUMTOGGLE => "KBDILLUMTOGGLE",
        SDL_Scancode::SDL_SCANCODE_KBDILLUMDOWN => "KBDILLUMDOWN",
        SDL_Scancode::SDL_SCANCODE_KBDILLUMUP => "KBDILLUMUP",
        SDL_Scancode::SDL_SCANCODE_EJECT => "EJECT",
        SDL_Scancode::SDL_SCANCODE_SLEEP => "SLEEP",
        SDL_Scancode::SDL_SCANCODE_APP1 => "APP1",
        SDL_Scancode::SDL_SCANCODE_APP2 => "APP2",
        SDL_Scancode::SDL_SCANCODE_AUDIOREWIND => "AUDIOREWIND",
        SDL_Scancode::SDL_SCANCODE_AUDIOFASTFORWARD => "AUDIOFASTFORWARD",
        SDL_Scancode::SDL_NUM_SCANCODES => "",
    }
    .to_owned()
}

pub fn sdl_scancode_to_char(key: SDL_Scancode) -> Option<char> {
    match key {
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
    }
}

pub fn point_in_bounds(
    x: isize,
    y: isize,
    bound_x: isize,
    bound_y: isize,
    w: isize,
    h: isize,
) -> bool {
    bound_x <= x && x <= bound_x + w && bound_y <= y && y <= bound_y + h
}

pub fn world_to_screen(vec: &Vector3) -> Option<Vector2> {
    let w2v = unsafe { MaybeUninit::zeroed().assume_init() };
    let v2pr = unsafe { MaybeUninit::zeroed().assume_init() };
    let w2px = unsafe { MaybeUninit::zeroed().assume_init() };
    let w2s = unsafe { MaybeUninit::zeroed().assume_init() };

    let player_view = unsafe { MaybeUninit::zeroed().assume_init() };
    c!(i!(base_client), get_player_view, &player_view);

    c!(
        i!(render_view),
        get_matrices_for_view,
        &player_view,
        &w2v,
        &v2pr,
        &w2s,
        &w2px
    );
    let w = w2s[3][0] * vec.x + w2s[3][1] * vec.y + w2s[3][2] * vec.z + w2s[3][3];

    if w < 0.01 {
        return None;
    }

    let screen_w = 0;
    let screen_h = 0;

    c!(i!(base_engine), get_screen_size, &screen_w, &screen_h);

    let x = w2s[0][0] * vec.x + w2s[0][1] * vec.y + w2s[0][2] * vec.z + w2s[0][3];
    let y = w2s[1][0] * vec.x + w2s[1][1] * vec.y + w2s[1][2] * vec.z + w2s[1][3];

    Some(Vector2::new(
        screen_w as f32 / 2f32 * (1f32 + x / w),
        screen_h as f32 / 2f32 * (1f32 - y / w),
    ))
}
