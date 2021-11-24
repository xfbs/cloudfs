use bytes::Bytes;

/// Builder struct to generate an entry.
pub struct EntryBuilder {
    name: String,
}

impl EntryBuilder {
    pub fn new(name: &str) -> Self {
        EntryBuilder {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct Entry {
    data: Bytes,
}

impl Entry {
    pub fn name(&self) -> Bytes {
        self.data.clone()
    }
}
