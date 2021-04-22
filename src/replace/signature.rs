use crate::pkg::result::CommonResult;
use std::collections::HashSet;

pub fn class_to_signature(cls: Vec<String>) -> CommonResult<HashSet<String>> {
    let mut rsl: HashSet<String> = Default::default();
    for class_one_line in cls {
        let multi_css = class_one_line
            .split(" ")
            .filter(|c| !c.is_empty())
            .collect::<Vec<&str>>();
        for one_css in multi_css {
            let mut index = -1;
            rsl.insert(
                one_css
                    .split("-")
                    .map(|d| {
                        index += 1;
                        if index == 0 {
                            return d.to_string();
                        } else {
                            return format!("${}", index);
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("-"),
            );
        }
    }
    Ok(rsl)
}

#[test]
fn test_class_to_signature() {
    let in_param = vec![
        ".h-12 .w-12 .b-1-fff .tcp".to_string(),
        ".h-12 .w-12 .b-1-fff .tcp .hw-12-21".to_string(),
    ];
    dbg!(class_to_signature(in_param).unwrap());
}
