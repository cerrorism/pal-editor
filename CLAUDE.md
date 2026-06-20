# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build

```powershell
cargo build          # debug
cargo build --release
```

Output: `target\release\pal-editor.exe` (~9 MB)

No test project exists. Functional verification: run the exe, browse to the repo root, open a `.RPG` save file, and confirm character names, skills, and items display correctly in Traditional Chinese.

## Architecture

```
src/
  main.rs        — eframe entry point; calls setup_fonts() to embed jf 開放粉圓
  app.rs         — PalApp struct: all egui UI logic (panels, tabs, state)
  save_data.rs   — SaveData, CharacterStats, ItemEntry, CHAR_NAMES
  save_file.rs   — binary load/save; list_rpg_files(); backup logic
  catalogs.rs    — SKILL_CATALOG (103 entries, 0x27–0x8D) and ITEM_CATALOG (234 entries, 0x003D–0x0126)
assets/
  jf-openhuninn-2.1.ttf   — Traditional Chinese font, embedded via include_bytes!
```

**Data flow:** folder → `list_rpg_files()` → sidebar list → `save_file::load()` → `SaveData` → egui UI. Edits update `SaveData` in-place and set `is_dirty = true`. Save calls `save_file::save_with_backup()`: copies `X.RPG → X-bak.RPG`, then overwrites `X.RPG`.

**UI layout:** `TopPanel` (toolbar) + `SidePanel` (file list) + `CentralPanel` (3-tab: 屬性 / 仙術 / 物品). Character selection per tab is independent (`attr_char_idx`, `skill_char_idx`). No event suppression flags needed — egui's immediate mode naturally avoids cascading updates.

## Binary Format (.RPG files, little-endian)

| Data | Offset | Layout |
|---|---|---|
| Money | `0x0028` | uint32 |
| Cultivation | `0x0244` | uint16 × 6 chars, stride 2 |
| MaxHP | `0x0250` | uint16 × 6 |
| MaxMP | `0x025C` | uint16 × 6 |
| CurrentHP | `0x0268` | uint16 × 6 |
| CurrentMP | `0x0274` | uint16 × 6 |
| MartialArts | `0x02C8` | uint16 × 6 (equipment slots occupy 0x0280–0x02C7) |
| SpiritPower | `0x02D4` | uint16 × 6 |
| Defense | `0x02E0` | uint16 × 6 |
| Agility | `0x02EC` | uint16 × 6 |
| Luck | `0x02F8` | uint16 × 6 |
| Skills | `0x037C` | `slot×12 + charIndex×2`; byte[0]=skillId, byte[1]=0x01; 0x0000=empty |
| Items | `0x06C0` | 6 bytes each: `[id:u16][count:u16][used:u16]`; stop at id==0 or 256 entries |

Character order: 逍遙(0) 靈兒(1) 月如(2) 巫后(3) 阿奴(4) 不明(5). Skill IDs: 0x27–0x8D. Item IDs: 0x003D–0x0126.

Stat values in the save are **base stats**; the game adds equipment bonuses at runtime. Editors should read/write base stats only.

## Dependencies

- `eframe` / `egui` 0.31 — immediate mode GUI
- `rfd` 0.15 — native folder picker dialog (Windows)
