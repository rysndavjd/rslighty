use std::fs;
use std::path::Path;
use std::num::{ParseFloatError, ParseIntError};

fn get_brightness(device: &str) -> Result<f32, String> {
    let brightness = fs::read_to_string(format!("{}/brightness", device))
    .map_err(|err| format!("Reading brightness failed: {}", err))?;

    brightness.trim().parse::<f32>()
    .map_err(|err: ParseFloatError| format!("Failed to convert brightness to float: {}", err))

}

fn get_max_brightness(device: &str) -> Result<u32, String> {
    let max_brightness = fs::read_to_string(format!("{}/max_brightness", device))
        .map_err(|err| format!("Reading max brightness failed: {}", err))?;

    max_brightness.trim().parse::<u32>()
        .map_err(|err: ParseIntError| format!("Failed to convert max brightness to int: {}", err))

}


fn check_backlight(backlight: &str) -> Result<String, String> {
    let class: &Path = Path::new(&backlight);
    let class_backlight: String = format!("/sys/class/backlight/{}", backlight);
    let class_leds: String = format!("/sys/class/leds/{}", backlight);

    if class.exists() && backlight.contains("/sys/class") {
        return Ok(backlight.to_string());
    } else if Path::new(&class_backlight).exists() {
        return Ok(class_backlight);
    } else if Path::new(&class_leds).exists() {
        return Ok(class_leds);
    } else {
        return Err(backlight.to_string());
    }
}

fn main() {
    /* 
    match get_max_brightness("/sys/class/backlight/apple-panel-bl/") {
        Ok(brightness) => println!("Max brightness is: {}", brightness),
        Err(e) => eprintln!("Error: {}", e),
    }*/

    match check_backlight("apple-panel-bl") {
        Ok(o) => println!("works: {}", o),
        Err(e) => eprintln!("error: {}", e),
    }
}
