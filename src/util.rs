use libc::{dlclose, dlerror, dlopen, RTLD_LAZY, RTLD_NOLOAD};

use crate::*;

pub unsafe fn vmt_size(vmt: *const c_void) -> usize {
    let mut funcs = transmute::<_, *const *const c_void>(vmt);
    let size = std::mem::size_of::<*const c_void>();

    let mut i = 0;
    while !funcs.read().is_null() {
        i += 1;
        funcs = (funcs as usize + size) as *const *const c_void;
    }

    i * size
}

pub unsafe fn get_handle<T>(name: &str) -> Result<*mut T, Box<dyn Error>> {
    let handle = dlopen(CString::new(name)?.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
    if handle.is_null() {
        let error = CStr::from_ptr(dlerror()).to_str()?;
        return Err(Box::new(OxideError::new(&format!(
            "{} handle not found\n {}",
            name, error
        ))));
    }
    dlclose(handle);
    Ok(handle as *mut T)
}

#[macro_export]
macro_rules! cfn {
    ($r:ty,$($t:ty),*) => {unsafe extern "C-unwind" fn($($t), *) -> $r}
}
#[macro_export]
macro_rules! mea {
    ($m:ident) => {
        pub mod $m;
        pub use $m::*;
    };
}
