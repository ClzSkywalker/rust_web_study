use std::io;

use i18n::{trans, LANG_EN};



#[test]
fn t_test() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    a = a.trim().replace("\n", "");

    println!("{}", a);
    let a = a.parse::<i32>().unwrap();
    println!("{}", trans(LANG_EN, a, vec![("name".to_string(), "1111".to_string())]));
}