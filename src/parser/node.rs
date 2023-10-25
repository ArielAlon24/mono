use crate::tokenizer::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {
    Atom {
        value: Token,
    },
    BinaryOp {
        left: Box<Node>,
        operator: Token,
        right: Box<Node>,
    },
    UnaryOp {
        operator: Token,
        value: Box<Node>,
    },
    Assignment {
        identifier: Token,
        value: Box<Node>,
    },
    Access {
        identifier: Token,
    },
    If {
        condition: Box<Node>,
        block: Box<Node>,
        else_block: Option<Box<Node>>,
    },
    While {
        condition: Box<Node>,
        block: Box<Node>,
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
            Node::Atom { value } => write!(f, "{}Atom {}\n", current_prefix, value),
            Node::BinaryOp {
                left,
                operator,
                right,
            } => {
                write!(f, "{}BinaryOp {}\n", current_prefix, operator)?;
                write!(f, "{}│  Left\n", child_prefix)?;
                left.format_tree(f, &child_prefix, false, false)?;
                write!(f, "{}│  Right\n", child_prefix)?;
                right.format_tree(f, &child_prefix, false, true)
            }
            Node::UnaryOp { operator, value } => {
                write!(f, "{}UnaryOp {}\n", current_prefix, operator)?;
                write!(f, "{}│  Value\n", child_prefix)?;
                value.format_tree(f, &child_prefix, false, true)
            }
            Node::Assignment { identifier, value } => {
                write!(f, "{}Assignment {}\n", current_prefix, identifier)?;
                write!(f, "{}│  Value\n", child_prefix)?;
                value.format_tree(f, &child_prefix, false, true)
            }
            Node::Access { identifier } => write!(f, "{}Access {}\n", current_prefix, identifier),
            Node::If {
                condition,
                block,
                else_block,
            } => {
                write!(f, "{}If\n", current_prefix)?;
                write!(f, "{}│  Condition\n", child_prefix)?;
                condition.format_tree(f, &child_prefix, false, false)?;
                if let Some(some_else_block) = else_block {
                    write!(f, "{}│  Block\n", child_prefix)?;
                    block.format_tree(f, &child_prefix, false, false)?;
                    write!(f, "{}│  Else Block\n", child_prefix)?;
                    return some_else_block.format_tree(f, &child_prefix, false, true);
                }
                write!(f, "{}│  Block\n", child_prefix)?;
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
            Node::While { condition, block } => {
                write!(f, "{}While\n", current_prefix)?;
                write!(f, "{}│  Condition\n", child_prefix)?;
                condition.format_tree(f, &child_prefix, false, false)?;
                write!(f, "{}│  Block\n", child_prefix)?;
                block.format_tree(f, &child_prefix, false, true)
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_tree(f, "", true, false)
    }
}
