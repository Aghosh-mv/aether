#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
}

impl Url {
    pub fn parse(input: &str) -> Result<Self, String> {
        let (scheme, remainder) = input.split_once("://").ok_or("URL must include a scheme")?;
        let scheme = scheme.to_ascii_lowercase();
        if !matches!(scheme.as_str(), "http" | "https" | "file") {
            return Err(format!("unsupported scheme: {scheme}"));
        }
        let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
        let authority = &remainder[..authority_end];
        if authority.is_empty() && scheme != "file" {
            return Err("URL missing host".into());
        }
        let (host, port) = if let Some((host, port)) = authority.rsplit_once(':') {
            if let Ok(port) = port.parse() {
                (host.to_ascii_lowercase(), Some(port))
            } else {
                (authority.to_ascii_lowercase(), None)
            }
        } else {
            (authority.to_ascii_lowercase(), None)
        };
        let after_authority = &remainder[authority_end..];
        let without_fragment = after_authority
            .split_once('#')
            .map_or(after_authority, |(before, _)| before);
        let (path, query) = without_fragment
            .split_once('?')
            .map_or((without_fragment, None), |(path, query)| {
                (path, Some(query.to_string()))
            });
        Ok(Self {
            scheme,
            host,
            port,
            path: if path.is_empty() {
                "/".into()
            } else {
                path.into()
            },
            query,
        })
    }
    pub fn origin(&self) -> String {
        let port = self.port.map(|port| format!(":{port}")).unwrap_or_default();
        format!("{}://{}{}", self.scheme, self.host, port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_and_normalizes_http_urls() {
        let url = Url::parse("HTTPS://Example.COM:8443/docs?q=rust#part").unwrap();
        assert_eq!(url.scheme, "https");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, Some(8443));
        assert_eq!(url.path, "/docs");
        assert_eq!(url.query, Some("q=rust".into()));
        assert_eq!(url.origin(), "https://example.com:8443");
    }
    #[test]
    fn supplies_root_path_and_rejects_unsupported_or_missing_authority() {
        assert_eq!(Url::parse("https://example.com").unwrap().path, "/");
        assert!(Url::parse("javascript:alert(1)").is_err());
        assert!(Url::parse("https://").is_err());
    }
}
