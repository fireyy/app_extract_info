use eframe::egui;
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
            });
        });

        resp
    }
}