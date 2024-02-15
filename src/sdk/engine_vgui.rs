use crate::*;


pub type EngineVgui = WithVmt<VMTEngineVgui>;

#[allow(non_snake_case,non_camel_case_types,dead_code)]
#[repr(C)]
#[derive(Debug,Clone)]
pub struct VMTEngineVgui {
    _pad1: [u8;4*15],
    pub Paint:   cfn!(c_void, *const WithVmt<VMTBaseClient>),
    pub test:  cfn!(c_void, *const WithVmt<VMTBaseClient>)
}
