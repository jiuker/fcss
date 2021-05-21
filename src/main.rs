use clap::{App, Arg};
use fcss::config::config::Config;
use fcss::config::reg::parse;
use fcss::pkg::dir::walk_all_dir;
use fcss::watch::watch::Watch;
use inotify::{EventMask, Inotify, WatchMask};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::thread::spawn;

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
    println!("ready to load css!");
    let (_, mut css) = parse(config.reg.as_str()).ok().unwrap();
    css.extend_import().unwrap();
    println!("load css:{}", css.to_string().unwrap());
    let watch = Arc::new(Watch::new("vue".to_string()));
    for dir in config.watch_dir {
        watch.add(dir);
    }
    let w_c = watch.clone();
    spawn(move || {
        w_c.watch();
    });
    while let Ok(p) = watch.receiver.lock().unwrap().recv() {
        println!("{}", p);
        break;
    }
}
