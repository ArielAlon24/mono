use crate::tokenizer::token::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Atom {
        value: Token,
    },
    List {
        values: Vec<Box<Node>>,
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
    FuncDeclearion {
        identifier: Token,
        arguments: Vec<Token>,
        body: Box<Node>,
    },
    FuncCall {
        identifier: Token,
        parameters: Vec<Box<Node>>,
    },
    Assignment {
        identifier: Token,
        value: Box<Node>,
        is_declaration: bool,
    },
    ListAssignment {
        identifier: Token,
        index: Box<Node>,
        value: Box<Node>,
    },
    Access {
        identifier: Token,
    },
    Index {
        identifier: Token,
        index: Box<Node>,
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
    Return {
        value: Box<Node>,
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
            Node::List { values } => {
                write!(f, "{}List\n", current_prefix)?;
                for (index, value) in values.iter().enumerate() {
                    let is_last = index == values.len() - 1;
                    value.format_tree(f, &child_prefix, false, is_last)?;
                }
                Ok(())
            }
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
            Node::Assignment {
                identifier,
                value,
                is_declaration,
            } => {
                write!(
                    f,
                    "{}Assignment (Deceleration: {}) {}\n",
                    current_prefix, is_declaration, identifier
                )?;
                write!(f, "{}│  Value\n", child_prefix)?;
                value.format_tree(f, &child_prefix, false, true)
            }
            Node::ListAssignment {
                identifier,
                index,
                value,
            } => {
                write!(f, "{}ListAssignment: {}\n", current_prefix, identifier)?;
                write!(f, "{}│  Index\n", child_prefix)?;
                index.format_tree(f, &child_prefix, false, false)?;
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
            Node::While { condition, block } => {
                write!(f, "{}While\n", current_prefix)?;
                write!(f, "{}│  Condition\n", child_prefix)?;
                condition.format_tree(f, &child_prefix, false, false)?;
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
            Node::FuncDeclearion {
                identifier,
                arguments,
                body,
            } => {
                write!(f, "{}FuncDeclearion {}\n", current_prefix, identifier)?;
                write!(f, "{}│  Arguments\n", child_prefix)?;
                for argument in arguments.iter() {
                    write!(f, "{}├──── {:?}\n", child_prefix, argument.kind)?;
                }
                write!(f, "{}│  Body\n", child_prefix)?;
                body.format_tree(f, &child_prefix, false, true)
            }
            Node::FuncCall {
                identifier,
                parameters,
            } => {
                write!(f, "{}FuncCall {}\n", current_prefix, identifier)?;
                write!(f, "{}│  Parameters\n", child_prefix)?;
                for (index, parameter) in parameters.iter().enumerate() {
                    let is_last = index == parameters.len() - 1;
                    parameter.format_tree(f, &child_prefix, false, is_last)?;
                }
                Ok(())
            }
            Self::Return { value } => {
                write!(f, "{}Return\n", current_prefix)?;
                value.format_tree(f, &child_prefix, false, true)
            }
            Node::Index { identifier, index } => {
                write!(f, "{}Index {}\n", current_prefix, identifier)?;
                write!(f, "{}│  At\n", child_prefix)?;
                index.format_tree(f, &child_prefix, false, true)
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_tree(f, "", true, false)
    }
}
