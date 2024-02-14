use ctor::{ctor, dtor};
use libc::{dladdr, dlclose, dlerror, dlopen, Dl_info, RTLD_LAZY, RTLD_NOLOAD};
use log::{debug, error, info};
use std::ffi::*;
use std::{
    mem::MaybeUninit,
    thread::{self, sleep},
    time::Duration,
};

fn src(str: &str) -> *const i8 {
    CString::new(str).unwrap().as_ptr()
}
fn scr(str: *const i8) -> String {
    unsafe { CStr::from_ptr(str).to_str().unwrap().to_owned() }
}

#[ctor]
fn load() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    thread::spawn(|| unsafe {
        info!("loading");
        self_unload();
    });
}

unsafe fn self_unload() {
    info!("self unloading");
    #[allow(invalid_value)]
    let mut info: Dl_info = MaybeUninit::<Dl_info>::uninit().assume_init();
    dladdr(load as *const _, &mut info as *mut _);
    let handle = dlopen(info.dli_fname, RTLD_LAZY | RTLD_NOLOAD);
    dlclose(handle);
    dlclose(handle);
}

#[dtor]
fn unload() {
    info!("unloaded")
}
