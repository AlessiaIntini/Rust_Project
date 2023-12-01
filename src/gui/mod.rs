use std::{borrow::Cow, path::PathBuf};
mod draw;
use crate::utility::screenshots::{
    get_all_display, take_screenshot_all_displays, take_screenshot_area, take_screenshot_display,
};
use arboard::Clipboard;
use eframe::egui;
use eframe::egui::ColorImage;
use egui::{Align, Context, Layout, Rect, TextureHandle, TopBottomPanel, Ui, Vec2, FontFamily};
use egui_plot::{Legend, Line, Plot, PlotPoints};
use image::DynamicImage;
use rfd::FileDialog;

use self::draw::ProperDraw;

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
enum Mood {
    Edit,
    None,
}

struct RustScreenRecorder {
    screen_index: Option<u8>,
    image: TextureHandle,
    timer: Option<i64>,
    screenshot: Option<DynamicImage>,
    path: Option<PathBuf>,
    edit: Mood, 
    vec_shape: Vec<draw::Shape>,
    property:ProperDraw,
    font:FontFamily,
    draw_dim_variable:i32,
    text:String,
    screens: Vec<screenshots::Screen>,
    window_size: Vec2,
    border_size: Vec2,
    image_size: Vec2,
    draw_shape: bool,
    flag:i32,
}

impl RustScreenRecorder {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // cc.egui_ctx.set_visuals(egui::Visuals::light());
        let screenshot = take_screenshot_all_displays().unwrap();
        let img = cc.egui_ctx.load_texture(
            "screenshot",
            egui::ColorImage::from_rgba_unmultiplied(
                [screenshot.width() as usize, screenshot.height() as usize],
                screenshot.as_bytes(),
            ),
            Default::default(),
        );
        let pic_dir = match dirs::picture_dir() {
            Some(path) => path,
            None => {
                eprintln!("Unable to determine home directory");
                dirs::home_dir().unwrap()
            }
        };

        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            screen_index: Some(0),
            image: img,
            timer: Some(0),
            screenshot: Some(screenshot),
            path: Some(pic_dir),
            edit: Mood::None,
            vec_shape: Vec::new(),
            screens: Vec::new(),
            window_size: Vec2::new(0.0, 0.0),
            border_size: Vec2::new(0.0, 0.0),
            image_size: Vec2::new(0.0, 0.0),
            draw_shape: false,
            flag:0,
            text:"".to_string(),
            font:FontFamily::Monospace,
            property:ProperDraw::new(
                Some(-1),
                draw::Rgb::new(0, 0, 0),
                false,
                draw::Rgb::new(0, 0, 0),
                1.0,
            ),
            draw_dim_variable:0,
        }
    }
    fn show_menu(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.screens = get_all_display();
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                if ui.button("All Screens").clicked() {
                    if self.timer.unwrap() != 0 {
                        std::thread::sleep(std::time::Duration::from_secs(
                            self.timer.unwrap() as u64
                        ));
                    }
                    self.screenshot = take_screenshot_all_displays();
                    self.image = ctx.load_texture(
                        "screenshot",
                        egui::ColorImage::from_rgba_unmultiplied(
                            [
                                self.screenshot.as_ref().unwrap().width() as usize,
                                self.screenshot.as_ref().unwrap().height() as usize,
                            ],
                            self.screenshot.as_ref().unwrap().as_bytes(),
                        ),
                        Default::default(),
                    );
                }
                if ui.button("Screen area").clicked() {
                    match self.screen_index {
                        Some(screen_index) => {
                            if self.timer.unwrap() != 0 {
                                std::thread::sleep(std::time::Duration::from_secs(
                                    self.timer.unwrap() as u64,
                                ));
                            }
                            self.screenshot = take_screenshot_area(
                                self.screens[screen_index as usize].clone(),
                                100,
                                100,
                                100,
                                100,
                            );
                            self.image = ctx.load_texture(
                                "screenshot",
                                egui::ColorImage::from_rgba_unmultiplied(
                                    [
                                        self.screenshot.as_ref().unwrap().width() as usize,
                                        self.screenshot.as_ref().unwrap().height() as usize,
                                    ],
                                    self.screenshot.as_ref().unwrap().as_bytes(),
                                ),
                                Default::default(),
                            );
                        }
                        None => (),
                    }
                }
                if ui.button("Selected Screen").clicked() {
                    match self.screen_index {
                        Some(screen_index) => {
                            if self.timer.unwrap() != 0 {
                                std::thread::sleep(std::time::Duration::from_secs(
                                    self.timer.unwrap() as u64,
                                ));
                            }
                            self.screenshot = take_screenshot_display(
                                self.screens[screen_index as usize].clone(),
                            );
                            self.image = ctx.load_texture(
                                "screenshot",
                                egui::ColorImage::from_rgba_unmultiplied(
                                    [
                                        self.screenshot.as_ref().unwrap().width() as usize,
                                        self.screenshot.as_ref().unwrap().height() as usize,
                                    ],
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
                    self.edit = Mood::Edit;
                    if self.flag==0{
                        self.flag=1;
                        self.property.draw=Some(0);
                    }
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

            if self.edit == Mood::Edit {
               // self.shape=Some(0);
                self.show_edit(ctx, frame);
            }
        });
    }
    fn show_edit(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("bottom panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                
                if ui.button("Shape").clicked() {
                    self.draw_shape = true;
                }
                let mut first="";
                match self.property.draw.unwrap(){
                    0=>first="Cicle",
                    1=>first="Square",
                    2=>first="Arrow",
                    _=>(),
                }
                if self.draw_shape {
                    
                    egui::ComboBox::from_label("")
                        .width(80.0)
                        .selected_text(first.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.property.draw, Some(0), "cicle");
                            ui.selectable_value(&mut self.property.draw, Some(1), "square");
                            ui.selectable_value(&mut self.property.draw, Some(2), "arrow");
                        });
                    ui.separator();
                    ui.add(egui::Slider::new(&mut self.property.color.red, 0..=255).text("Red"));
                    ui.add(egui::Slider::new(&mut self.property.color.green, 0..=255).text("Green"));
                    ui.add(egui::Slider::new(&mut self.property.color.blue, 0..=255).text("Blue"));

                    if self.property.draw.unwrap() >= 0 {
                        draw::create_figure(
                            self.vec_shape.as_mut(),
                            ctx,
                            self.property,
                            self.text.to_string(),
                            &mut self.draw_dim_variable,
                            self.font.clone(),
                            self.border_size.x,
                            self.border_size.y,
                            self.image_size.x,
                            self.image_size.y,
                        );
                    }
                }

                if ui.button("Text").clicked() {
                    self.draw_shape = false;
                }
                if ui.button("Cut").clicked() {
                    self.draw_shape = false;
                }
                if ui.button("Cancel").clicked() {
                    self.draw_shape = false;
                }
                if ui.button("Back").clicked() {
                    self.vec_shape.pop();
                }

                if ui.button("Save").clicked() {
                    self.flag=0;
                    self.draw_shape = false;
                    let origin =
                        Vec2::new((self.border_size.x / 2.0) + 18.0, self.border_size.y + 60.0);
                    let image_width = self.image_size.x + 60.0;
                    let image_hight = self.image_size.y + 20.0;
                    self.screenshot = take_screenshot_area(
                        self.screens[self.screen_index.unwrap() as usize].clone(),
                        origin.x as i32,
                        origin.y as i32,
                        image_width as u32,
                        image_hight as u32,
                    );
                    self.image = ctx.load_texture(
                        "screenshot",
                        egui::ColorImage::from_rgba_unmultiplied(
                            [
                                self.screenshot.as_ref().unwrap().width() as usize,
                                self.screenshot.as_ref().unwrap().height() as usize,
                            ],
                            self.screenshot.as_ref().unwrap().as_bytes(),
                        ),
                        Default::default(),
                    );
                    self.edit = Mood::None;
                    self.property.draw = None;
                    self.vec_shape = Vec::new();
                }

                if ui.button("Exit").clicked() {
                    self.draw_shape = false;
                    self.edit = Mood::None;
                    self.vec_shape = Vec::new();
                    self.flag=0;
                }
            });
        });
    }

    fn select_monitor(&mut self, ui: &mut Ui) {
        let screens = get_all_display();
        egui::ComboBox::from_label("Monitor")
            .selected_text(format!("Monitor {:?}", self.screen_index.unwrap()))
            .show_ui(ui, |ui| {
                for (i, display) in screens.iter().enumerate() {
                    ui.selectable_value(
                        &mut self.screen_index,
                        Some(i as u8),
                        format!(
                            "🖵 Display {}  {}x{}",
                            i, display.display_info.width, display.display_info.height
                        ),
                    )
                    .on_hover_text("Select display");
                }
            });
        ui.label(format!(
            "You have selected Monitor {}",
            self.screen_index.unwrap()
        ));
    }

    fn select_timer(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("")
            .width(80.0)
            .selected_text(format!("🕓 {:?} sec", self.timer.unwrap()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.timer, Some(0), "🕓 0 sec")
                    .on_hover_text("Delay screenshot");
                ui.selectable_value(&mut self.timer, Some(5), "🕓 5 sec")
                    .on_hover_text("Delay screenshot");
                ui.selectable_value(&mut self.timer, Some(10), "🕓 10 sec")
                    .on_hover_text("Delay screenshot");
            });
    }
    fn save_as_image(&mut self) {
        let path = FileDialog::new()
            .add_filter("PNG", &["png"])
            .add_filter("JPG", &["jpg"])
            .add_filter("GIF", &["gif"])
            .add_filter("BMP", &["bmp"])
            .set_directory(self.path.as_ref().unwrap().clone())
            .set_file_name(format!(
                "screenshot_{}",
                chrono::Local::now().format("%Y_%m_%d_%H_%M_%S").to_string()
            ))
            .save_file();
        match path {
            Some(path) => {
                match image::save_buffer(
                    path,
                    &self.screenshot.as_ref().unwrap().as_bytes(),
                    self.screenshot.as_ref().unwrap().width() as u32,
                    self.screenshot.as_ref().unwrap().height() as u32,
                    image::ColorType::Rgba8,
                ) {
                    Ok(_) => println!("Screenshot saved"),
                    Err(err) => println!("{}", err),
                }
            }
            None => {}
        }
    }
    fn save_image(&mut self) {
        let mut p = self.path.as_ref().unwrap().clone();
        p = p.join(format!(
            "screenshot_{}.png",
            chrono::Local::now().format("%Y_%m_%d_%H_%M_%S").to_string()
        ));
        //TODO: notify the user if the file exists
        self.screenshot.as_ref().unwrap().save(p.clone()).unwrap();
    }
    fn copy_image(&mut self) {
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
        self.show_menu(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.window_size = Vec2::new(
                frame.info().window_info.size.x,
                frame.info().window_info.size.y,
            );
            ui.vertical_centered(|ui| {
                // ui.spacing();
                let available_size = ui.available_size();
                let image_size = self.image.size_vec2();

                let aspect_ratio = image_size.x / image_size.y;
                let (image_width, image_height) =
                    if available_size.x / aspect_ratio < available_size.y {
                        (available_size.x, available_size.x / aspect_ratio)
                    } else {
                        (available_size.y * aspect_ratio, available_size.y)
                    };
                let x_offset = (available_size.x - image_width) / 2.0;
                let y_offset = (available_size.y - image_height) / 2.0;

                self.border_size = Vec2::new(
                    self.window_size.x - image_width,
                    self.window_size.y - image_height,
                );
                self.image_size.x = image_width;
                self.image_size.y = image_height;
                ui.add(
                    egui::Image::new((self.image.id(), Vec2::new(image_width, image_height)))
                        .maintain_aspect_ratio(true)
                        .fit_to_exact_size(Vec2::new(
                            frame.info().window_info.size.x,
                            frame.info().window_info.size.y,
                        )),
                )
                .with_new_rect(egui::Rect::from_min_size(
                    egui::Pos2::new(x_offset - 255.0, y_offset),
                    Vec2::new(image_width, image_height),
                ));
                draw::draw(ui, self.vec_shape.as_mut());
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
