use arboard::Clipboard;
use image::DynamicImage;
use rfd::FileDialog;
use std::borrow::Cow;
use std::path::PathBuf;

use super::settings::SettingsHandler;

pub fn save_as_image(settings: &SettingsHandler, screenshot: &Option<DynamicImage>) {
    let path = FileDialog::new()
        .add_filter("Image", &["png", "jpg", "gif", "bmp"])
        .set_directory(settings.get_settings().screenshot_path.clone())
        .set_file_name(format!(
            "screenshot_{}",
            chrono::Local::now().format("%Y_%m_%d_%H_%M_%S").to_string()
        ))
        .save_file();
    match path {
        Some(path) => {
            let p = if cfg!(target_os = "macos") {
                PathBuf::from(path)
            } else {
                PathBuf::from(format!(
                    "{}.{}",
                    path.to_string_lossy(),
                    settings.get_settings().get_screenshot_default_ext()
                ))
            };
            match screenshot.as_ref().unwrap().save(p) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e)
                }
            }
        }
        None => {}
    }
}
pub fn save_image(settings: &SettingsHandler, screenshot: &Option<DynamicImage>) {
    let mut p: PathBuf = settings.get_settings().screenshot_path.clone();
    p = p.join(format!(
        "screenshot_{}.{}",
        chrono::Local::now().format("%Y_%m_%d_%H_%M_%S").to_string(),
        settings.get_settings().get_screenshot_default_ext()
    ));
    screenshot.as_ref().unwrap().save(p.clone()).unwrap();
}
pub fn copy_image(screenshot: &Option<DynamicImage>) {
    let mut clipboard = Clipboard::new().unwrap();
    let final_image = screenshot.as_ref().unwrap().clone();
    let bytes = final_image.as_bytes();
    let img = arboard::ImageData {
        width: final_image.width() as usize,
        height: final_image.height() as usize,
        bytes: Cow::from(bytes),
    };
    let _done = clipboard.set_image(img);
}
