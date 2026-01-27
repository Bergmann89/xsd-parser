pub type Message = MessageType;
#[derive(Debug)]
pub struct MessageType;
impl xsd_parser_types::WithNamespace for MessageType {
    fn prefix() -> Option<&'static str> {
        Some("test")
    }
    fn namespace() -> Option<&'static str> {
        Some("http://example.com/test/01p00")
    }
}
