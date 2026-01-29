use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use dirs;
use dotenv::dotenv;

#[tauri::command]
fn is_dark_mode() -> bool {
    let home_path = dirs::home_dir().unwrap().display().to_string();
    let mut whole_path = home_path.to_owned();
    whole_path.push_str("/.config/cosmic/com.system76.CosmicTheme.Mode/v1/is_dark");

    let mut file = match File::open(&whole_path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", whole_path, why),
        Ok(_) => println!("{} has been read", whole_path),
    }

    content == "true"
}

#[tauri::command]
fn read_times() -> Vec<String> {
    let whole_path_string = get_alarms_path();

    let paths = fs::read_dir(&whole_path_string).unwrap();

    let mut v: Vec<String> = Vec::new();

    for path in paths {
        let time = path.unwrap().file_name().display().to_string();
        let (hours, minutes) = time.split_at(2);
        let mut time_as_string = hours.to_owned();
        time_as_string.push_str(":");
        time_as_string.push_str(minutes);
        v.push(time_as_string);
    }

    v.sort();
    v
}

#[tauri::command]
fn add_time(time: String) {
    println!("time will be added: {}", time);

    let alarms_path_string = get_path_for_time(time);
    
    let path_exists = Path::new(&alarms_path_string).exists();

    if !path_exists {
        println!("creating file {}", alarms_path_string);
        File::create_new(&alarms_path_string).expect("creation failed");
    }
}

#[tauri::command]
fn delete_time(time: String) {
    println!("time will be deleted: {}", time);

    let file_path = get_path_for_time(time);

    fs::remove_file(file_path).unwrap();
}

fn get_path_for_time(time: String) -> String {
    let sanitized_time = time.replace(":", "");
    let mut alarms_path_string = get_alarms_path();
    alarms_path_string.push_str("/");
    alarms_path_string.push_str(&sanitized_time);   
    alarms_path_string
}

fn get_alarms_path() -> String {
    let alarms = env::var("ALARMS").unwrap();
    let directory = env::var("DIRECTORY").unwrap();
    let home_path = dirs::home_dir().unwrap().display().to_string();
    
    let mut whole_path_string : String = home_path.to_owned();
    whole_path_string.push_str(&directory);
    whole_path_string.push_str(&alarms);
    whole_path_string
}

fn create_directories_if_not_existing() {
    let alarms = env::var("ALARMS").unwrap();
    let directory = env::var("DIRECTORY").unwrap();
    let home_path = dirs::home_dir().unwrap().display().to_string();

    let mut alarms_path_string : String = home_path.to_owned();
    alarms_path_string.push_str(&directory);
    alarms_path_string.push_str(&alarms);

    fs::create_dir_all(alarms_path_string).unwrap();
    /* 
    let directory_path = Path::new(&whole_path_string);

    let directory_exists = directory_path.exists();

    if !directory_exists {
        println!("creating directory at {}", whole_path_string);
        fs::create_dir_all(directory_path).unwrap();
    }

    whole_path_string.push_str("/");
    whole_path_string.push_str(&filename);

    let path_exists = Path::new(&whole_path_string).exists();

    if !path_exists {
        println!("creating file at {}", whole_path_string);
        let mut file = File::create_new(&whole_path_string).expect("creation failed");

        file.write("[]".as_bytes()).expect("write failed");
    }
    */
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    create_directories_if_not_existing();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_times, add_time, delete_time, is_dark_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
