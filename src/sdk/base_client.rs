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


#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTBaseClient {
    _pad1: [u32; 6],
    pub LevelInitPostEntity: cfn!(c_void, *mut BaseClient),
    pub LevelShutdown: cfn!(c_void, *const BaseClient),
    _pad2: [u32; 2],
    pub HudProcessInput: cfn!(c_void, *const BaseClient, bool),
    pub HudUpdate: cfn!(c_void, *const BaseClient, bool),
    _pad3: [u32; 2],
    pub IN_ActivateMouse: cfn!(c_void, *const BaseClient),
    _pad4: [u32; 20],
    pub FrameStageNotify: cfn!(c_void, *const BaseClient, ClientFrameStage),
    _pad5: [u32; 23],
    pub GetPlayerView: cfn!(bool, *const BaseClient, *const ViewSetup),
}
