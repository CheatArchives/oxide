use crate::*;

pub type Panel = WithVmt<VMTPanel>;


type VPanel = usize;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTPanel {
    _pad1: [u32; 37],
    pub get_name: cfn!(&'static char, &'static Panel, VPanel),
    _pad2: [u32; 4],
    pub paint_traverse: cfn!(c_void, &'static Panel, VPanel, bool, bool),
}
