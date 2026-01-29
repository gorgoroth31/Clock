use dotenv::dotenv;
use chrono::prelude::*;
use std::{thread, time};
use std::time::{SystemTime};
use std::fs::File;
use std::io::BufReader;
use src_helper;

fn main() {
    dotenv().ok();
    src_helper::create_directories_if_not_existing();

    loop {
        let all_alarms = src_helper::read_alarms();
        let current_time = SystemTime::now();

        let datetime: DateTime<Local> = current_time.into();

        let hour = datetime.hour();
        let minute = datetime.minute();

        for _alarm in all_alarms {
            println!("{}:{}", hour, minute);
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                    .expect("open default audio stream");

            let file = BufReader::new(File::open("alarm.mp3").unwrap());
            let _sink = rodio::play(&stream_handle.mixer(), file).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));
        }

        let current_seconds = Utc::now().second();
        let remaining = 62 - current_seconds;
        let remaining_seconds_to_full_minute = time::Duration::new(u64::from(remaining), 0);
        thread::sleep(remaining_seconds_to_full_minute);
    }
}
