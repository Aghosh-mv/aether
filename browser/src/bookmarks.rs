#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
    pub folder: String,
}

#[derive(Debug, Default)]
pub struct BookmarkStore {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(
        &mut self,
        title: impl Into<String>,
        url: impl Into<String>,
        folder: impl Into<String>,
    ) -> bool {
        let bookmark = Bookmark {
            title: title.into(),
            url: url.into(),
            folder: folder.into(),
        };
        if self
            .bookmarks
            .iter()
            .any(|existing| existing.url == bookmark.url && existing.folder == bookmark.folder)
        {
            return false;
        }
        self.bookmarks.push(bookmark);
        true
    }
    pub fn remove_url(&mut self, url: &str) -> usize {
        let before = self.bookmarks.len();
        self.bookmarks.retain(|bookmark| bookmark.url != url);
        before - self.bookmarks.len()
    }
    pub fn search(&self, query: &str) -> Vec<&Bookmark> {
        let query = query.to_ascii_lowercase();
        self.bookmarks
            .iter()
            .filter(|bookmark| {
                bookmark.title.to_ascii_lowercase().contains(&query)
                    || bookmark.url.to_ascii_lowercase().contains(&query)
                    || bookmark.folder.to_ascii_lowercase().contains(&query)
            })
            .collect()
    }
    pub fn in_folder(&self, folder: &str) -> Vec<&Bookmark> {
        self.bookmarks
            .iter()
            .filter(|bookmark| bookmark.folder == folder)
            .collect()
    }
    pub fn len(&self) -> usize {
        self.bookmarks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds_deduplicates_and_searches_bookmarks() {
        let mut store = BookmarkStore::new();
        assert!(store.add("Rust", "https://rust-lang.org", "Dev"));
        assert!(!store.add("Rust again", "https://rust-lang.org", "Dev"));
        assert!(store.add("News", "https://news.ycombinator.com", "Reading"));
        assert_eq!(store.search("rust").len(), 1);
        assert_eq!(store.in_folder("Reading").len(), 1);
    }
    #[test]
    fn removes_all_matching_urls() {
        let mut store = BookmarkStore::new();
        store.add("A", "https://a.test", "One");
        store.add("A", "https://a.test", "Two");
        assert_eq!(store.remove_url("https://a.test"), 2);
        assert_eq!(store.len(), 0);
    }
}
