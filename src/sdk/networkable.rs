use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClientClass{
    _pad1: [usize;2],
    pub get_client_class: cfn!(&ClientClass,&Networkable),
    pub network_name: &'static CStr,
    _pad2: [usize;2],
    class_id: usize
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTNetworkable{
    _pad1: [usize;2],
    pub get_client_class: cfn!(&ClientClass,&Networkable),
    _pad2: [usize;5],
    pub is_dormant: cfn!(bool,&Networkable),
    pub get_index: cfn!(isize,&Networkable),
}


pub type Networkable = WithVmt<VMTNetworkable>;
