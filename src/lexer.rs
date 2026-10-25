#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenKind<'a> {
    Ident(&'a str),

    IntConstant(&'a str),
    RealConstant(&'a str),
    String(&'a str),

    // keywords
    Program,

    Var,
    Const,

    Int,
    Real,
    Bool,

    True,
    False,

    If,
    Else,
    For,
    To,
    Downto,

    Print,
    Read,

    And,
    Or,
    Not,

    // Operators
    Assign,
    Sum,
    Rest,
    Mult,
    Div,
    Mod,
    Power,
    LessThan,
    MoreThan,
    LessThanEq,
    MoreThanEq,
    Equals,
    NotEquals,

    Semicolon,
    Colon,
    OpenParen,
    ClosedParen,
    OpenCurly,
    ClosedCurly,

    End,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub file_path: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub loc: Location,
}

#[derive(Debug)]
pub enum TokenError {
    UnknownTokenError(Location),
    NumberError(Location),
    StringError(Location),
    CommentError(Location),
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    exhausted: bool,
    loc: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, file_path: String) -> Self {
        Self {
            input: input,
            pos: 0,
            exhausted: false,
            loc: Location {
                file_path,
                line: 0,
                column: 0,
            },
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.input[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        match c {
            '\n' => {
                self.loc.line += 1;
                self.loc.column = 0;
            },
            _ => self.loc.column += c.len_utf8(),
        }
        Some(c)
    }

    fn next_char_if(&mut self, predicate: impl FnOnce(char) -> bool) -> Option<char> {
        let c = self.peek_char()?;

        if predicate(c) {
            self.next_char()
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn lex_string(&mut self, loc: Location) -> Result<Token<'a>, TokenError> {
        use TokenKind::*;

        let start = self.pos;
        loop {
            match self.peek_char() {
                Some('"') => break,
                Some('\n') | None => return Err(TokenError::StringError(loc)),
                Some(_) => { self.next_char(); }
            }
        }
        let content = &self.input[start..self.pos];
        self.next_char();

        Ok(Token { kind: String(content), loc })
    }

    fn lex_word(&mut self, start: usize) -> TokenKind<'a> {
        use TokenKind::*;

        while matches!(self.peek_char(), Some(c) if c.is_alphanumeric() || c == '_') {
            self.next_char();
        }

        match &self.input[start..self.pos] {
            "program" => Program,
            "var" => Var,
            "const" => Const,
            "int" => Int,
            "real" => Real,
            "bool" => Bool,
            "true" => True,
            "false" => False,
            "if" => If,
            "else" => Else,
            "for" => For,
            "to" => To,
            "downto" => Downto,
            "print" => Print,
            "read" => Read,
            "and" => And,
            "or" => Or,
            "not" => Not,
            id => Ident(&id),
        }
    }

    fn eat_digits(&mut self) -> usize {
        let start = self.pos;
        while matches!(self.peek_char(), Some('0'..='9')) {
            self.next_char();
        }

        self.pos - start
    }

    fn lex_number(&mut self, start: usize, loc: Location) -> Result<Token<'a>, TokenError> {
        use TokenKind::*;
        enum NumberKind {
            Int,
            Real,
        }

        self.eat_digits();
        let mut kind = NumberKind::Int;

        if self.peek_char() == Some('.') {
            self.next_char();
            if self.eat_digits() == 0 {
                return Err(TokenError::NumberError(loc))
            }
            kind = NumberKind::Real
        }

        if matches!(self.peek_char(), Some('e' | 'E')) {
            self.next_char();
            if self.peek_char() == Some('+') {
                self.next_char();
            } else if self.peek_char() == Some('-') {
                self.next_char();
                kind = NumberKind::Real;
            }
            if self.eat_digits() == 0 {
                return Err(TokenError::NumberError(loc))
            }
        }

        match kind {
            NumberKind::Int => Ok(Token { kind: IntConstant(&self.input[start..self.pos]), loc }),
            NumberKind::Real => Ok(Token { kind: RealConstant(&self.input[start..self.pos]), loc }),
        }
    }

    fn skip_unnecessary(&mut self) -> Result<(), TokenError> {
        loop {
            let rest = &self.input[self.pos..];
            if rest.starts_with("//") {
                while self.peek_char().is_some_and(|c| c != '\n') {
                    self.next_char();
                }
            } else if rest.starts_with("/*") {
                let loc = self.loc.clone();
                self.next_char();
                self.next_char();
                loop {
                    if self.input[self.pos..].starts_with("*/") {
                        self.next_char();
                        self.next_char();
                        break;
                    }
                    if self.next_char().is_none() {
                        return Err(TokenError::CommentError(loc));
                    }
                }
            } else if self.peek_char().is_some_and(|x| x.is_whitespace()) {
                self.next_char();
            } else {
                return Ok(());
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, TokenError>;

    fn next(&mut self) -> Option<Self::Item> {
        use TokenKind::*;

        if let Err(e) = self.skip_unnecessary() {
            return Some(Err(e));
        }

        let loc = self.loc.clone();
        let start = self.pos;

        let Some(c) = self.next_char() else {
            if self.exhausted {
                return None;
            }
            self.exhausted = true;
            return Some(Ok(Token { kind: End, loc }));
        };

        let kind = match c {
            ';' => Semicolon,
            ':' => Colon,
            '(' => OpenParen,
            ')' => ClosedParen,
            '{' => OpenCurly,
            '}' => ClosedCurly,
            '+' => Sum,
            '-' => Rest,
            '/' => Div,
            '%' => Mod,
            '=' => if self.next_char_if(|x| x == '=').is_some() { Equals } else { Assign },
            '*' => if self.next_char_if(|x| x == '*').is_some() { Power } else { Mult },
            '<' => if self.next_char_if(|x| x == '=').is_some() { LessThanEq } else { LessThan },
            '>' => if self.next_char_if(|x| x == '=').is_some() { MoreThanEq } else { MoreThan },
            '!' => {
                if self.next_char_if(|x| x == '=').is_some() {
                    NotEquals
                } else {
                    return Some(Err(TokenError::UnknownTokenError(loc)));
                }
            }
            '"' => return Some(self.lex_string(loc)),
            c if c.is_alphabetic() || c == '_' => self.lex_word(start),
            c if c.is_ascii_digit() => return Some(self.lex_number(start, loc)),
            _ => return Some(Err(TokenError::UnknownTokenError(loc))),
        };

        Some(Ok(Token { kind, loc }))
    }
}
