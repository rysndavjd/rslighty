use std::{fs::File, fs};
use std::path::Path;
use std::io::Write;
use std::env;
use clap::{Arg, command, ArgAction};

fn list_devices() -> Result<String, Box<dyn std::error::Error>> {
    
    return Ok(String::new());
}

fn get_brightness(device: &str) -> Result<u32, Box<dyn std::error::Error>> {
    return Ok(fs::read_to_string(format!("{}/brightness", device))?.trim().parse::<u32>()?);
}

fn get_max_brightness(device: &str) -> Result<u32, Box<dyn std::error::Error>> {
    return Ok(fs::read_to_string(format!("{}/max_brightness", device))?.trim().parse::<u32>()?);
}

fn get_brightness_percent(device: &str) -> Result<u8, Box<dyn std::error::Error>> {
    return Ok(((get_brightness(device)? as f32 / get_max_brightness(device)? as f32 ) * 100.0 ).round() as u8);
}

fn set_brightness_absolute_percent(device: &str, mut percentage: &u8) -> Result<(), Box<dyn std::error::Error>> {
    if percentage > &100 {
        eprintln!("Warning cannot set brightness higher then 100%, truncating to 100%");
        percentage = &100;
    }
    let new_brightness: f32 = ( *percentage as f32 / 100.0 ) * get_max_brightness(device)? as f32;     

    let mut brightness = File::create(format!("{}/brightness", device))?;
    brightness.write(new_brightness.round().to_string().as_bytes())?;
    brightness.flush()?;

    return Ok(());
}

enum Sign {
    Plus,
    Minus,
}

fn set_brightness_relative_percent(device: &str, percentage: &u8, operation: &Sign) -> Result<(), Box<dyn std::error::Error>> {
    let new_brightness_percent: f32;
    let brightness: f32 = get_brightness(device)? as f32;
    let max_brightness: f32 = get_max_brightness(device)? as f32;
    
    match operation {
        Sign::Plus => {
            new_brightness_percent = ( ( brightness / max_brightness ) * 100.00 ) + *percentage as f32;
        },
        Sign::Minus => {
            new_brightness_percent = ( ( brightness / max_brightness ) * 100.00 ) - *percentage as f32;
        },
    }
    set_brightness_absolute_percent(device, &(new_brightness_percent as u8))?;

    return Ok(());
}

fn find_backlight(backlight: &str) -> Result<String, Box<dyn std::error::Error>> {
    let class_path: &Path = Path::new(&backlight);
    if class_path.exists() && backlight.contains("/sys/class") {
        return Ok(backlight.to_string());
    }
    let led_path = format!("/sys/class/leds/{}", backlight);
    if Path::new(&led_path).exists() {
        return Ok(led_path);
    }
    let backlight_path = format!("/sys/class/backlight/{}", backlight);
    if Path::new(&backlight_path).exists() {
        return Ok(backlight_path);
    }
    
    return Err("Device not found".into());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let matches = command!()
    .arg(Arg::new("list")
            .long("list")
            .help("lists available backlight and led devices")
            .action(ArgAction::SetTrue))
    .arg(Arg::new("device")
            .long("device")
            .help("Device to adjust brightness of")
            .value_name("device")
            .action(ArgAction::Set))
    .arg(Arg::new("get")
            .long("get")
            .help("Get current brightness of selected device in percent")
            .action(ArgAction::SetTrue)
            .requires("device"))
    .arg(Arg::new("get-steps")
            .long("get-steps")
            .help("Get brightness steps of selected device")
            .action(ArgAction::SetTrue)
            .requires("device"))
    .arg(Arg::new("set")
            .long("set")
            .help("Set brightness in percent of selected device")
            .value_name("percent")
            .action(ArgAction::Set)
            .requires("device"))
    .arg(Arg::new("inc")
            .long("inc")
            .help("Increase brightness in percent of selected device")
            .value_name("percent")
            .action(ArgAction::Set)
            .requires("device"))
    .arg(Arg::new("dec")
            .long("dec")
            .help("Decrease brightness in percent of selected device")
            .value_name("percent")
            .action(ArgAction::Set)
            .requires("device"))
    .get_matches();

    if matches.get_flag("list") {
        list_devices()?;
        return Ok(());
    }

    if matches.contains_id("get") {
        println!("{}", get_brightness_percent(matches.get_one::<String>("device").ok_or("Device not specified")?)?);
        return Ok(());
    }

    if matches.get_flag("get-steps") {
        println!("{}", get_max_brightness(matches.get_one::<String>("device").ok_or("Device not specified")?)?);
        return Ok(());
    }

    if matches.get_flag("get") {
        set_brightness_absolute_percent(matches.get_one::<String>("device").ok_or("Device not specified")?, 
        matches.get_one::<u8>("get").ok_or("Device not specified")?)?;
        return Ok(());
    }

    if matches.contains_id("inc") {
        set_brightness_relative_percent(matches.get_one::<String>("device").ok_or("Device not specified")?, 
        matches.get_one::<u8>("get").ok_or("Device not specified")?, &Sign::Plus)?;
        return Ok(());
    }

    if matches.contains_id("dec") {
        set_brightness_relative_percent(matches.get_one::<String>("device").ok_or("Device not specified")?, 
        matches.get_one::<u8>("get").ok_or("Device not specified")?, &Sign::Minus)?;
        return Ok(());
    }

    return Ok(());
}
