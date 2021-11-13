use std::path::PathBuf;

use notmuch_rs::{db, status::Status};

mod common;

#[test]
fn open_db() {
    let env = common::TestEnv::new();

    let db = db::Database::open(&env.maildir.path(), db::OpenMode::ReadOnly);

    assert!(db.is_ok());
}

#[test]
fn open_db_no_db() {
    let db = db::Database::open(&PathBuf::from("/dev/null"), db::OpenMode::ReadOnly);

    assert!(db.is_err());
    assert_eq!(Err(Status::FileError), db);
}
