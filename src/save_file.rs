use std::fs;
use std::path::{Path, PathBuf};
use crate::save_data::{CharacterStats, ItemEntry, SaveData};

const MONEY_OFF: usize = 0x0028;
const CULTIVATION_BASE: usize = 0x0244;
const MAX_HP_BASE: usize = 0x0250;
const MAX_MP_BASE: usize = 0x025C;
const CUR_HP_BASE: usize = 0x0268;
const CUR_MP_BASE: usize = 0x0274;
const MARTIAL_ARTS_BASE: usize = 0x02C8;
const SPIRIT_POWER_BASE: usize = 0x02D4;
const DEFENSE_BASE: usize = 0x02E0;
const AGILITY_BASE: usize = 0x02EC;
const LUCK_BASE: usize = 0x02F8;
const SKILL_BASE: usize = 0x037C;
const ITEM_BASE: usize = 0x06C0;
const MIN_FILE_SIZE: usize = 0x0CC0;
const MAX_ITEMS: usize = 256;

fn read_u16(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([buf[off], buf[off + 1]])
}

fn read_u32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

fn write_u16(buf: &mut [u8], off: usize, val: u16) {
    let bytes = val.to_le_bytes();
    buf[off] = bytes[0];
    buf[off + 1] = bytes[1];
}

fn write_u32(buf: &mut [u8], off: usize, val: u32) {
    let bytes = val.to_le_bytes();
    buf[off] = bytes[0];
    buf[off + 1] = bytes[1];
    buf[off + 2] = bytes[2];
    buf[off + 3] = bytes[3];
}

pub fn load(path: &Path) -> Result<SaveData, String> {
    let buf = fs::read(path).map_err(|e| e.to_string())?;
    if buf.len() < MIN_FILE_SIZE {
        return Err(format!("File too small: {} bytes (need {})", buf.len(), MIN_FILE_SIZE));
    }

    let money = read_u32(&buf, MONEY_OFF);

    let mut characters: [CharacterStats; 6] = Default::default();
    for c in 0..6usize {
        let ch = &mut characters[c];
        ch.cultivation  = read_u16(&buf, CULTIVATION_BASE  + c * 2);
        ch.max_hp       = read_u16(&buf, MAX_HP_BASE        + c * 2);
        ch.max_mp       = read_u16(&buf, MAX_MP_BASE        + c * 2);
        ch.cur_hp       = read_u16(&buf, CUR_HP_BASE        + c * 2);
        ch.cur_mp       = read_u16(&buf, CUR_MP_BASE        + c * 2);
        ch.martial_arts = read_u16(&buf, MARTIAL_ARTS_BASE  + c * 2);
        ch.spirit_power = read_u16(&buf, SPIRIT_POWER_BASE  + c * 2);
        ch.defense      = read_u16(&buf, DEFENSE_BASE       + c * 2);
        ch.agility      = read_u16(&buf, AGILITY_BASE       + c * 2);
        ch.luck         = read_u16(&buf, LUCK_BASE          + c * 2);

        for slot in 0..32usize {
            let off = SKILL_BASE + slot * 12 + c * 2;
            let id   = buf[off];
            let flag = buf[off + 1];
            ch.skills[slot] = if flag == 0x01 { id } else { 0x00 };
        }
    }

    let mut items = Vec::new();
    for i in 0..MAX_ITEMS {
        let off = ITEM_BASE + i * 6;
        if off + 6 > buf.len() { break; }
        let item_id = read_u16(&buf, off);
        if item_id == 0 { break; }
        let count = read_u16(&buf, off + 2);
        let used  = read_u16(&buf, off + 4);
        items.push(ItemEntry { item_id, count, used });
    }

    Ok(SaveData {
        file_path: path.to_path_buf(),
        money,
        characters,
        items,
        is_dirty: false,
    })
}

pub fn save_with_backup(data: &SaveData) -> Result<(), String> {
    let path = &data.file_path;
    let mut buf = fs::read(path).map_err(|e| e.to_string())?;

    // Write money
    write_u32(&mut buf, MONEY_OFF, data.money);

    // Write character stats
    for c in 0..6usize {
        let ch = &data.characters[c];
        write_u16(&mut buf, CULTIVATION_BASE  + c * 2, ch.cultivation);
        write_u16(&mut buf, MAX_HP_BASE        + c * 2, ch.max_hp);
        write_u16(&mut buf, MAX_MP_BASE        + c * 2, ch.max_mp);
        write_u16(&mut buf, CUR_HP_BASE        + c * 2, ch.cur_hp);
        write_u16(&mut buf, CUR_MP_BASE        + c * 2, ch.cur_mp);
        write_u16(&mut buf, MARTIAL_ARTS_BASE  + c * 2, ch.martial_arts);
        write_u16(&mut buf, SPIRIT_POWER_BASE  + c * 2, ch.spirit_power);
        write_u16(&mut buf, DEFENSE_BASE       + c * 2, ch.defense);
        write_u16(&mut buf, AGILITY_BASE       + c * 2, ch.agility);
        write_u16(&mut buf, LUCK_BASE          + c * 2, ch.luck);

        for slot in 0..32usize {
            let off = SKILL_BASE + slot * 12 + c * 2;
            let id = ch.skills[slot];
            buf[off]     = id;
            buf[off + 1] = if id != 0 { 0x01 } else { 0x00 };
        }
    }

    // Clear and rewrite items
    let clear_end = (ITEM_BASE + MAX_ITEMS * 6).min(buf.len());
    for b in &mut buf[ITEM_BASE..clear_end] { *b = 0; }
    for (i, item) in data.items.iter().enumerate() {
        let off = ITEM_BASE + i * 6;
        write_u16(&mut buf, off,     item.item_id);
        write_u16(&mut buf, off + 2, item.count);
        write_u16(&mut buf, off + 4, item.used);
    }

    // Create backup
    let backup = make_backup_path(path);
    fs::copy(path, &backup).map_err(|e| format!("Backup failed: {e}"))?;

    // Overwrite original
    fs::write(path, &buf).map_err(|e| e.to_string())?;
    Ok(())
}

fn make_backup_path(path: &Path) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let parent = path.parent().unwrap_or(Path::new("."));
    let candidate = parent.join(format!("{stem}-bak.RPG"));
    if !candidate.exists() {
        return candidate;
    }
    let mut n = 2u32;
    loop {
        let c = parent.join(format!("{stem}-bak{n}.RPG"));
        if !c.exists() { return c; }
        n += 1;
    }
}

pub fn list_rpg_files(folder: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(folder) else { return vec![]; };
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .map(|ext| ext.to_ascii_uppercase() == "RPG")
                    .unwrap_or(false)
        })
        .collect();
    files.sort_by(|a, b| {
        a.file_name().unwrap_or_default().to_ascii_uppercase()
            .cmp(&b.file_name().unwrap_or_default().to_ascii_uppercase())
    });
    files
}
