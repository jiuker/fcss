use crate::pkg::result::CommonResult;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::fs::read_dir;
use std::ops::Add;

pub fn walk_all_dir(
    path: &String,
) -> CommonResult<(Vec<String>, HashMap<String, HashSet<String>>)> {
    let current_list = std::fs::read_dir(path)?;
    let mut dirs: Vec<String> = vec![];
    let mut file_dirs: HashMap<String, HashSet<String>> = Default::default();
    for entry in current_list {
        let each_path = entry?;
        let x_meta = each_path.metadata()?;
        if x_meta.is_dir() {
            let (child_dirs, child_file_dirs) =
                walk_all_dir(&each_path.path().to_str().unwrap().to_owned())?;
            for path in child_dirs {
                dirs.push(path);
            }
            for f_d in child_file_dirs {
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
            dirs.push(each_path.path().to_str().unwrap().to_string());
        } else {
            let file_name = each_path.file_name().to_str().unwrap().to_string();
            match file_dirs.get_mut(&file_name) {
                None => {
                    let mut set = HashSet::new();
                    set.insert(path.clone());
                    file_dirs.insert(file_name, set);
                }
                Some(d) => {
                    d.insert(path.clone());
                }
            };
        }
    }
    dirs.push(path.clone());
    Ok((dirs, file_dirs))
}
