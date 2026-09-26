#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Visit {
    pub url: String,
    pub title: String,
    pub visited_at: u64,
}

#[derive(Debug, Default)]
pub struct HistoryStore {
    visits: Vec<Visit>,
    private: bool,
}

impl HistoryStore {
    pub fn new(private: bool) -> Self {
        Self {
            visits: Vec::new(),
            private,
        }
    }
    pub fn record(&mut self, url: impl Into<String>, title: impl Into<String>, visited_at: u64) {
        if !self.private {
            self.visits.push(Visit {
                url: url.into(),
                title: title.into(),
                visited_at,
            });
        }
    }
    pub fn search(&self, query: &str) -> Vec<&Visit> {
        let query = query.to_ascii_lowercase();
        self.visits
            .iter()
            .filter(|visit| {
                visit.url.to_ascii_lowercase().contains(&query)
                    || visit.title.to_ascii_lowercase().contains(&query)
            })
            .collect()
    }
    pub fn remove_site(&mut self, host_fragment: &str) -> usize {
        let before = self.visits.len();
        self.visits
            .retain(|visit| !visit.url.contains(host_fragment));
        before - self.visits.len()
    }
    pub fn clear_range(&mut self, start: u64, end: u64) -> usize {
        let before = self.visits.len();
        self.visits
            .retain(|visit| visit.visited_at < start || visit.visited_at > end);
        before - self.visits.len()
    }
    pub fn len(&self) -> usize {
        self.visits.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn records_searches_and_clears_history() {
        let mut history = HistoryStore::new(false);
        history.record("https://example.com/docs", "Example Docs", 10);
        history.record("https://news.test", "Daily News", 20);
        assert_eq!(history.search("docs").len(), 1);
        assert_eq!(history.remove_site("news.test"), 1);
        assert_eq!(history.clear_range(0, 15), 1);
        assert_eq!(history.len(), 0);
    }
    #[test]
    fn private_history_does_not_persist_visits() {
        let mut history = HistoryStore::new(true);
        history.record("https://private.test", "Private", 1);
        assert_eq!(history.len(), 0);
        assert!(history.search("private").is_empty());
    }
}
