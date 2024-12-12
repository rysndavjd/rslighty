use std::fs;
use std::path::Path;
use std::num::{ParseFloatError, ParseIntError};
use std::io::{Error, Write, ErrorKind};
use std::env;
use std::process::exit;

fn list_devices() {
    return;
}

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

fn set_brightness_absolute_percent(device: &str, percentage: &u8) {
    let max_brightness: u32 = get_max_brightness(device).unwrap();
    let new_brightness: f32 = ( *percentage as f32 / 100.0 ) * *&max_brightness as f32;
    let brightness: Result<fs::File, Error> = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .open(format!("{device}/brightness"));
    
    match brightness {
        Ok(mut file) => {
            file.write_all(new_brightness.round().to_string().as_bytes()).expect("Unable to write brightness.");
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

fn help() {
    println!("usage: rslighty [--help] [--version] [--list] [--device DEVICE] [--get]");
    println!("                [--get-steps] [--set PERCENT] [--inc PERCENT] [--dec PERCENT]");
    return;
}

fn version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
    return;
}

fn main() {    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    println!("Arguments: {:?}", args);
    let mut percent: Option<String> = None;
    let mut operation: Option<String> = None;
    let mut device: Option<String> = None;
    let mut get_percent: bool = false;
    let mut get_steps: bool = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => help(),
            "-h" => help(),
            "--version" => version(),
            "-v" => version(),
            "--list" => list_devices(),
            "--device" => {
                if i + 1 < args.len() {
                    //flag = Some(true);
                    device = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    println!("Error: '--flag' requires a value");
                    return;
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                return;
            }
        }
        i += 1;
    }
    match device {
        Some(s) => println!("{s}"),
        None => println!("None"), 
    };
        //match flag {
        //    Some(true) => {
        //        println!("Flag is set, with value: {}", value.unwrap());
        //    }
        //    None => {
        //        println!("Flag is not set.");
        //    }
        //}

    //match check_backlight("apple-panel-bl") {
    //    Ok(backlight) => {
    //        //if percentage > &100 {
    //        //    eprintln!("Cannot set display brightness higher then 100%.");
    //        //    percentage = &100;
    //        //}
    //        set_brightness_absolute_percent(&backlight, &100);
    //    },
    //    Err(e) => eprintln!("Error: {}", e),
    //}
}
