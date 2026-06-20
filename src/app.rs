use std::path::PathBuf;
use egui::{ComboBox, DragValue, ScrollArea, Ui};

use crate::catalogs::{ITEM_CATALOG, SKILL_CATALOG};
use crate::save_data::{CharacterStats, ItemEntry, SaveData, CHAR_NAMES};
use crate::save_file;

const STAT_LABELS: [&str; 10] = [
    "修行", "最大體力", "最大真氣", "目前體力", "目前真氣",
    "武術", "靈力", "防禦", "身法", "吉運",
];

fn stat_field<'a>(ch: &'a mut CharacterStats, idx: usize) -> &'a mut u16 {
    match idx {
        0 => &mut ch.cultivation,
        1 => &mut ch.max_hp,
        2 => &mut ch.max_mp,
        3 => &mut ch.cur_hp,
        4 => &mut ch.cur_mp,
        5 => &mut ch.martial_arts,
        6 => &mut ch.spirit_power,
        7 => &mut ch.defense,
        8 => &mut ch.agility,
        _ => &mut ch.luck,
    }
}

#[derive(PartialEq)]
enum Tab { Attributes, Skills, Items }

pub struct PalApp {
    folder_path: Option<PathBuf>,
    save_files: Vec<PathBuf>,
    selected_file_idx: Option<usize>,
    save_data: Option<SaveData>,
    attr_char_idx: usize,
    skill_char_idx: usize,
    tab: Tab,
    status_msg: String,
    add_item_id: u16,
}

impl Default for PalApp {
    fn default() -> Self {
        Self {
            folder_path: None,
            save_files: Vec::new(),
            selected_file_idx: None,
            save_data: None,
            attr_char_idx: 0,
            skill_char_idx: 0,
            tab: Tab::Attributes,
            status_msg: String::new(),
            add_item_id: ITEM_CATALOG[0].0,
        }
    }
}

impl eframe::App for PalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.side_panel(ctx);
        self.central_panel(ctx);
    }
}

impl PalApp {
    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("📂 瀏覽資料夾").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.save_files = save_file::list_rpg_files(&folder);
                        self.folder_path = Some(folder);
                        self.selected_file_idx = None;
                        self.save_data = None;
                        self.status_msg.clear();
                    }
                }

                // Folder path (truncated)
                let folder_str = self
                    .folder_path
                    .as_deref()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default();
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(if folder_str.is_empty() { "（未選擇資料夾）" } else { &folder_str })
                            .weak(),
                    )
                    .truncate(),
                );

                ui.separator();

                // Money
                ui.label("金錢:");
                let money_enabled = self.save_data.is_some();
                ui.add_enabled_ui(money_enabled, |ui| {
                    if let Some(data) = &mut self.save_data {
                        let old = data.money;
                        let resp = ui.add(
                            DragValue::new(&mut data.money)
                                .range(0u32..=4_294_967_295u32)
                                .speed(1.0),
                        );
                        if resp.changed() && data.money != old {
                            data.is_dirty = true;
                        }
                    } else {
                        let mut dummy = 0u32;
                        ui.add(DragValue::new(&mut dummy));
                    }
                });

                ui.separator();

                let can_save = self.save_data.as_ref().map(|d| d.is_dirty).unwrap_or(false);
                ui.add_enabled_ui(can_save, |ui| {
                    if ui.button("💾 儲存").clicked() {
                        if let Some(data) = &self.save_data {
                            match save_file::save_with_backup(data) {
                                Ok(()) => {
                                    self.status_msg = "儲存成功，已建立備份。".to_owned();
                                    if let Some(d) = &mut self.save_data {
                                        d.is_dirty = false;
                                    }
                                }
                                Err(e) => self.status_msg = format!("儲存失敗：{e}"),
                            }
                        }
                    }
                });

                if !self.status_msg.is_empty() {
                    ui.separator();
                    ui.label(
                        egui::RichText::new(&self.status_msg)
                            .small()
                            .color(egui::Color32::DARK_GREEN),
                    );
                }
            });
        });
    }

    fn side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("file_list")
            .min_width(160.0)
            .max_width(280.0)
            .show(ctx, |ui| {
                ui.heading("存檔清單");
                ui.separator();
                ScrollArea::vertical().show(ui, |ui| {
                    for i in 0..self.save_files.len() {
                        let name = self.save_files[i]
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        let selected = self.selected_file_idx == Some(i);
                        if ui.selectable_label(selected, &name).clicked() && !selected {
                            self.selected_file_idx = Some(i);
                            let path = self.save_files[i].clone();
                            match save_file::load(&path) {
                                Ok(data) => {
                                    self.save_data = Some(data);
                                    self.status_msg.clear();
                                }
                                Err(e) => self.status_msg = format!("讀取失敗：{e}"),
                            }
                        }
                    }
                });
            });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.save_data.is_none() {
                ui.centered_and_justified(|ui| {
                    ui.label("請選擇左側的存檔檔案");
                });
                return;
            }

            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Attributes, "屬性");
                ui.selectable_value(&mut self.tab, Tab::Skills,     "仙術");
                ui.selectable_value(&mut self.tab, Tab::Items,      "物品");
            });
            ui.separator();

            match self.tab {
                Tab::Attributes => self.tab_attributes(ui),
                Tab::Skills     => self.tab_skills(ui),
                Tab::Items      => self.tab_items(ui),
            }
        });
    }

    fn tab_attributes(&mut self, ui: &mut Ui) {
        let data = self.save_data.as_mut().unwrap();

        ui.horizontal(|ui| {
            ui.label("角色：");
            ComboBox::from_id_salt("attr_char")
                .selected_text(CHAR_NAMES[self.attr_char_idx])
                .show_ui(ui, |ui| {
                    for (i, name) in CHAR_NAMES.iter().enumerate() {
                        ui.selectable_value(&mut self.attr_char_idx, i, *name);
                    }
                });
        });

        ui.add_space(8.0);

        egui::Grid::new("attr_grid")
            .num_columns(2)
            .spacing([16.0, 6.0])
            .striped(true)
            .show(ui, |ui| {
                let ch = &mut data.characters[self.attr_char_idx];
                for stat_idx in 0..10usize {
                    ui.label(STAT_LABELS[stat_idx]);
                    let val = stat_field(ch, stat_idx);
                    let old = *val;
                    let resp = ui.add(
                        DragValue::new(val)
                            .range(0u16..=65535u16)
                            .speed(1.0),
                    );
                    if resp.changed() && *val != old {
                        data.is_dirty = true;
                    }
                    ui.end_row();
                }
            });
    }

    fn tab_skills(&mut self, ui: &mut Ui) {
        let data = self.save_data.as_mut().unwrap();

        ui.horizontal(|ui| {
            ui.label("角色：");
            ComboBox::from_id_salt("skill_char")
                .selected_text(CHAR_NAMES[self.skill_char_idx])
                .show_ui(ui, |ui| {
                    for (i, name) in CHAR_NAMES.iter().enumerate() {
                        ui.selectable_value(&mut self.skill_char_idx, i, *name);
                    }
                });
        });

        ui.add_space(8.0);

        ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("skill_grid")
                .num_columns(2)
                .spacing([12.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    let ch = &mut data.characters[self.skill_char_idx];
                    for slot in 0..32usize {
                        ui.label(format!("仙術 {}", slot + 1));

                        let current_id = ch.skills[slot];
                        let current_idx = SKILL_CATALOG
                            .iter()
                            .position(|&(id, _)| id == current_id)
                            .unwrap_or(0);
                        let current_name = SKILL_CATALOG[current_idx].1;

                        let mut chosen_idx = current_idx;
                        ComboBox::from_id_salt(format!("skill_{slot}"))
                            .selected_text(current_name)
                            .width(180.0)
                            .show_ui(ui, |ui| {
                                for (i, &(_, name)) in SKILL_CATALOG.iter().enumerate() {
                                    ui.selectable_value(&mut chosen_idx, i, name);
                                }
                            });

                        if chosen_idx != current_idx {
                            ch.skills[slot] = SKILL_CATALOG[chosen_idx].0;
                            data.is_dirty = true;
                        }

                        ui.end_row();
                    }
                });
        });
    }

    fn tab_items(&mut self, ui: &mut Ui) {
        let data = self.save_data.as_mut().unwrap();
        let max_items = 256;

        // Add item row
        ui.horizontal(|ui| {
            ui.label("新增物品：");
            let add_name = ITEM_CATALOG
                .iter()
                .find(|&&(id, _)| id == self.add_item_id)
                .map(|&(_, n)| n)
                .unwrap_or("(未知)");
            let mut chosen_idx = ITEM_CATALOG
                .iter()
                .position(|&(id, _)| id == self.add_item_id)
                .unwrap_or(0);
            ComboBox::from_id_salt("add_item_combo")
                .selected_text(add_name)
                .width(200.0)
                .show_ui(ui, |ui| {
                    for (i, &(_, name)) in ITEM_CATALOG.iter().enumerate() {
                        ui.selectable_value(&mut chosen_idx, i, name);
                    }
                });
            self.add_item_id = ITEM_CATALOG[chosen_idx].0;

            let can_add = data.items.len() < max_items;
            ui.add_enabled_ui(can_add, |ui| {
                if ui.button("➕ 新增").clicked() {
                    data.items.push(ItemEntry {
                        item_id: self.add_item_id,
                        count: 1,
                        used: 0,
                    });
                    data.is_dirty = true;
                }
            });

            ui.label(
                egui::RichText::new(format!("{}/{max_items}", data.items.len()))
                    .weak()
                    .small(),
            );
        });

        ui.separator();

        // Column headers
        egui::Grid::new("item_header")
            .num_columns(4)
            .min_col_width(20.0)
            .show(ui, |ui| {
                ui.strong("物品名稱");
                ui.strong("數量");
                ui.strong("已用");
                ui.label("");
                ui.end_row();
            });

        ScrollArea::vertical().show(ui, |ui| {
            let mut to_remove: Option<usize> = None;

            egui::Grid::new("item_grid")
                .num_columns(4)
                .spacing([8.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for (i, item) in data.items.iter_mut().enumerate() {
                        // Name combo
                        let current_name = ITEM_CATALOG
                            .iter()
                            .find(|&&(id, _)| id == item.item_id)
                            .map(|&(_, n)| n)
                            .unwrap_or("(未知)");
                        let mut chosen_idx = ITEM_CATALOG
                            .iter()
                            .position(|&(id, _)| id == item.item_id)
                            .unwrap_or(0);
                        ComboBox::from_id_salt(format!("item_name_{i}"))
                            .selected_text(current_name)
                            .width(200.0)
                            .show_ui(ui, |ui| {
                                for (j, &(_, name)) in ITEM_CATALOG.iter().enumerate() {
                                    ui.selectable_value(&mut chosen_idx, j, name);
                                }
                            });
                        let new_id = ITEM_CATALOG[chosen_idx].0;
                        if new_id != item.item_id {
                            item.item_id = new_id;
                            data.is_dirty = true;
                        }

                        // Count
                        let old_count = item.count;
                        let resp = ui.add(
                            DragValue::new(&mut item.count)
                                .range(0u16..=65535u16)
                                .speed(1.0),
                        );
                        if resp.changed() && item.count != old_count {
                            data.is_dirty = true;
                        }

                        // Used
                        let old_used = item.used;
                        let resp = ui.add(
                            DragValue::new(&mut item.used)
                                .range(0u16..=65535u16)
                                .speed(1.0),
                        );
                        if resp.changed() && item.used != old_used {
                            data.is_dirty = true;
                        }

                        // Delete
                        if ui.button("✖").clicked() {
                            to_remove = Some(i);
                        }

                        ui.end_row();
                    }
                });

            if let Some(idx) = to_remove {
                data.items.remove(idx);
                data.is_dirty = true;
            }
        });
    }
}
