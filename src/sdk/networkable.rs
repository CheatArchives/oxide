use crate::*;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClientClass{
    _pad1: [usize;2],
    pub GetClientClass: cfn!(&mut ClientClass,&mut Networkable),
    pub network_name: *const c_char,
    _pad2: [usize;2],
    class_id: usize
}
#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTNetworkable{
    _pad1: [usize;2],
    pub GetClientClass: cfn!(&mut ClientClass,&mut Networkable),
    _pad2: [usize;5],
    pub IsDormant: cfn!(bool,&mut Networkable),
    pub GetIndex: cfn!(isize,&mut Networkable),
}


pub type Networkable = WithVmt<VMTNetworkable>;
