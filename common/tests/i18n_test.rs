use std::{collections::HashSet, str::EncodeUtf16};

use common::i18n::i18n::{hello_world, localizer};


/// Test using the library without using the provided
/// [`localizer()`](library_fluent::localizer()) method.
#[test]
fn test_no_localizer() {
    let _a=&hello_world();
    println!("{}",_a);
    assert_eq!(hello_world(), "Hello World!abc")
}

#[test]
fn test_available_languages() {
    let localizer = localizer();
    assert_eq!(
        &localizer.language_loader().fallback_language().to_string(),
        "en"
    );

    let available_ids: HashSet<String> = HashSet::from_iter(
        localizer
            .available_languages()
            .unwrap()
            .into_iter()
            .map(|id| id.to_string()),
    );

    let expected_available_ids: HashSet<String> =
        HashSet::from_iter(vec!["en".to_string(),]);

    assert_eq!(available_ids, expected_available_ids)
}

/// Test loading the `en` language.
#[test]
fn test_select_english() {
    localizer().select(&["en".parse().unwrap()]).unwrap();
    assert_eq!("Hello World!", &hello_world())
}

/// Test loading the `fr` language.
#[test]
fn test_select_french() {
    localizer().select(&["fr".parse().unwrap()]).unwrap();
    assert_eq!("Bonjour le monde!", &hello_world())
}