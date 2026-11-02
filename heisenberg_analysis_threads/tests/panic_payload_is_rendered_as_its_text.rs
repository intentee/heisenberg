use heisenberg_analysis_threads::panic_payload::PanicPayload;

#[test]
fn panic_payload_is_rendered_as_its_text() {
    assert_eq!(
        [
            PanicPayload(Box::new("static message")),
            PanicPayload(Box::new(String::from("formatted message"))),
            PanicPayload(Box::new(7_u8)),
        ]
        .map(|payload| payload.to_string()),
        [
            "static message",
            "formatted message",
            "the panic carries no text",
        ]
    );
}
