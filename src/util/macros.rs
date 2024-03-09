#[macro_export]
macro_rules! cfn {
    ($r:ty,$($t:ty),*) => {
        unsafe extern "C-unwind" fn($($t), *) -> $r
    }
}

#[macro_export]
macro_rules! module_export {
    ($m:ident) => {
        pub mod $m;
        pub use $m::*;
    };
}

#[macro_export]
macro_rules! oxide {
    () => {
        unsafe { &mut *(OXIDE.unwrap() as *mut _ as *mut Oxide) }
    };
}

#[macro_export]
macro_rules! draw {
    () => {
        unsafe { &mut *(DRAW.unwrap() as *mut _ as *mut Draw) }
    };
}

#[macro_export]
macro_rules! settings {
    () => {
        unsafe { &mut *(SETTINGS.unwrap() as *mut _ as *mut Settings) }
    };
}

#[macro_export]
macro_rules! interface_vmt {
    ($n:ident) => {
        (*oxide!().interfaces.$n.get_vmt())
    };
}

#[macro_export]
macro_rules! interface {
    ($n:ident) => {
        oxide!().interfaces.$n.interface_ref()
    };
}
#[macro_export]
macro_rules! call {
    ($i:expr,$f:ident $(,$args: expr)*) => {
        ((*$i.vmt).$f)($i,$($args),*)
    };
}
#[macro_export]
macro_rules! call_interface {
    ($i:ident,$f:ident $(,$args: expr)*) => {
        ((*interface_ref!($i)).vmt.$f)(interface_ref!($i),$($args),*)
    };
}

#[macro_export]
macro_rules! impl_has_vmt {
    ($t:tt,$tv:tt) => {
        impl HasVmt<$tv> for $t {
            fn get_vmt(&self) -> &'static $tv {
                unsafe { &*self.vmt }
            }

            fn set_vmt(&mut self, vmt: *mut $tv) {
                self.vmt = vmt
            }
        }
    };
}

#[macro_export]
macro_rules! hex_to_rgb {
    ($h:expr) => {
        (($h >> 16) as u8, ($h >> 8) as u8, $h as u8)
    };
}
#[macro_export]
macro_rules! rgb_to_hex {
    ($r:expr,$g:expr, $b:expr) => {
        (($r as usize) << 16) + (($g as usize) << 8) + $b as usize
    };
}

#[macro_export]
macro_rules! amt {
    ($t:ty) => {
        Arc<Mutex<$t>>
    };
}

#[macro_export]
macro_rules! am {
    ($v:expr) => {
        Arc::new(Mutex::new($v))
    };
}
