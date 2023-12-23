use crate::utility::shortcuts;
use crate::utility::shortcuts::{modifier_to_string, Shortcut, ShortcutManager};
use egui::Modifiers;
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
    pub show_window: bool,
    settings: Settings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    shortcut_manager: shortcuts::ShortcutManager,
    pub screenshot_path: PathBuf,
    screenshot_default_ext: ImageFormat,
    // do not serialize this field
    #[serde(skip)]
    tmp_shortcut_manager: ShortcutManager,
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
            shortcut_manager: ShortcutManager::new(),
            screenshot_path: PathBuf::new(),
            screenshot_default_ext: ImageFormat::Png,
            tmp_shortcut_manager: ShortcutManager::new(),
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
            return match serde_json::from_reader(std::fs::File::open(file_settings).unwrap()) {
                Ok(settings) => settings,
                Err(e) => {
                    eprintln!("Unable to load {}: {}", SETTINGS_FILE, e);
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
                eprintln!("Unable to save {}: {}", SETTINGS_FILE, e);
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
    pub fn get_shortcuts_manager(&self) -> &ShortcutManager {
        &self.settings.shortcut_manager
    }
    pub fn render_window(&mut self, ui: &mut egui::Ui) {
        let mut tmp_shortcuts = self.settings.shortcut_manager.get_shortcuts().to_owned();
        egui::Window::new("Settings")
            .open(&mut self.show_window)
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                if self.settings.shortcut_manager.get_is_shortcut_changing() {
                    let tmp_key = &mut self.settings.tmp_shortcut_manager.tmp_key;
                    let tmp_modifier = &mut self.settings.tmp_shortcut_manager.tmp_modifier;
                    let is_editing_shortcut =
                        &mut self.settings.tmp_shortcut_manager.is_editing_shortcut;
                    let tmp_command = &self.settings.tmp_shortcut_manager.tmp_command;
                    ui.label("Press a modifier (CTRL, ALT, COMMAND, SHIFT) and key to change the shortcut");
                    if tmp_modifier.is_some() && tmp_key.is_some() {
                        *is_editing_shortcut = true;
                    }
                    if !(*is_editing_shortcut) {
                        ui.input(|i| {
                            *tmp_key = match i.keys_down.iter().next() {
                                Some(key) => Some(key.to_owned()),
                                None => return,
                            };
                            if i.modifiers.alt{
                                *tmp_modifier = Some(Modifiers::ALT);
                            } else if i.modifiers.command{
                                *tmp_modifier = Some(Modifiers::COMMAND);
                            } else if i.modifiers.shift{
                                *tmp_modifier = Some(Modifiers::SHIFT);
                            } else {
                                *tmp_modifier = None;
                            }
                        });
                    }
                    ui.separator();
                    if *is_editing_shortcut {
                        ui.label(format!(
                            "New shortcut: {} + {}",
                            modifier_to_string(tmp_modifier.unwrap()),
                            tmp_key.unwrap().name()
                        ));
                        ui.separator();
                    }
                    ui.columns(5, |columns| {
                        if columns[0].button("Go back").clicked() {
                            self.settings
                                .shortcut_manager
                                .set_is_shortcut_changing(false);
                        }
                        if *is_editing_shortcut && columns[2].button("Save").clicked() {
                            self.settings.shortcut_manager.update_shortcut(
                                tmp_command.as_ref().unwrap().to_owned(),
                                Shortcut::new(tmp_modifier.unwrap(), tmp_key.unwrap()),
                            );
                            *is_editing_shortcut = false;
                            *tmp_key = None;
                            *tmp_modifier = None;
                            self.settings
                                .shortcut_manager
                                .set_is_shortcut_changing(false);
                        }
                        if columns[4].button("Reset").clicked() {
                            *is_editing_shortcut = false;
                            *tmp_key = None;
                            *tmp_modifier = None;
                        }
                    });
                } else {
                    for (command, shortcut) in
                        self.settings.shortcut_manager.get_shortcuts().clone()
                    {
                        ui.columns(3, |columns| {
                            columns[0].label(format!("{}", command));
                            columns[1].vertical_centered(|ui| {
                                ui.label(format!("{}", shortcut));
                            });
                            columns[2].vertical_centered(|ui| {
                                if ui.button("Edit").clicked() {
                                    tmp_shortcuts.insert(command.to_owned(), shortcut.clone());
                                    self.settings
                                        .shortcut_manager
                                        .set_is_shortcut_changing(true);
                                    self.settings.tmp_shortcut_manager.tmp_command =
                                        Some(command.to_owned());
                                }
                            })
                        });
                        ui.separator();
                    }
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
                }
            });
    }
    pub fn show_window(&mut self) {
        self.show_window = true;
    }
}
