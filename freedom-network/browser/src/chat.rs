// Minimal chat logic placeholder
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
}

impl ChatMessage {
    pub fn new(sender: &str, content: &str) -> Self {
        Self { sender: sender.into(), content: content.into() }
    }
}