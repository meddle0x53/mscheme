use std::fmt;

// SyntaxError can be raised when the input program is being tokenized:

pub enum SyntaxError {
    InvalidSymbol(usize, usize, String),
    StringNotClosed,
    UnexpectedEOL
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SyntaxError::InvalidSymbol(line, column, ref message) => {
                write!(
                    f,
                    "SyntaxError: {} (line: {}, column: {})",
                    message, line, column + 1
                )
            },
            &SyntaxError::StringNotClosed => {
                write!(f, "SyntaxError: String literal is not properly closed")
            },
            &SyntaxError::UnexpectedEOL => {
                write!(f, "SyntaxError: Unexpected end of input")
            }
        }
    }
}

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[macro_export]
macro_rules! invalid_symbol_error(
    ($line:ident, $column:ident, $($arg:tt)*) => (
        return Err(SyntaxError::InvalidSymbol($line, $column, format!($($arg)*)))
    )
);

// SyntaxError END

// ParseError can be raised when the AST is being built:
pub enum ParseError {
    ClosingParenWithoutOpening,
    UnexpectedEOI
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParseError::ClosingParenWithoutOpening => {
                write!(
                    f, "ParseError: Closing parens don't match the opening ones"
                )
            },
            &ParseError::UnexpectedEOI => {
                write!(f, "ParseError: Unexpected end of input")
            }
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// ParseError END

// RuntimeError can be raised when we evaluate the AST:

pub struct RuntimeError {
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.message)
    }
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[macro_export]
macro_rules! runtime_error(
    ($($arg:tt)*) => (
        return Err(RuntimeError { message: format!($($arg)*) })
    )
);

// RuntimeError END
