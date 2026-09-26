#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub permissions: Vec<String>,
    pub host_permissions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension {
    pub manifest: ExtensionManifest,
    pub enabled: bool,
    pub unpacked: bool,
}

#[derive(Debug, Default)]
pub struct ExtensionManager {
    extensions: Vec<Extension>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn install(&mut self, manifest: ExtensionManifest, unpacked: bool) -> bool {
        if self
            .extensions
            .iter()
            .any(|extension| extension.manifest.id == manifest.id)
        {
            return false;
        }
        self.extensions.push(Extension {
            manifest,
            enabled: true,
            unpacked,
        });
        true
    }
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> bool {
        let Some(extension) = self
            .extensions
            .iter_mut()
            .find(|extension| extension.manifest.id == id)
        else {
            return false;
        };
        extension.enabled = enabled;
        true
    }
    pub fn get(&self, id: &str) -> Option<&Extension> {
        self.extensions
            .iter()
            .find(|extension| extension.manifest.id == id)
    }
    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.extensions.len();
        self.extensions
            .retain(|extension| extension.manifest.id != id);
        before != self.extensions.len()
    }
    pub fn len(&self) -> usize {
        self.extensions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn manifest(id: &str) -> ExtensionManifest {
        ExtensionManifest {
            id: id.into(),
            name: "Test Extension".into(),
            version: "1.0.0".into(),
            permissions: vec!["storage".into()],
            host_permissions: vec!["https://example.com/*".into()],
        }
    }
    #[test]
    fn installs_disables_and_removes_extensions() {
        let mut manager = ExtensionManager::new();
        assert!(manager.install(manifest("one"), true));
        assert!(!manager.install(manifest("one"), false));
        assert!(manager.get("one").unwrap().unpacked);
        assert!(manager.set_enabled("one", false));
        assert!(!manager.get("one").unwrap().enabled);
        assert!(manager.remove("one"));
        assert_eq!(manager.len(), 0);
    }
    #[test]
    fn missing_extension_operations_are_safe() {
        let mut manager = ExtensionManager::new();
        assert!(!manager.set_enabled("missing", true));
        assert!(!manager.remove("missing"));
    }
}
