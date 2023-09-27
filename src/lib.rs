use std::{thread::{self, sleep}, time::Duration};
use ctor::{ctor, dtor};
use std::ffi::*;

fn cs(str: &str) -> CString {
    CString::new(str).unwrap()
}

#[ctor]
fn load() {
    thread::spawn(|| {
        sleep(Duration::from_secs(5));
        print!("test");
        eprint!("teste");
        unsafe{
            let handle = libc::dlopen(cs("target/").as_ptr(), libc::RTLD_NOLOAD | libc::RTLD_LAZY);
            dbg!(handle);
        }
    });

}

fn self_unload() {
}

#[dtor]
fn unload() {

}
