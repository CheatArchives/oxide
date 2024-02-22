#![feature(associated_type_defaults)]
#![allow(unused)]
#![deny(warnings)]

use std::{
    alloc::{alloc, Layout},
    error::Error,
    thread,
};

pub use libc::wchar_t;
pub use log::{debug, error, info, log, trace, warn};
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

static mut OXIDE: *mut c_void = std::ptr::null_mut() as *mut _ as *mut c_void;
static mut MENU: *mut c_void = std::ptr::null_mut() as *mut _ as *mut c_void;

unsafe fn main() -> Result<(), std::boxed::Box<dyn Error>> {
    info!("loading");

    let oxide_ptr = alloc(Layout::new::<Oxide>()) as *mut _ as *mut Oxide;
    *oxide_ptr = Oxide::init()?;
    OXIDE = oxide_ptr as *mut _ as *mut c_void;

    info!("loaded");
    Ok(())
}

#[link_section = ".init_array"]
static LOAD: unsafe extern "C" fn() = {
    #[link_section = ".text.startup"]
    unsafe extern "C" fn load() {
        libc::atexit(unload);
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();

        thread::spawn(|| unsafe {
            if let Err(e) = main() {
                error!("{}\n{:?}", e, e)
            }
        });
    }
    load
};

#[link_section = ".text.exit"]
extern "C" fn unload() {
    unsafe {
        info!("unloading");
        o!().unload();
        m!().unload();
        info!("unloaded");
    }
}
