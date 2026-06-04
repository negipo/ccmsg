use ccmsg::db::{basename, Database};
use ccmsg::models::{ReceiveOutcome, RegisterOutcome};

#[test]
fn test_basename_returns_last_path_component() {
    assert_eq!(basename("/Users/alice/src/example-repo"), "example-repo");
    assert_eq!(basename("/Users/alice/src/example-repo/"), "example-repo");
    assert_eq!(basename("example-repo"), "example-repo");
}

#[test]
fn test_in_memory_db_creates_schema() {
    let db = Database::in_memory().unwrap();
    assert_eq!(db.list_agents().unwrap(), Vec::<String>::new());
}

#[test]
fn test_register_self_inserts_then_unchanged() {
    let db = Database::in_memory().unwrap();
    let first = db
        .register_self("repo-a", "/Users/alice/src/repo-a")
        .unwrap();
    assert_eq!(first, RegisterOutcome::Registered);
    let second = db
        .register_self("repo-a", "/Users/alice/src/repo-a")
        .unwrap();
    assert_eq!(second, RegisterOutcome::Unchanged);
    assert_eq!(db.list_agents().unwrap(), vec!["repo-a".to_string()]);
}

#[test]
fn test_register_self_detects_collision() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-a", "/Users/alice/src/repo-a")
        .unwrap();
    let outcome = db
        .register_self("repo-a", "/Users/bob/clone/repo-a")
        .unwrap();
    assert_eq!(
        outcome,
        RegisterOutcome::Collision {
            existing_path: "/Users/alice/src/repo-a".to_string()
        }
    );
    assert_eq!(db.list_agents().unwrap(), vec!["repo-a".to_string()]);
}

#[test]
fn test_send_to_known_agent_succeeds() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    db.register_self("repo-b", "/p/repo-b").unwrap();
    db.send_message("repo-a", "repo-b", "hello").unwrap();
    let unread = db.claim_unread("repo-b").unwrap();
    assert_eq!(unread.len(), 1);
    assert_eq!(unread[0].body, "hello");
    assert_eq!(unread[0].from_agent, "repo-a");
}

#[test]
fn test_send_to_unknown_agent_fails() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    let err = db.send_message("repo-a", "ghost", "hello").unwrap_err();
    assert!(err.to_string().contains("ghost"));
}

#[test]
fn test_send_to_self_fails() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    let err = db.send_message("repo-a", "repo-a", "hello").unwrap_err();
    assert!(err.to_string().contains("yourself"));
}

#[test]
fn test_send_empty_body_fails() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    db.register_self("repo-b", "/p/repo-b").unwrap();
    let err = db.send_message("repo-a", "repo-b", "   ").unwrap_err();
    assert!(err.to_string().contains("empty"));
}

#[test]
fn test_claim_unread_marks_read_and_returns_once() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    db.register_self("repo-b", "/p/repo-b").unwrap();
    db.send_message("repo-a", "repo-b", "m1").unwrap();
    db.send_message("repo-a", "repo-b", "m2").unwrap();

    let first = db.claim_unread("repo-b").unwrap();
    assert_eq!(first.len(), 2);
    let second = db.claim_unread("repo-b").unwrap();
    assert_eq!(second.len(), 0);
}

#[test]
fn test_claim_unread_only_returns_own_messages() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    db.register_self("repo-b", "/p/repo-b").unwrap();
    db.send_message("repo-a", "repo-b", "for-b").unwrap();
    let a_unread = db.claim_unread("repo-a").unwrap();
    assert_eq!(a_unread.len(), 0);
}

#[test]
fn test_peek_unread_does_not_mark_read() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    db.register_self("repo-b", "/p/repo-b").unwrap();
    db.send_message("repo-a", "repo-b", "m1").unwrap();

    let peeked = db.peek_unread("repo-b").unwrap();
    assert_eq!(peeked.len(), 1);
    let claimed = db.claim_unread("repo-b").unwrap();
    assert_eq!(claimed.len(), 1);
}

#[test]
fn test_receive_once_claims_when_no_collision() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-b", "/p/repo-b").unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    db.send_message("repo-a", "repo-b", "hi").unwrap();

    let outcome = db.receive_once("repo-b", "/p/repo-b").unwrap();
    match outcome {
        ReceiveOutcome::Claimed(msgs) => assert_eq!(msgs.len(), 1),
        _ => panic!("claim を期待"),
    }
    assert_eq!(db.peek_unread("repo-b").unwrap().len(), 0);
}

#[test]
fn test_receive_once_holds_read_on_collision() {
    let db = Database::in_memory().unwrap();
    db.register_self("repo-b", "/p/repo-b").unwrap();
    db.register_self("repo-a", "/p/repo-a").unwrap();
    db.send_message("repo-a", "repo-b", "hi").unwrap();

    let outcome = db.receive_once("repo-b", "/other/repo-b").unwrap();
    match outcome {
        ReceiveOutcome::Collision {
            existing_path,
            unread,
        } => {
            assert_eq!(existing_path, "/p/repo-b");
            assert_eq!(unread.len(), 1);
        }
        _ => panic!("collision を期待"),
    }
    assert_eq!(db.peek_unread("repo-b").unwrap().len(), 1);
}
