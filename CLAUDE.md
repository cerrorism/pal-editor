# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Publish

All commands run from the repo root. Always include `-r win-x64 /p:Platform=x64` — the project defaults to ARM64 on ARM machines, which produces an exe that won't run on x64 Windows.

```powershell
# Debug build
dotnet build editor\editor.csproj -c Debug -r win-x64 /p:Platform=x64

# Release single-file exe (output: editor\bin\publish\win-x64\editor.exe, ~177 MB)
dotnet publish editor\editor.csproj -c Release /p:PublishProfile=win-x64 /p:Platform=x64
```

No test project exists. Functional verification: run the exe, open `1.rpg` (in repo root), and check that Money=605, 逍遙修行=2, MaxHP=168, CurHP=157, Skill1=氣療術, Item0=止血草.

## Architecture

```
editor/
  Models/SaveData.cs          — All domain types + static catalogs (no WPF dependency)
  Services/SaveFileService.cs — Binary load/save (pure C#, BinaryPrimitives)
  MainWindow.xaml             — XAML layout: toolbar + file list panel + TabControl
  MainWindow.xaml.cs          — Code-behind: constructs attr/skill rows, wires events
  App.xaml                    — WPF-UI ThemesDictionary + ControlsDictionary (Theme="Light")
  editor.csproj               — net8.0-windows, WPF-UI 4.3.0
```

**Data flow:** folder → `Directory.GetFiles("*.RPG")` → `ListBox` → `SaveFileService.Load()` → `SaveData` → UI. Edits write back to `SaveData` in-place immediately. Saving calls `SaveFileService.SaveWithBackup()`, which copies `X.RPG → X-bak.RPG` then overwrites `X.RPG`.

**Programmatic UI:** The 10 attribute `NumberBox` rows and 32 skill `ComboBox` rows are built in the `MainWindow` constructor (stored as `_attrBoxes[10]` and `_skillCombos[32]`) rather than in XAML, to avoid repetition. Item rows use a XAML `DataTemplate` inside a `ListView`.

## Binary Format (.RPG files, little-endian)

| Data | Offset | Layout |
|---|---|---|
| Money | `0x0028` | uint32 |
| Cultivation | `0x0244` | uint16 × 6 chars, stride 2 |
| MaxHP | `0x0250` | uint16 × 6 |
| MaxMP | `0x025C` | uint16 × 6 |
| CurrentHP | `0x0268` | uint16 × 6 |
| CurrentMP | `0x0274` | uint16 × 6 |
| MartialArts | `0x02C8` | uint16 × 6 (gap 0x0280–0x02C7 skipped) |
| SpiritPower | `0x02D4` | uint16 × 6 |
| Defense | `0x02E0` | uint16 × 6 |
| Agility | `0x02EC` | uint16 × 6 |
| Luck | `0x02F8` | uint16 × 6 |
| Skills | `0x037C` | `slot*12 + charIndex*2`; byte[0]=skillId, byte[1]=0x01; 0x0000=empty |
| Items | `0x06C0` | 6 bytes each: `[id:u16][count:u16][used:u16]`; stop at id==0 or 256 entries |

Character order: 逍遙(0) 靈兒(1) 月如(2) 巫后(3) 阿奴(4) 不明(5). Skill IDs: 0x27–0x8D. Item IDs: 0x003D–0x0126.

## WPF-UI 4.3.0 Quirks

- XAML namespace: `xmlns:ui="http://schemas.lepo.co/wpfui/2022/xaml"`
- `NumberBox.Value` is `double?` — always extract with `if (box.Value is not double rawVal) return;`
- `NumberBox.ValueChanged` event signature is plain `RoutedEventHandler (object sender, RoutedEventArgs e)` — cast `sender` to read `.Value`
- No `SpinButtonsEnabled`, `SpinButtonPlacementMode`, or `ValidationMode` in v4.x
- Spin buttons consume ~70px; keep `NumberBox` columns ≥160px or digits get clipped
- Accent button: `<ui:Button Appearance="Primary">`
- `Grid` has no `BorderThickness` — wrap in `<Border>` instead
- WPF-UI exports `TextBlock`, `MessageBox`, `MessageBoxButton` — add `using` aliases to resolve ambiguity with `System.Windows.*`
