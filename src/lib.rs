#![feature(core_intrinsics)]

use std::{
    error::Error,
    panic::catch_unwind,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use ctor::{ctor, dtor};

pub use libc::wchar_t;
pub use log::{debug, error, info, log, trace, warn};
pub use std::{ffi::*, mem::transmute};

mod util;
pub use util::*;

mea!(oxide);
mea!(sdk);
mea!(error);

static mut OXIDE: Option<Arc<Mutex<Oxide>>> = None;

unsafe fn main() -> Result<(), Box<dyn Error>> {
    info!("loading");
    OXIDE = Some(Arc::new(Mutex::new(Oxide::init()?)));
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
    };
    load
};
#[link_section = ".text.exit"]
extern "C" fn unload() {
    info!("unload")
}
