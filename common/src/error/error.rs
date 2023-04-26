pub struct CodeError {
    code: i32,
    msg: String,
    field: Vec<String>,
}

struct ErrorLangInfo {
    id: i32,
    lang: String,
    msg: String,
}

pub enum ErrorEnum {
    // TaskNotfound { info: ErrorLangInfo{} },
}

// pub fn trans(lang: &str, ce: &mut CodeError) {
//     let lang =match lang {
//         LANG_EN=>lang,
//         LANG_CN=>lang,
//         _=>LANG_EN,
//     };
//     let lang=match lang.parse::<LanguageIdentifier>() {
//         Ok(r) => r,
//         Err(e) => panic!("lang parse err:{}",e),
//     };
//     match localizer().select(&[lang]) {
//         Ok(r) => r,
//         Err(_) => {},
//     };
// }
