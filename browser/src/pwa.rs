#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayMode {
    Browser,
    Standalone,
    MinimalUi,
    Fullscreen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebManifest {
    pub name: String,
    pub start_url: String,
    pub icon_url: Option<String>,
    pub display: DisplayMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledWebApp {
    pub manifest: WebManifest,
    pub isolated_storage: bool,
}

pub fn parse_manifest(input: &str) -> Result<WebManifest, String> {
    let name = field(input, "name").ok_or("manifest missing name")?;
    let start_url = field(input, "start_url").ok_or("manifest missing start_url")?;
    let icon_url = field(input, "icon");
    let display = match field(input, "display").as_deref() {
        Some("standalone") => DisplayMode::Standalone,
        Some("minimal-ui") => DisplayMode::MinimalUi,
        Some("fullscreen") => DisplayMode::Fullscreen,
        _ => DisplayMode::Browser,
    };
    Ok(WebManifest {
        name,
        start_url,
        icon_url,
        display,
    })
}

fn field(input: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\"");
    let after = input.split_once(&marker)?.1.split_once(':')?.1.trim_start();
    let value = after.strip_prefix('"')?.split_once('"')?.0;
    Some(value.to_string())
}

pub fn install(manifest: WebManifest) -> InstalledWebApp {
    InstalledWebApp {
        manifest,
        isolated_storage: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_and_installs_manifest() {
        let manifest = parse_manifest(r#"{"name":"Mail","start_url":"https://mail.test","icon":"https://mail.test/icon.png","display":"standalone"}"#).unwrap();
        assert_eq!(manifest.name, "Mail");
        assert_eq!(manifest.display, DisplayMode::Standalone);
        let app = install(manifest);
        assert!(app.isolated_storage);
    }
    #[test]
    fn rejects_manifest_without_required_fields() {
        assert!(parse_manifest(r#"{"name":"Missing URL"}"#).is_err());
    }
}
