use std::env;
use std::fs::{self, File};
use std::path::Path;
use dotenv::dotenv;
use src_helper;

#[tauri::command]
fn is_dark_mode() -> bool {
    let is_dark_mode;
    
    match dark_light::detect().unwrap() {
        dark_light::Mode::Dark => is_dark_mode = true,
        dark_light::Mode::Light => is_dark_mode = false,
        dark_light::Mode::Unspecified => is_dark_mode = true,
    }

    is_dark_mode
}

#[tauri::command]
fn read_alarms() -> Vec<String> {
    let v = src_helper::read_alarms();
    v
}

#[tauri::command]
fn add_time(time: String) {
    println!("time will be added: {}", time);

    let alarms_path_string = src_helper::get_path_for_time(time);
    
    let path_exists = Path::new(&alarms_path_string).exists();

    if !path_exists {
        println!("creating file {}", alarms_path_string);
        File::create_new(&alarms_path_string).expect("creation failed");
    }
}

#[tauri::command]
fn delete_time(time: String) {
    println!("time will be deleted: {}", time);

    let file_path = src_helper::get_path_for_time(time);

    fs::remove_file(file_path).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    src_helper::create_directories_if_not_existing();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_alarms, add_time, delete_time, is_dark_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
