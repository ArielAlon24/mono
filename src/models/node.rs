use crate::models::token::Token;

#[derive(Debug)]
pub enum Expression {
    Atom(Token),
    BinaryOp(Box<Expression>, Token, Box<Expression>),
    UnrayOp(Box<Expression>),
}
