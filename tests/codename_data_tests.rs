use discordbot::{CODENAME_DATA, CodenameData, codename_data_setup_from_path};

#[tokio::test]
async fn test_codename_data_setup_executes_and_prevents_double_set() {
    // Ensure the OnceCell is initialized by calling the actual async setup helper
    // so the function itself is exercised (for tarpaulin coverage).
    if CODENAME_DATA.get().is_none() {
        codename_data_setup_from_path("./assets/CodenameData.json").await;
    }

    // After setup, the OnceCell should contain data
    let data = CODENAME_DATA
        .get()
        .expect("CODENAME_DATA should be initialized");
    assert!(
        !data.adjectives.is_empty(),
        "adjectives should not be empty"
    );
    assert!(!data.animals.is_empty(), "animals should not be empty");

    // Attempting to set the OnceCell again should fail
    let other = CodenameData {
        adjectives: vec!["other".to_string()],
        animals: vec!["thing".to_string()],
    };
    let res = CODENAME_DATA.set(other);
    assert!(
        res.is_err(),
        "Setting CODENAME_DATA a second time must fail"
    );
}
