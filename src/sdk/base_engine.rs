use crate::*;

pub type BaseEngine = WithVmt<VMTBaseEngine>;

const MAX_PLAYER_NAME_LENGTH: usize = 32;
const SIGNED_GUID_LEN: usize = 32;
const MAX_CUSTOM_FILES: usize = 4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlayerInfo {
    name: [c_char; MAX_PLAYER_NAME_LENGTH],
    user_id: isize,
    guid: [c_char; SIGNED_GUID_LEN + 1],
    friends_id: usize,
    friends_name: [c_char; SIGNED_GUID_LEN + 1],
    fakeplayer: bool,
    ishltv: bool,
    custom_files: [usize; MAX_CUSTOM_FILES],
    files_downloaded: c_uchar,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTBaseEngine {
    _pad1: [u32; 5],
    pub get_screen_size: cfn!(c_void, &'static BaseEngine, &'static isize, &'static isize),
    _pad2: [u32; 2],
    pub get_player_info: cfn!(bool, &'static BaseEngine, &'static isize, &'static PlayerInfo),
    _pad3: [u32; 3],
    pub get_local_player: cfn!(isize, *const BaseEngine),
    _pad4: [u32; 6],
    pub get_view_angles: cfn!(c_void, &'static BaseEngine, Angles),
    pub set_view_angles: cfn!(c_void, &'static BaseEngine, Angles),
    pub get_max_clients: cfn!(isize, &'static BaseEngine),
    _pad5: [u32; 4],
    pub is_in_game: cfn!(bool, &'static BaseEngine),
    pub is_connected: cfn!(bool, &'static BaseEngine),
    _pad6: [u32; 8],
    pub world_to_screen_matrix: cfn!(VMatrix, &'static BaseEngine),
    _pad7: [u32; 48],
    pub is_taking_screenshot: cfn!(bool, &'static BaseEngine),
}
