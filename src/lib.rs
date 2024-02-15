use std::{error::Error, panic::catch_unwind, sync::{Arc, Mutex}, thread, time::Duration};

use ctor::{ctor, dtor};

pub use log::{debug, error, info, log, trace, warn};
pub use std::{ffi::*, mem::transmute};
pub use libc::wchar_t;

mod globals;
mod util;
pub use util::*;

mea!(oxide);
mea!(sdk);
mea!(error);

static mut OXIDE: Option<Arc<Mutex<Oxide>>> = None;

unsafe fn main()-> Result<(),Box<dyn Error>> {
    info!("loading");
    OXIDE = Some(Arc::new(Mutex::new(Oxide::init()?)));
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
    thread::spawn(|| {
        if let Err(e) = main() {
            error!("{}\n{:?}",e,e)
        }
        
    });
}

#[dtor]
fn unload() {
    info!("unloaded");
}
