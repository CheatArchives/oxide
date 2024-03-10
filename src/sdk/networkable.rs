use std::ffi::c_char;

use crate::{cfn, sdk::*};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClientClass{
    _pad1: [usize;2],
    pub get_client_class: cfn!(&ClientClass, &Networkable),
    pub network_name: *const c_char,
    _pad2: [usize;2],
    class_id: usize
}
#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct VMTNetworkable{
    _pad1: [usize;2],
    pub get_client_class: cfn!(ClientClass,*const Networkable),
    _pad2: [usize;5],
    pub is_dormant: cfn!(bool,*const Networkable),
    pub get_index: cfn!(isize,*const Networkable),
}

pub type Networkable = WithVmt<VMTNetworkable>;

