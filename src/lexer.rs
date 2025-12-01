#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    //name like var fns and stuff
    Ident(String),

    //keywords( why not name? because its f**cking slow i guess bc bc )
    KwLet, //var decalre
    KwFn, //defun
    KwIf,
    KwElse,
    KwFor,
    KwStruct,
    KwWhile,
    KwReturn,
    KwOut, // break
    KwSkip, // continue
    KwTrue,
    KwFalse,
    KwConst,
    KwMut, // changeable? like mutable
    KwImport,

    // literals ( Why the hell do they need to call the long word instead of data type s**t )
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),

    // yeah 'nil' WHY NOT NONE OR NULL laugh over life lang design nerds
    KwNil,

    // ops
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    EqEq, // ik ik its funny
    BangEq, // been a hell of a ride but i think its my time to grow bang BaNg BANG
    Less,
    LessEq,
    Greater,
    GreaterEq,

    Bang,
    AndAnd,
    OrOr,

    Dot,
    FatArrow, // => "Fat Arrow" these "Naming conventions" are killing me
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,

    Eof,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    len: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        let chars: Vec<char> = src.chars().collect();
        let len = chars.len();
        Self { chars, pos: 0, len }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.len
    }

    fn curr(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.curr()?;
        self.pos += 1;
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.curr() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        // assumes we've just seen "//"
        while let Some(c) = self.curr() {
            self.pos += 1;
            if c == '\n' {
                break;
            }
        }
    }

    fn ident_or_kw(&mut self) -> TokenKind {
        let start = self.pos;
        self.pos += 1;

        while let Some(c) = self.curr() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }

        let text: String = self.chars[start..self.pos].iter().collect();

        match text.as_str() {
            "let"    => TokenKind::KwLet,
            "mut"    => TokenKind::KwMut,
            "fn"     => TokenKind::KwFn,
            "if"     => TokenKind::KwIf,
            "else"   => TokenKind::KwElse,
            "struct" => TokenKind::KwStruct,
            "for"    => TokenKind::KwFor,
            "while"  => TokenKind::KwWhile,
            "return" => TokenKind::KwReturn,
            "out"    => TokenKind::KwOut,
            "skip"   => TokenKind::KwSkip,
            "true"   => TokenKind::KwTrue,
            "false"  => TokenKind::KwFalse,
            "const"  => TokenKind::KwConst,
            "mut"    => TokenKind::KwMut,
            "import" => TokenKind::KwImport,
            "nil"    => TokenKind::KwNil,
            _        => TokenKind::Ident(text),
        }
    }

    fn lex_number(&mut self) -> TokenKind {
        let start = self.pos;
        self.pos += 1;

        while let Some(c) = self.curr() {
            if c.is_ascii_digit() {
                self.pos += 1;
            } else {
                break;
            }
        }

        let mut is_float = false;

        if let Some('.') = self.curr() {
            if let Some(next) = self.next() {
                if next.is_ascii_digit() {
                    is_float = true;
                    self.pos += 1;
                    while let Some(c) = self.curr() {
                        if c.is_ascii_digit() {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        let text: String = self.chars[start..self.pos].iter().collect();
        if is_float {
            let v: f64 = text.parse().expect("invalid float literal");
            TokenKind::FloatLiteral(v)
        } else {
            let v: i64 = text.parse().expect("invalid int literal");
            TokenKind::IntLiteral(v)
        }
    }

    fn lex_string(&mut self) -> TokenKind {
        self.pos += 1;
        let mut result = String::new();

        while let Some(c) = self.curr() {
            self.pos += 1;
            match c {
                '"' | '\'' => {
                    return TokenKind::StringLiteral(result);
                }
                '\\' => {
                    // esc char
                    let esc = self.curr().expect("unterminated escape");
                    self.pos += 1;
                    let ch = match esc {
                        'n'  => '\n',
                        't'  => '\t',
                        'r'  => '\r',
                        '\\' => '\\',
                        '"'  => '"',
                        '0'  => '\0',
                        other => {
                            panic!("Unknown escape sequence: \\{}", other);
                        }
                    };
                    result.push(ch);
                }
                other => {
                    result.push(other);
                }
            }
        }

        panic!("Unterminated string literal");
    }

    pub fn next_token(&mut self) -> TokenKind {
        loop {
            self.skip_whitespace();

            if self.is_eof() {
                return TokenKind::Eof;
            }

            let c = self.curr().unwrap();

            // identifier / keyword
            if c.is_ascii_alphabetic() || c == '_' {
                return self.ident_or_kw();
            }

            // number
            if c.is_ascii_digit() {
                return self.lex_number();
            }

            // string
            if c == '"' || c == '\'' {
                return self.lex_string();
            }

            // the non boomer stuff: operators / punctuation / comments
            match c {
                // comments: //
                '/' => {
                    if let Some('/') = self.next() {
                        // line comment
                        self.pos += 2; // skip "//"
                        self.skip_line_comment();
                        continue; // restart loop
                    } else {
                        // / or /=
                        self.pos += 1;
                        if let Some('=') = self.curr() {
                            self.pos += 1;
                            return TokenKind::SlashEqual;
                        }
                        return TokenKind::Slash;
                    }
                }

                '+' => {
                    self.pos += 1;
                    if let Some('=') = self.curr() {
                        self.pos += 1;
                        return TokenKind::PlusEqual;
                    }
                    return TokenKind::Plus;
                }

                '-' => {
                    self.pos += 1;
                    if let Some('=') = self.curr() {
                        self.pos += 1;
                        return TokenKind::MinusEqual;
                    }
                    return TokenKind::Minus;
                }

                '*' => {
                    self.pos += 1;
                    if let Some('=') = self.curr() {
                        self.pos += 1;
                        return TokenKind::StarEqual;
                    }
                    return TokenKind::Star;
                }

                '%' => {
                    self.pos += 1;
                    return TokenKind::Percent;
                }

                '=' => {
                    self.pos += 1;
                    if let Some('=') = self.curr() {
                        self.pos += 1;
                        return TokenKind::EqEq;
                    }
                    if let Some('>') = self.curr() {
                        self.pos += 1;
                        return TokenKind::FatArrow;
                    }
                    return TokenKind::Equal;
                }

                '!' => {
                    self.pos += 1;
                    if let Some('=') = self.curr() {
                        self.pos += 1;
                        return TokenKind::BangEq;
                    }
                    return TokenKind::Bang;
                }

                '<' => {
                    self.pos += 1;
                    if let Some('=') = self.curr() {
                        self.pos += 1;
                        return TokenKind::LessEq;
                    }
                    return TokenKind::Less;
                }

                '>' => {
                    self.pos += 1;
                    if let Some('=') = self.curr() {
                        self.pos += 1;
                        return TokenKind::GreaterEq;
                    }
                    return TokenKind::Greater;
                }

                '&' => {
                    self.pos += 1;
                    if let Some('&') = self.curr() {
                        self.pos += 1;
                        return TokenKind::AndAnd;
                    }
                    panic!("single '&' not supported yet");
                }

                '|' => {
                    self.pos += 1;
                    if let Some('|') = self.curr() {
                        self.pos += 1;
                        return TokenKind::OrOr;
                    }
                    panic!("single '|' not supported yet");
                }

                '.' => {
                    self.pos += 1;
                    return TokenKind::Dot;
                }

                '(' => { self.pos += 1; return TokenKind::LParen; }
                ')' => { self.pos += 1; return TokenKind::RParen; }
                '{' => { self.pos += 1; return TokenKind::LBrace; }
                '}' => { self.pos += 1; return TokenKind::RBrace; }
                '[' => { self.pos += 1; return TokenKind::LBracket; }
                ']' => { self.pos += 1; return TokenKind::RBracket; }
                ',' => { self.pos += 1; return TokenKind::Comma; }
                ';' => { self.pos += 1; return TokenKind::Semicolon; }
                ':' => { self.pos += 1; return TokenKind::Colon; }

                other => {
                    panic!("Unexpected character in lexer: '{}'", other);
                }
            }
        }
    }
}

pub fn lex(content: &str) -> Vec<TokenKind> {
    let mut lexer = Lexer::new(content);
    let mut tokens = Vec::new();

    loop {
        let tok = lexer.next_token();
        let done = matches!(tok, TokenKind::Eof);
        tokens.push(tok);
        if done {
            break;
        }
    }

    tokens
}