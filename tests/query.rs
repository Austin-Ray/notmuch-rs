use notmuch_rs::{db, query::Query};

mod common;

#[test]
fn query_simple() {
    let env = common::TestEnv::new();

    let email = lettre::Message::builder()
        .from("jdoe@example".parse().unwrap())
        .to("jdoe2@example".parse().unwrap())
        .body(String::new())
        .unwrap();

    env.add_email(email);
    env.notmuch(["new"]);

    let db = db::Database::open(&env.maildir.path(), db::OpenMode::ReadOnly).unwrap();
    let query = Query::create(&db, "from:jdoe@example");
    let cnt = query.search_messages().into_iter().count();

    assert_eq!(1, cnt);
}
