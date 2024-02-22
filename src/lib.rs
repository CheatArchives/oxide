#![feature(associated_type_defaults)]

#![allow(unused)]
#![allow(improper_ctypes_definitions)]

#![deny(warnings)]

use std::{
    alloc::{alloc, Layout},
    error::Error,
    thread,
};

pub use libc::wchar_t;
pub use std::{
    ffi::*,
    mem::transmute,
    ptr::{addr_of, addr_of_mut},
};

mod util;
pub use util::*;

module_export!(oxide);
module_export!(sdk);
module_export!(error);
module_export!(math);

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

static mut OXIDE: *mut c_void = std::ptr::null_mut() as *mut _ as *mut c_void;
static mut MENU: *mut c_void = std::ptr::null_mut() as *mut _ as *mut c_void;

unsafe fn main() -> Result<(), std::boxed::Box<dyn Error>> {
    println!("loading");

    let oxide_ptr = alloc(Layout::new::<Oxide>()) as *mut _ as *mut Oxide;
    *oxide_ptr = Oxide::init()?;
    OXIDE = oxide_ptr as *mut _ as *mut c_void;

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
        o!().unload();
        m!().unload();
        println!("unloaded");
    }
}
