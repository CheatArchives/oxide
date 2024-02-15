use crate::*;

#[repr(C)]
#[derive(Debug, Clone)]
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

unsafe impl Send for BaseClient{}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTBaseClient {
    _pad1: [u8; 4 * 6],
    pub LevelInitPostEntity: cfn!(c_void, *const BaseClient),
    pub LevelShutdown: cfn!(c_void, *const BaseClient),
    _pad2: [u8; 4 * 2],
    pub HudProcessInput: cfn!(c_void, *const BaseClient, bool),
    pub HudUpdate: cfn!(c_void, *const BaseClient, bool),
    _pad3: [u8; 4 * 2],
    pub IN_ActivateMouse: cfn!(c_void, *const BaseClient),
    _pad4: [u8; 4 * 20],
    pub FrameStageNotify: cfn!(c_void, *const BaseClient, ClientFrameStage),
    _pad5: [u8; 4 * 23],
    pub GetPlayerView: cfn!(bool, *const BaseClient, *const ViewSetup),
}
