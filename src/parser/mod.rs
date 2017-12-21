use errors::ParseError;
use lexer::token::Token;

use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Identifier(String),
    Integer(isize),
    Boolean(bool),
    StringNode(String),
    List(Vec<ASTNode>),
}

use self::ASTNode::*;

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<ASTNode>, ParseError> {
    parse_level(&mut tokens.iter(), 0)
}

fn parse_level(tokens: &mut Iter<Token>, level: u32) -> Result<Vec<ASTNode>, ParseError> {
    let mut ast_nodes = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            &Token::OpenParen => {
                let inner = try!(parse_level(tokens, level + 1));
                ast_nodes.push(List(inner));
            },
            &Token::CloseParen => {
                if level > 0 {
                    return Ok(ast_nodes);
                } else {
                    return Err(ParseError::ClosingParenWithoutOpening)
                }
            },
            &Token::Identifier(ref val) => {
                ast_nodes.push(Identifier(val.clone()));
            },
            &Token::Integer(ref val) => {
                ast_nodes.push(Integer(val.clone()));
            },
            &Token::Boolean(ref val) => {
                ast_nodes.push(Boolean(val.clone()));
            },
            &Token::StringToken(ref val) => {
                ast_nodes.push(StringNode(val.clone()));
            },
        };
    }

    if level == 0 {
        Ok(ast_nodes)
    } else {
        Err(ParseError::UnexpectedEOI)
    }
}

#[cfg(test)]
mod tests {
    use lexer::token::Token;

    use super::ASTNode;
    use super::ASTNode::*;
    use super::parse;

    fn id_token(id: &str) -> Token { Token::Identifier(id.to_string()) }
    fn id(id: &str) -> ASTNode { ASTNode::Identifier(id.to_string()) }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                &vec![
                    Token::OpenParen, id_token("+"), Token::Integer(1),
                    Token::Integer(2), Token::CloseParen
                ]
            ).unwrap(),
            vec![List(vec![id("+"), Integer(1), Integer(2)])]
        );

        assert_eq!(
            parse(
                &vec![
                    Token::OpenParen, id_token("+"), Token::Integer(1),
                    Token::OpenParen, id_token("-"), Token::Integer(5),
                    Token::Integer(4), Token::CloseParen, Token::CloseParen
                ]
            ).unwrap(),
            vec![
                List(
                    vec![
                        id("+"), Integer(1),
                        List(vec![id("-"), Integer(5), Integer(4)])
                    ]
                )
            ]
        );
    }

    #[test]
    fn test_parse_err() {
        assert!(parse(&vec![Token::OpenParen, id_token("+")]).is_err())
    }
}
