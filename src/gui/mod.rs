use eframe::{egui, Frame};
use egui::{menu, Button, Context, TopBottomPanel, Color32};
use screenshots::Screen;
use crate::gui;


pub fn main_window() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions{
        maximized: true,
        ..Default::default()
    };
    // eframe::run_native("My egui App", native_options, Box::new(|cc|
    //     
    //     Box::new(RustScreenRecorder::new(cc))))
   
        eframe::run_native(
            "My egui App",
            native_options,
            Box::new(|cc| {
                // This gives us image support:
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Box::<RustScreenRecorder>::default()
            }),
        )
}

#[derive(Default)]
struct RustScreenRecorder {}

impl RustScreenRecorder {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
    fn show_menu(&mut self, ctx: &Context, frame:&mut Frame) {
        let screens = Screen::all().unwrap()[0];
        
        TopBottomPanel::top("top panel").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("screen ", |ui| {
                    let image_ref = screens.capture().unwrap();
                    image_ref.save(format!("target/{}.png", screens.display_info.id))
                    .unwrap();
            });
            ui.menu_button("select area", |ui| {
                
            });
            ui.menu_button("edit", |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
            ui.menu_button("save", |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
            ui.menu_button("save as", |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
            ui.menu_button("monitor", |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
            ui.menu_button("settings", |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
            ui.menu_button("copy", |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
        });
        
    });
    });
}
}


impl eframe::App for RustScreenRecorder {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
   
    self.show_menu(ctx, frame);
       egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                egui::Image::new(egui::include_image!( "../../target/1.png"))
                        .fit_to_original_size(0.48)
        );
       
       });
   }
}