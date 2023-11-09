use eframe::{egui, Frame};
use egui::{menu, Button, Context, TopBottomPanel, Color32};
use image::RgbaImage;
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
struct RustScreenRecorder {
    screen:bool,
    image:RgbaImage,
    ctx: Context,
}

impl RustScreenRecorder {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self{
            screen:false,
            ..Default::default()
        }
    }
    fn show_menu(&mut self, ctx: &Context, frame:&mut Frame) {
        let screens = Screen::all().unwrap()[0];
        
        TopBottomPanel::top("top panel").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            ui.horizontal(|ui| {
            if ui.button("Screen").clicked() {
                //frame.request_screenshot();
                   self.image = screens.capture().unwrap();
                   self.image.save(format!("target/{}.png", screens.display_info.id))
                    .unwrap();
               
                    self.screen=true;
                    self.ctx=ctx.clone();
                  
            }           
            if ui.button("Screen area").clicked() {
                // …
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
                    // …
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
   if self.screen==true {
    egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(
                    egui::Image::new(egui::include_image!( "../../target/1.png"))
                            .fit_to_original_size(0.48)
                );
          
           });
   }
    self.show_menu(ctx, frame);
      
   }
   fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
     egui::CentralPanel::default().show(&self.ctx, |ui| {
                ui.add(
                    egui::Image::new(egui::include_image!( "../../target/1.png"))
                            .fit_to_original_size(0.48)
                );
          
           });

}
}