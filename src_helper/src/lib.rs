use std::{env, fs};
use dirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Alarm {
    pub hour: String,
    pub minute: String,
}

pub fn string_to_alarm(s: String) -> Alarm {
    println!("mapping {}", s);

    let alarm: Alarm = Alarm {
        hour: s.get(0..2).unwrap().to_string(),
        minute: s.get(3..5).unwrap().to_string(),
    };
    alarm
}

pub fn read_alarms() -> Vec<Alarm> {
    let whole_path_string = get_alarms_path();

    let paths = fs::read_dir(&whole_path_string).unwrap();

    let mut alarms: Vec<Alarm> = Vec::new();

    for path in paths {
        let time = path.unwrap().file_name().display().to_string();
        let (hours, minutes) = time.split_at(2);

        let alarm: Alarm = Alarm {
            hour: hours.parse().unwrap(),
            minute: minutes.parse().unwrap(),
        };

        alarms.push(alarm);
    }

    alarms
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

pub fn get_path_for_alarm(alarm: Alarm) -> String {
    let mut alarm_string = alarm.hour.to_owned().to_string();
    alarm_string.push_str(&alarm.minute.to_string());
    let mut alarms_path_string = get_alarms_path();
    alarms_path_string.push_str("/");
    alarms_path_string.push_str(&alarm_string);   
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