use screenshots::{
    image::{GenericImage, RgbaImage},
    Screen,
};
use std::{
    env::consts::OS,
    fs,
    io::{self},
};

struct DisplayImage {
    screen: Screen,
    image: RgbaImage,
}

pub fn get_all_display() -> Vec<Screen> {
    let screens = Screen::all();
    match screens {
        Ok(screens) => screens,
        Err(_) => Vec::new(),
    }
}

pub fn take_screenshot_all_displays() -> Option<RgbaImage> {
    let multi_display_images: Vec<DisplayImage> = get_all_display()
        .into_iter()
        .map(|screen| {
            let image = screen.capture().unwrap();
            DisplayImage { screen, image }
        })
        .collect();

    let x_min = multi_display_images
        .iter()
        .map(|s| s.screen.display_info.x)
        .min()?;
    let y_min = multi_display_images
        .iter()
        .map(|s| s.screen.display_info.y)
        .min()?;
    let x_max = multi_display_images
        .iter()
        .map(|s| s.screen.display_info.x + s.image.width() as i32)
        .max()?;
    let y_max = multi_display_images
        .iter()
        .map(|s| s.screen.display_info.y + s.image.height() as i32)
        .max()?;
    let offset = (x_min, y_min);
    let size = ((x_max - x_min) as u32, (y_max - y_min) as u32);
    let mut image = RgbaImage::new(size.0, size.1);
    for img in multi_display_images {
        image
            .copy_from(
                &img.image,
                (img.screen.display_info.x - offset.0) as u32,
                (img.screen.display_info.y - offset.1) as u32,
            )
            .unwrap();
    }
    Some(image)
}

pub fn take_screenshot_area(
    screen: Screen,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Option<RgbaImage> {
    match screen.capture_area(x, y, width, height) {
        Ok(image) => Some(image),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

pub fn take_screenshot_display(screen: Screen) -> Option<RgbaImage> {
    match screen.capture() {
        Ok(image) => Some(image),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

pub fn save_file<T: AsRef<[u8]>>(
    file: T,
    filename: String,
    path: Option<String>,
) -> io::Result<()> {
    match path {
        Some(p) => {
            let file_path = format!("{}/{}", p, filename);
            fs::write(file_path, file)
        }
        None => fs::write(format!("target/{}", filename), file),
    }
}

pub fn os_type() {
    println!("{:?}", OS)
}
