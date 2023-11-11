use crate::utility::screenshots::{
    get_all_display, take_screenshot_all_displays, take_screenshot_display,
};
use eframe::{egui, Frame};
use egui::{menu, Context, TextureHandle, TopBottomPanel, Vec2};

pub fn main_window() -> eframe::Result<()> {
    let window_option = eframe::NativeOptions {
        resizable: true,
        follow_system_theme: true,
        ..Default::default()
    };
    eframe::run_native(
        "RustScreenRecorder",
        window_option,
        Box::new(|cc| Box::new(RustScreenRecorder::new(cc))),
    )
}

struct RustScreenRecorder {
    screen_index: Option<u8>,
    image: TextureHandle,
    // ctx: Context,
}

impl RustScreenRecorder {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // cc.egui_ctx.set_visuals(egui::Visuals::light());
        let screenshot = take_screenshot_all_displays().unwrap();
        let i = cc.egui_ctx.load_texture(
            "screenshot",
            egui::ColorImage::from_rgba_unmultiplied(
                [screenshot.width() as usize, screenshot.height() as usize],
                screenshot.as_bytes(),
            ),
            Default::default(),
        );
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            screen_index: None,
            image: i.clone(),
        }
    }
    fn show_menu(&mut self, ctx: &Context, frame: &mut Frame) {
        let screens = get_all_display();

        TopBottomPanel::top("top panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("All Screens").clicked() {
                        take_screenshot_all_displays();
                    }
                    if ui.button("Screen area").clicked() {
                        // …
                    }
                    if ui.button("Selected Screen").clicked() {
                        match self.screen_index {
                            Some(screen_index) => {
                                let screenshot =
                                    take_screenshot_display(screens[screen_index as usize].clone())
                                        .unwrap();
                                self.image = ctx.load_texture(
                                    "screenshot",
                                    egui::ColorImage::from_rgba_unmultiplied(
                                        [screenshot.width() as usize, screenshot.height() as usize],
                                        screenshot.as_bytes(),
                                    ),
                                    Default::default(),
                                );
                            }
                            None => (),
                        }
                    }
                    if ui.button("Edit").clicked() {
                        // …
                    }
                    if ui.button("Save").clicked() {
                        // …
                    }
                    if ui.button("save as").clicked() {
                        // …
                    }
                    if ui.button("Monitor").clicked() {
                        self.screen_index = Some(0);
                    }
                    if ui.button("Settings").clicked() {
                        // …
                    }
                    if ui.button("Copy").clicked() {
                        // …
                    }
                });
            });
        });
    }
}

impl eframe::App for RustScreenRecorder {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.show_menu(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing();
                // println!("{:?}", frame.info().window_info.size);
                // ui.image((self.image.id(), frame.info().window_info.size));
                Vec2::new(0.0, 0.0);
                ui.add(egui::Image::new((
                    self.image.id(),
                    Vec2::new(frame.info().window_info.size.x, self.image.size_vec2().y),
                )));
            });
        });
    }
    fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
        // if self.screen==true {
        //     egui::CentralPanel::default().show(&self.ctx, |ui| {
        //                 ui.add(
        //                     egui::Image::new(egui::include_image!( "../../target/1.png"))
        //                             .fit_to_original_size(0.48)
        //                 );

        //            });

        //    }
    }
}
