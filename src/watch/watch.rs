use crate::pkg::result::CommonResult;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::path::Path;
use std::time::SystemTime;

trait WatchT<P: AsRef<Path> + Eq + Hash + Clone> {
    fn add(&mut self, path: P) -> bool;
    fn remove(&mut self, path: P) -> bool;
}
#[derive(Debug, Default)]
struct Watch<P: AsRef<Path> + Eq + Hash + Clone + Default> {
    dir: HashSet<P>,
    path_modified: HashMap<P, SystemTime>,
    handle_pool: Vec<P>,
}

impl<P: AsRef<Path> + Eq + Hash + Clone + Default> WatchT<P> for Watch<P> {
    fn add(&mut self, path: P) -> bool {
        self.handle_pool.push(path.clone());
        self.dir.insert(path)
    }

    fn remove(&mut self, path: P) -> bool {
        self.dir.remove(&path)
    }
}
impl<P: AsRef<Path> + Eq + Hash + Clone + Default> Watch<P> {
    pub fn new() -> Self {
        Watch::default()
    }
}
impl<P: AsRef<Path> + Eq + Hash + Clone + Default> Iterator for Watch<P> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        self.handle_pool.pop()
    }
}
#[test]
fn test_watch() {
    let mut w = Watch::new();
    w.add("./a");
    w.add("./b");
    for i in w {
        dbg!(i);
    }
}
