use crate::models::token::Token;

pub enum Node {
    Expression(Expression),
}

pub enum Expression {
    Atom(Token),
    BinaryOp(Box<Expression>, Token, Box<Expression>),
    UnrayOp(Box<Expression>),
}
