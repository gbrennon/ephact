#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    path: String,
    content: Vec<u8>,
    mode: u32,
}

impl FileEntry {
    pub fn new(path: impl Into<String>, content: Vec<u8>, mode: u32) -> Self {
        Self {
            path: path.into(),
            content,
            mode,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn mode(&self) -> u32 {
        self.mode
    }

    pub fn into_parts(self) -> (String, Vec<u8>, u32) {
        (self.path, self.content, self.mode)
    }
}

#[cfg(test)]
mod tests {
    use super::FileEntry;

    #[test]
    fn file_entry_exposes_copy_metadata() {
        let entry = FileEntry::new("script.sh", vec![1, 2, 3], 3u32);

        assert_eq!(entry.path(), "script.sh");
        assert_eq!(entry.content(), &[1, 2, 3]);
        assert_eq!(entry.mode(), 3u32);
    }

    #[test]
    fn into_parts_exposes_tuple_data() {
        let entry = FileEntry::new("foo.ra", vec![3, 2, 1], 55u32);

        let expected_entry_parts = ("foo.ra".into(), vec![3, 2, 1], 55u32);
        assert_eq!(expected_entry_parts, entry.into_parts());
    }
}
