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
}
