use crate::*;

pub type Panel = WithVmt<VMTPanel>;


type VPanel = c_uint;


#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTPanel {
    _pad1: [u8; 4 * 37],
    pub GetName: cfn!(*const char, *const Panel, VPanel),
    _pad2: [u8; 4 * 4],
    pub PaintTraverse: cfn!(c_void, *const Panel, VPanel, bool, bool),
}
