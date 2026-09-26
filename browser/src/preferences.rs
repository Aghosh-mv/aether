#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchEngine {
    pub name: String,
    pub query_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmniboxTarget {
    Url(String),
    Search(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserSettings {
    pub search_engine: SearchEngine,
    pub reduced_motion: bool,
    pub theme: String,
    pub homepage: String,
    pub show_bookmarks_bar: bool,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            search_engine: SearchEngine {
                name: "Aether Search".into(),
                query_url: "https://www.google.com/search?q=%s".into(),
            },
            reduced_motion: false,
            theme: "system".into(),
            homepage: "aether://newtab".into(),
            show_bookmarks_bar: false,
        }
    }
}

pub fn classify_omnibox(input: &str) -> OmniboxTarget {
    let trimmed = input.trim();
    if trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("file://")
        || (trimmed.contains('.') && !trimmed.contains(' '))
    {
        OmniboxTarget::Url(trimmed.into())
    } else {
        OmniboxTarget::Search(trimmed.into())
    }
}

pub fn search_url(engine: &SearchEngine, query: &str) -> String {
    engine.query_url.replace("%s", &percent_encode(query))
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            b' ' => "+".into(),
            other => format!("%{other:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn classifies_urls_and_searches() {
        assert_eq!(
            classify_omnibox("https://example.com"),
            OmniboxTarget::Url("https://example.com".into())
        );
        assert_eq!(
            classify_omnibox("rust browser engine"),
            OmniboxTarget::Search("rust browser engine".into())
        );
    }
    #[test]
    fn builds_encoded_search_url() {
        let engine = SearchEngine {
            name: "test".into(),
            query_url: "https://search.test/?q=%s".into(),
        };
        assert_eq!(
            search_url(&engine, "a+b test"),
            "https://search.test/?q=a%2Bb+test"
        );
    }
    #[test]
    fn settings_have_personalization_defaults() {
        let settings = BrowserSettings::default();
        assert_eq!(settings.theme, "system");
        assert!(!settings.reduced_motion);
    }
}
