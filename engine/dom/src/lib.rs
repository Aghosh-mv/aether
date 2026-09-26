#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Document {
        children: Vec<Node>,
    },
    Element {
        tag: String,
        attributes: Vec<(String, String)>,
        children: Vec<Node>,
    },
    Text(String),
}

impl Node {
    pub fn tag_name(&self) -> Option<&str> {
        match self {
            Self::Element { tag, .. } => Some(tag),
            _ => None,
        }
    }
    pub fn attributes(&self) -> &[(String, String)] {
        match self {
            Self::Element { attributes, .. } => attributes,
            _ => &[],
        }
    }
    pub fn children(&self) -> &[Node] {
        match self {
            Self::Document { children } | Self::Element { children, .. } => children,
            Self::Text(_) => &[],
        }
    }
}

impl Node {
    pub fn document(children: Vec<Node>) -> Self {
        Self::Document { children }
    }
    pub fn text(&self) -> String {
        match self {
            Self::Document { children } | Self::Element { children, .. } => {
                children.iter().map(Node::text).collect()
            }
            Self::Text(value) => value.clone(),
        }
    }

    pub fn query_selector_all<'a>(&'a self, selector: &str) -> Vec<&'a Node> {
        let mut matches = Vec::new();
        self.collect_matches(selector.trim().to_ascii_lowercase().as_str(), &mut matches);
        matches
    }

    pub fn query_selector(&self, selector: &str) -> Option<&Node> {
        self.query_selector_all(selector).into_iter().next()
    }

    fn collect_matches<'a>(&'a self, selector: &str, matches: &mut Vec<&'a Node>) {
        match self {
            Self::Document { children } => {
                for child in children {
                    child.collect_matches(selector, matches);
                }
            }
            Self::Element {
                tag,
                attributes,
                children,
            } => {
                let found = if let Some(id) = selector.strip_prefix('#') {
                    attributes
                        .iter()
                        .any(|(name, value)| name == "id" && value.eq_ignore_ascii_case(id))
                } else if let Some(class) = selector.strip_prefix('.') {
                    attributes.iter().any(|(name, value)| {
                        name == "class"
                            && value
                                .split_whitespace()
                                .any(|item| item.eq_ignore_ascii_case(class))
                    })
                } else {
                    tag == selector
                };
                if found {
                    matches.push(self);
                }
                for child in children {
                    child.collect_matches(selector, matches);
                }
            }
            Self::Text(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_elements_by_tag_id_and_class() {
        let doc = Node::document(vec![Node::Element {
            tag: "main".into(),
            attributes: vec![("id".into(), "root".into())],
            children: vec![Node::Element {
                tag: "p".into(),
                attributes: vec![("class".into(), "notice featured".into())],
                children: vec![Node::Text("hello".into())],
            }],
        }]);
        assert_eq!(doc.query_selector_all("p").len(), 1);
        assert_eq!(
            doc.query_selector("#root").unwrap().tag_name(),
            Some("main")
        );
        assert_eq!(doc.query_selector(".featured").unwrap().text(), "hello");
    }
}
