use crate::pkg::result::CommonResult;
use std::fs::read_dir;
use std::ops::Add;

pub fn walk_all_dir(path: &String) -> CommonResult<Vec<String>> {
    let current_list = std::fs::read_dir(path)?;
    let mut paths: Vec<String> = vec![];
    for entry in current_list {
        let each_path = entry?;
        let x_meta = each_path.metadata()?;
        if x_meta.is_dir() {
            for path in walk_all_dir(
                &match each_path.path().to_str() {
                    Some(d) => d,
                    None => {
                        return Err(Box::from("没有找到该目录"));
                    }
                }
                .to_owned(),
            )? {
                paths.push(path);
            }
            paths.push(each_path.path().to_str().unwrap().to_string());
        }
    }
    paths.push(path.clone());
    Ok(paths)
}
