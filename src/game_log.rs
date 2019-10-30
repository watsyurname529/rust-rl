pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn new(size: usize) -> GameLog {
        GameLog {
            entries: Vec::with_capacity(size),
        }
    }

    pub fn add_message(&mut self, msg: String) {
        if self.entries.len() == self.entries.capacity() {
            self.entries.pop();
        }

        self.entries.insert(0, msg);
    }
}
