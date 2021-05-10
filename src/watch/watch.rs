use crate::pkg::dir::walk_all_dir;
use crate::pkg::result::CommonResult;
use inotify::{EventMask, Inotify, WatchMask};
use nom::dbg_dmp;
use nom::lib::std::iter::FilterMap;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::{read_dir, File, FileType, Metadata};
use std::hash::Hash;
use std::ops::Deref;
use std::path::Path;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, SystemTime};

pub struct Watch {
    file_type: String,
    file_notify: Arc<Mutex<Inotify>>,
    file_dirs: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    sender: SyncSender<String>,
    pub receiver: Arc<Mutex<Receiver<String>>>,
}

impl Watch {
    pub fn new(t: String) -> Self {
        let (sender, receiver) = sync_channel::<String>(10);
        Watch {
            file_type: t,
            file_notify: Arc::new(Mutex::new(Inotify::init().unwrap())),
            file_dirs: Default::default(),
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
    pub fn add(&self, path: String) {
        let (each_dirs, file_dirs) = walk_all_dir(&path).unwrap();
        for each_dir in each_dirs {
            self.file_notify
                .lock()
                .unwrap()
                .add_watch(&each_dir, WatchMask::MODIFY | WatchMask::CREATE)
                .expect("Failed to add inotify watch");
            println!("Add dir {} to watch Success", each_dir)
        }
        for f_d in file_dirs {
            println!("Add file {} to watch Success", f_d.0.clone());
            let mut file_dirs = self.file_dirs.lock().unwrap();
            match file_dirs.get_mut(&f_d.0) {
                None => {
                    let mut set = HashSet::new();
                    set.extend(f_d.1);
                    file_dirs.insert(f_d.0, set);
                }
                Some(d) => {
                    d.extend(f_d.1);
                }
            };
        }
    }

    pub fn watch(&self) {
        let mut buffer = [0u8; 4096];
        loop {
            let events = self
                .file_notify
                .lock()
                .unwrap()
                .read_events_blocking(&mut buffer)
                .expect("Read File Events Error");
            for event in events {
                if event.mask.contains(EventMask::CREATE) || event.mask.contains(EventMask::MODIFY)
                {
                    if !event.mask.contains(EventMask::ISDIR) {
                        match self
                            .file_dirs
                            .lock()
                            .unwrap()
                            .get(&*event.name.unwrap().to_str().unwrap().to_string())
                        {
                            None => {}
                            Some(d) => {
                                for dir in d.iter() {
                                    let full_path =
                                        Path::new(dir).join(event.name.unwrap().to_str().unwrap());
                                    let m_time = full_path.metadata().unwrap().modified().unwrap();
                                    if SystemTime::now()
                                        .duration_since(m_time)
                                        .unwrap()
                                        .as_secs_f32()
                                        < 1.0
                                    {
                                        self.sender.send(full_path.to_str().unwrap().to_string());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_watch() {
    let w = Arc::new(Watch::new("vue".to_string()));
    w.add("/home/jiuker/goworkspace/src/dmallRedisSync/center/res/vue-element-admin".to_string());
    let w_c = w.clone();
    spawn(move || {
        w_c.watch();
    });
    while let Ok(p) = w.receiver.lock().unwrap().recv() {
        println!("{}", p);
        break;
    }
}
