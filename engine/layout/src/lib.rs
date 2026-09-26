use aether_dom::Node;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRun {
    pub text: String,
    pub x: u32,
    pub y: u32,
}
pub fn layout(doc: &Node, width: u32) -> Vec<TextRun> {
    let mut out = Vec::new();
    let mut y = 24;
    fn walk(n: &Node, out: &mut Vec<TextRun>, y: &mut u32, width: u32) {
        match n {
            Node::Text(s) => {
                for line in s.split_whitespace() {
                    if !line.is_empty() {
                        out.push(TextRun {
                            text: line.to_string(),
                            x: 16,
                            y: *y,
                        });
                        *y += 18;
                    }
                }
            }
            Node::Document { children } | Node::Element { children, .. } => {
                for child in children {
                    walk(child, out, y, width);
                }
            }
        }
        let _ = width;
    }
    walk(doc, &mut out, &mut y, width);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use aether_dom::Node;
    #[test]
    fn stacks_text() {
        let doc = Node::document(vec![Node::Text("one two".into())]);
        let runs = layout(&doc, 400);
        assert_eq!(runs.len(), 2);
        assert!(runs[1].y > runs[0].y);
    }
}
