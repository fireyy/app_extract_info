#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod holder;
mod meta;

use crate::holder::{HolderHandOff, HolderScreen};
use crate::meta::{MetaHandOff, MetaScreen};

fn main() {
    eframe::run_native(
        "Xray",
        eframe::NativeOptions {
            drag_and_drop_support: true,
            initial_window_size: Some(egui::vec2(400.0, 400.0)),
            ..Default::default()
        },
        Box::new(|_cc: &eframe::CreationContext| {
            let holder = HolderScreen::new();
            Box::new(Xray::Holder(holder))
        }),
    );
}

enum Xray {
    Holder(HolderScreen),
    Meta(Box<MetaScreen>),
}

impl eframe::App for Xray {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self {
            Self::Holder(screen) => {
                if let Some(handoff) = screen.update(ctx) {
                    let HolderHandOff { manifest } = handoff;

                    let meta = MetaScreen::new(manifest);
                    *self = Self::Meta(Box::new(meta));

                    ctx.request_repaint();
                }
            }
            Self::Meta(screen) => {
                if let Some(handoff) = screen.update(ctx) {
                    let MetaHandOff { back: _back } = handoff;

                    let holder = HolderScreen::new();
                    *self = Self::Holder(holder);

                    ctx.request_repaint();
                }
            }
        }
    }
}