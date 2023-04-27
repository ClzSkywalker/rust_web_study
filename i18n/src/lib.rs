use common::error::i18n_key::{ERROR_MAP, ErrorKey};

pub const LANG_ZH_CN: &str = "zh-CN";
pub const LANG_EN: &str = "en";

rust_i18n::i18n!("locales");

pub fn trans(lang: &str, id: i32, args: Vec<(String, String)>) -> String {
    let lang = match lang {
        LANG_ZH_CN => lang,
        LANG_EN => lang,
        _ => LANG_EN,
    };

    // i18n_key::ErrorKey.get();
    let msg = match ERROR_MAP.get(&id) {
        Some(r) => r.clone(),
        None => ErrorKey::System.to_string(),
    };

    let mut message = crate::translate(lang, msg.as_str());
    for ele in args {
        let var = ele.0;
        let mut holder = std::string::String::from("%{");
        holder += &var;
        holder.push('}');

        message = message.replace(&holder, &format!("{}", ele.1));
    }
    message
}
