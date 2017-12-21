use std::iter::Peekable;
use std::iter::Enumerate;
use std::str::Chars;

use errors::SyntaxError;
use lexer::token::Token;
use lexer::token::Token::*;

pub struct LexerIterator<'a> {
    chars: Peekable<Enumerate<Chars<'a>>>,
    current: Option<char>,
    line: usize,
}

impl<'a> Iterator for LexerIterator<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == Some('\x0a') {
            self.line += 1;
        }

        let result: Option<(usize, char)> = self.chars.next();

        if let Some((_, ch)) = result {
            self.current = Some(ch)
        } else {
            self.current = None
        }

        result
    }
}

impl<'a> LexerIterator<'a> {
    pub fn new(input: &'a str) -> LexerIterator<'a> {
        LexerIterator {
            chars: input.chars().enumerate().peekable(),
            current: None,
            line: 1
        }
    }

    pub fn peek(&mut self) -> Option<&(usize, char)> {
        self.chars.peek()
    }

    pub fn invalid_symbol(&self, index: usize, chr: char) -> Result<Vec<Token>, SyntaxError> {
        let line = self.line;
        invalid_symbol_error!(line, index, "Unexpected character: {}", chr)
    }

    pub fn next_char(&mut self, chr: char, buffer: &mut String) {
        buffer.push(chr);
        self.next();
    }

    pub fn next_number(&mut self, sign: char) -> Result<Token, SyntaxError> {
        let mut number_buffer = String::new();

        while let Some(&(_, ch)) = self.peek() {
            match ch {
                '0' ... '9' => self.next_char(ch, &mut number_buffer),
                _ => break
            }
        }

        let value = number_buffer.parse().unwrap();
        Ok(Integer(if sign == '-' { -1 * value } else { value }))
    }

    pub fn next_boolean(&mut self) -> Result<Token, SyntaxError> {
        self.next();
        match self.next() {
            Some((_, 't')) => { Ok(Boolean(true)) },
            Some((_, 'f')) => { Ok(Boolean(false)) },
            Some((index, symbol)) => {
                let line = self.line;
                invalid_symbol_error!(
                    line, index,
                    "Unexpected character when looking for t/f: {:?}", symbol
                )
            },
            None => return Err(SyntaxError::UnexpectedEOL),
        }
    }

    pub fn next_identifier(&mut self) -> Result<Token, SyntaxError> {
        let mut id_buffer = String::new();

        while let Some(&(_, chr)) = self.peek() {
            match chr {
                'A' ... 'Z' | 'a' ... 'z' | '0' ... '9' => {
                    self.next_char(chr, &mut id_buffer);
                },
                '/' | '!' | '$' | '%' | '*' | ':' | '<' ... '?' | '_' | '-' | '+' => {
                    self.next_char(chr, &mut id_buffer);
                },
                _ => break
            }
        }

        Ok(Identifier(id_buffer))
    }


    pub fn next_string(&mut self) -> Result<Token, SyntaxError> {
        self.next();
        let mut string_buffer = String::new();

        while let Some((_, chr)) = self.next() {
            if chr == '\"' {
                return Ok(StringToken(string_buffer))
            } else {
                string_buffer.push(chr);
            }
        }

        Err(SyntaxError::StringNotClosed)
    }


    pub fn next_delim(&mut self) -> Result<Option<Token>, SyntaxError> {
        let mut result = None;

        if let Some(&(index, c)) = self.peek() {
            match c {
                ')' => {
                    result = Some(CloseParen);
                    self.next();
                },
                ' ' | '\x09' | '\x0a' | '\x0d' => (),
                _ => {
                    let line = self.line;
                    invalid_symbol_error!(
                        line, index,
                        "Unexpected symbol '{}'. Expected white space or closing paren.",
                        c
                    )
                }
            }
        }

        Ok(result)
    }
}
