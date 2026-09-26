#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Style {
    pub display: Display,
    pub color: u32,
    pub background: u32,
    pub font_size: u32,
    pub margin: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Block,
    Inline,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub selector: String,
    pub declarations: Style,
}

pub fn parse_stylesheet(input: &str) -> Vec<Rule> {
    input
        .split('}')
        .filter_map(|chunk| chunk.split_once('{'))
        .flat_map(|(selector, body)| {
            let declarations = parse_declarations(body);
            selector.split(',').map(move |selector| Rule {
                selector: selector.trim().to_ascii_lowercase(),
                declarations: declarations.clone(),
            })
        })
        .filter(|rule| !rule.selector.is_empty())
        .collect()
}

pub fn parse_declarations(input: &str) -> Style {
    let mut style = Style {
        color: 0xff20252b,
        background: 0xfff7f4ed,
        font_size: 16,
        ..Style::default()
    };
    for declaration in input.split(';') {
        let Some((property, value)) = declaration.split_once(':') else {
            continue;
        };
        let value = value.trim();
        match property.trim().to_ascii_lowercase().as_str() {
            "display" => {
                style.display = match value {
                    "none" => Display::None,
                    "inline" => Display::Inline,
                    _ => Display::Block,
                }
            }
            "color" => {
                if let Some(color) = hex_color(value) {
                    style.color = color
                }
            }
            "background" | "background-color" => {
                if let Some(color) = hex_color(value) {
                    style.background = color
                }
            }
            "font-size" => {
                if let Some(px) = value.strip_suffix("px").and_then(|n| n.trim().parse().ok()) {
                    style.font_size = px
                }
            }
            "margin" => {
                if let Some(px) = value.strip_suffix("px").and_then(|n| n.trim().parse().ok()) {
                    style.margin = px
                }
            }
            _ => {}
        }
    }
    style
}

fn default_style() -> Style {
    Style {
        color: 0xff20252b,
        background: 0xfff7f4ed,
        font_size: 16,
        ..Style::default()
    }
}

fn hex_color(value: &str) -> Option<u32> {
    let digits = value.strip_prefix('#')?;
    let expanded = if digits.len() == 3 {
        digits.chars().flat_map(|c| [c, c]).collect::<String>()
    } else {
        digits.to_string()
    };
    if expanded.len() != 6 {
        return None;
    }
    u32::from_str_radix(&expanded, 16)
        .ok()
        .map(|rgb| 0xff000000 | rgb)
}

pub fn style_for(tag: &str, attributes: &[(String, String)], rules: &[Rule]) -> Style {
    let mut result = default_style();
    for rule in rules {
        if selector_matches(&rule.selector, tag, attributes) {
            apply_specified(&mut result, &rule.declarations);
        }
    }
    if let Some((_, inline)) = attributes.iter().find(|(name, _)| name == "style") {
        apply_specified(&mut result, &parse_declarations(inline));
    }
    result
}

fn selector_matches(selector: &str, tag: &str, attributes: &[(String, String)]) -> bool {
    let classes = attributes
        .iter()
        .find(|(name, _)| name == "class")
        .map(|(_, value)| value.split_whitespace().collect::<Vec<_>>())
        .unwrap_or_default();
    let id = attributes
        .iter()
        .find(|(name, _)| name == "id")
        .map(|(_, value)| value.as_str());
    if selector == "*" {
        return true;
    }
    if let Some((wanted_tag, wanted_id)) = selector.split_once('#') {
        return (wanted_tag.is_empty() || wanted_tag == tag) && id == Some(wanted_id);
    }
    if let Some((wanted_tag, wanted_class)) = selector.split_once('.') {
        return (wanted_tag.is_empty() || wanted_tag == tag) && classes.contains(&wanted_class);
    }
    if let Some(wanted) = selector.strip_prefix('#') {
        return id == Some(wanted);
    }
    if let Some(wanted) = selector.strip_prefix('.') {
        return classes.contains(&wanted);
    }
    selector == tag
}

fn apply_specified(base: &mut Style, incoming: &Style) {
    if incoming.display != Display::Block {
        base.display = incoming.display;
    }
    if incoming.color != 0xff20252b {
        base.color = incoming.color;
    }
    if incoming.background != 0xfff7f4ed {
        base.background = incoming.background;
    }
    if incoming.font_size != 16 {
        base.font_size = incoming.font_size;
    }
    if incoming.margin != 0 {
        base.margin = incoming.margin;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_rules_and_values() {
        let rules = parse_stylesheet("p { color: #123456; margin: 8px; }");
        assert_eq!(rules[0].selector, "p");
        assert_eq!(rules[0].declarations.color, 0xff123456);
        assert_eq!(rules[0].declarations.margin, 8);
    }
    #[test]
    fn supports_three_digit_colors_and_display() {
        let style = parse_declarations("display:none; color:#abc");
        assert_eq!(style.display, Display::None);
        assert_eq!(style.color, 0xffaabbcc);
    }
    #[test]
    fn matches_id_class_and_inline_style() {
        let rules = parse_stylesheet(".notice { margin: 4px; } #hero { display: none; }");
        let attrs = vec![
            ("class".into(), "notice".into()),
            ("style".into(), "margin: 9px".into()),
        ];
        assert_eq!(style_for("p", &attrs, &rules).margin, 9);
        assert_eq!(
            style_for("p", &[("id".into(), "hero".into())], &rules).display,
            Display::None
        );
    }
    #[test]
    fn expands_selector_lists_and_preserves_source_order() {
        let rules = parse_stylesheet("h1, h2 { margin: 4px; } h1 { margin: 8px; }");
        assert_eq!(rules.len(), 3);
        assert_eq!(style_for("h1", &[], &rules).margin, 8);
        assert_eq!(style_for("h2", &[], &rules).margin, 4);
    }
    #[test]
    fn matches_compound_and_universal_selectors() {
        let rules = parse_stylesheet("p.notice { margin: 2px; } * { color: #abcdef; }");
        let attrs = vec![("class".into(), "notice".into())];
        assert_eq!(style_for("p", &attrs, &rules).margin, 2);
        assert_eq!(style_for("section", &[], &rules).color, 0xffabcdef);
    }
}
