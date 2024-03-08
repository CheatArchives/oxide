#![feature(
    c_variadic,
    pointer_is_aligned,
    core_intrinsics,
    associated_type_defaults
)]
#![allow(improper_ctypes_definitions, internal_features, unused)]
#![deny(warnings)]

use std::{
    alloc::{alloc, Layout},
    error::Error,
    mem::ManuallyDrop,
    thread,
    time::Duration,
};

pub use libc::wchar_t;
pub use std::{
    ffi::*,
    intrinsics::breakpoint,
    mem::{transmute, MaybeUninit},
    ptr::{addr_of, addr_of_mut},
};

mod util;
pub use derivative::*;
pub use util::*;

module_export!(oxide);
module_export!(sdk);
module_export!(error);
module_export!(math);
module_export!(draw);
module_export!(settings);

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

static mut OXIDE: Option<*mut c_void> = None;
static mut DRAW: Option<*mut c_void> = None;
static mut SETTINGS: Option<*mut c_void> = None;

unsafe fn main() -> Result<(), std::boxed::Box<dyn Error>> {
    println!("loading");

    let settings_ptr = alloc(Layout::new::<Settings>()) as *mut _ as *mut ManuallyDrop<Settings>;
    *settings_ptr = ManuallyDrop::new(Settings::new());
    SETTINGS = Some(settings_ptr as *mut _ as *mut c_void);

    let oxide_ptr = alloc(Layout::new::<Oxide>()) as *mut _ as *mut ManuallyDrop<Oxide>;
    *oxide_ptr = ManuallyDrop::new(Oxide::init()?);
    OXIDE = Some(oxide_ptr as *mut _ as *mut c_void);

    println!("loaded");
    Ok(())
}

#[link_section = ".init_array"]
static LOAD: unsafe extern "C" fn() = {
    #[link_section = ".text.startup"]
    unsafe extern "C" fn load() {
        libc::atexit(unload);

        thread::spawn(|| unsafe {
            if let Err(e) = main() {
                eprintln!("{}", e);
            }
        });
    }
    load
};

#[link_section = ".text.exit"]
extern "C" fn unload() {
    unsafe {
        println!("unloading");

        if DRAW.is_some() {
            draw!().restore();
            std::ptr::drop_in_place(draw!());
        }
        if OXIDE.is_some() {
            oxide!().restore();
            std::ptr::drop_in_place(oxide!());
        }

        println!("unloaded");
    }
}
