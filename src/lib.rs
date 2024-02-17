#![feature(core_intrinsics)]

use std::{
    alloc::{self, alloc, Layout}, borrow::BorrowMut, error::Error, panic::catch_unwind, sync::{Arc, Mutex}, thread, time::Duration
};


pub use libc::wchar_t;
pub use log::{debug, error, info, log, trace, warn};
pub use std::{ffi::*, mem::transmute};

mod util;
pub use util::*;

mea!(oxide);
mea!(sdk);
mea!(error);

static mut OXIDE: *mut c_void = std::ptr::null_mut() as *mut _ as *mut c_void;

unsafe fn main() -> Result<(), Box<dyn Error>> {
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
    unsafe{
        info!("unloading");
        let oxide = *(OXIDE as *mut _ as *mut Oxide);
        oxide.close();
        info!("unloaded");
    }
}
