const LANG_ZH_CN: &str = "zh-CN";
const LANG_EN: &str = "en";

rust_i18n::i18n!("locales");

pub fn trans(lang: &str, key: &str, args: &Vec<(&str, &str)>) -> String {
    let lang = match lang {
        LANG_ZH_CN => lang,
        LANG_EN => lang,
        _ => LANG_EN,
    };
    let mut message = crate::translate(lang, key);
    for ele in args {
        let var = ele.0;
        let mut holder = std::string::String::from("%{");
        holder.push_str(var);
        holder.push('}');

        message = message.replace(&holder, &format!("{}", ele.1));
    }
    message
}

#[test]
fn t_test() {
    println!(
        "{}",
        trans(LANG_EN, "messages.hello", &vec![("name", "1111")])
    );
}
