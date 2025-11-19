use discordbot::{db_setup, insert_command_history_sync, log_command_usage_with_author};
use rusqlite::Connection;
use tempfile::NamedTempFile;

#[tokio::test]
async fn db_setup_creates_table() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path().to_str().expect("path to str");

    let dbdata = db_setup(path).await;

    let mut stmt = dbdata
        .db
        .prepare(
            "SELECT count(name) FROM sqlite_master WHERE type='table' AND name='command_history';",
        )
        .expect("prepare stmt");
    let count: i64 = stmt.query_row([], |r| r.get(0)).expect("query row");
    assert_eq!(count, 1, "command_history table should exist");
}

#[tokio::test]
async fn insert_command_history_creates_row() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path().to_str().expect("path to str");

    let _ = db_setup(path).await;
    insert_command_history_sync(path, "42", "testuser", "testcmd", "ok").expect("insert");

    let conn = Connection::open(path).expect("open conn");
    let mut stmt = conn
        .prepare(
            "SELECT user_id, username, command, output FROM command_history WHERE user_id = ?1",
        )
        .expect("prepare");
    let mut rows = stmt.query(["42"]).expect("query");
    let row = rows.next().expect("row").expect("row unwrap");
    let user_id: String = row.get(0).expect("get user_id");
    let username: String = row.get(1).expect("get username");
    let command: String = row.get(2).expect("get command");
    let output: String = row.get(3).expect("get output");

    assert_eq!(user_id, "42");
    assert_eq!(username, "testuser");
    assert_eq!(command, "testcmd");
    assert_eq!(output, "ok");
}

#[tokio::test]
#[should_panic]
async fn db_setup_panics_on_directory_path() {
    let tmpdir = tempfile::tempdir().expect("create temp dir");
    let dir_path = tmpdir.path().to_str().expect("path to str");

    let _ = db_setup(dir_path).await;
}

#[tokio::test]
#[should_panic(expected = "Failed to set up database")]
async fn db_setup_panics_on_invalid_db_file() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path();
    std::fs::write(path, "not a sqlite db").expect("write garbage to file");
    let path_str = path.to_str().expect("path to str");

    let _ = db_setup(path_str).await;
}

#[tokio::test]
async fn log_command_usage_with_author_inserts_row() {
    let tmp = NamedTempFile::new().expect("create temp file");
    let path = tmp.path().to_str().expect("path to str");

    let _ = db_setup(path).await;
    log_command_usage_with_author(path, "7", "asyncuser", "acmd", "done").await;

    let conn = Connection::open(path).expect("open conn");
    let mut stmt = conn
        .prepare(
            "SELECT user_id, username, command, output FROM command_history WHERE user_id = ?1",
        )
        .expect("prepare");
    let mut rows = stmt.query(["7"]).expect("query");
    let row = rows.next().expect("row").expect("row unwrap");
    let user_id: String = row.get(0).expect("get user_id");
    let username: String = row.get(1).expect("get username");
    let command: String = row.get(2).expect("get command");
    let output: String = row.get(3).expect("get output");

    assert_eq!(user_id, "7");
    assert_eq!(username, "asyncuser");
    assert_eq!(command, "acmd");
    assert_eq!(output, "done");
}
