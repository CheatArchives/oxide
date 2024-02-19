use libc::{dlclose, dlerror, dlopen, RTLD_LAZY, RTLD_NOLOAD};

use crate::*;

pub mod macros;
pub use macros::*;

mea!(vec);

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

pub unsafe fn get_handle(name: &str) -> Result<*mut c_void, Box<dyn Error>> {
    let handle = dlopen(CString::new(name)?.as_ptr(), RTLD_NOLOAD | RTLD_LAZY);
    if handle.is_null() {
        let error = CStr::from_ptr(dlerror()).to_str()?;
        return Err(Box::new(OxideError::new(&format!(
            "{} handle not found\n {}",
            name, error
        ))));
    }
    dlclose(handle);
    Ok(handle)
}

pub unsafe fn get_networkabe(ent: &Entity) -> &Networkable{
    return &*((ent as *const Entity as usize + 0x8) as *mut c_void as *mut _ as *mut Networkable);
}

pub unsafe fn get_ent(id: i32) -> Option<&'static mut Entity> {
    let ent = c!(entity_list, GetClientEntity, id);
    if ent.is_null() {
        return None;
    }
    let ent = &mut *ent;
    let net = get_networkabe(ent);

    if (!ent || (net.c().IsDormant)() || (ent.c().IsAlive))
                continue;

    if (*(*ent).vmt). {
        return None;
    }
    return Some(ent)
}

pub unsafe fn get_plocal() -> Option<&'static mut Entity> {
    let ent = c!(entity_list, GetClientEntity, c!(base_engine, GetLocalPlayer));
    if ent.is_null() {
        return None;
    }
    return Some(&mut *ent)
}
