// Chat message structure for Freedom Network
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
}

impl ChatMessage {
    pub fn new(sender: &str, content: &str) -> Self {
        Self {
            sender: sender.into(),
            content: content.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    pub fn format(&self) -> String {
        format!("[{}] {}: {}", self.timestamp, self.sender, self.content)
    }
}