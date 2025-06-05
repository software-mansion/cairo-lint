use cairo_lint::context::get_all_fix_messages;

#[test]
fn check_fix_message() {
    let fix_messages = get_all_fix_messages();

    // Ensure all lints with a fixer have a non-empty fix_message
    for msg in fix_messages {
        assert!(
            msg.is_some(),
            "Every lint with `has_fixer` must provide a `fix_message`"
        );
        assert!(
            !msg.unwrap().is_empty(),
            "`fix_message` should not be an empty string"
        );
    }
}

#[test]
#[should_panic(expected = "`fix_message` should not be an empty string")]
fn test_empty_fix_message_panics() {
    let mut fix_messages = get_all_fix_messages();

    // Simulate a lint with an empty fix_message (for testing purposes)
    let empty_fix_message: Option<&'static str> = Some("");
    fix_messages.push(empty_fix_message);

    // Trigger the assertion that checks for non-empty messages
    for msg in fix_messages {
        assert!(
            msg.is_some(),
            "Every lint with `has_fixer` must provide a `fix_message`"
        );
        assert!(
            !msg.unwrap().is_empty(),
            "`fix_message` should not be an empty string"
        );
    }
}
