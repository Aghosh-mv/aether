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
