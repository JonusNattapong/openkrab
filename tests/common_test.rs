use krabkrab::common::Message;

#[test]
fn create_message_simple() {
    let m = Message::simple("hi");
    assert_eq!(m.text, "hi");
}
