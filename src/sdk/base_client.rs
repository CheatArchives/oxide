use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClientFrameStage {
    FrameUndefined = -1,
    FrameStart,
    FrameNetUpdateStart,
    FrameNetUpdatePostdataupdateStart,
    FrameNetUpdatePostdataupdateEnd,
    FrameNetUpdateEnd,
    FrameRenderStart,
    FrameRenderEnd,
}

pub type BaseClient = WithVmt<VMTBaseClient>;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTBaseClient {
    _pad1: [u32; 6],
    pub level_init_post_entity: cfn!(c_void, &'static mut BaseClient),
    pub level_shutdown: cfn!(c_void, &'static BaseClient),
    _pad2: [u32; 2],
    pub hud_process_input: cfn!(c_void, &'static BaseClient, bool),
    pub hud_update: cfn!(c_void, &'static BaseClient, bool),
    _pad3: [u32; 2],
    pub in_activate_mouse: cfn!(c_void, &'static BaseClient),
    _pad4: [u32; 20],
    pub frame_stage_notify: cfn!(c_void, &'static BaseClient, ClientFrameStage),
    _pad5: [u32; 23],
    pub get_player_view: cfn!(bool, &'static BaseClient, &'static ViewSetup),
}
