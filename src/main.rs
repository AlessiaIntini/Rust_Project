mod utility;
use utility::screenshots::{
    get_all_display, os_type, take_screenshot_all_displays, take_screenshot_area,
    take_screenshot_display,
};

fn main() {
    let screens = get_all_display();
    for screen in &screens {
        println!("screen {:?}", screen);
    }
    os_type();
    take_screenshot_all_displays()
        .unwrap()
        .save("target/0.jpg")
        .unwrap();
    for screen in &screens {
        take_screenshot_display(*screen)
            .unwrap()
            .save(format!("target/{}.jpg", screen.display_info.id))
            .unwrap();
    }
    take_screenshot_area(screens[0], 0, 0, 800, 400)
        .unwrap()
        .save("target/1.png")
        .unwrap();
}
