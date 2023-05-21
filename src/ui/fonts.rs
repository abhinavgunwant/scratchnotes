use egui::{ Context, FontDefinitions, FontData, FontFamily };

pub fn setup_custom_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    let my_font: String = String::from("fira_code");

    fonts.font_data.insert(
        my_font.clone(),
        FontData::from_static(include_bytes!(
            "../../fonts/FiraCode-VF.ttf"
        )),
    );

    fonts.families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, my_font);

    ctx.set_fonts(fonts);
}

