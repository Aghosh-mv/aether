use aether_css::{style_for, Display, Rule};
use aether_dom::Node;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRun {
    pub text: String,
    pub x: u32,
    pub y: u32,
}
pub fn layout(doc: &Node, width: u32) -> Vec<TextRun> {
    layout_with_styles(doc, width, &[])
}
pub fn layout_with_styles(doc: &Node, width: u32, rules: &[Rule]) -> Vec<TextRun> {
    let mut out = Vec::new();
    let mut y = 24;
    fn walk(n: &Node, out: &mut Vec<TextRun>, y: &mut u32, width: u32, rules: &[Rule]) {
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
            Node::Document { children } => {
                for child in children {
                    walk(child, out, y, width, rules);
                }
            }
            Node::Element { tag, children, .. } => {
                let style = style_for(tag, rules);
                if style.display == Display::None {
                    return;
                }
                *y += style.margin;
                for child in children {
                    walk(child, out, y, width, rules);
                }
                *y += style.margin;
            }
        }
        let _ = width;
    }
    walk(doc, &mut out, &mut y, width, rules);
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
