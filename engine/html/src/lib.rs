use aether_dom::Node;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Start(String, Vec<(String, String)>),
    SelfClosing(String, Vec<(String, String)>),
    End(String),
    Text(String),
}

type OpenElement = (String, Vec<(String, String)>, Vec<Node>);

fn tokenize(input: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut rest = input;
    while let Some(open) = rest.find('<') {
        if open > 0 {
            out.push(Token::Text(rest[..open].to_string()));
        }
        let Some(close) = rest[open..].find('>') else {
            out.push(Token::Text(rest[open..].to_string()));
            break;
        };
        let raw = rest[open + 1..open + close].trim();
        if let Some(name) = raw.strip_prefix('/') {
            out.push(Token::End(name.trim().to_ascii_lowercase()));
        } else if !raw.starts_with('!') && !raw.starts_with('?') {
            let self_closing = raw.ends_with('/');
            let (tag, attrs) = parse_start_tag(raw.trim_end_matches('/').trim());
            if !tag.is_empty() {
                out.push(if self_closing || is_void_element(&tag) {
                    Token::SelfClosing(tag, attrs)
                } else {
                    Token::Start(tag, attrs)
                });
            }
        }
        rest = &rest[open + close + 1..];
    }
    if !rest.is_empty() {
        out.push(Token::Text(rest.to_string()));
    }
    out
}

fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn parse_start_tag(raw: &str) -> (String, Vec<(String, String)>) {
    let chars: Vec<char> = raw.chars().collect();
    let mut i = 0;
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }
    let start = i;
    while i < chars.len() && !chars[i].is_whitespace() {
        i += 1;
    }
    let tag: String = chars[start..i]
        .iter()
        .collect::<String>()
        .to_ascii_lowercase();
    let mut attrs = Vec::new();
    while i < chars.len() {
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }
        let key_start = i;
        while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '=' {
            i += 1;
        }
        let key: String = chars[key_start..i]
            .iter()
            .collect::<String>()
            .to_ascii_lowercase();
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        let mut value = String::new();
        if i < chars.len() && chars[i] == '=' {
            i += 1;
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            if i < chars.len() && (chars[i] == '\'' || chars[i] == '"') {
                let quote = chars[i];
                i += 1;
                let value_start = i;
                while i < chars.len() && chars[i] != quote {
                    i += 1;
                }
                value = chars[value_start..i.min(chars.len())].iter().collect();
                if i < chars.len() {
                    i += 1;
                }
            } else {
                let value_start = i;
                while i < chars.len() && !chars[i].is_whitespace() {
                    i += 1;
                }
                value = chars[value_start..i].iter().collect();
            }
        }
        if !key.is_empty() {
            attrs.push((key, value));
        }
    }
    (tag, attrs)
}

pub fn parse(input: &str) -> Node {
    let mut roots = Vec::new();
    let mut stack: Vec<OpenElement> = Vec::new();
    for token in tokenize(input) {
        match token {
            Token::Text(text) if !text.trim().is_empty() => {
                let text = decode_entities(&text);
                if let Some(last) = stack.last_mut() {
                    last.2.push(Node::Text(text));
                } else {
                    roots.push(Node::Text(text));
                }
            }
            Token::Text(_) => {}
            Token::Start(tag, attrs) => stack.push((tag, attrs, Vec::new())),
            Token::SelfClosing(tag, attrs) => {
                let node = Node::Element {
                    tag,
                    attributes: attrs,
                    children: Vec::new(),
                };
                if let Some(parent) = stack.last_mut() {
                    parent.2.push(node);
                } else {
                    roots.push(node);
                }
            }
            Token::End(tag) => {
                if let Some(pos) = stack.iter().rposition(|item| item.0 == tag) {
                    while stack.len() > pos {
                        let (name, attrs, children) = stack.pop().unwrap();
                        let node = Node::Element {
                            tag: name,
                            attributes: attrs,
                            children,
                        };
                        if let Some(parent) = stack.last_mut() {
                            parent.2.push(node);
                        } else {
                            roots.push(node);
                        }
                    }
                }
            }
        }
    }
    while let Some((tag, attrs, children)) = stack.pop() {
        let node = Node::Element {
            tag,
            attributes: attrs,
            children,
        };
        if let Some(parent) = stack.last_mut() {
            parent.2.push(node);
        } else {
            roots.push(node);
        }
    }
    Node::document(roots)
}

fn decode_entities(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('&') {
        result.push_str(&rest[..start]);
        let Some(end) = rest[start..].find(';') else {
            result.push_str(&rest[start..]);
            break;
        };
        let entity = &rest[start + 1..start + end];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some('\u{00a0}'),
            value if value.starts_with("#x") => u32::from_str_radix(&value[2..], 16)
                .ok()
                .and_then(char::from_u32),
            value if value.starts_with('#') => {
                value[1..].parse::<u32>().ok().and_then(char::from_u32)
            }
            _ => None,
        };
        if let Some(character) = decoded {
            result.push(character);
            rest = &rest[start + end + 1..];
        } else {
            result.push('&');
            rest = &rest[start + 1..];
        }
    }
    if !rest.is_empty() {
        result.push_str(rest);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_nested_markup() {
        let doc = parse("<h1 class=title>Hello</h1><p>World</p>");
        assert_eq!(doc.text(), "HelloWorld");
    }
    #[test]
    fn preserves_quoted_attributes_and_self_closing_tags() {
        let doc = parse(r#"<div data-label="hello world"><img alt='a small icon' /></div>"#);
        let div = doc.query_selector("div").unwrap();
        assert_eq!(
            div.attributes()[0],
            ("data-label".into(), "hello world".into())
        );
        let image = doc.query_selector("img").unwrap();
        assert_eq!(image.attributes()[0], ("alt".into(), "a small icon".into()));
    }
    #[test]
    fn decodes_named_and_numeric_character_references() {
        let doc = parse("<p>A &amp; B &lt; 3 &#x1F600; &#169;</p>");
        assert_eq!(doc.text(), "A & B < 3 😀 ©");
    }
    #[test]
    fn treats_standard_void_elements_as_leaf_nodes() {
        let doc = parse("<p>before<br><img alt=icon>after</p><p>next</p>");
        let paragraphs = doc.query_selector_all("p");
        assert_eq!(paragraphs.len(), 2);
        assert_eq!(paragraphs[0].text(), "beforeafter");
        assert_eq!(doc.query_selector("img").unwrap().children().len(), 0);
    }
}
