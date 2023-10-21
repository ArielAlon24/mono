use crate::tokenizer::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum Node {
    Atom(Token),
    BinaryOp(Box<Node>, Token, Box<Node>),
    UnaryOp(Token, Box<Node>),
    Assignment(Token, Box<Node>),
    Access(Token),
    If {
        condition: Box<Node>,
        block: Box<Node>,
        else_block: Option<Box<Node>>,
    },
    Program {
        statements: Vec<Box<Node>>,
    },
}

impl Node {
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
            Node::Assignment(identifier, expr) => {
                write!(f, "{}Assignment {}\n", current_prefix, identifier)?;
                expr.format_tree(f, &child_prefix, false, true)
            }
            Node::Access(identifier) => write!(f, "{}Access {}\n", current_prefix, identifier),
            Node::If {
                condition,
                block,
                else_block,
            } => {
                write!(f, "{}If\n", current_prefix)?;
                condition.format_tree(f, &child_prefix, false, false)?;
                if let Some(some_else_block) = else_block {
                    block.format_tree(f, &child_prefix, false, false)?;
                    return some_else_block.format_tree(f, &child_prefix, false, true);
                }
                block.format_tree(f, &child_prefix, false, true)
            }
            Node::Program { statements } => {
                write!(f, "{}Program\n", current_prefix)?;
                for (index, statement) in statements.iter().enumerate() {
                    let is_last = index == statements.len() - 1;
                    statement.format_tree(f, &child_prefix, false, is_last)?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_tree(f, "", true, false)
    }
}
