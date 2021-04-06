use crate::pkg::result::CommonResult;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::path::Path;
use std::time::SystemTime;

trait WatchT<P: AsRef<Path> + Eq + Hash> {
    fn add(&mut self, path: P) -> bool;
    fn remove(&mut self, path: P) -> bool;
}
#[derive(Debug)]
struct Watch<P: AsRef<Path> + Eq + Hash> {
    dir: HashSet<P>,
    path_modified: HashMap<P, SystemTime>,
}

impl<P: AsRef<Path> + Eq + Hash> WatchT<P> for Watch<P> {
    fn add(&mut self, path: P) -> bool {
        self.dir.insert(path)
    }

    fn remove(&mut self, path: P) -> bool {
        self.dir.remove(&path)
    }
}
impl<P: AsRef<Path> + Eq + Hash> Watch<P> {
    pub fn new() -> Self {
        Watch {
            dir: Default::default(),
            path_modified: Default::default(),
        }
    }
}

#[test]
fn test_watch() {
    let mut w = Watch::new();
    w.add("./a");
    w.add("./b");
    dbg!(w);
}
