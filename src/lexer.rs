#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Number, String, Ident,
    LBrace, RBrace, LParen, RParen, LBracket, RBracket,
    Comma, Colon, Semi, Dot,
    Plus, Minus, Star, Slash, Percent,
    Eq, EqEq, Neq, Lt, Lte, Gt, Gte, And, Or, Not,
    Arrow, FatArrow,
    Eof, Unknown,
    Import, Export, As, Async, Await, Spawn, TaskKw, Function, Endpoint, Model, Type, Class, Server, Middleware,
    Return, Throw, Try, Catch, If, Else, For, While, In, Break, Continue,
    Let, Const, True, False, Null, Get, Post, Put, Delete, Patch,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tt: TokenType,
    pub value: String,
    pub line: usize,
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source: source.chars().collect(), pos: 0, line: 1, col: 1 }
    }
    
    fn peek(&self, off: isize) -> char {
        self.source.get((self.pos as isize + off) as usize).copied().unwrap_or('\0')
    }
    
    fn current(&self) -> char { self.peek(0) }
    
    fn at_end(&self) -> bool { self.pos >= self.source.len() }
    
    fn advance(&mut self) {
        if self.current() == '\n' { self.line += 1; self.col = 1; }
        else { self.col += 1; }
        self.pos += 1;
    }
    
    fn make(&self, tt: TokenType, val: &str) -> Token {
        Token { tt, value: val.into(), line: self.line }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while !self.at_end() {
            let c = self.current();
            
            if c.is_whitespace() { self.advance(); continue; }
            
            if c == '/' && self.peek(1) == '/' {
                while !self.at_end() && self.current() != '\n' { self.advance(); }
                continue;
            }
            if c == '/' && self.peek(1) == '*' {
                self.advance(); self.advance();
                while !self.at_end() && !(self.current() == '*' && self.peek(1) == '/') { self.advance(); }
                if !self.at_end() { self.advance(); self.advance(); }
                continue;
            }
            
            if c == '"' || c == '\'' {
                let q = c;
                self.advance();
                let mut val = String::new();
                while self.current() != q && !self.at_end() {
                    if self.current() == '\\' {
                        self.advance();
                        let escaped = match self.current() {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '\\' => '\\',
                            '"' => '"',
                            '\'' => '\'',
                            '0' => '\0',
                            c => c,
                        };
                        val.push(escaped);
                    } else {
                        val.push(self.current());
                    }
                    self.advance();
                }
                if !self.at_end() { self.advance(); }
                tokens.push(self.make(TokenType::String, &val));
                continue;
            }
            
            if c.is_ascii_digit() || (c == '.' && self.peek(1).is_ascii_digit()) {
                let mut val = String::new();
                while self.current().is_ascii_digit() || self.current() == '.' || self.current() == 'e' || self.current() == 'E' || ((self.current() == '-' || self.current() == '+') && (self.peek(-1) == 'e' || self.peek(-1) == 'E')) {
                    val.push(self.current());
                    self.advance();
                }
                tokens.push(self.make(TokenType::Number, &val));
                continue;
            }
            
            if c.is_alphabetic() || c == '_' {
                let mut val = String::new();
                while self.current().is_alphanumeric() || self.current() == '_' {
                    val.push(self.current());
                    self.advance();
                }
                let tt = match val.as_str() {
                    "import" => TokenType::Import,
                    "export" | "pub" => TokenType::Export,
                    "as" => TokenType::As,
                    "async" => TokenType::Async,
                    "await" => TokenType::Await,
                    "spawn" => TokenType::Spawn,
                    "task" => TokenType::TaskKw,
                    "function" | "func" | "fn" => TokenType::Function,
                    "endpoint" | "route" => TokenType::Endpoint,
                    "model" => TokenType::Model,
                    "type" => TokenType::Type,
                    "class" => TokenType::Class,
                    "server" => TokenType::Server,
                    "middleware" => TokenType::Middleware,
                    "return" => TokenType::Return,
                    "throw" => TokenType::Throw,
                    "try" => TokenType::Try,
                    "catch" => TokenType::Catch,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "for" => TokenType::For,
                    "while" => TokenType::While,
                    "in" => TokenType::In,
                    "break" => TokenType::Break,
                    "continue" => TokenType::Continue,
                    "let" => TokenType::Let,
                    "const" => TokenType::Const,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    "null" => TokenType::Null,
                    "GET" => TokenType::Get,
                    "POST" => TokenType::Post,
                    "PUT" => TokenType::Put,
                    "DELETE" => TokenType::Delete,
                    "PATCH" => TokenType::Patch,
                    _ => TokenType::Ident,
                };
                tokens.push(self.make(tt, &val));
                continue;
            }
            
            let two = format!("{}{}", c, self.peek(1));
            match two.as_str() {
                "==" => { tokens.push(self.make(TokenType::EqEq, "==")); self.advance(); self.advance(); }
                "!=" => { tokens.push(self.make(TokenType::Neq, "!=")); self.advance(); self.advance(); }
                "<=" => { tokens.push(self.make(TokenType::Lte, "<=")); self.advance(); self.advance(); }
                ">=" => { tokens.push(self.make(TokenType::Gte, ">=")); self.advance(); self.advance(); }
                "=>" => { tokens.push(self.make(TokenType::FatArrow, "=>")); self.advance(); self.advance(); }
                "->" => { tokens.push(self.make(TokenType::Arrow, "->")); self.advance(); self.advance(); }
                "&&" => { tokens.push(self.make(TokenType::And, "&&")); self.advance(); self.advance(); }
                "||" => { tokens.push(self.make(TokenType::Or, "||")); self.advance(); self.advance(); }
                "+=" | "-=" | "*=" | "/=" => { let op = two.clone(); tokens.push(self.make(TokenType::Unknown, &op)); self.advance(); self.advance(); }
                _ => {
                    let tt = match c {
                        '{' => TokenType::LBrace, '}' => TokenType::RBrace,
                        '(' => TokenType::LParen, ')' => TokenType::RParen,
                        '[' => TokenType::LBracket, ']' => TokenType::RBracket,
                        ',' => TokenType::Comma, ':' => TokenType::Colon,
                        ';' => TokenType::Semi, '.' => TokenType::Dot,
                        '+' => TokenType::Plus, '-' => TokenType::Minus,
                        '*' => TokenType::Star, '/' => TokenType::Slash,
                        '%' => TokenType::Percent, '=' => TokenType::Eq,
                        '<' => TokenType::Lt, '>' => TokenType::Gt,
                        '!' => TokenType::Not,
                        _ => TokenType::Unknown,
                    };
                    tokens.push(self.make(tt, &c.to_string()));
                    self.advance();
                }
            }
        }
        
        tokens.push(self.make(TokenType::Eof, ""));
        tokens
    }
}