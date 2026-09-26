use aether_css::{inherit, style_for, Display, Rule, Style};
use aether_dom::Node;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRun {
    pub text: String,
    pub x: u32,
    pub y: u32,
    pub color: u32,
    pub font_size: u32,
}
pub fn layout(doc: &Node, width: u32) -> Vec<TextRun> {
    layout_with_styles(doc, width, &[])
}
pub fn layout_with_styles(doc: &Node, width: u32, rules: &[Rule]) -> Vec<TextRun> {
    let mut out = Vec::new();
    let mut y = 24;
    fn walk(
        n: &Node,
        out: &mut Vec<TextRun>,
        y: &mut u32,
        width: u32,
        rules: &[Rule],
        parent_style: &Style,
    ) {
        match n {
            Node::Text(s) => {
                let max_chars = width.saturating_sub(32).max(7) / 7;
                let mut line = String::new();
                for word in s.split_whitespace() {
                    let candidate = if line.is_empty() {
                        word.to_string()
                    } else {
                        format!("{line} {word}")
                    };
                    if candidate.chars().count() as u32 > max_chars && !line.is_empty() {
                        out.push(TextRun {
                            text: line,
                            x: 16,
                            y: *y,
                            color: parent_style.color,
                            font_size: parent_style.font_size,
                        });
                        *y += 18;
                        line = word.to_string();
                    } else {
                        line = candidate;
                    }
                }
                if !line.is_empty() {
                    out.push(TextRun {
                        text: line,
                        x: 16,
                        y: *y,
                        color: parent_style.color,
                        font_size: parent_style.font_size,
                    });
                    *y += 18;
                }
            }
            Node::Document { children } => {
                for child in children {
                    walk(child, out, y, width, rules, parent_style);
                }
            }
            Node::Element {
                tag,
                attributes,
                children,
            } => {
                let style = inherit(parent_style, &style_for(tag, attributes, rules));
                if style.display == Display::None {
                    return;
                }
                *y += style.margin;
                for child in children {
                    walk(child, out, y, width, rules, &style);
                }
                *y += style.margin;
            }
        }
        let _ = width;
    }
    let default_style = style_for("", &[], &[]);
    walk(doc, &mut out, &mut y, width, rules, &default_style);
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
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].text, "one two");
    }
    #[test]
    fn wraps_to_available_width() {
        let doc = Node::document(vec![Node::Text("one two three".into())]);
        let runs = layout(&doc, 60);
        assert!(runs.len() > 1);
        assert!(runs[1].y > runs[0].y);
    }
}
