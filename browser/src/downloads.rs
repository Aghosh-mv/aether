#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadState {
    Downloading,
    Paused,
    Complete,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Download {
    pub id: u64,
    pub url: String,
    pub filename: String,
    pub received: u64,
    pub total: Option<u64>,
    pub state: DownloadState,
}

#[derive(Debug, Default)]
pub struct DownloadManager {
    next_id: u64,
    items: Vec<Download>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            items: Vec::new(),
        }
    }
    pub fn start(
        &mut self,
        url: impl Into<String>,
        filename: impl Into<String>,
        total: Option<u64>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(Download {
            id,
            url: url.into(),
            filename: filename.into(),
            received: 0,
            total,
            state: DownloadState::Downloading,
        });
        id
    }
    pub fn get(&self, id: u64) -> Option<&Download> {
        self.items.iter().find(|item| item.id == id)
    }
    pub fn pause(&mut self, id: u64) -> bool {
        self.transition(id, DownloadState::Paused, |state| {
            *state == DownloadState::Downloading
        })
    }
    pub fn resume(&mut self, id: u64) -> bool {
        self.transition(id, DownloadState::Downloading, |state| {
            *state == DownloadState::Paused || *state == DownloadState::Failed
        })
    }
    pub fn cancel(&mut self, id: u64) -> bool {
        self.transition(id, DownloadState::Cancelled, |state| {
            matches!(
                state,
                DownloadState::Downloading | DownloadState::Paused | DownloadState::Failed
            )
        })
    }
    pub fn receive(&mut self, id: u64, bytes: u64) -> bool {
        let Some(item) = self.items.iter_mut().find(|item| item.id == id) else {
            return false;
        };
        if item.state != DownloadState::Downloading {
            return false;
        }
        item.received = item.received.saturating_add(bytes);
        if item.total.is_some_and(|total| item.received >= total) {
            item.state = DownloadState::Complete;
        }
        true
    }
    pub fn fail(&mut self, id: u64) -> bool {
        self.transition(id, DownloadState::Failed, |state| {
            *state == DownloadState::Downloading
        })
    }
    fn transition(
        &mut self,
        id: u64,
        target: DownloadState,
        allowed: impl FnOnce(&DownloadState) -> bool,
    ) -> bool {
        let Some(item) = self.items.iter_mut().find(|item| item.id == id) else {
            return false;
        };
        if !allowed(&item.state) {
            return false;
        }
        item.state = target;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tracks_progress_pause_resume_and_completion() {
        let mut manager = DownloadManager::new();
        let id = manager.start("https://example.com/a.zip", "a.zip", Some(10));
        assert!(manager.receive(id, 4));
        assert_eq!(manager.get(id).unwrap().received, 4);
        assert!(manager.pause(id));
        assert!(!manager.receive(id, 6));
        assert!(manager.resume(id));
        assert!(manager.receive(id, 6));
        assert_eq!(manager.get(id).unwrap().state, DownloadState::Complete);
        assert!(!manager.pause(id));
    }
    #[test]
    fn supports_failure_retry_and_cancel() {
        let mut manager = DownloadManager::new();
        let id = manager.start("https://example.com/a", "a", None);
        assert!(manager.fail(id));
        assert!(manager.resume(id));
        assert!(manager.cancel(id));
        assert!(!manager.resume(id));
    }
}
