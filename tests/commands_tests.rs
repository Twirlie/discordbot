use discordbot::{CodenameData, generate_codename};

#[test]
fn codename_file_parses_and_has_entries() {
    let data = std::fs::read_to_string("./assets/CodenameData.json").expect("Failed to read CodenameData.json");
    let animal_data: CodenameData = serde_json::from_str(&data).expect("Failed to parse JSON");

    assert!(!animal_data.animals.is_empty(), "Animals list should not be empty");
    assert!(!animal_data.adjectives.is_empty(), "Adjectives list should not be empty");
}

#[test]
fn generate_codename_returns_nonempty() {
    let data = std::fs::read_to_string("./assets/CodenameData.json").expect("failed to read CodenameData.json");
    let codenamedata: CodenameData = serde_json::from_str(&data).expect("d");
    let codename: String = generate_codename(&codenamedata).unwrap();
    assert!(!codename.is_empty(), "Generated codename should not be empty");
}
