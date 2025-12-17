use crate::lexer::TokenKind;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Nil,
    Ident(String),

    Prefix {
        op: PrefixOp,
        rhs: Box<Expr>,
    },

    Infix {
        op: InfixOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },

    Group(Box<Expr>),

    StructLiteral {
        name: String,
        fields: Vec<(String, Expr)>
    },

    FieldAccess {
        target: Box<Expr>,
        field: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum PrefixOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,

    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        valuetype: Type,
        value: Option<Expr>,
    },

    Import {
        name: String,
    },

    ExprStmt(Expr),

    Return(Option<Expr>),

    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },

    While {
        cond: Expr,
        body: Block,
    },

    If {
        cond: Expr,
        then_branch: Block,
        else_branch: Option<BlockOrIf>,
    },

    Func {
        name: String,
        params: Vec<(String, Type)>,
        returntype: Type,
        body: Block,
    },

    Out,   // break
    Skip,  // continue

    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Nil,
    Bool,
    Custom(String)
}

#[derive(Debug, Clone)]
pub enum BlockOrIf {
    Block(Block),
    If(Box<Stmt>), // nested `if` for `else if`
}

pub type Block = Vec<Stmt>;

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // pretty printing the vector
        for stmt in &self.stmts {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Let { name, .. } => write!(f, "Let({})", name),
            Stmt::Return(expr) => write!(f, "Return({:?})", expr),
            _ => write!(f, "<stmt>"), // fill out more as needed
        }
    }
}

// =======================
// ======== PARSER =======
// =======================

pub struct Parser {
    tokens: Vec<TokenKind>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self { tokens, pos: 0 }
    }

    //  CAMBIO 1: Función de ayuda para manejar errores y salir con código 1
    fn error_and_exit(msg: &str) -> ! {
        // Códigos ANSI: \x1b[31m = Rojo, \x1b[0m = Resetear
        eprintln!("\x1b[31m[BUTTER COMPILER ERROR]\x1b[0m {}", msg);
        // Salir con código 1 (error)
        std::process::exit(1);
    }

    fn peek(&self) -> &TokenKind {
        self.tokens.get(self.pos).unwrap_or(&TokenKind::Eof)
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek(), TokenKind::Eof)
    }

    fn bump(&mut self) -> TokenKind {
        let t = self.peek().clone();
        if !self.is_eof() {
            self.pos += 1;
        }
        t
    }

    fn matches(&mut self, kind: &TokenKind) -> bool {
        if self.peek() == kind {
            self.bump();
            true
        } else {
            false
        }
    }

    //  CAMBIO 2: Reemplazar panic! en expect
    fn expect(&mut self, kind: &TokenKind, msg: &str) {
        if !self.matches(kind) {
            let error_msg = format!(
                "Expected token {:?}, but got {:?}. (Contexto: {})",
                kind,
                self.peek(),
                msg
            );
            Self::error_and_exit(&error_msg);
        }
    }

    //  CAMBIO 3: Reemplazar panic! en take_ident
    fn take_ident(&mut self, msg: &str) -> String {
        match self.bump() {
            TokenKind::Ident(s) => s,
            other => {
                let error_msg = format!("Expected identifier, but got {:?}. (Contexto: {})", other, msg);
                Self::error_and_exit(&error_msg);
            }
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut stmts = Vec::new();
        while !self.is_eof() {
            stmts.push(self.parse_decl());
        }
        Program { stmts }
    }

    fn parse_decl(&mut self) -> Stmt {
        match self.peek() {
            TokenKind::KwFn => self.parse_func(),
            TokenKind::KwLet | TokenKind::KwConst => self.parse_let(),
            TokenKind::KwStruct => self.parse_struct(),
            _ => self.parse_stmt(),
        }
    }

    fn parse_type(&mut self) -> Type {
        match self.bump() {
            TokenKind::Ident(name) => {
                match name.as_str() {
                    "Int" => Type::Int,
                    "Float" => Type::Float,
                    "Bool" => Type::Bool,
                    "String" => Type::String,
                    //"Nil" => Type::Nil,

                    other => Type::Custom(other.to_string()),
                }
            }

            TokenKind::KwNil => Type::Nil,
            other => {
                let error_msg = format!("Expected type name, got {:?}", other);
                Self::error_and_exit(&error_msg);
            }
        }
    }

    fn parse_struct(&mut self) -> Stmt {
        self.expect(&TokenKind::KwStruct, "expected 'struct'");
        let name = self.take_ident("struct name");
        self.expect(&TokenKind::LBrace, "Expected '{' after struct name");

        let mut fields = Vec::new();

        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            let field_name = self.take_ident("field name");
            self.expect(&TokenKind::Colon, "expected ':' after field name");
            let field_type = self.parse_type();

            fields.push((field_name, field_type));

            if !self.matches(&TokenKind::Comma) {
                break;
            }
        }

        self.expect(&TokenKind::RBrace, "Expected '}' to end a struct");

        Stmt::Struct {name, fields}
    }

    fn parse_func(&mut self) -> Stmt {
        self.expect(&TokenKind::KwFn, "expected 'fn'");

        let name = self.take_ident("function name");
        self.expect(&TokenKind::LParen, "expected '(' after function name");

        let mut params = Vec::new();
        if !matches!(self.peek(), TokenKind::RParen) {
            loop {
                let param = self.take_ident("parameter name");
                self.expect(&TokenKind::Colon, "Expected ':' after parameter for type declaration");
                let paramtype = self.parse_type();
                params.push((param, paramtype));

                if !self.matches(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenKind::RParen, "expected ')' after parameters");

        self.expect(&TokenKind::FatArrow, "Expected '=>' after function parameter");
        let returntype = self.parse_type();


        let body = self.parse_block();

        Stmt::Func { name, params, returntype, body }
    }

    fn parse_let(&mut self) -> Stmt {
        let is_const = self.matches(&TokenKind::KwConst);
        if !is_const {
            self.expect(&TokenKind::KwLet, "expected 'let' or 'const'");
        }

        let mut mutable = false;
        if self.matches(&TokenKind::KwMut) {
            mutable = true;
        }

        let name = self.take_ident("variable name");
        self.expect(&TokenKind::Colon, "expected ':' after variable Identifer for type declaration");
        let valuetype = self.parse_type();

        let value = if self.matches(&TokenKind::Equal) {
            Some(self.parse_expr())
        } else {
            None
        };

        self.expect(&TokenKind::Semicolon, "expected ';' after let/const");

        if is_const {
            mutable = false;
        }

        Stmt::Let { name, mutable, valuetype, value }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            TokenKind::KwReturn => {
                self.bump();
                if self.matches(&TokenKind::Semicolon) {
                    Stmt::Return(None)
                } else {
                    let expr = self.parse_expr();
                    self.expect(&TokenKind::Semicolon, "expected ';' after return expression");
                    Stmt::Return(Some(expr))
                }
            }

            TokenKind::KwWhile => self.parse_while(),

            TokenKind::KwIf => self.parse_if(),

            TokenKind::LBrace => {
                let block = self.parse_block();
                Stmt::Block(block)
            }

            TokenKind::KwOut => {
                self.bump();
                self.expect(&TokenKind::Semicolon, "expected ';' after 'out'");
                Stmt::Out
            }

            TokenKind::KwSkip => {
                self.bump();
                self.expect(&TokenKind::Semicolon, "expected ';' after 'skip'");
                Stmt::Skip
            }

            _ => {
                let expr = self.parse_expr();
                self.expect(&TokenKind::Semicolon, "expected ';' after expression");
                Stmt::ExprStmt(expr)
            }
        }
    }

    fn parse_block(&mut self) -> Block {
        self.expect(&TokenKind::LBrace, "expected '{' to start block");
        let mut stmts = Vec::new();

        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_decl());
        }

        self.expect(&TokenKind::RBrace, "expected '}' to end block");
        stmts
    }

    fn parse_while(&mut self) -> Stmt {
        self.expect(&TokenKind::KwWhile, "expected 'while'");
        let cond = self.parse_expr();
        let body = self.parse_block();
        Stmt::While { cond, body }
    }

    fn parse_if(&mut self) -> Stmt {
        self.expect(&TokenKind::KwIf, "expected 'if'");
        let cond = self.parse_expr();
        let then_branch = self.parse_block();

        let else_branch = if self.matches(&TokenKind::KwElse) {
            // else if ...
            if matches!(self.peek(), TokenKind::KwIf) {
                let nested_if = self.parse_if();
                Some(BlockOrIf::If(Box::new(nested_if)))
            } else {
                // else { ... }
                let block = self.parse_block();
                Some(BlockOrIf::Block(block))
            }
        } else {
            None
        };

        Stmt::If {
            cond,
            then_branch,
            else_branch,
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Expr {
        let lhs = self.parse_or();

        let op = match self.peek() {
            TokenKind::Equal      => Some(InfixOp::Assign),
            TokenKind::PlusEqual  => Some(InfixOp::AddAssign),
            TokenKind::MinusEqual => Some(InfixOp::SubAssign),
            TokenKind::StarEqual  => Some(InfixOp::MulAssign),
            TokenKind::SlashEqual => Some(InfixOp::DivAssign),
            _ => None,
        };

        if let Some(op) = op {
            self.bump();
            let rhs = self.parse_assignment();
            Expr::Infix {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        } else {
            lhs
        }
    }

    fn parse_or(&mut self) -> Expr {
        let mut expr = self.parse_and();
        while self.matches(&TokenKind::OrOr) {
            let rhs = self.parse_and();
            expr = Expr::Infix {
                op: InfixOp::Or,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            };
        }
        expr
    }

    fn parse_and(&mut self) -> Expr {
        let mut expr = self.parse_equality();
        while self.matches(&TokenKind::AndAnd) {
            let rhs = self.parse_equality();
            expr = Expr::Infix {
                op: InfixOp::And,
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            };
        }
        expr
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();
        loop {
            if self.matches(&TokenKind::EqEq) {
                let rhs = self.parse_comparison();
                expr = Expr::Infix {
                    op: InfixOp::Eq,
                    lhs: Box::new(expr),
                    rhs: Box::new(rhs),
                };
            } else if self.matches(&TokenKind::BangEq) {
                let rhs = self.parse_comparison();
                expr = Expr::Infix {
                    op: InfixOp::Ne,
                    lhs: Box::new(expr),
                    rhs: Box::new(rhs),
                };
            } else {
                break;
            }
        }
        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();
        loop {
            let op = if self.matches(&TokenKind::Less) {
                Some(InfixOp::Lt)
            } else if self.matches(&TokenKind::LessEq) {
                Some(InfixOp::Le)
            } else if self.matches(&TokenKind::Greater) {
                Some(InfixOp::Gt)
            } else if self.matches(&TokenKind::GreaterEq) {
                Some(InfixOp::Ge)
            } else {
                None
            };

            if let Some(op) = op {
                let rhs = self.parse_term();
                expr = Expr::Infix {
                    op,
                    lhs: Box::new(expr),
                    rhs: Box::new(rhs),
                };
            } else {
                break;
            }
        }
        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        loop {
            if self.matches(&TokenKind::Plus) {
                let rhs = self.parse_factor();
                expr = Expr::Infix {
                    op: InfixOp::Add,
                    lhs: Box::new(expr),
                    rhs: Box::new(rhs),
                };
            } else if self.matches(&TokenKind::Minus) {
                let rhs = self.parse_factor();
                expr = Expr::Infix {
                    op: InfixOp::Sub,
                    lhs: Box::new(expr),
                    rhs: Box::new(rhs),
                };
            } else {
                break;
            }
        }
        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();
        loop {
            if self.matches(&TokenKind::Star) {
                let rhs = self.parse_unary();
                expr = Expr::Infix {
                    op: InfixOp::Mul,
                    lhs: Box::new(expr),
                    rhs: Box::new(rhs),
                };
            } else if self.matches(&TokenKind::Slash) {
                let rhs = self.parse_unary();
                expr = Expr::Infix {
                    op: InfixOp::Div,
                    lhs: Box::new(expr),
                    rhs: Box::new(rhs),
                };
            } else if self.matches(&TokenKind::Percent) {
                let rhs = self.parse_unary();
                expr = Expr::Infix {
                    op: InfixOp::Mod,
                    lhs: Box::new(expr),
                    rhs: Box::new(rhs),
                };
            } else {
                break;
            }
        }
        expr
    }

    fn parse_unary(&mut self) -> Expr {
        if self.matches(&TokenKind::Bang) {
            let rhs = self.parse_unary();
            return Expr::Prefix {
                op: PrefixOp::Not,
                rhs: Box::new(rhs),
            };
        }

        if self.matches(&TokenKind::Minus) {
            let rhs = self.parse_unary();
            return Expr::Prefix {
                op: PrefixOp::Neg,
                rhs: Box::new(rhs),
            };
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            if self.matches(&TokenKind::LParen) {
                let mut args = Vec::new();
                if !matches!(self.peek(), TokenKind::RParen) {
                    loop {
                        args.push(self.parse_expr());
                        if !self.matches(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RParen, "expected ')' after arguments");
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            }

            // index: arr[expr]
            else if self.matches(&TokenKind::LBracket) {
                let index = self.parse_expr();
                self.expect(&TokenKind::RBracket, "expected ']' after index");
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
            }

            // struct literal: Person { field = value, ... }


            // ⭐ NEW: field access: expr.field
            else if self.matches(&TokenKind::Dot) {
                let field = self.take_ident("field name after '.'");
                expr = Expr::FieldAccess {
                    target: Box::new(expr),
                    field,
                };
            }

            else {
                break;
            }
        }

        expr
    }


    fn parse_primary(&mut self) -> Expr {
        match self.bump() {
            TokenKind::IntLiteral(v) => Expr::Int(v),
            TokenKind::FloatLiteral(v) => Expr::Float(v),
            TokenKind::StringLiteral(s) => Expr::String(s),

            TokenKind::KwTrue => Expr::Bool(true),
            TokenKind::KwFalse => Expr::Bool(false),
            TokenKind::KwNil => Expr::Nil,

            TokenKind::Ident(name) => {
            if self.matches(&TokenKind::LBrace) {
                let mut fields = Vec::new();

                while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
                    let field_name = self.take_ident("field name in struct literal");
                    self.expect(&TokenKind::Equal, "expected '=' after field name");
                    let value = self.parse_expr();

                    fields.push((field_name, value));

                    if !self.matches(&TokenKind::Comma) {
                        break;
                    }
                }

                self.expect(&TokenKind::RBrace, "expected '}' after struct literal");

                Expr::StructLiteral {
                    name,
                    fields,
                }
            } else {
                Expr::Ident(name)
            }
        }


            TokenKind::LParen => {
                let expr = self.parse_expr();
                self.expect(&TokenKind::RParen, "expected ')' after expression");
                Expr::Group(Box::new(expr))
            }

            // ⭐ CAMBIO 4: Reemplazar panic! en el final de parse_primary
            other => {
                let error_msg = format!("Parser error: unexpected token in primary: {:?}", other);
                Self::error_and_exit(&error_msg);
            }
        }
    }
}

pub fn parse_tokens(tokens: Vec<TokenKind>) -> Program {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}