# 仙劍奇俠傳 存檔修改器

A save file editor for the classic 1995 RPG **PAL (仙劍奇俠傳 / Legend of Sword and Fairy)**.

Built with Rust + [egui](https://github.com/emilk/egui). Single self-contained `.exe`, no installer, no runtime required.

---

## Features

- Browse a folder of `.RPG` save files and load any with one click
- Edit **money** (金錢)
- Edit **character attributes** for all six party members (修行, 最大體力, 最大真氣, 目前體力, 目前真氣, 武術, 靈力, 防禦, 身法, 吉運)
- Assign any of the **103 learnable skills** to each character's 32 skill slots
- Add, remove, and edit quantities of any of the **234 in-game items**
- **Safe saving**: automatically backs up `X.RPG → X-bak.RPG` before writing

## Download

Grab the latest `pal-editor.exe` from the [Releases](../../releases) page. Drop it anywhere and run it — no install needed.

Requires Windows 10 / 11 x64.

## Build from Source

Requires the [Rust toolchain](https://rustup.rs/).

```powershell
cargo build --release
```

Output: `target\release\pal-editor.exe` (~9 MB, includes embedded font)

## Project Structure

```
src/
  main.rs        — eframe entry point, font setup
  app.rs         — full egui UI (toolbar, file list, attribute/skill/item tabs)
  save_data.rs   — domain types (SaveData, CharacterStats, ItemEntry)
  save_file.rs   — binary load/save, backup logic, RPG file scanner
  catalogs.rs    — static skill (103 entries) and item (234 entries) catalogs
assets/
  jf-openhuninn-2.1.ttf   — embedded Traditional Chinese font (SIL OFL)
```

## Binary Format

The `.RPG` save files are little-endian binary. Key offsets:

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
| Skills | `0x037C` | `slot×12 + charIndex×2`; byte[0]=skillId, byte[1]=0x01 (0x00=empty) |
| Items | `0x06C0` | 6 bytes each: `[id:u16][count:u16][used:u16]`; stop at id==0 |

Character order: 逍遙(0) 靈兒(1) 月如(2) 巫后(3) 阿奴(4) 不明(5)

> Note: stat values in the editor are **base stats** as stored in the save file. The game adds equipment bonuses on top when rendering the HUD, so displayed values may differ slightly from what you see in-game.

## Credits

- [jf 開放粉圓](https://github.com/justfont/open-huninn-font) — Traditional Chinese font by justfont, licensed under [SIL Open Font License 1.1](https://scripts.sil.org/OFL)
- [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) — immediate mode GUI framework for Rust
