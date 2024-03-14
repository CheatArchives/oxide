use std::{ffi::CString, fs::File, io::Write};

use crate::{
    c,
    draw::fonts::NERD_FONT,
    sdk::font::{Font, FontFlags},
};

use super::interfaces::Interfaces;

pub mod esp;
pub mod hitbox;

#[derive(Debug)]
pub struct Paint {
    pub normal: Font,
}

impl Paint {
    pub fn init(interfaces: &Interfaces) -> Paint {
        let surface = interfaces.surface.interface_ref();

        let file_name = "HackNerdFont-Regular.ttf";

        let mut file = File::create(file_name).unwrap();
        file.write_all(NERD_FONT).unwrap();

        let name = CString::new("Hack").unwrap();
        let path = CString::new(file_name).unwrap();

        c!(surface, add_custom_font_file, name.as_ptr(), path.as_ptr());

        let id = c!(surface, create_font);

        let normal = Font {
            name: name.as_ptr(),
            tall: 15,
            weight: 700,
            flags: FontFlags::DROPSHADOW as i32 | FontFlags::ANTIALIAS as i32,
            id,
        };

        c!(
            surface,
            set_font_glyph_set,
            normal.id,
            normal.name,
            normal.tall,
            normal.weight,
            0,
            0,
            normal.flags,
            0,
            0
        );

        Paint { normal }
    }
}
