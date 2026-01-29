use std::{env, fs};
use dirs;


pub fn read_alarms() -> Vec<String> {
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

pub fn get_alarms_path() -> String {
    let alarms = env::var("ALARMS").unwrap();
    let directory = env::var("DIRECTORY").unwrap();
    let home_path = dirs::home_dir().unwrap().display().to_string();

    let mut whole_path_string: String = home_path.to_owned();
    whole_path_string.push_str(&directory);
    whole_path_string.push_str(&alarms);
    whole_path_string
}

pub fn get_path_for_time(time: String) -> String {
    let sanitized_time = time.replace(":", "");
    let mut alarms_path_string = get_alarms_path();
    alarms_path_string.push_str("/");
    alarms_path_string.push_str(&sanitized_time);   
    alarms_path_string
}

pub fn create_directories_if_not_existing() {
    let alarms = env::var("ALARMS").unwrap();
    let directory = env::var("DIRECTORY").unwrap();
    let home_path = dirs::home_dir().unwrap().display().to_string();

    let mut alarms_path_string : String = home_path.to_owned();
    alarms_path_string.push_str(&directory);
    alarms_path_string.push_str(&alarms);

    fs::create_dir_all(alarms_path_string).unwrap();
}