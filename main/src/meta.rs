use eframe::egui;
use egui_extras::RetainedImage;
use app_extract_info::{
    manifest::Manifest
};

pub struct MetaHandOff {
    pub back: bool,
}

pub struct MetaScreen {
    data: Manifest,
}

impl MetaScreen {
    pub fn new(data: Manifest) -> Self {
        Self {
            data,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) -> Option<MetaHandOff> {
        let mut resp = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            let data = self.data.clone();
            if ui.button("<-Back").clicked() {
                resp = Some(MetaHandOff {
                    back: true
                })
            }
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.label(data.name);
                });
                ui.horizontal(|ui| {
                    ui.label("Bundle ID:");
                    ui.label(data.bundle_id);
                });
                ui.horizontal(|ui| {
                    ui.label("Version:");
                    ui.label(data.version);
                });
                ui.horizontal(|ui| {
                    ui.label("Build:");
                    ui.label(data.build_number);
                });
                ui.horizontal(|ui| {
                    ui.label("Icon:");
                    match base64::decode(data.icon) {
                        Ok(data) => {
                            let icon = RetainedImage::from_image_bytes(
                                "app_icon.png",
                                &data,
                            )
                            .unwrap();
                            icon.show_size(ui, egui::vec2(64.0, 64.0));
                        }
                        Err(_) => {}
                    }
                });
            });
        });

        resp
    }
}