#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credential {
    pub origin: String,
    pub username: String,
    pub secret_key: String,
}

pub trait SecretStore {
    type Error;
    fn put(&mut self, key: &str, secret: &[u8]) -> Result<(), Self::Error>;
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error>;
    fn delete(&mut self, key: &str) -> Result<(), Self::Error>;
}

pub struct CredentialVault<S> {
    store: S,
    records: Vec<Credential>,
}

impl<S: SecretStore> CredentialVault<S> {
    pub fn new(store: S) -> Self {
        Self {
            store,
            records: Vec::new(),
        }
    }
    pub fn save(
        &mut self,
        origin: impl Into<String>,
        username: impl Into<String>,
        secret: &[u8],
    ) -> Result<(), S::Error> {
        let origin = origin.into();
        let username = username.into();
        let secret_key = format!("aether/credential/{}", self.records.len());
        self.store.put(&secret_key, secret)?;
        self.records
            .retain(|record| !(record.origin == origin && record.username == username));
        self.records.push(Credential {
            origin,
            username,
            secret_key,
        });
        Ok(())
    }
    pub fn find(&self, origin: &str, username: &str) -> Option<&Credential> {
        self.records
            .iter()
            .find(|record| record.origin == origin && record.username == username)
    }
    pub fn secret(&self, credential: &Credential) -> Result<Option<Vec<u8>>, S::Error> {
        self.store.get(&credential.secret_key)
    }
    pub fn delete(&mut self, origin: &str, username: &str) -> Result<bool, S::Error> {
        let Some(index) = self
            .records
            .iter()
            .position(|record| record.origin == origin && record.username == username)
        else {
            return Ok(false);
        };
        let record = self.records.remove(index);
        self.store.delete(&record.secret_key)?;
        Ok(true)
    }
    pub fn len(&self) -> usize {
        self.records.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[derive(Default)]
    struct MockStore {
        values: HashMap<String, Vec<u8>>,
    }
    impl SecretStore for MockStore {
        type Error = ();
        fn put(&mut self, key: &str, secret: &[u8]) -> Result<(), Self::Error> {
            self.values.insert(key.into(), secret.to_vec());
            Ok(())
        }
        fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
            Ok(self.values.get(key).cloned())
        }
        fn delete(&mut self, key: &str) -> Result<(), Self::Error> {
            self.values.remove(key);
            Ok(())
        }
    }
    #[test]
    fn saves_updates_reads_and_deletes_without_exposing_store_details() {
        let mut vault = CredentialVault::new(MockStore::default());
        vault
            .save("https://example.com", "alice", b"secret")
            .unwrap();
        let record = vault.find("https://example.com", "alice").unwrap().clone();
        assert_eq!(vault.secret(&record).unwrap(), Some(b"secret".to_vec()));
        vault.save("https://example.com", "alice", b"new").unwrap();
        assert_eq!(vault.len(), 1);
        assert!(vault.delete("https://example.com", "alice").unwrap());
        assert!(vault.find("https://example.com", "alice").is_none());
    }
}
