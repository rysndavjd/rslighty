use std::fs;
use std::path::Path;
use std::num::{ParseFloatError, ParseIntError};
use std::io::{Error, Write, ErrorKind};

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

fn set_brightness_absolute_percent(device: &str, percentage: &u32) {
    let max_brightness: u32 = get_max_brightness(device).unwrap();
    let new_brightness: u32 = ( percentage / 100 ) * max_brightness;
    println!("new_brightness: {new_brightness}");
    
    let brightness: Result<fs::File, Error> = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .open(format!("{device}/brightness"));
    
    match brightness {
        Ok(mut file) => {
            file.write_all(new_brightness.to_string().as_bytes()).expect("Unable to write brightness");
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => eprintln!("The file was not found."),
            ErrorKind::PermissionDenied => eprintln!("Permission denied."),
            e => eprintln!("An error occurred: {}", e),
        }
    }
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
    set_brightness_absolute_percent("/sys/class/backlight/apple-panel-bl/", &100);
    match check_backlight("apple-panel-bl") {
        Ok(o) => println!("works: {}", o),
        Err(e) => eprintln!("error: {}", e),
    }
}
