#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Default)]
pub struct CookieJar {
    cookies: Vec<Cookie>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set(&mut self, cookie: Cookie) {
        self.cookies.retain(|existing| {
            !(existing.name == cookie.name
                && existing.domain == cookie.domain
                && existing.path == cookie.path)
        });
        self.cookies.push(cookie);
    }
    pub fn cookies_for(&self, host: &str, path: &str, secure: bool, now: u64) -> Vec<&Cookie> {
        self.cookies
            .iter()
            .filter(|cookie| {
                let domain_matches = host == cookie.domain
                    || host.ends_with(&format!(".{}", cookie.domain.trim_start_matches('.')));
                let path_matches = path == cookie.path
                    || path.starts_with(cookie.path.trim_end_matches('/').to_owned().as_str());
                let secure_matches = !cookie.secure || secure;
                let not_expired = cookie.expires_at.is_none_or(|expires| expires > now);
                domain_matches && path_matches && secure_matches && not_expired
            })
            .collect()
    }
    pub fn header_for(&self, host: &str, path: &str, secure: bool, now: u64) -> String {
        self.cookies_for(host, path, secure, now)
            .iter()
            .map(|cookie| format!("{}={}", cookie.name, cookie.value))
            .collect::<Vec<_>>()
            .join("; ")
    }
    pub fn len(&self) -> usize {
        self.cookies.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cookie(
        name: &str,
        domain: &str,
        path: &str,
        secure: bool,
        expires_at: Option<u64>,
    ) -> Cookie {
        Cookie {
            name: name.into(),
            value: "value".into(),
            domain: domain.into(),
            path: path.into(),
            secure,
            expires_at,
        }
    }
    #[test]
    fn matches_domains_paths_and_secure_transport() {
        let mut jar = CookieJar::new();
        jar.set(cookie("session", ".example.com", "/app", true, None));
        jar.set(cookie("root", "example.com", "/", false, None));
        assert_eq!(
            jar.header_for("www.example.com", "/app/page", true, 1),
            "session=value; root=value"
        );
        assert_eq!(
            jar.header_for("www.example.com", "/app/page", false, 1),
            "root=value"
        );
        assert_eq!(jar.header_for("other.test", "/app", true, 1), "");
    }
    #[test]
    fn replaces_same_cookie_and_filters_expired_values() {
        let mut jar = CookieJar::new();
        jar.set(cookie("id", "example.com", "/", false, Some(10)));
        jar.set(Cookie {
            value: "new".into(),
            ..cookie("id", "example.com", "/", false, Some(20))
        });
        assert_eq!(jar.len(), 1);
        assert_eq!(jar.header_for("example.com", "/", false, 15), "id=new");
        assert_eq!(jar.header_for("example.com", "/", false, 25), "");
    }
}
