mod app;
mod catalogs;
mod save_data;
mod save_file;

use app::PalApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("仙劍奇俠傳 存檔修改器")
            .with_inner_size([1050.0, 720.0])
            .with_min_inner_size([820.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "仙劍奇俠傳 存檔修改器",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(PalApp::default()))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    // jf 開放粉圓 — SIL Open Font License, Traditional Chinese, embedded at compile time.
    const CJK_FONT: &[u8] = include_bytes!("../assets/jf-openhuninn-2.1.ttf");

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("cjk".to_owned(), egui::FontData::from_static(CJK_FONT).into());

    // Append as fallback so ASCII still uses the default crisp egui font.
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        fonts.families.entry(family).or_default().push("cjk".to_owned());
    }

    ctx.set_fonts(fonts);
}
