use chrono::Local;
use fs_watch_rs::event::{EventFilter, EventKind, WatchEvent};
use std::path::PathBuf;

#[test]
fn test_event_filter_extensions() {
    let filter = EventFilter::new(vec!["rs".to_string(), "toml".to_string()], None).unwrap();

    let mut event = WatchEvent {
        kind: EventKind::Created,
        path: PathBuf::from("src/main.rs"),
        timestamp: Local::now(),
    };

    assert!(filter.matches(&event), "Rust files should match");

    event.path = PathBuf::from("Cargo.toml");
    assert!(filter.matches(&event), "TOML files should match");

    event.path = PathBuf::from("README.md");
    assert!(!filter.matches(&event), "Markdown files should NOT match");
}

#[test]
fn test_event_filter_pattern() {
    let filter = EventFilter::new(vec![], Some("*.log")).unwrap();

    let mut event = WatchEvent {
        kind: EventKind::Modified,
        path: PathBuf::from("/var/log/system.log"),
        timestamp: Local::now(),
    };

    assert!(filter.matches(&event), "Log files should match pattern");

    event.path = PathBuf::from("system.txt");
    assert!(
        !filter.matches(&event),
        "Txt files should NOT match pattern"
    );
}

#[test]
fn test_event_filter_combined() {
    let filter = EventFilter::new(vec!["rs".to_string()], Some("test_*")).unwrap();

    let mut event = WatchEvent {
        kind: EventKind::Created,
        path: PathBuf::from("test_app.rs"),
        timestamp: Local::now(),
    };
    assert!(
        filter.matches(&event),
        "Should match both extension and pattern"
    );

    event.path = PathBuf::from("app.rs");
    assert!(!filter.matches(&event), "Should NOT match pattern");

    event.path = PathBuf::from("test_app.txt");
    assert!(!filter.matches(&event), "Should NOT match extension");
}
