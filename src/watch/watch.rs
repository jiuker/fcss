use crate::pkg::result::CommonResult;
use nom::dbg_dmp;
use nom::lib::std::iter::FilterMap;
use std::cell::RefCell;
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

#[derive(Debug)]
pub struct Watch {
    file_type: String,
    dir: Arc<Mutex<HashSet<String>>>,
    path_modified: Arc<Mutex<HashMap<String, SystemTime>>>,
    sender: SyncSender<String>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Watch {
    pub fn new(t: String) -> Self {
        let (sender, receiver) = sync_channel::<String>(10);
        Watch {
            file_type: t,
            dir: Arc::new(Mutex::new(Default::default())),
            path_modified: Arc::new(Mutex::new(Default::default())),
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
    fn add(&self, path: String) -> bool {
        self.dir.lock().unwrap().insert(path);
        self.path_modified.lock().unwrap().clear();
        for d in self.dir.lock().unwrap().iter() {
            for (p, s) in self.walk(d).unwrap() {
                match self.path_modified.lock().unwrap().insert(p, s) {
                    _ => {}
                };
            }
        }
        return true;
    }

    fn remove(&mut self, path: String) -> bool {
        self.dir.lock().unwrap().remove(&path)
    }

    fn watch(&self) {
        loop {
            sleep(Duration::new(1, 0));
            {
                for (p, s) in self.path_modified.lock().unwrap().iter_mut() {
                    let ns = match self.read_file_modify_time(p) {
                        Ok(d) => d,
                        Err(_) => continue,
                    };
                    if !(*s).eq(&ns) {
                        *s = ns;
                        self.sender.send(p.clone());
                    }
                }
            }
        }
    }
    fn read_file_modify_time(&self, p: &String) -> CommonResult<SystemTime> {
        Ok(File::open(p)?.metadata()?.modified()?)
    }
    pub fn walk(&self, dir: &String) -> CommonResult<HashMap<String, SystemTime>> {
        let mut h = HashMap::default();
        let p_dir = read_dir(dir)?;
        for p_file_or_dir in p_dir {
            let d = p_file_or_dir?;
            let m = d.metadata()?;
            let p = match d.path().to_str() {
                Some(d) => d,
                None => continue,
            }
            .to_string();
            if m.is_dir() {
                for (p, t) in self.walk(&p)? {
                    h.insert(p, t);
                }
            } else {
                // 需要的才处理
                if p.ends_with(&self.file_type) {
                    h.insert(p, m.modified()?);
                }
            }
        }
        Ok(h)
    }
}

#[test]
fn test_watch() {
    let mut w = Arc::new(Watch::new("vue".to_string()));
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
