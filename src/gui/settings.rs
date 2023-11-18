use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const SETTINGS_FILE: &str = "settings.json";

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum ImageFormat {
    /// An Image in PNG Format
    Png,
    /// An Image in JPEG Format
    Jpg,
    /// An Image in GIF Format
    Gif,
    /// An Image in BMP Format
    Bmp,
}

pub struct SettingsHandler {
    show_window: bool,
    settings: Settings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    //TODO: change when shortcuts are implemented
    shortcuts: Vec<String>,
    pub screenshot_path: PathBuf,
    screenshot_default_ext: ImageFormat,
}

impl ImageFormat {
    pub fn get_ext(&self) -> String {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpg => "jpg",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
        }
        .to_string()
    }
}

impl Settings {
    fn new() -> Self {
        let mut default_settings: Settings = Self {
            shortcuts: Vec::new(),
            screenshot_path: PathBuf::new(),
            screenshot_default_ext: ImageFormat::Png,
        };
        let def_path = match dirs::picture_dir() {
            Some(path) => path,
            None => {
                eprintln!("Unable to determine home directory");
                dirs::home_dir().unwrap()
            }
        };
        //create new folder in def_path named "RustScreenshot" if it doesn't exist
        let def_path = def_path.join("RustScreenshot");
        if !def_path.exists() {
            std::fs::create_dir(&def_path).unwrap();
        }
        default_settings.screenshot_path = def_path.clone();
        // if settings.json exists, load it
        let file_settings = def_path.clone().join(SETTINGS_FILE);
        if file_settings.exists() {
            default_settings =
                match serde_json::from_reader(std::fs::File::open(file_settings).unwrap()) {
                    Ok(settings) => settings,
                    Err(e) => {
                        eprintln!("Unable to load settings.json: {}", e);
                        default_settings
                    }
                };
        }
        default_settings
    }
    pub fn get_screenshot_default_ext(&self) -> String {
        self.screenshot_default_ext.get_ext()
    }
    fn save(&mut self) {
        let file_settings = self.screenshot_path.clone().join(SETTINGS_FILE);
        match serde_json::to_writer(std::fs::File::create(file_settings).unwrap(), &self) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Unable to save settings.json: {}", e);
            }
        }
    }
}

impl SettingsHandler {
    pub fn new() -> Self {
        Self {
            show_window: false,
            settings: Settings::new(),
        }
    }
    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }
    pub fn render_window(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Settings")
            .open(&mut self.show_window)
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.columns(2, |columns| {
                    columns[0].label(format!(
                        "{}",
                        self.settings.screenshot_path.to_str().unwrap()
                    ));
                    columns[1].vertical_centered(|ui| {
                        if ui.button("Change Default Path").clicked() {
                            match rfd::FileDialog::new().pick_folder() {
                                Some(path) => self.settings.screenshot_path = path,
                                None => {}
                            }
                        }
                    });
                });
                ui.separator();
                ui.columns(2, |columns| {
                    columns[0].label("Default Extension");
                    columns[1].vertical_centered(|ui| {
                        egui::ComboBox::from_id_source("default_ext")
                            .selected_text(format!(
                                "{}",
                                self.settings.get_screenshot_default_ext().to_uppercase()
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.settings.screenshot_default_ext,
                                    ImageFormat::Png,
                                    "PNG",
                                );
                                ui.selectable_value(
                                    &mut self.settings.screenshot_default_ext,
                                    ImageFormat::Jpg,
                                    "JPG",
                                );
                                ui.selectable_value(
                                    &mut self.settings.screenshot_default_ext,
                                    ImageFormat::Gif,
                                    "GIF",
                                );
                                ui.selectable_value(
                                    &mut self.settings.screenshot_default_ext,
                                    ImageFormat::Bmp,
                                    "BMP",
                                );
                            });
                    });
                });
                ui.separator();
                if ui.button("Save Settings").clicked() {
                    self.settings.save();
                }
            });
    }
    pub fn show_window(&mut self) {
        self.show_window = true;
    }
}
