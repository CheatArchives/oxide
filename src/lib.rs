use std::{error::Error, panic::catch_unwind, thread, time::Duration};

use ctor::{ctor, dtor};

pub use log::{debug, error, info, log, trace, warn};
pub use std::{ffi::*, mem::transmute};
pub use libc::wchar_t;

mod globals;
mod sdk;
mod util;
mod error;

pub use error::OxideError;
pub use sdk::*;
pub use util::*;


unsafe fn main() {
    info!("loading");
    let base_client = globals::init_globals().unwrap();
    info!("loaded");
    loop {
        thread::sleep(Duration::from_secs(5));
    }
}

#[ctor]
unsafe fn load() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    thread::spawn(|| main());
}

#[dtor]
fn unload() {
    info!("unloaded");
}
