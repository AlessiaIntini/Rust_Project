use crate::utility::draw::{self, *};
mod settings;
use crate::utility::screenshots::{
    get_all_display, take_screenshot_all_displays, take_screenshot_area, take_screenshot_display,
};
use std::path::PathBuf;
use std::time::Duration;
mod mod_screen;
use self::mod_screen::*;

use self::settings::ImageFormat;
use crate::utility::shortcuts;
use eframe::egui;
use eframe::epaint::RectShape;
use egui::{
    Align, Color32, Context, FontFamily, Layout, Pos2, Rounding, TextureHandle, TopBottomPanel, Ui,
    Vec2,
};
use image::DynamicImage;
use rfd::FileDialog;

pub fn main_window() -> eframe::Result<()> {
    let window_option = eframe::NativeOptions {
        follow_system_theme: true,
        maximized: true,
        transparent: false,
        min_window_size: Some(Vec2::new(793.0, 547.0)),
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
#[derive(PartialEq, Eq, Debug)]
enum TypeEdit {
    Shape,
    Draw,
    Text,
    None,
}

enum ScreenshotType {
    AllScreens,
    ScreenArea,
    SelectedScreen,
}

struct RustScreenRecorder {
    screen_index: Option<u8>,
    image: TextureHandle,
    timer: Option<i64>,
    screenshot: Option<DynamicImage>,
    settings: settings::SettingsHandler,
    edit: Mood,       // se è in fase di modifica
    type_e: TypeEdit, //per il tipo di edit che c'è
    vec_shape: Vec<Shape>,
    property: ProperDraw,
    font: FontFamily,
    draw_dim_variable: i32,
    text: String,
    screens: Vec<screenshots::Screen>,
    window_size: Vec2,
    border_size: Vec2,
    image_size: Vec2,
    draw_shape: bool,
    draw_draw: bool,
    flag: i32,
    draw_text: bool,
    crop: bool,
    cut_rect: RectShape,
    cut: i32,
    pos_start: Pos2,
    pos_mouse: Pos2,
    window_width: Option<u32>,
    window_height: Option<u32>,
    border: Option<i32>,
    selected_ext: ImageFormat,
    is_taking_screenshot: bool,
    screenshot_type: ScreenshotType,
}

impl RustScreenRecorder {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let screeens = get_all_display();
        let screenshot = take_screenshot_display(screeens[0]).unwrap();
        let img = cc.egui_ctx.load_texture(
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
            screen_index: Some(0),
            image: img,
            timer: Some(0),
            screenshot: Some(screenshot),
            settings: settings::SettingsHandler::new(),
            edit: Mood::None,
            type_e: TypeEdit::None,
            vec_shape: Vec::new(),
            screens: Vec::new(),
            window_size: Vec2::new(0.0, 0.0),
            border_size: Vec2::new(0.0, 0.0),
            image_size: Vec2::new(0.0, 0.0),
            draw_shape: false,
            flag: 0,
            text: "".to_string(),
            font: FontFamily::Monospace,
            property: crate::utility::draw::ProperDraw::new(
                Some(-1),
                crate::utility::draw::Rgb::new(0, 0, 0),
                false,
                crate::utility::draw::Rgb::new(0, 0, 0),
                10.0,
            ),
            draw_dim_variable: 0,
            draw_draw: false,
            draw_text: false,
            crop: false, //first in filled
            cut_rect: egui::epaint::RectShape::new(
                egui::Rect {
                    min: egui::epaint::pos2(0., 0.),
                    max: egui::epaint::pos2(0., 0.),
                },
                Rounding::ZERO,
                Color32::TRANSPARENT,
                egui::Stroke {
                    width: 0.,
                    color: Color32::TRANSPARENT,
                },
            ),
            cut: -1,
            pos_start: Pos2::new(0.0, 0.0),
            pos_mouse: Pos2::new(0.0, 0.0),
            window_height: Some(0),
            window_width: Some(0),
            border: Some(0),
            selected_ext: ImageFormat::Png,
            is_taking_screenshot: false,
            screenshot_type: ScreenshotType::SelectedScreen,
        }
    }
    //main menu
    fn show_menu(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.screens = get_all_display();
        TopBottomPanel::top("top panel").show(ctx, |ui| {
            if self.edit == Mood::Edit {
                self.show_edit(ctx, frame);
                return;
            }
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                self.settings.render_window(ui);
                let shortcuts = self
                    .settings
                    .get_shortcuts_manager()
                    .get_shortcuts()
                    .clone();
                let qs_shortcut = shortcuts
                    .get(&shortcuts::KeyCommand::QuickSaveScreenshot)
                    .unwrap();
                let s_shortcut = shortcuts
                    .get(&shortcuts::KeyCommand::SaveScreenshot)
                    .unwrap();
                let e_shortcut = shortcuts.get(&shortcuts::KeyCommand::Edit).unwrap();
                let c_shortcut = shortcuts.get(&shortcuts::KeyCommand::Copy).unwrap();
                let t_shortcut = shortcuts
                    .get(&shortcuts::KeyCommand::TakeScreenshot)
                    .unwrap();
                if ui.button("New Screen").clicked()
                    || ui.input_mut(|i| {
                        i.consume_shortcut(&egui::KeyboardShortcut::new(
                            t_shortcut.modifier,
                            t_shortcut.key,
                        ))
                    })
                {
                    if self.timer.unwrap() != 0 {
                        std::thread::sleep(std::time::Duration::from_secs(
                            self.timer.unwrap() as u64
                        ));
                    }
                    self.is_taking_screenshot = true;
                    self.screenshot_type = ScreenshotType::AllScreens;
                    frame.set_visible(false);
                }
                self.select_timer(ui);
                if ui.button("Edit").clicked()
                    || ui.input_mut(|i| {
                        i.consume_shortcut(&egui::KeyboardShortcut::new(
                            e_shortcut.modifier,
                            e_shortcut.key,
                        ))
                    })
                {
                    self.edit = Mood::Edit;
                    if self.flag == 0 {
                        self.flag = 1;
                        self.property.draw = Some(0);
                    }
                }
                if ui.button("Save").clicked()
                    || ui.input_mut(|i| {
                        i.consume_shortcut(&egui::KeyboardShortcut::new(
                            qs_shortcut.modifier,
                            qs_shortcut.key,
                        ))
                    })
                {
                    save_image(&self.settings, &self.screenshot);
                }

                if ui.button("Save as").clicked()
                    || ui.input_mut(|i| {
                        i.consume_shortcut(&egui::KeyboardShortcut::new(
                            s_shortcut.modifier,
                            s_shortcut.key,
                        ))
                    })
                {
                    save_as_image(&self.settings, &self.screenshot, &self.selected_ext);
                }
                if ui.button("Settings").clicked() {
                    self.settings.show_window();
                }
                if ui.button("Copy").clicked()
                    || ui.input_mut(|i| {
                        i.consume_shortcut(&egui::KeyboardShortcut::new(
                            c_shortcut.modifier,
                            c_shortcut.key,
                        ))
                    })
                {
                    copy_image(&self.screenshot);
                }
                self.select_monitor(ui);
                //TODO move to settings
                // self.select_save_as_ext(ui);
            });
        });
    }
    fn show_edit(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("bottom panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let shortcuts = self
                    .settings
                    .get_shortcuts_manager()
                    .get_shortcuts()
                    .clone();
                let e_shortcut = shortcuts.get(&shortcuts::KeyCommand::Edit).unwrap();
                if ui.button("Shape").clicked() {
                    self.draw_shape = true;
                    self.draw_draw = false;
                    self.draw_text = false;
                    self.crop = false;
                    self.type_e = TypeEdit::Shape;
                    self.property.draw = Some(0);
                }

                if ui.button("Draw").clicked() {
                    self.type_e = TypeEdit::Draw;
                    self.draw_draw = true;
                    self.draw_shape = false;
                    self.draw_text = false;
                    self.crop = false;
                }

                if ui.button("Text").clicked() {
                    self.draw_shape = false;
                    self.draw_draw = false;
                    self.property.draw = Some(3);
                    self.draw_text = true;
                    self.type_e = TypeEdit::Text;
                    self.crop = false;
                }

                if ui.button("Crop").clicked() {
                    self.draw_shape = false;
                    self.draw_draw = false;
                    self.draw_text = false;
                    self.crop = true;
                }
                if self.crop {
                    let width = 1.;
                    // cut_figure(ctx, &mut self.cut,&mut self.cutRect,self.screen_index,self.timer,self.screens,self.screenshot.clone(),self.image.clone());
                    if ctx.input(|i| i.pointer.primary_clicked()) {
                        self.cut = 0
                    }
                    if ctx.input(|i| i.pointer.primary_down()) {
                        self.pos_start = ctx.input(|i| i.pointer.press_origin().unwrap());
                        self.pos_mouse = ctx.input(|i| i.pointer.hover_pos().unwrap());
                        let rectangle = egui::epaint::RectShape::new(
                            egui::Rect {
                                min: self.pos_start,
                                max: egui::epaint::pos2(self.pos_mouse.x, self.pos_mouse.y),
                            },
                            Rounding::ZERO,
                            Color32::TRANSPARENT,
                            egui::Stroke {
                                width: 1.,
                                color: Color32::BLACK,
                            },
                        );
                        self.cut_rect = rectangle;
                    }
                    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) && self.cut != -1 {
                        println!("{:?}", ctx.available_rect());
                        match self.screen_index {
                            Some(index) => {
                                if self.timer.unwrap() != 0 {
                                    std::thread::sleep(std::time::Duration::from_secs(
                                        self.timer.unwrap() as u64,
                                    ));
                                }
                                println!("{:?} {:?}", self.pos_start, self.pos_mouse);
                                let display_info =
                                    self.screens[index as usize].clone().display_info;
                                self.screenshot = take_screenshot_area(
                                    self.screens[index as usize].clone(),
                                    (self.pos_start.x + width - ctx.available_rect().max.x) as i32
                                        + display_info.width as i32,
                                    (self.pos_start.y + width - ctx.available_rect().max.y) as i32
                                        + display_info.height as i32,
                                    (self.pos_mouse.x - self.pos_start.x - 1.8 * width) as u32,
                                    (self.pos_mouse.y - self.pos_start.y - width) as u32,
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
                        self.cut_rect = egui::epaint::RectShape::new(
                            egui::Rect {
                                min: egui::epaint::pos2(0., 0.),
                                max: egui::epaint::pos2(0., 0.),
                            },
                            Rounding::ZERO,
                            Color32::TRANSPARENT,
                            egui::Stroke {
                                width: 0.,
                                color: Color32::TRANSPARENT,
                            },
                        );
                        self.cut = -1;
                    }
                }
                if ui.button("Cancel").clicked() {
                    self.draw_shape = false;
                    self.draw_draw = false;
                    self.draw_text = false;
                    self.crop = false;
                }
                if ui.button("Back").clicked() {
                    self.vec_shape.pop();
                }

                if ui.button("Save").clicked() {
                    self.flag = 0;
                    self.draw_shape = false;
                    self.draw_text = false;
                    self.draw_draw = false;
                    self.crop = false;
                    let display_info = self.screens[self.screen_index.unwrap() as usize]
                        .clone()
                        .display_info;
                    self.screenshot = take_screenshot_area(
                        self.screens[self.screen_index.unwrap() as usize].clone(),
                        0. as i32,
                        display_info.height as i32 - ctx.available_rect().max.y as i32
                            + self.border_size.y as i32
                            - 1,
                        ctx.available_rect().max.x as u32,
                        ctx.available_rect().max.y as u32 - self.border_size.y as u32,
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

                if ui.button("Exit").clicked()
                    || ui.input_mut(|i| {
                        i.consume_shortcut(&egui::KeyboardShortcut::new(
                            e_shortcut.modifier,
                            e_shortcut.key,
                        ))
                    })
                {
                    self.draw_shape = false;
                    self.draw_draw = false;
                    self.draw_text = false;
                    self.crop = false;
                    self.edit = Mood::None;
                    self.vec_shape = Vec::new();
                    self.flag = 0;
                    self.type_e = TypeEdit::None;
                }
            });
            match self.type_e {
                TypeEdit::Shape => {
                    let mut first = "";
                    match self.property.draw.unwrap() {
                        0 => first = "Cicle",
                        1 => first = "Square",
                        2 => first = "Arrow",
                        _ => (),
                    }
                    self.show_shape(ctx, frame, first, ui);
                }
                TypeEdit::Draw => {
                    self.show_draw(ctx, frame, ui);
                }
                TypeEdit::Text => {
                    TopBottomPanel::top("3bottom panel").show(ctx, |ui| {
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.text_edit_singleline(&mut self.text);

                            ui.separator();
                            ui.add(
                                egui::Slider::new(&mut self.property.width, 0.0..=50.)
                                    .text("Width"),
                            );
                            egui::ComboBox::from_label("Font")
                                .width(80.0)
                                .selected_text(self.font.to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.font,
                                        FontFamily::Monospace,
                                        "monospace",
                                    );
                                    ui.selectable_value(
                                        &mut self.font,
                                        FontFamily::Proportional,
                                        "Proportional",
                                    );
                                });
                            ui.add(
                                egui::Slider::new(&mut self.property.color.red, 0..=255)
                                    .text("Red"),
                            );
                            ui.add(
                                egui::Slider::new(&mut self.property.color.green, 0..=255)
                                    .text("Green"),
                            );
                            ui.add(
                                egui::Slider::new(&mut self.property.color.blue, 0..=255)
                                    .text("Blue"),
                            );

                            egui::widgets::color_picker::show_color(
                                ui,
                                Color32::from_rgb(
                                    self.property.color.red,
                                    self.property.color.green,
                                    self.property.color.blue,
                                ),
                                Vec2::new(18.0, 18.0),
                            );
                            if ui.button("Exit").clicked() {
                                self.type_e = TypeEdit::None;
                            }

                            self.property.draw = Some(3);
                            crate::utility::draw::create_figure(
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
                        });
                    });
                }
                TypeEdit::None => {}
            }
        });
    }
    fn show_shape(&mut self, ctx: &Context, frame: &mut eframe::Frame, first: &str, ui: &mut Ui) {
        TopBottomPanel::top("2bottom panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                egui::ComboBox::from_label("")
                    .width(80.0)
                    .selected_text(first.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.property.draw, Some(0), "cicle");
                        ui.selectable_value(&mut self.property.draw, Some(1), "square");
                        ui.selectable_value(&mut self.property.draw, Some(2), "arrow");
                    });

                egui::widgets::color_picker::show_color(
                    ui,
                    Color32::from_rgb(
                        self.property.color.red,
                        self.property.color.green,
                        self.property.color.blue,
                    ),
                    Vec2::new(18.0, 18.0),
                );
                ui.add(egui::Slider::new(&mut self.property.color.red, 0..=255).text("Red"));
                ui.add(egui::Slider::new(&mut self.property.color.green, 0..=255).text("Green"));
                ui.add(egui::Slider::new(&mut self.property.color.blue, 0..=255).text("Blue"));

                ui.separator();
                ui.add(egui::Slider::new(&mut self.property.width, 0.0..=50.).text("Width"));

                if ui.checkbox(&mut self.property.filled, "Fill").clicked() {
                    self.property.filled = self.property.filled;
                }
                if ui.button("Exit").clicked() {
                    self.type_e = TypeEdit::None;
                    self.property.filled = false;
                }

                if self.property.draw.unwrap() >= 0 {
                    crate::utility::draw::create_figure(
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
            });
        });
    }
    fn show_draw(&mut self, ctx: &Context, frame: &mut eframe::Frame, ui: &mut Ui) {
        TopBottomPanel::top("3bottom panel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                self.property.draw = Some(4);
                egui::widgets::color_picker::show_color(
                    ui,
                    Color32::from_rgb(
                        self.property.color.red,
                        self.property.color.green,
                        self.property.color.blue,
                    ),
                    Vec2::new(18.0, 18.0),
                );
                ui.add(egui::Slider::new(&mut self.property.color.red, 0..=255).text("Red"));
                ui.add(egui::Slider::new(&mut self.property.color.green, 0..=255).text("Green"));
                ui.add(egui::Slider::new(&mut self.property.color.blue, 0..=255).text("Blue"));

                ui.separator();
                ui.add(egui::Slider::new(&mut self.property.width, 0.0..=50.).text("Width"));

                if ui.button("Exit").clicked() {
                    self.type_e = TypeEdit::None;
                }
                crate::utility::draw::create_figure(
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
            });
        });
    }
    fn select_monitor(&mut self, ui: &mut Ui) {
        let screens = get_all_display();
        egui::ComboBox::from_id_source("Monitor")
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

    fn select_save_as_ext(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_id_source("save_as_ext")
            .width(80.0)
            .selected_text(format!("{}", self.selected_ext.get_ext().to_uppercase()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected_ext, ImageFormat::Png, "PNG");
                ui.selectable_value(&mut self.selected_ext, ImageFormat::Jpg, "JPG");
                ui.selectable_value(&mut self.selected_ext, ImageFormat::Bmp, "BMP");
                ui.selectable_value(&mut self.selected_ext, ImageFormat::Gif, "GIF");
            });
    }
}

impl eframe::App for RustScreenRecorder {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.is_taking_screenshot {
            self.is_taking_screenshot = false;
            std::thread::sleep(Duration::from_millis(250));
            match self.screenshot_type {
                ScreenshotType::AllScreens => {
                    let screenshot = take_screenshot_all_displays();
                    self.image = ctx.load_texture(
                        "screenshot",
                        egui::ColorImage::from_rgba_unmultiplied(
                            [
                                screenshot.as_ref().unwrap().width() as usize,
                                screenshot.as_ref().unwrap().height() as usize,
                            ],
                            screenshot.as_ref().unwrap().as_bytes(),
                        ),
                        Default::default(),
                    );
                }
                ScreenshotType::ScreenArea => {}
                ScreenshotType::SelectedScreen => match self.screen_index {
                    Some(screen_index) => {
                        self.screenshot =
                            take_screenshot_display(self.screens[screen_index as usize].clone());
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
                },
            }
            frame.set_visible(true);
            return;
        }
        self.show_menu(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.property.filled {
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    egui::widgets::color_picker::show_color(
                        ui,
                        Color32::from_rgb(
                            self.property.color_fill.red,
                            self.property.color_fill.green,
                            self.property.color_fill.blue,
                        ),
                        Vec2::new(18.0, 18.0),
                    );

                    ui.add(
                        egui::Slider::new(&mut self.property.color_fill.red, 0..=255).text("Red"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.property.color_fill.green, 0..=255)
                            .text("Green"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.property.color_fill.blue, 0..=255).text("Blue"),
                    );
                });
            }

            self.window_size = Vec2::new(
                frame.info().window_info.size.x,
                frame.info().window_info.size.y,
            );
            ui.vertical_centered_justified(|ui| {
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
                ui.add(egui::Image::new((
                    self.image.id(),
                    Vec2::new(image_width, image_height),
                )));
                crate::utility::draw::draw_rect(ui, &self.cut_rect);
                crate::utility::draw::draw(ui, self.vec_shape.as_mut());
            });
        });
    }
}
