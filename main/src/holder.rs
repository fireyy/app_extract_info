use std::{sync::mpsc, thread, path::PathBuf};
use eframe::egui;
use app_extract_info::{
    error::{ExtResult}, get_loaders, manifest::Manifest
};

pub struct HolderHandOff {
    pub manifest: Manifest,
}

enum Update {
    AppParsed(ExtResult<Manifest>),
}

enum State {
    Idle(Route),
    Busy(Route),
}

#[derive(Clone, Copy, PartialEq)]
enum Route {
    Parse,
}

pub struct HolderScreen {
    update_tx: mpsc::Sender<Update>,
    update_rx: mpsc::Receiver<Update>,
    state: State,
    err: Option<String>,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl HolderScreen {
    pub fn new() -> Self {
        let (update_tx, update_rx) = mpsc::channel();

        Self {
            update_tx,
            update_rx,
            state: State::Idle(Route::Parse),
            err: None,
            dropped_files: vec![],
            picked_path: None,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) -> Option<HolderHandOff> {
        let mut resp = None;

        while let Ok(update) = self.update_rx.try_recv() {
            match update {
                Update::AppParsed(result) => match result {
                    Ok(manifest) => {
                        resp = Some(HolderHandOff {
                            manifest
                        });
                    }
                    Err(msg) => {
                        self.state = State::Idle(Route::Parse);
                        self.err = Some(msg.to_string());
                    }
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered_justified(|ui| {
                    match &mut self.state {
                        State::Idle(ref mut route) => {
                            match route {
                                Route::Parse => {
                                    if ui.button("Open file…").clicked() {
                                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                                            self.picked_path = Some(path.display().to_string());
                                        }
                                    }
                                    if let Some(err) = &self.err {
                                        ui.label(err);
                                    }
                                }
                            }
                        }
                        State::Busy(route) => match route {
                            Route::Parse => {
                                ui.spinner();
                                ui.heading("Parsing file...");
                            }
                        }
                    }
                });
            });
        });

        if !self.dropped_files.is_empty() {
            // for file in &self.dropped_files {

            // }
            let file = self.dropped_files.first().unwrap();
            let info = if let Some(path) = &file.path {
                path.display().to_string()
            } else if !file.name.is_empty() {
                file.name.clone()
            } else {
                "???".to_owned()
            };
            self.picked_path = Some(info);
        }

        self.parse_file(ctx);

        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }

        resp
    }

    fn parse_file(&mut self, ctx: &egui::Context) {
        if let Some(picked_path) = self.picked_path.clone() {
            self.picked_path = None;
            self.state = State::Busy(Route::Parse);

            let update_tx = self.update_tx.clone();
            let ctx = ctx.clone();
            let path = picked_path.clone();

            thread::spawn(move || {
                let result = get_loaders(&PathBuf::from(path));
                update_tx.send(Update::AppParsed(result)).unwrap();
                ctx.request_repaint();
            });
        }
    }
}