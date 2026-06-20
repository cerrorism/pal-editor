# 仙劍奇俠傳 存檔修改器

A save file editor for the classic 1995 RPG **PAL (仙劍奇俠傳 / Legend of Sword and Fairy)**.

## Features

- Browse a folder of `.RPG` save files and load any of them with one click
- Edit character attributes (修行, 體力, 真氣, 武術, 靈力, 防禦, 身法, 吉運) for all six party members
- Assign any of the 103 learnable skills to each of a character's 32 skill slots
- Add, remove, and adjust quantities of any of the 234 in-game items
- Edit money (金錢)
- **Safe saving**: the original `.RPG` is backed up to `X-bak.RPG` before any changes are written

## Requirements

Windows 10 / 11 x64. No installation needed — the release build is a single self-contained `.exe`.

## Build

Requires [.NET 8 SDK](https://dotnet.microsoft.com/download/dotnet/8).

```powershell
dotnet publish editor\editor.csproj -c Release /p:PublishProfile=win-x64 /p:Platform=x64
```

Output: `editor\bin\publish\win-x64\editor.exe`

## Stack

- .NET 8, WPF
- [WPF-UI 4.3.0](https://github.com/lepoco/wpfui) — Fluent Design theme
