#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Default)]
pub struct NavigationHistory {
    entries: Vec<HistoryEntry>,
    current: Option<usize>,
}

impl NavigationHistory {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn navigate(&mut self, url: impl Into<String>) {
        let entry = HistoryEntry {
            url: url.into(),
            title: None,
        };
        if let Some(index) = self.current {
            self.entries.truncate(index + 1);
        }
        self.entries.push(entry);
        self.current = Some(self.entries.len() - 1);
    }
    pub fn current(&self) -> Option<&HistoryEntry> {
        self.current.and_then(|index| self.entries.get(index))
    }
    pub fn can_go_back(&self) -> bool {
        self.current.is_some_and(|index| index > 0)
    }
    pub fn can_go_forward(&self) -> bool {
        self.current
            .is_some_and(|index| index + 1 < self.entries.len())
    }
    pub fn back(&mut self) -> Option<&HistoryEntry> {
        if self.can_go_back() {
            self.current = Some(self.current.unwrap() - 1);
        }
        self.current()
    }
    pub fn forward(&mut self) -> Option<&HistoryEntry> {
        if self.can_go_forward() {
            self.current = Some(self.current.unwrap() + 1);
        }
        self.current()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn navigates_back_and_forward() {
        let mut history = NavigationHistory::new();
        history.navigate("https://a.test");
        history.navigate("https://b.test");
        assert!(history.can_go_back());
        assert_eq!(history.back().unwrap().url, "https://a.test");
        assert!(history.can_go_forward());
        assert_eq!(history.forward().unwrap().url, "https://b.test");
    }
    #[test]
    fn new_navigation_after_back_discards_forward_branch() {
        let mut history = NavigationHistory::new();
        history.navigate("a");
        history.navigate("b");
        history.back();
        history.navigate("c");
        assert_eq!(history.len(), 2);
        assert!(!history.can_go_forward());
        assert_eq!(history.current().unwrap().url, "c");
    }
}
