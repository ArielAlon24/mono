use crate::models::token::Token;

#[derive(Debug)]
pub enum Node {
    Atom(Token),
    BinaryOp(Box<Node>, Token, Box<Node>),
    UnaryOp(Token, Box<Node>),
}
