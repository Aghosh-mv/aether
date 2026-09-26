#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowSnapshot {
    pub tabs: Vec<String>,
    pub active_tab: usize,
    pub private: bool,
}

#[derive(Debug, Default)]
pub struct SessionStore {
    windows: Vec<WindowSnapshot>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn save(&mut self, windows: impl IntoIterator<Item = WindowSnapshot>) {
        self.windows = windows
            .into_iter()
            .filter(|window| !window.private && !window.tabs.is_empty())
            .map(|mut window| {
                window.active_tab = window.active_tab.min(window.tabs.len() - 1);
                window
            })
            .collect();
    }
    pub fn restore(&self) -> Vec<WindowSnapshot> {
        self.windows.clone()
    }
    pub fn clear(&mut self) {
        self.windows.clear();
    }
    pub fn len(&self) -> usize {
        self.windows.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restores_normal_windows_and_clamps_active_tab() {
        let mut store = SessionStore::new();
        store.save([WindowSnapshot {
            tabs: vec!["a".into(), "b".into()],
            active_tab: 9,
            private: false,
        }]);
        let restored = store.restore();
        assert_eq!(restored[0].active_tab, 1);
        assert_eq!(restored[0].tabs, vec!["a", "b"]);
    }
    #[test]
    fn never_persists_private_or_empty_windows() {
        let mut store = SessionStore::new();
        store.save([
            WindowSnapshot {
                tabs: vec!["private".into()],
                active_tab: 0,
                private: true,
            },
            WindowSnapshot {
                tabs: Vec::new(),
                active_tab: 0,
                private: false,
            },
        ]);
        assert_eq!(store.len(), 0);
        store.clear();
        assert!(store.restore().is_empty());
    }
}
