use std::{borrow::Cow, path::PathBuf, time::UNIX_EPOCH};
mod draw;
use crate::utility::screenshots::{
    get_all_display, take_screenshot_all_displays, take_screenshot_area, take_screenshot_display,
};
use arboard::Clipboard;
use eframe::egui;
use egui::{Align, Context, Layout, TextureHandle, TopBottomPanel, Ui, Vec2};
use image::DynamicImage;
use rfd::FileDialog;

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
    shape: Option<i32>, // ctx: Context,
    vec_shape: Vec<draw::Shape>,
    color: draw::Rgb,
    screens: Vec<screenshots::Screen>,
    window_width: Option<u32>,
    window_height: Option<u32>,
    border: Option<i32>,
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
        let p = PathBuf::new();

        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            screen_index: Some(0),
            image: img,
            timer: Some(0),
            screenshot: Some(screenshot),
            path: Some(p),
            edit: Mood::None,
            shape: Some(0),
            vec_shape: Vec::new(),
            color: draw::Rgb::new(0, 0, 0),
            screens: Vec::new(),
            window_height: Some(0),
            window_width: Some(0),
            border: Some(0),
        }
    }
    fn show_menu(&mut self, ctx: &Context) {
        self.screens = get_all_display();
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                if ui.button("All Screens").clicked() {
                    take_screenshot_all_displays();
                }
                if ui.button("Screen area").clicked() {
                    match self.screen_index {
                        Some(screen_index) => {
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
        if self.edit == Mood::Edit {
            self.show_edit(ctx);
        }
    }
    fn show_edit(&mut self, ctx: &Context) {
        TopBottomPanel::top("bottom panel").show(ctx, |ui| {
            self.border = Some(ui.available_size().y as i32);
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                //self.shape=Some(0);
                egui::ComboBox::from_label("")
                    .width(80.0)
                    .selected_text(self.shape.as_ref().unwrap().to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.shape, Some(0), "cicle");
                        ui.selectable_value(&mut self.shape, Some(1), "square");
                        ui.selectable_value(&mut self.shape, Some(2), "arrow");
                    });
                ui.separator();
                ui.add(egui::Slider::new(&mut self.color.red, 0..=255).text("Red"));
                ui.add(egui::Slider::new(&mut self.color.green, 0..=255).text("Green"));
                ui.add(egui::Slider::new(&mut self.color.blue, 0..=255).text("Blue"));
                if self.shape.unwrap() >= 0 {
                    draw::create_figure(
                        self.vec_shape.as_mut(),
                        ctx,
                        self.shape.unwrap(),
                        self.color,
                    );
                }

                if ui.button("Text").clicked() {}
                if ui.button("Cut").clicked() {}
                if ui.button("Cancel").clicked() {}
                if ui.button("Back").clicked() {}
                if ui.button("Save").clicked() {
                    //self.border=Some(self.window_width.unwrap() as i32-self.border.unwrap()as i32);
                    match self.screen_index {
                        Some(screen_index) => {
                            self.screenshot = take_screenshot_area(
                                self.screens[screen_index as usize].clone(),
                                0,
                                self.border.unwrap(),
                                self.window_width.unwrap(),
                                self.window_height.unwrap(),
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
                    self.edit = Mood::None;
                    self.shape = None;
                }
                if ui.button("Exit").clicked() {
                    self.edit = Mood::None;
                }
            });
        });
    }

    fn select_monitor(&mut self, ui: &mut Ui) {
        let screens = get_all_display();
        egui::ComboBox::from_label("Monitor")
            .selected_text(format!("Monitor {:?}", self.screen_index.unwrap()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.screen_index, Some(0), "Monitor 0");
                if screens.len() > 1 {
                    ui.selectable_value(&mut self.screen_index, Some(1), "Monitor 1");
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
        let time = match std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time_scr) => time_scr.as_secs().to_string(),
            Err(_) => "".to_string(),
        };
        let path = FileDialog::new()
            .add_filter("PNG", &["png"])
            .add_filter("JPG", &["jpg"])
            .add_filter("GIF", &["gif"])
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
        let time = match std::time::SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time_scr) => time_scr.as_secs().to_string(),
            Err(_) => "".to_string(),
        };
        p.push(format!("target/Screen{}", time.as_str()));
        p.set_extension("png");
        match Some(p) {
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
        self.show_menu(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.window_width = Some(ui.available_size().x as u32);
            self.window_height = Some(ui.available_size().y as u32);

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
                ui.add(
                    egui::Image::new((self.image.id(), Vec2::new(image_width, image_height)))
                        .maintain_aspect_ratio(true)
                        .fit_to_exact_size(Vec2::new(
                            frame.info().window_info.size.x,
                            frame.info().window_info.size.y,
                        )),
                )
                .with_new_rect(egui::Rect::from_min_size(
                    egui::Pos2::new(x_offset, y_offset),
                    Vec2::new(image_width, image_height),
                ));
                // draw::draw(ui, self.vec_shape.as_mut());
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
