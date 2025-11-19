use discordbot::CodenameData;
use discordbot::{CODENAME_DATA, generate_codename};

#[test]
fn codename_data_oncecell_is_empty_by_default() {
    assert!(CODENAME_DATA.get().is_none());
}

#[test]
fn error_and_context_aliases_compile() {
    let _ = std::any::type_name::<discordbot::Error>();
    let _ = std::any::type_name::<discordbot::Context<'static>>();
}

#[test]
fn generate_codename_errors_on_empty_parts() {
    let empty = CodenameData {
        animals: vec![],
        adjectives: vec![],
    };
    assert!(generate_codename(&empty).is_err());

    let only_animals = CodenameData {
        animals: vec!["fox".to_string()],
        adjectives: vec![],
    };
    assert!(generate_codename(&only_animals).is_err());

    let only_adjectives = CodenameData {
        animals: vec![],
        adjectives: vec!["quick".to_string()],
    };
    assert!(generate_codename(&only_adjectives).is_err());
}

#[test]
fn generate_codename_returns_ok_for_valid_data() {
    let data = CodenameData {
        animals: vec!["fox".to_string()],
        adjectives: vec!["quick".to_string()],
    };
    let res = generate_codename(&data).expect("should generate");
    assert!(res.contains("quick"));
    assert!(res.contains("fox"));
}

// Simple property-style test: generate multiple codenames and ensure they are non-empty
#[test]
fn generate_codename_multiple_runs() {
    let data = CodenameData {
        animals: vec!["fox".to_string(), "dog".to_string()],
        adjectives: vec!["quick".to_string(), "brown".to_string()],
    };

    for _ in 0..10 {
        let res = generate_codename(&data).expect("should generate");
        assert!(!res.is_empty());
    }
}
