use crate::tokenizer::token::Token;
use std::fmt;

/*
--- Node (enum) ---

The Node enum represent every possible node and his
attributes from the AST that is created by the Parser
*/
#[derive(Debug)]
pub enum Node {
    Atom(Token),
    BinaryOp(Box<Node>, Token, Box<Node>),
    UnaryOp(Token, Box<Node>),
}

impl Node {
    /*
    The format_tree method creates recursively an easy to
    understand representation of the AST, by indenting forward
    a child and applying like `tree` command Unicode art.
    */
    pub fn format_tree(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        is_root: bool,
        last: bool,
    ) -> fmt::Result {
        let (current_prefix, child_prefix) = if is_root {
            (String::from(prefix), String::from(""))
        } else if last {
            (format!("{}└──── ", prefix), format!("{}      ", prefix))
        } else {
            (format!("{}├──── ", prefix), format!("{}│     ", prefix))
        };

        match self {
            Node::Atom(token) => write!(f, "{}Atom {}\n", current_prefix, token),
            Node::BinaryOp(left, token, right) => {
                write!(f, "{}BinaryOp {}\n", current_prefix, token)?;
                left.format_tree(f, &child_prefix, false, false)?;
                right.format_tree(f, &child_prefix, false, true)
            }
            Node::UnaryOp(token, node) => {
                write!(f, "{}UnaryOp {}\n", current_prefix, token)?;
                node.format_tree(f, &child_prefix, false, true)
            }
        }
    }
}

impl fmt::Display for Node {
    /*
    The fmt (toString) method takes the default formatter and
    formats the self Node using the format_tree method.
    */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_tree(f, "", true, false)
    }
}
