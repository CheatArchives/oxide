use crate::*;

pub type Panel = WithVmt<VMTPanel>;


type VPanel = c_uint;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTPanel {
    _pad1: [u32; 37],
    pub get_name: cfn!(*const char, *const Panel, VPanel),
    _pad2: [u32; 4],
    pub paint_traverse: cfn!(c_void, *const Panel, VPanel, bool, bool),
}
