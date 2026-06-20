use std::path::PathBuf;

pub const CHAR_NAMES: [&str; 6] = ["逍遙", "靈兒", "月如", "巫后", "阿奴", "不明"];

#[derive(Clone)]
pub struct CharacterStats {
    pub cultivation: u16,
    pub max_hp: u16,
    pub max_mp: u16,
    pub cur_hp: u16,
    pub cur_mp: u16,
    pub martial_arts: u16,
    pub spirit_power: u16,
    pub defense: u16,
    pub agility: u16,
    pub luck: u16,
    pub skills: [u8; 32],
}

impl Default for CharacterStats {
    fn default() -> Self {
        Self {
            cultivation: 0,
            max_hp: 0,
            max_mp: 0,
            cur_hp: 0,
            cur_mp: 0,
            martial_arts: 0,
            spirit_power: 0,
            defense: 0,
            agility: 0,
            luck: 0,
            skills: [0u8; 32],
        }
    }
}

#[derive(Clone)]
pub struct ItemEntry {
    pub item_id: u16,
    pub count: u16,
    pub used: u16,
}

pub struct SaveData {
    pub file_path: PathBuf,
    pub money: u32,
    pub characters: [CharacterStats; 6],
    pub items: Vec<ItemEntry>,
    pub is_dirty: bool,
}
