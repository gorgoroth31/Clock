use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
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
    let whole_path_string = get_file_path();

    let path = Path::new(&whole_path_string);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} has been read", display),
    }

    let mut v: Vec<String> = serde_json::from_str(&s).unwrap();
    v.sort();
    v
}

#[tauri::command]
fn add_time(time: String) {
    println!("time will be added: {}", time);

    let mut data = read_times();

    data.push(time);

    let serialized = serde_json::to_string(&data).unwrap();

    let whole_path_string = get_file_path();

    let mut file = OpenOptions::new()
        .write(true)
        .open(whole_path_string)
        .expect("cannot open file");

    file.write(serialized.as_bytes()).expect("adding time failes");
}

#[tauri::command]
fn delete_time(time: String) {
    println!("time will be deleted: {}", time);

    let mut data = read_times();

    data.retain(|v| v != &time);

    let serialized = serde_json::to_string(&data).unwrap();

    let whole_path_string = get_file_path();

    let mut file = OpenOptions::new()
        .write(true)
        .open(whole_path_string)
        .expect("cannot open file");

    // needed to remove contents of file before writing new data
    // no idea why, but without this line, the data is not being written properly
    file.set_len(0).unwrap();

    file.write(serialized.as_bytes()).expect("deleting time failes");
}

fn get_file_path() -> String {
    let filename = env::var("FILE_NAME").unwrap();
    let directory = env::var("DIRECTORY").unwrap();
    let home_path = dirs::home_dir().unwrap().display().to_string();
    
    let mut whole_path_string : String = home_path.to_owned();
    whole_path_string.push_str(&directory);
    whole_path_string.push_str("/");
    whole_path_string.push_str(&filename);
    whole_path_string
}

fn create_file_if_not_exists() {
    let filename = env::var("FILE_NAME").unwrap();
    let directory = env::var("DIRECTORY").unwrap();
    let home_path = dirs::home_dir().unwrap().display().to_string();

    let mut whole_path_string : String = home_path.to_owned();
    whole_path_string.push_str(&directory);
    
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    create_file_if_not_exists();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_times, add_time, delete_time, is_dark_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
