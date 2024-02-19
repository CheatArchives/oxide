use crate::*;

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

#[macro_export]
macro_rules! o {
    () => {
        *(OXIDE as *mut _ as *mut Oxide)
    };
}

#[macro_export]
macro_rules! m {
    () => {
        *(MENU as *mut _ as *mut Menu)
    };
}

#[macro_export]
macro_rules! i {
    ($n:ident) => {
        (*o!().interfaces.$n.get_vmt())
    };
}

#[macro_export]
macro_rules! r {
    ($n:ident) => {
        o!().interfaces.$n.interface_ref
    };
}

#[macro_export]
macro_rules! c {
    ($i:ident,$f:ident $(,$args: expr)*) => {
        (i!($i).$f)(r!($i),$($args),*)
    };
}

