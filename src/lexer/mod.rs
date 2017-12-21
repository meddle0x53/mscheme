pub mod token;
mod iterator;

use errors::SyntaxError;
use self::token::Token;
use self::iterator::LexerIterator;

pub fn tokenize(input: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = Vec::new();
    let mut it = LexerIterator::new(input);

    while let Some(&(index, c)) = it.peek() {
         match c {
                '(' => {
                    tokens.push(Token::OpenParen);
                    it.next();
                },
                ')' => {
                    tokens.push(Token::CloseParen);
                    it.next();
                },
                '+' | '-' => {
                    it.next();
                    match it.peek() {
                        Some(&(_, '0' ... '9')) => {
                            tokens.push(it.next_number(c)?);

                            if let Some(token) = try!(it.next_delim()) {
                                tokens.push(token);
                            }
                        },
                        _ => {
                            tokens.push(Token::Identifier(c.to_string()));
                            if let Some(token) = try!(it.next_delim()) {
                                tokens.push(token);
                            }
                        }
                    }
                },
                '#' => {
                    tokens.push(try!(it.next_boolean()));
                    if let Some(token) = try!(it.next_delim()) {
                        tokens.push(token);
                    }
                },
                'A' ... 'Z' | 'a' ... 'z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' ... '?' | '_' | '^'  => {
                    tokens.push(it.next_identifier()?);
                    if let Some(token) = try!(it.next_delim()) {
                        tokens.push(token);
                    }
                },
                '0' ... '9' => {
                    tokens.push(it.next_number('+')?);
                    if let Some(token) = try!(it.next_delim()) {
                        tokens.push(token);
                    }
                },
                '\"' => {
                    tokens.push(it.next_string()?);
                    if let Some(token) = try!(it.next_delim()) {
                        tokens.push(token);
                    }
                },
                ' ' | '\x09' | '\x0a' | '\x0d' => { it.next(); },
                _  => return it.invalid_symbol(index, c)
            }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::token::Token;
    use super::token::Token::*;

    use super::tokenize;

    fn id(id: &str) -> Token { Identifier(id.to_string()) }

    #[test]
    fn test_simple_expression_tokenize() {
        assert_eq!(
            tokenize("(+ 1 4)").unwrap(),
            vec![
                OpenParen, id("+"), Integer(1), Integer(4), CloseParen
            ]
        );

        assert_eq!(
            tokenize("(-5)").unwrap(),
            vec![OpenParen, Integer(-5), CloseParen]
        );
    }

    #[test]
    fn test_tokenize_invalid_syntax() {
        assert!(tokenize("(')").is_err());
        assert!(tokenize("+%").is_err());
        assert!(tokenize("(-23+)").is_err());
        assert!(tokenize("(24r2+)").is_err())
    }

    #[test]
    fn test_white_space_tokenize() {
        assert_eq!(
            tokenize("(+ 3    2)\n(-  \n \t   2\t1 \t)\r\n \t \n").unwrap(),
            vec![
                OpenParen, id("+"), Integer(3), Integer(2), CloseParen,
                OpenParen, id("-"), Integer(2), Integer(1), CloseParen
            ]
        )
    }

    #[test]
    fn test_tokenize_strings() {
        assert_eq!(
            tokenize("\"Sh!t and f*ck & stu$$$6!\"").unwrap(),
            vec![StringToken("Sh!t and f*ck & stu$$$6!".to_string())]
        );

        assert!(tokenize("\"down, down").is_err());
    }


    #[test]
    fn test_integer_tokenize() {
        assert_eq!(
            tokenize("(+ -4 +1 -713 -5 6)").unwrap(),
            vec![
                OpenParen,
                id("+"),
                Integer(-4),
                Integer(1),
                Integer(-713),
                Integer(-5),
                Integer(6),
                CloseParen
            ]
        );

        assert_eq!(
            tokenize("(- 7 49)").unwrap(),
            vec![OpenParen, id("-"), Integer(7), Integer(49), CloseParen]
        );

        assert_eq!(
            tokenize("( - 778899 (+ 2131 4362))").unwrap(),
            vec![
                OpenParen, id("-"), Integer(778899), OpenParen, id("+"),
                Integer(2131), Integer(4362), CloseParen, CloseParen
            ]
        )
    }

    #[test]
    fn test_identifiers() {
        for identifier in ["+", ">=", "ho!", "unless", "it", "$salam"].iter() {
            assert_eq!(tokenize(*identifier).unwrap(), vec![id(identifier)])
        }
    }

    #[test]
    fn test_tokenize_booleans() {
        assert_eq!(tokenize("#t").unwrap(), vec![Boolean(true)]);
        assert_eq!(tokenize("#f").unwrap(), vec![Boolean(false)]);

        assert!(tokenize("#a").is_err());
        assert!(tokenize("#T").is_err());
    }
}
