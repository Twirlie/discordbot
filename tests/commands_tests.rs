use discordbot::{
    CodenameData, format_age_response, format_codename_response, format_register_response,
    generate_codename,
};

#[test]
fn codename_file_parses_and_has_entries() {
    let data = std::fs::read_to_string("./assets/CodenameData.json")
        .expect("Failed to read CodenameData.json");
    let animal_data: CodenameData = serde_json::from_str(&data).expect("Failed to parse JSON");

    assert!(
        !animal_data.animals.is_empty(),
        "Animals list should not be empty"
    );
    assert!(
        !animal_data.adjectives.is_empty(),
        "Adjectives list should not be empty"
    );
}

#[test]
fn generate_codename_returns_nonempty() {
    let data = std::fs::read_to_string("./assets/CodenameData.json")
        .expect("failed to read CodenameData.json");
    let codenamedata: CodenameData = serde_json::from_str(&data).expect("d");
    let codename: String = generate_codename(&codenamedata).unwrap();
    assert!(
        !codename.is_empty(),
        "Generated codename should not be empty"
    );
}

// --- Formatting helper unit tests (previously in commands_unit_tests.rs) ---

#[test]
fn register_response_format() {
    assert_eq!(
        format_register_response(),
        "Registered application commands"
    );
}

#[test]
fn age_response_format() {
    let s = format_age_response("Alice", "2020-01-01T00:00:00Z");
    assert!(s.contains("Alice"));
    assert!(s.contains("2020-01-01T00:00:00Z"));
}

#[test]
fn codename_response_format() {
    let s = format_codename_response("quick fox");
    assert!(s.contains("quick fox"));
}
