use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClientClass{
    _pad1: [usize;2],
    pub get_client_class: cfn!(&mut ClientClass,&mut Networkable),
    pub network_name: *const c_char,
    _pad2: [usize;2],
    class_id: usize
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTNetworkable{
    _pad1: [usize;2],
    pub get_client_class: cfn!(&mut ClientClass,&mut Networkable),
    _pad2: [usize;5],
    pub is_dormant: cfn!(bool,&mut Networkable),
    pub get_index: cfn!(isize,&mut Networkable),
}


pub type Networkable = WithVmt<VMTNetworkable>;
