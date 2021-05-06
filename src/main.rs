use clap::{App, Arg};
use fcss::config::config::Config;
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
}
