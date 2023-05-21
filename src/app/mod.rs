pub mod file;

use eframe::{
    egui::{
        Context, CentralPanel, vec2, Color32, ScrollArea,
        widgets::TextEdit,
    },
    App, CreationContext, Frame,
};
use egui::{FontFamily, FontId};
use crate::ui::fonts::setup_custom_fonts;
use file::load_file;

use self::file::save_file;

pub struct ScratchNotesApp {
    text: String,
}

impl ScratchNotesApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let mut load_text: String = load_file();

        if load_text.is_empty() {
            load_text = String::from("=====test=====");
        }

        Self {
            text: load_text,
        }
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}

impl App for ScratchNotesApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let central_panel_frame = egui::containers::Frame::none()
            .fill(Color32::from_rgb(0, 43, 54));

        let central_panel = CentralPanel::default()
            .frame(central_panel_frame);

        let txt = self.text.clone();

        let text_edit = TextEdit::multiline(&mut self.text)
            .min_size(_frame.info().window_info.size)
            .desired_width(f32::INFINITY)
            .margin(vec2(0.0, 0.0))
            .frame(false)
            .font(FontId::new(16.0, FontFamily::Proportional));

        central_panel.show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let response = ui.add(text_edit);
                if response.changed() {
                    save_file(txt);
                }
            });
        });
    }
}

