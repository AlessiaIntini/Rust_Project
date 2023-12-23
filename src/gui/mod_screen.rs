use std::path::PathBuf;
use arboard::Clipboard;
use rfd::FileDialog;
use image::DynamicImage;
use std::borrow::Cow;

pub fn save_as_image(path:&Option<PathBuf>,screenshot:&Option<DynamicImage>) {
    let path = FileDialog::new()
        .add_filter("PNG", &["png"])
        .add_filter("JPG", &["jpg"])
        .add_filter("GIF", &["gif"])
        .add_filter("BMP", &["bmp"])
        .set_directory(path.as_ref().unwrap().clone())
        .set_file_name(format!(
            "screenshot_{}",
            chrono::Local::now().format("%Y_%m_%d_%H_%M_%S").to_string()
        ))
        .save_file();
    match path {
        Some(path) => {
            match image::save_buffer(
                path,
                &screenshot.as_ref().unwrap().as_bytes(),
                screenshot.as_ref().unwrap().width() as u32,
                screenshot.as_ref().unwrap().height() as u32,
                image::ColorType::Rgba8,
            ) {
                Ok(_) => println!("Screenshot saved"),
                Err(err) => println!("{}", err),
            }
        }
        None => {}
    }
}
pub fn save_image(path:&Option<PathBuf>,screenshot:&Option<DynamicImage>) {
    let mut p = path.as_ref().unwrap().clone();
    p = p.join(format!(
        "screenshot_{}.png",
        chrono::Local::now().format("%Y_%m_%d_%H_%M_%S").to_string()
    ));
    //TODO: notify the user if the file exists
    screenshot.as_ref().unwrap().save(p.clone()).unwrap();
}
pub fn copy_image(screenshot:&Option<DynamicImage>) {
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
