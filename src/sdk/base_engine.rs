use crate::*;

pub type BaseEngine = WithVmt<VMTBaseEngine>;

const MAX_PLAYER_NAME_LENGTH: usize = 32;
const SIGNED_GUID_LEN: usize = 32;
const MAX_CUSTOM_FILES: usize = 4;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    name: [c_char; MAX_PLAYER_NAME_LENGTH],
    user_id: c_int,
    guid: [c_char; SIGNED_GUID_LEN + 1],
    friends_id: c_uint,
    friends_name: [c_char; SIGNED_GUID_LEN + 1],
    fakeplayer: bool,
    ishltv: bool,
    custom_files: [c_uint; MAX_CUSTOM_FILES],
    files_downloaded: c_uchar,
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTBaseEngine {
    _pad1: [u32; 5],
    pub GetScreenSize: cfn!(c_void, *const BaseEngine, *const c_int, *const c_int),
    _pad2: [u32; 2],
    pub GetPlayerInfo: cfn!(bool, *const BaseEngine, *const c_int, *const PlayerInfo),
    _pad3: [u32; 3],
    pub GetLocalPlayer: cfn!(c_int, *const BaseEngine),
    _pad4: [u32; 6],
    pub GetViewAngles: cfn!(c_void, *const BaseEngine, Angles),
    pub SetViewAngles: cfn!(c_void, *const BaseEngine, Angles),
    pub GetMaxClients: cfn!(c_int, *const BaseEngine),
    _pad5: [u32; 4],
    pub IsInGame: cfn!(bool, *const BaseEngine),
    pub IsConnected: cfn!(bool, *const BaseEngine),
    _pad6: [u32; 8],
    pub WorldToScreenMatrix: cfn!(VMatrix, *const BaseEngine),
    _pad7: [u32; 48],
    pub IsTakingScreenshot: cfn!(bool, *const BaseEngine),
}
