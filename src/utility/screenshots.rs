use screenshots::{
    image::DynamicImage,
    Screen,
};

// struct DisplayImage {
//     screen: Screen,
//     image: RgbaImage,
// }

pub fn get_all_display() -> Vec<Screen> {
    let screens = Screen::all();
    match screens {
        Ok(screens) => screens,
        Err(_) => Vec::new(),
    }
}


pub fn take_screenshot_area(
    screen: Screen,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Option<DynamicImage> {
    match screen.capture_area(x, y, width, height) {
        Ok(image) => Some(DynamicImage::from(image)),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

pub fn take_screenshot_display(screen: Screen) -> Option<DynamicImage> {
    match screen.capture() {
        Ok(image) => Some(DynamicImage::from(image)),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}
