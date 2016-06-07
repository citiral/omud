pub struct Event {
    id: String,
}

impl Event {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}
