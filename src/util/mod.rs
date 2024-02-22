use std::mem::MaybeUninit;

use libc::{dlclose, dlerror, dlopen, RTLD_LAZY, RTLD_NOLOAD};

use crate::*;

pub mod macros;
pub use macros::*;


pub unsafe fn vmt_size(vmt: *const c_void) -> usize {
    let mut funcs = transmute::<_, *const *const c_void>(vmt);
    let size = std::mem::size_of::<*const c_void>();

    let mut i = 0;
    while !(*funcs).is_null() {
        i += 1;
        funcs = (funcs as usize + size) as *const *const c_void;
    }

    i * size
}

pub unsafe fn get_handle(name: &str) -> Result<*mut c_void, std::boxed::Box<dyn Error>> {
    let handle = dlopen(CString::new(name)?.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
    if handle.is_null() {
        let error = CStr::from_ptr(dlerror()).to_str()?;
        return Err(std::boxed::Box::new(OxideError::new(&format!(
            "{} handle not found\n {}",
            name, error
        ))));
    }
    dlclose(handle);
    Ok(handle)
}


pub unsafe fn get_plocal() -> Option<&'static mut Entity> {
    let ent = call!(interface_ref!(entity_list), GetClientEntity, call!(interface_ref!(base_engine), GetLocalPlayer));
    if ent.is_null() {
        return None;
    }
    Some(&mut *ent)
}
