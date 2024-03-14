
use std::ffi::c_char;

use crate::math::angles::Angles;

use super::*;

pub type BaseEngine = WithVmt<VMTBaseEngine>;

const MAX_PLAYER_NAME_LENGTH: usize = 32;
const SIGNED_GUID_LEN: usize = 32;
const MAX_CUSTOM_FILES: usize = 4;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: [u8; MAX_PLAYER_NAME_LENGTH],
    pub user_id: isize,
    pub guid: [u8; SIGNED_GUID_LEN + 1],
    pub friends_id: usize,
    pub friends_name: [c_char; SIGNED_GUID_LEN + 1],
    pub fakeplayer: bool,
    pub ishltv: bool,
    pub custom_files: [usize; MAX_CUSTOM_FILES],
    pub files_downloaded: c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTBaseEngine {
    _pad1: [u32; 5],
    pub get_screen_size: cfn!((), &BaseEngine, &isize, &isize),
    _pad2: [u32; 2],
    pub get_player_info: cfn!(
        bool,
        &BaseEngine,
        isize,
        &mut PlayerInfo
    ),
    _pad3: [u32; 3],
    pub get_local_player: cfn!(isize, &BaseEngine),
    _pad4: [u32; 6],
    pub get_view_angles: cfn!((), &BaseEngine, Angles),
    pub set_view_angles: cfn!((), &BaseEngine, Angles),
    pub get_max_clients: cfn!(isize, &BaseEngine),
    _pad5: [u32; 4],
    pub is_in_game: cfn!(bool, &BaseEngine),
    pub is_connected: cfn!(bool, &BaseEngine),
    _pad6: [u32; 8],
    pub world_to_screen_matrix: cfn!(VMatrix, &BaseEngine),
    _pad7: [u32; 48],
    pub is_taking_screenshot: cfn!(bool, &BaseEngine),
}
