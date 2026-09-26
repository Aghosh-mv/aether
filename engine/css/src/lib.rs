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
        .map(|(selector, body)| Rule {
            selector: selector.trim().to_ascii_lowercase(),
            declarations: parse_declarations(body),
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

pub fn style_for<'a>(tag: &str, rules: &'a [Rule]) -> Style {
    rules
        .iter()
        .filter(|rule| rule.selector == tag)
        .map(|rule| rule.declarations.clone())
        .last()
        .unwrap_or_else(|| parse_declarations(""))
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
}
