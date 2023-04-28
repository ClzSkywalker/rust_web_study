use std::fmt::{self, Debug};

use super::i18n_key::ErrorKey;

/**
 * @Author         : ClzSkywalker
 * @Date           : 2023-04-27
 * @Description    : 内部错误
 * @return          {*}
 */
#[derive(Debug)]
pub struct CodeError {
    key: ErrorKey,
    // field: Vec<(String, String)>,
}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "key:{}", self.key as i32)
    }
}
