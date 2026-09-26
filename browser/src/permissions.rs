#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionDecision {
    Ask,
    Allow,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Camera,
    Microphone,
    Location,
    Notifications,
    Clipboard,
    Downloads,
}

#[derive(Debug, Default)]
pub struct PermissionStore {
    entries: Vec<(String, Permission, PermissionDecision)>,
}

impl PermissionStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn decision(&self, origin: &str, permission: Permission) -> PermissionDecision {
        self.entries
            .iter()
            .rev()
            .find(|(saved_origin, saved_permission, _)| {
                saved_origin == origin && *saved_permission == permission
            })
            .map(|(_, _, decision)| *decision)
            .unwrap_or(PermissionDecision::Ask)
    }
    pub fn set(
        &mut self,
        origin: impl Into<String>,
        permission: Permission,
        decision: PermissionDecision,
    ) {
        let origin = origin.into();
        self.entries.retain(|(saved_origin, saved_permission, _)| {
            !(saved_origin == &origin && *saved_permission == permission)
        });
        self.entries.push((origin, permission, decision));
    }
    pub fn clear_origin(&mut self, origin: &str) -> usize {
        let before = self.entries.len();
        self.entries
            .retain(|(saved_origin, _, _)| saved_origin != origin);
        before - self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_to_ask_and_updates_per_origin() {
        let mut store = PermissionStore::new();
        assert_eq!(
            store.decision("https://a.test", Permission::Camera),
            PermissionDecision::Ask
        );
        store.set(
            "https://a.test",
            Permission::Camera,
            PermissionDecision::Allow,
        );
        store.set(
            "https://b.test",
            Permission::Camera,
            PermissionDecision::Block,
        );
        assert_eq!(
            store.decision("https://a.test", Permission::Camera),
            PermissionDecision::Allow
        );
        assert_eq!(
            store.decision("https://b.test", Permission::Camera),
            PermissionDecision::Block
        );
        assert_eq!(
            store.decision("https://a.test", Permission::Microphone),
            PermissionDecision::Ask
        );
    }
    #[test]
    fn clear_origin_removes_all_site_permissions() {
        let mut store = PermissionStore::new();
        store.set(
            "https://a.test",
            Permission::Camera,
            PermissionDecision::Allow,
        );
        store.set(
            "https://a.test",
            Permission::Location,
            PermissionDecision::Block,
        );
        assert_eq!(store.clear_origin("https://a.test"), 2);
        assert_eq!(
            store.decision("https://a.test", Permission::Camera),
            PermissionDecision::Ask
        );
    }
    #[test]
    fn covers_sensitive_permission_categories() {
        let mut store = PermissionStore::new();
        store.set(
            "https://a.test",
            Permission::Notifications,
            PermissionDecision::Block,
        );
        store.set(
            "https://a.test",
            Permission::Clipboard,
            PermissionDecision::Allow,
        );
        store.set(
            "https://a.test",
            Permission::Downloads,
            PermissionDecision::Allow,
        );
        assert_eq!(
            store.decision("https://a.test", Permission::Notifications),
            PermissionDecision::Block
        );
        assert_eq!(
            store.decision("https://a.test", Permission::Clipboard),
            PermissionDecision::Allow
        );
        assert_eq!(
            store.decision("https://a.test", Permission::Downloads),
            PermissionDecision::Allow
        );
    }
}
