#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileKind {
    Normal,
    Guest,
    Incognito,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountState {
    LoggedOut,
    Creating,
    LoggedIn { account_label: String },
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub id: u64,
    pub name: String,
    pub kind: ProfileKind,
    pub account_states: Vec<(String, AccountState)>,
}

#[derive(Debug, Default)]
pub struct ProfileManager {
    next_id: u64,
    profiles: Vec<Profile>,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            profiles: Vec::new(),
        }
    }
    pub fn create(&mut self, name: impl Into<String>, kind: ProfileKind) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.profiles.push(Profile {
            id,
            name: name.into(),
            kind,
            account_states: Vec::new(),
        });
        id
    }
    pub fn get(&self, id: u64) -> Option<&Profile> {
        self.profiles.iter().find(|profile| profile.id == id)
    }
    pub fn set_account_state(
        &mut self,
        profile_id: u64,
        origin: impl Into<String>,
        state: AccountState,
    ) -> bool {
        let Some(profile) = self
            .profiles
            .iter_mut()
            .find(|profile| profile.id == profile_id)
        else {
            return false;
        };
        let origin = origin.into();
        if let Some(entry) = profile
            .account_states
            .iter_mut()
            .find(|(saved_origin, _)| saved_origin == &origin)
        {
            entry.1 = state;
        } else {
            profile.account_states.push((origin, state));
        }
        true
    }
    pub fn account_state(&self, profile_id: u64, origin: &str) -> Option<&AccountState> {
        self.get(profile_id)?
            .account_states
            .iter()
            .find(|(saved_origin, _)| saved_origin == origin)
            .map(|(_, state)| state)
    }
    pub fn len(&self) -> usize {
        self.profiles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_normal_guest_and_incognito_profiles() {
        let mut manager = ProfileManager::new();
        let normal = manager.create("Personal", ProfileKind::Normal);
        let guest = manager.create("Guest", ProfileKind::Guest);
        let private = manager.create("Private", ProfileKind::Incognito);
        assert_eq!(manager.len(), 3);
        assert_eq!(manager.get(guest).unwrap().kind, ProfileKind::Guest);
        assert_eq!(manager.get(private).unwrap().kind, ProfileKind::Incognito);
        assert_eq!(manager.get(normal).unwrap().name, "Personal");
    }
    #[test]
    fn tracks_account_creation_login_and_logout_state_per_origin() {
        let mut manager = ProfileManager::new();
        let profile = manager.create("Personal", ProfileKind::Normal);
        assert!(manager.set_account_state(
            profile,
            "https://accounts.test",
            AccountState::Creating
        ));
        assert_eq!(
            manager.account_state(profile, "https://accounts.test"),
            Some(&AccountState::Creating)
        );
        manager.set_account_state(
            profile,
            "https://accounts.test",
            AccountState::LoggedIn {
                account_label: "alice".into(),
            },
        );
        assert_eq!(
            manager.account_state(profile, "https://accounts.test"),
            Some(&AccountState::LoggedIn {
                account_label: "alice".into()
            })
        );
        manager.set_account_state(profile, "https://accounts.test", AccountState::LoggedOut);
        assert_eq!(
            manager.account_state(profile, "https://accounts.test"),
            Some(&AccountState::LoggedOut)
        );
        manager.set_account_state(profile, "https://accounts.test", AccountState::Failed);
        assert_eq!(
            manager.account_state(profile, "https://accounts.test"),
            Some(&AccountState::Failed)
        );
    }
}
