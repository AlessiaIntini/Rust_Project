use std::{path::PathBuf, time::UNIX_EPOCH, borrow::Cow};

use crate::utility::screenshots::{
    get_all_display, take_screenshot_all_displays, take_screenshot_display,take_screenshot_area
};
use eframe::egui;
use egui::{Context, TextureHandle, TopBottomPanel, Vec2, Ui, Align, Layout};
use image::DynamicImage;
use rfd::FileDialog;
use arboard::Clipboard;

pub fn main_window() -> eframe::Result<()> {
    let window_option = eframe::NativeOptions {
        resizable: true,
        follow_system_theme: true,
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        "RustScreenRecorder",
        window_option,
        Box::new(|cc| Box::new(RustScreenRecorder::new(cc))),
    )
}
#[derive(PartialEq, Eq)]
enum Mood{
    Edit,
    None,
}
struct RustScreenRecorder {
    screen_index: Option<u8>,
    image: TextureHandle,
    timer:Option<i64>,
    screenshot:Option<DynamicImage>,
    path:Option<PathBuf>,
    edit:Mood,
    shape:Option<String>
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
        let p=PathBuf::new();
        
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            screen_index: Some(0),
            image: i.clone(),
            timer:Some(0),
            screenshot:Some(screenshot),
            path:Some(p),
            edit:Mood::None,
            shape:Some("Cicle".to_string()),
        }
    }
    fn show_menu(&mut self, ctx: &Context) {
        let screens = get_all_display();
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    if ui.button("All Screens").clicked() {
                        take_screenshot_all_displays();
                    }
                    if ui.button("Screen area").clicked() {
                        match self.screen_index {
                            Some(screen_index)=>{
                                self.screenshot =
                                    take_screenshot_area(screens[screen_index as usize].clone(),100,100,100,100)
                                        ;
                                self.image = ctx.load_texture(
                                    "screenshot",
                                    egui::ColorImage::from_rgba_unmultiplied(
                                        [self.screenshot.as_ref().unwrap().width() as usize, self.screenshot.as_ref().unwrap().height() as usize],
                                        self.screenshot.as_ref().unwrap().as_bytes(),
                                    ),
                                    Default::default(),
                                );    
                            }
                            None=>(),   
                        }
                    }
                    if ui.button("Selected Screen").clicked() {
                        match self.screen_index {
                            Some(screen_index) => {
                                self.screenshot =
                                    take_screenshot_display(screens[screen_index as usize].clone())
                                        ;
                                self.image = ctx.load_texture(
                                    "screenshot",
                                    egui::ColorImage::from_rgba_unmultiplied(
                                        [self.screenshot.as_ref().unwrap().width() as usize, self.screenshot.as_ref().unwrap().height() as usize],
                                        self.screenshot.as_ref().unwrap().as_bytes(),
                                    ),
                                    Default::default(),
                                );
                            }
                            None => (),
                        }
                    }
                    self.select_timer(ui);

                    if ui.button("Edit").clicked() {
                        self.edit=Mood::Edit;
                    }
                    if ui.button("Save").clicked() {
                        self.save_image();
                    }

                    if ui.button("Save as").clicked() {
                        self.save_as_image();
                    }
                    if ui.button("Settings").clicked() {
                        // …
                    }
                    if ui.button("Copy").clicked() {
                      self.copy_image();
                    }
                    self.select_monitor(ui);
                   
                });
            });
                if self.edit==Mood::Edit{
                   
                    TopBottomPanel::top("bottom panel").show(ctx, |ui| {
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            egui::ComboBox::from_label("")
                            .width(80.0)
                            .selected_text( self.shape.as_ref().unwrap().to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.shape, Some("square".to_string()), "square");
                                ui.selectable_value(&mut self.shape, Some("cicle".to_string()), "circle");
                                ui.selectable_value(&mut self.shape, Some("arrow".to_string()), "arrow");
                            });
                                if ui.button("Text").clicked() {

                                }
                                if ui.button("Cut").clicked() {
                                }
                                if ui.button("Cancel").clicked() {
                                }
                                if ui.button(" Back ").clicked() {
                                }
                                if ui.button("Save").clicked() {
                                }
                                if ui.button("Exit").clicked() {
                                    self.edit=Mood::None;
                                }
                            });
});
        }
       
        
    }


    fn select_monitor(&mut self,ui:&mut Ui){
        let screens=get_all_display();
        egui::ComboBox::from_label("Monitor")
        .selected_text(format!("Monitor {:?}", self.screen_index.unwrap()))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut self.screen_index,Some(0), "Monitor 0");
            if screens.len()>1{
            ui.selectable_value(&mut self.screen_index, Some(1), "Monitor 1");
            }
        });
        ui.label(format!("You have selected Monitor {}", self.screen_index.unwrap()));
    }

    fn select_timer(&mut self,ui:&mut Ui){
        egui::ComboBox::from_label("")
        .width(80.0)
        .selected_text(format!("🕓 {:?} sec", self.timer.unwrap()))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut self.timer, Some(0), "🕓 0 sec").on_hover_text("Delay screenshot");
            ui.selectable_value(&mut self.timer, Some(5), "🕓 5 sec").on_hover_text("Delay screenshot");
            ui.selectable_value(&mut self.timer, Some(10), "🕓 10 sec").on_hover_text("Delay screenshot");
        });
    }
    fn save_as_image(&mut self){
        let time = match std::time::SystemTime::now().duration_since(UNIX_EPOCH)
        {
            Ok(time_scr)=> time_scr.as_secs().to_string(),
            Err(_) => "".to_string(),
        };
        let path =
       FileDialog::new().add_filter("PNG", &["png"])
            .add_filter("JPG", &["jpg"]).add_filter("GIF", &["gif"])
            .add_filter("BMP", &["bmp"])
            .set_directory("./")
            .set_file_name(format!("Screen{}", time.as_str()))
            .save_file();
        match path {
            Some(path) => {
                match image::save_buffer(
                    path,
                    &self.screenshot.as_ref().unwrap().as_bytes(),
                    self.screenshot.as_ref().unwrap().width() as u32,
                    self.screenshot.as_ref().unwrap().height() as u32,
            image::ColorType::Rgba8,) 
                {
                Ok(_) => println!("Screenshot saved"),
                Err(err) => println!("{}", err),
            }
        }
        None => {}
    }
    }
    fn save_image(&mut self){
        let mut p=self.path.as_ref().unwrap().clone();
        let time = match std::time::SystemTime::now().duration_since(UNIX_EPOCH)
        {
            Ok(time_scr)=> time_scr.as_secs().to_string(),
            Err(_) => "".to_string(),
        };
        p.push(format!("target/Screen{}", time.as_str()));
        p.set_extension("png");
        match  Some(p) {
            Some(path) => {
                match image::save_buffer(
                    path,
                    &self.screenshot.as_ref().unwrap().as_bytes(),
                    self.screenshot.as_ref().unwrap().width() as u32,
                    self.screenshot.as_ref().unwrap().height() as u32,
            image::ColorType::Rgba8,) 
                {
                Ok(_) => println!("Screenshot saved"),
                Err(err) => println!("{}", err),
            }
        }
        None => {}
    }
    }
    fn copy_image(&mut self){
        let mut clipboard = Clipboard::new().unwrap();
        let final_image = self.screenshot.as_ref().unwrap().clone();
        let bytes = final_image.as_bytes();
        let img = arboard::ImageData {
            width: final_image.width() as usize,
            height: final_image.height() as usize,
            bytes: Cow::from(bytes),
        };
        let _done = clipboard.set_image(img);
    }
}
    


impl eframe::App for RustScreenRecorder {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.show_menu(ctx);
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
    // fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
    //     // if self.screen==true {
    //     //     egui::CentralPanel::default().show(&self.ctx, |ui| {
    //     //                 ui.add(
    //     //                     egui::Image::new(egui::include_image!( "../../target/1.png"))
    //     //                             .fit_to_original_size(0.48)
    //     //                 );

    //     //            });

    //     //    }
    // }
}
