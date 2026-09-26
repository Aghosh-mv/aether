use aether_dom::Node;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Start(String, Vec<(String, String)>),
    End(String),
    Text(String),
}

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
            let mut parts = raw.split_whitespace();
            let tag = parts
                .next()
                .unwrap_or("")
                .trim_end_matches('/')
                .to_ascii_lowercase();
            let attrs = parts
                .filter_map(|part| part.split_once('='))
                .map(|(k, v)| {
                    (
                        k.to_ascii_lowercase(),
                        v.trim_matches(['\"', '\'']).to_string(),
                    )
                })
                .collect();
            if !tag.is_empty() {
                out.push(Token::Start(tag, attrs));
            }
        }
        rest = &rest[open + close + 1..];
    }
    if !rest.is_empty() {
        out.push(Token::Text(rest.to_string()));
    }
    out
}

pub fn parse(input: &str) -> Node {
    let mut roots = Vec::new();
    let mut stack: Vec<(String, Vec<(String, String)>, Vec<Node>)> = Vec::new();
    for token in tokenize(input) {
        match token {
            Token::Text(text) if !text.trim().is_empty() => {
                if let Some(last) = stack.last_mut() {
                    last.2.push(Node::Text(text));
                } else {
                    roots.push(Node::Text(text));
                }
            }
            Token::Text(_) => {}
            Token::Start(tag, attrs) => stack.push((tag, attrs, Vec::new())),
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_nested_markup() {
        let doc = parse("<h1 class=title>Hello</h1><p>World</p>");
        assert_eq!(doc.text(), "HelloWorld");
    }
}
