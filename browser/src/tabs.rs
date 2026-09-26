#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tab {
    pub url: String,
    pub title: String,
    pub pinned: bool,
    pub muted: bool,
}

#[derive(Debug, Default)]
pub struct TabStrip {
    tabs: Vec<Tab>,
    active: Option<usize>,
    recently_closed: Vec<Tab>,
}

impl TabStrip {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn open(&mut self, url: impl Into<String>) -> usize {
        self.tabs.push(Tab {
            url: url.into(),
            title: "New Tab".into(),
            pinned: false,
            muted: false,
        });
        self.active = Some(self.tabs.len() - 1);
        self.tabs.len() - 1
    }
    pub fn active(&self) -> Option<&Tab> {
        self.active.and_then(|index| self.tabs.get(index))
    }
    pub fn select(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active = Some(index);
            true
        } else {
            false
        }
    }
    pub fn close(&mut self, index: usize) -> Option<Tab> {
        if index >= self.tabs.len() || self.tabs[index].pinned {
            return None;
        }
        let closed = self.tabs.remove(index);
        self.recently_closed.push(closed.clone());
        self.active = match self.active {
            Some(_active) if self.tabs.is_empty() => None,
            Some(active) if active > index => Some(active - 1),
            Some(active) if active == index => Some(active.min(self.tabs.len() - 1)),
            other => other,
        };
        Some(closed)
    }
    pub fn reopen_closed(&mut self) -> Option<usize> {
        let tab = self.recently_closed.pop()?;
        self.tabs.push(tab);
        self.active = Some(self.tabs.len() - 1);
        self.active
    }
    pub fn toggle_pin(&mut self, index: usize) -> bool {
        if let Some(tab) = self.tabs.get_mut(index) {
            tab.pinned = !tab.pinned;
            true
        } else {
            false
        }
    }
    pub fn toggle_mute(&mut self, index: usize) -> bool {
        if let Some(tab) = self.tabs.get_mut(index) {
            tab.muted = !tab.muted;
            true
        } else {
            false
        }
    }
    pub fn len(&self) -> usize {
        self.tabs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opens_selects_and_reopens_tabs() {
        let mut strip = TabStrip::new();
        strip.open("a");
        strip.open("b");
        assert_eq!(strip.active().unwrap().url, "b");
        assert!(strip.select(0));
        assert_eq!(strip.active().unwrap().url, "a");
        strip.close(0);
        assert_eq!(strip.active().unwrap().url, "b");
        strip.reopen_closed();
        assert_eq!(strip.active().unwrap().url, "a");
    }
    #[test]
    fn pin_and_mute_protect_and_update_tabs() {
        let mut strip = TabStrip::new();
        strip.open("a");
        assert!(strip.toggle_pin(0));
        assert!(strip.close(0).is_none());
        assert!(strip.toggle_mute(0));
        assert!(strip.active().unwrap().muted);
        assert_eq!(strip.len(), 1);
    }
}
