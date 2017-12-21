#[derive(PartialEq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Identifier(String),
    Integer(isize),
    Boolean(bool),
    StringToken(String)
}
