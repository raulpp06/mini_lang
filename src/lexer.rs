#[derive(Debug)]
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

    SemiColon,
    Colon,
    OpenParen,
    ClosedParen,
    OpenBrackets,
    ClosedBrackets,

    // TODO: remove
    Unknown,
}

enum TokenError {
    NumberError,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input,
            pos: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.input[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
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

    fn lex_word(&mut self, start: usize) -> TokenKind<'a> {
        use TokenKind::*;

        while let Some(c) = self.input[self.pos..].chars().next() {
            if c.is_alphanumeric() || c == '_' {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }

        let identifier = &self.input[start..self.pos];

        match identifier {
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

    fn lex_number(&mut self, start: usize) -> Result<TokenKind<'a>, TokenError> {
        use TokenKind::*;

        while let Some(c) = self.input[self.pos..].chars().next() {
            if c.is_numeric() || ".eE-+".contains(c) {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }

        let number = &self.input[start..self.pos];
        if is_real(number) {
            Ok(RealConstant(&number))
        } else if is_int(number) {
            Ok(IntConstant(&number))
        } else {
            Err(TokenError::NumberError)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenKind<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        use TokenKind::*;

        while self.peek_char().is_some_and(|c| c.is_whitespace()) {
            self.next_char();
        }

        let start = self.pos;
        let c = self.next_char()?;

        match c {
            ';' => Some(SemiColon),
            ':' => Some(Colon),
            '(' => Some(OpenBrackets),
            ')' => Some(ClosedBrackets),
            '{' => Some(OpenBrackets),
            '}' => Some(ClosedBrackets),

            '=' => {
                if self.next_char_if(|x| x == '=').is_some() {
                    return Some(Equals);
                }
                return Some(Assign);
            },
            '+' => Some(Sum),
            '-' => Some(Rest),
            '*' => {
                if self.next_char_if(|x| x == '*').is_some() {
                    return Some(Power);
                }
                return Some(Mult);
            },
            '/' => Some(Div),
            '%' => Some(Mod),
            '<' => {
                if self.next_char_if(|x| x == '=').is_some() {
                    return Some(LessThanEq);
                }
                return Some(LessThan);
            },
            '>' => {
                if self.next_char_if(|x| x == '=').is_some() {
                    return Some(MoreThanEq);
                }
                return Some(MoreThan);
            },
            '!' => {
                if self.next_char_if(|x| x == '=').is_some() {
                    return Some(NotEquals);
                }
                return Some(Unknown);
            }

            c if c.is_alphabetic() || c == '_' => {
                Some(self.lex_word(start))
            }
            c if c.is_numeric() => {
                Some(self.lex_number(start).unwrap_or(Unknown))
            }

            _ => Some(Unknown),
        }
    }
}

fn is_int(number: &str) -> bool {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum IntState {
        Start,
        Integer,
        Exponent,
        ExponentSign,
        ExponentDigits,
    }
    use IntState::*;

    let mut state = Start;

    for c in number.chars() {
        state = match (state, c) {
            (Start, c) if c.is_ascii_digit() => Integer,
            (Integer, c) if c.is_ascii_digit() => Integer,
            (Integer, c) if "eE".contains(c) => Exponent,
            (Exponent, c) if c == '+' => ExponentSign,
            (Exponent, c) if c.is_ascii_digit() => ExponentDigits,
            (ExponentSign, c) if c.is_ascii_digit() => ExponentDigits,
            (ExponentDigits, c) if c.is_ascii_digit() => ExponentDigits,
            _ => return false,
        }
    }

    matches!(state, Integer | ExponentDigits)
}

fn is_real(number: &str) -> bool {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RealState {
        Start,
        Integer,
        Decimal,
        Fraction,
        Exponent,
        ExponentSign,
        ExponentDigits,
    }
    use RealState::*;

    let mut state = Start;

    for c in number.chars() {
        state = match (state, c) {
            (Start, c) if c.is_ascii_digit() => Integer,
            (Integer, c) if c.is_ascii_digit() => Integer,
            (Integer, c) if c == '.' => Decimal,
            (Integer, c) if "eE".contains(c) => Exponent,
            (Decimal, c) if c.is_ascii_digit() => Fraction,
            (Fraction, c) if c.is_ascii_digit() => Fraction,
            (Fraction, c) if "eE".contains(c) => Exponent,
            (Exponent, c) if "+-".contains(c) => ExponentSign,
            (Exponent, c) if c.is_ascii_digit()=> ExponentDigits,
            (ExponentSign, c) if c.is_ascii_digit() => ExponentDigits,
            (ExponentDigits, c) if c.is_ascii_digit() => ExponentDigits,
            _ => return false
        }
    }

    matches!(state, Fraction | ExponentDigits)
}
