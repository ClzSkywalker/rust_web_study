use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug,Clone, Copy,strum_macros::Display)]
pub enum ErrorKey {
    System=500,
    Hello=100001,
}

pub static ERROR_MAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let task = vec![ErrorKey::Hello];
    let mut one: HashMap<i32, String> = HashMap::new();

    for ele in task {
        one.insert(ele as i32, ele.to_string());
    }
    one
});
