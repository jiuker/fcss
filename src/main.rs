use clap::{App, Arg};
use fcss::config::config::Config;
use fcss::pkg::dir::walk_all_dir;
use inotify::{EventMask, Inotify, WatchMask};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("fcss")
        .version("1.0.1")
        .author("jiuker")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a config file")
                .takes_value(true),
        )
        .get_matches();
    let config_path = matches
        .value_of("config")
        .unwrap_or("./res/test/config.json");
    println!("load config {} file", config_path);
    let config: Config;
    {
        config = serde_json::from_reader(File::open(config_path).unwrap()).unwrap();
    }
    println!("config {:?}", config);
    let mut file_notify = Inotify::init().unwrap();
    for parent_dir in config.watch_dir {
        for each_dir in walk_all_dir(&parent_dir).unwrap() {
            file_notify
                .add_watch(&each_dir, WatchMask::MODIFY | WatchMask::CREATE)
                .expect("Failed to add inotify watch");
            println!("Add dir {} to watch Success", each_dir)
        }
    }
    let mut buffer = [0u8; 4096];
    loop {
        let events = file_notify
            .read_events_blocking(&mut buffer)
            .expect("Read File Events Error");
        for event in events {
            if event.mask.contains(EventMask::CREATE) {
                if !event.mask.contains(EventMask::ISDIR) {
                    println!("File created: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::MODIFY) {
                if !event.mask.contains(EventMask::ISDIR) {
                    println!("File modified: {:?}", event.name);
                }
            }
        }
    }
}
