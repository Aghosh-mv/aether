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

    pub fn set_text_content(&mut self, value: impl Into<String>) {
        let text = Node::Text(value.into());
        match self {
            Self::Document { children } | Self::Element { children, .. } => {
                children.clear();
                children.push(text);
            }
            Self::Text(current) => {
                *current = match text {
                    Node::Text(value) => value,
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn append_child(&mut self, child: Node) -> Result<(), Node> {
        match self {
            Self::Document { children } | Self::Element { children, .. } => {
                children.push(child);
                Ok(())
            }
            Self::Text(_) => Err(child),
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
    #[test]
    fn mutates_owned_tree_text_and_children() {
        let mut root = Node::Element {
            tag: "main".into(),
            attributes: Vec::new(),
            children: Vec::new(),
        };
        root.set_text_content("updated");
        assert_eq!(root.text(), "updated");
        root.append_child(Node::Element {
            tag: "p".into(),
            attributes: Vec::new(),
            children: vec![Node::Text("child".into())],
        })
        .unwrap();
        assert_eq!(root.query_selector("p").unwrap().text(), "child");
        assert!(Node::Text("leaf".into())
            .append_child(Node::Text("nope".into()))
            .is_err());
    }
}
