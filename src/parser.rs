use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub imports: Vec<String>,
    pub functions: Vec<Function>,
    pub endpoints: Vec<Endpoint>,
    pub models: Vec<Model>,
    pub servers: Vec<Server>,
    pub middlewares: Vec<Middleware>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Function { pub name: String, pub params: Vec<(String, String)>, pub body: Vec<Stmt>, pub ret: String }

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Endpoint { pub method: String, pub path: String, pub params: Vec<String>, pub middlewares: Vec<String>, pub body: Vec<Stmt>, pub ret: String }

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Model { pub name: String, pub fields: Vec<(String, String)> }

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Server { pub name: String, pub port: i32, pub host: String, pub cors: bool }

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Middleware { pub name: String, pub body: Vec<Stmt> }

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr, usize),
    Return(Option<Expr>, usize),
    Var { name: String, value: Option<Expr>, mutable: bool, line: usize },
    If { cond: Expr, then: Vec<Stmt>, else_: Vec<Stmt>, line: usize },
    While { cond: Expr, body: Vec<Stmt>, line: usize },
    For { var: String, iter: Expr, body: Vec<Stmt>, line: usize },
    Break(usize),
    Continue(usize),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Ident(String),
    BinOp { op: String, left: Box<Expr>, right: Box<Expr> },
    UnOp { op: String, val: Box<Expr> },
    Call { callee: String, args: Vec<Expr> },
    Arr(Vec<Expr>),
    Obj(Vec<(String, Expr)>),
    Member(Box<Expr>, String),
    Index(Box<Expr>, Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    
    fn cur(&self) -> &Token {
        if self.pos < self.tokens.len() { &self.tokens[self.pos] }
        else { &self.tokens.last().unwrap() }
    }
    
    fn at(&self) -> bool { matches!(self.cur().tt, TokenType::Eof) }
    
    fn adv(&mut self) { if !self.at() { self.pos += 1; } }
    
    fn check(&self, tt: TokenType) -> bool { !self.at() && self.cur().tt == tt }
    fn check_val(&self, val: &str) -> bool { !self.at() && self.cur().value == val }
    
    pub fn parse(&mut self) -> Program {
        let mut prog = Program::default();
        
        while !self.at() {
            let tt = self.cur().tt;
            match tt {
                TokenType::Import => { self.adv(); prog.imports.push(self.cur().value.clone()); self.adv(); }
                TokenType::Function => prog.functions.push(self.fn_()),
                TokenType::Endpoint => prog.endpoints.push(self.endpoint()),
                TokenType::Model => prog.models.push(self.model()),
                TokenType::Server => prog.servers.push(self.server()),
                TokenType::Middleware => prog.middlewares.push(self.middleware()),
                TokenType::Semi => self.adv(),
                _ => prog.body.push(self.stmt()),
            }
        }
        
        prog
    }
    
    fn fn_(&mut self) -> Function {
        self.adv();
        let name = if self.check(TokenType::Ident) { let n = self.cur().value.clone(); self.adv(); n } else { "fn".into() };
        let mut params = vec![];
        if self.check(TokenType::LParen) {
            self.adv();
            while !self.check(TokenType::RParen) && !self.at() {
                if self.check(TokenType::Ident) { 
                    let pname = self.cur().value.clone(); 
                    self.adv();
                    let ptype = if self.check(TokenType::Colon) { self.adv(); let t = self.cur().value.clone(); self.adv(); t } else { "any".into() };
                    params.push((pname, ptype));
                }
                if self.check(TokenType::Comma) { self.adv(); }
            }
            if self.check(TokenType::RParen) { self.adv(); }
        }
        let ret = if self.check(TokenType::Arrow) { self.adv(); let r = self.cur().value.clone(); self.adv(); r } else { "void".into() };
        let body = self.block();
        Function { name, params, body, ret }
    }
    
    fn endpoint(&mut self) -> Endpoint {
        self.adv();
        let mut method = "GET".to_string();
        let mut path = "/".to_string();
        let mut middlewares = vec![];
        
        // Check for middleware annotation
        if self.check_val("middleware") {
            self.adv();
            if self.check(TokenType::LParen) {
                self.adv();
                while !self.check(TokenType::RParen) && !self.at() {
                    if self.check(TokenType::Ident) {
                        middlewares.push(self.cur().value.clone());
                        self.adv();
                    }
                    if self.check(TokenType::Comma) { self.adv(); }
                }
                if self.check(TokenType::RParen) { self.adv(); }
            }
        }
        
        if self.check(TokenType::Get) || self.check(TokenType::Post) || self.check(TokenType::Put) || self.check(TokenType::Delete) {
            method = self.cur().value.clone(); self.adv();
            if self.check(TokenType::String) { path = self.cur().value.clone(); self.adv(); }
        } else if self.check(TokenType::String) {
            path = self.cur().value.clone(); self.adv();
            if self.check(TokenType::Get) { method = self.cur().value.clone(); self.adv(); }
        }
        let ret = if self.check(TokenType::Arrow) { self.adv(); let r = self.cur().value.clone(); self.adv(); r } else { "Response".into() };
        let body = self.block();
        
        // Extract params from path (ex: /users/:id)
        let params: Vec<String> = path.split('/')
            .filter(|p| p.starts_with(':'))
            .map(|p| p.trim_start_matches(':').to_string())
            .collect();
        
        Endpoint { method, path, params, middlewares, body, ret }
    }
    
    fn model(&mut self) -> Model {
        self.adv();
        let name = if self.check(TokenType::Ident) { let n = self.cur().value.clone(); self.adv(); n } else { "T".into() };
        let mut fields = vec![];
        if self.check(TokenType::LBrace) {
            self.adv();
            while !self.check(TokenType::RBrace) {
                if self.check(TokenType::Ident) { fields.push((self.cur().value.clone(), "any".into())); self.adv(); }
                if self.check(TokenType::Colon) { self.adv(); if self.check(TokenType::Ident) { let t = fields.pop().unwrap(); fields.push((t.0, self.cur().value.clone())); self.adv(); } }
                if self.check(TokenType::Comma) { self.adv(); }
            }
            self.adv();
        }
        Model { name, fields }
    }
    
    fn server(&mut self) -> Server {
        self.adv(); // skip 'server'
        
        let name = if self.check(TokenType::Ident) || self.check(TokenType::String) {
            let n = self.cur().value.clone(); self.adv(); n
        } else { "app".into() };
        
        let mut port = 3000;
        let mut host = "0.0.0.0".to_string();
        let mut cors = false;
        
        if self.check(TokenType::LBrace) {
            self.adv(); // skip '{'
            
            while !self.check(TokenType::RBrace) && !self.at() {
                if self.check_val("port") {
                    self.adv(); // skip 'port'
                    if self.check(TokenType::Colon) { self.adv(); }
                    if self.check(TokenType::Number) {
                        port = self.cur().value.parse().unwrap_or(3000);
                        self.adv(); // skip number
                    }
                } else if self.check_val("host") {
                    self.adv(); // skip 'host'
                    if self.check(TokenType::Colon) { self.adv(); }
                    if self.check(TokenType::String) {
                        host = self.cur().value.clone();
                        self.adv(); // skip string
                    }
                } else if self.check_val("cors") {
                    self.adv();
                    if self.check(TokenType::Colon) { self.adv(); }
                    if self.check(TokenType::True) { cors = true; self.adv(); }
                    else if self.check(TokenType::String) { cors = true; self.adv(); }
                }
                if self.check(TokenType::Semi) {
                    self.adv();
                }
            }
            
            if self.check(TokenType::RBrace) {
                self.adv(); // skip '}'
            }
        }
        
        Server { name, port, host, cors }
    }
    
    fn middleware(&mut self) -> Middleware {
        self.adv();
        let name = if self.check(TokenType::Ident) { let n = self.cur().value.clone(); self.adv(); n } else { "mw".into() };
        Middleware { name, body: self.block() }
    }
    
    fn block(&mut self) -> Vec<Stmt> {
        let mut body = vec![];
        if self.check(TokenType::LBrace) {
            self.adv();
            while !self.check(TokenType::RBrace) && !self.at() {
                body.push(self.stmt());
            }
            self.adv();
        }
        body
    }
    
    fn stmt(&mut self) -> Stmt {
        let l = self.cur().line;
        if self.check(TokenType::Return) { self.adv(); Stmt::Return(if self.check(TokenType::Semi) { None } else { Some(self.expr()) }, l) }
        else if self.check(TokenType::If) { self.if_(l) }
        else if self.check(TokenType::While) { self.while_(l) }
        else if self.check(TokenType::For) { self.for_(l) }
        else if self.check(TokenType::Let) || self.check(TokenType::Const) { self.var_stmt(l) }
        else if self.check(TokenType::Break) { self.adv(); Stmt::Break(l) }
        else if self.check(TokenType::Continue) { self.adv(); Stmt::Continue(l) }
        else { Stmt::Expr(self.expr(), l) }
    }
    
    fn if_(&mut self, line: usize) -> Stmt {
        self.adv();
        let cond = self.expr();
        let then = self.block();
        let else_ = if self.check(TokenType::Else) { self.adv(); self.block() } else { vec![] };
        Stmt::If { cond, then, else_, line }
    }
    
    fn while_(&mut self, line: usize) -> Stmt {
        self.adv();
        let cond = self.expr();
        let body = self.block();
        Stmt::While { cond, body, line }
    }
    
    fn for_(&mut self, line: usize) -> Stmt {
        self.adv();
        let var = if self.check(TokenType::Ident) { let v = self.cur().value.clone(); self.adv(); v } else { "i".into() };
        self.adv();
        let iter = self.expr();
        let body = self.block();
        Stmt::For { var, iter, body, line }
    }
    
    fn var_stmt(&mut self, line: usize) -> Stmt {
        let mutable = self.cur().tt == TokenType::Let;
        self.adv();
        let name = if self.check(TokenType::Ident) { let n = self.cur().value.clone(); self.adv(); n } else { "x".into() };
        let value = if self.check(TokenType::Eq) { self.adv(); Some(self.expr()) } else { None };
        Stmt::Var { name, value, mutable, line }
    }
    
    fn expr(&mut self) -> Expr { self.assign() }
    
    fn assign(&mut self) -> Expr {
        let left = self.or();
        if self.check(TokenType::Eq) || self.check_val("+=") || self.check_val("-=") || self.check_val("*=") || self.check_val("/=") {
            let op = self.cur().value.clone();
            self.adv();
            return Expr::BinOp { op, left: Box::new(left), right: Box::new(self.assign()) };
        }
        left
    }
    
    fn or(&mut self) -> Expr {
        let mut left = self.and();
        while self.check(TokenType::Or) { self.adv(); left = Expr::BinOp { op: "||".into(), left: Box::new(left), right: Box::new(self.and()) }; }
        left
    }
    
    fn and(&mut self) -> Expr {
        let mut left = self.eq();
        while self.check(TokenType::And) { self.adv(); left = Expr::BinOp { op: "&&".into(), left: Box::new(left), right: Box::new(self.eq()) }; }
        left
    }
    
    fn eq(&mut self) -> Expr {
        let mut left = self.cmp();
        while self.check(TokenType::EqEq) || self.check(TokenType::Neq) {
            let op = self.cur().value.clone(); self.adv();
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(self.cmp()) };
        }
        left
    }
    
    fn cmp(&mut self) -> Expr {
        let mut left = self.add();
        while matches!(self.cur().tt, TokenType::Lt | TokenType::Gt | TokenType::Lte | TokenType::Gte) {
            let op = self.cur().value.clone(); self.adv();
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(self.add()) };
        }
        left
    }
    
    fn add(&mut self) -> Expr {
        let mut left = self.mul();
        while self.check(TokenType::Plus) || self.check(TokenType::Minus) {
            let op = self.cur().value.clone(); self.adv();
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(self.mul()) };
        }
        left
    }
    
    fn mul(&mut self) -> Expr {
        let mut left = self.unary();
        while matches!(self.cur().tt, TokenType::Star | TokenType::Slash | TokenType::Percent) {
            let op = self.cur().value.clone(); self.adv();
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(self.unary()) };
        }
        left
    }
    
    fn unary(&mut self) -> Expr {
        if self.check(TokenType::Not) || self.check(TokenType::Minus) {
            let op = self.cur().value.clone(); self.adv();
            return Expr::UnOp { op, val: Box::new(self.unary()) };
        }
        self.postfix()
    }
    
    fn postfix(&mut self) -> Expr {
        let mut node = self.primary();
        loop {
            if self.check(TokenType::Dot) { self.adv(); if self.check(TokenType::Ident) { node = Expr::Member(Box::new(node), self.cur().value.clone()); self.adv(); } }
            else if self.check(TokenType::LBracket) { self.adv(); let idx = self.expr(); self.adv(); node = Expr::Index(Box::new(node), Box::new(idx)); }
            else if self.check(TokenType::LParen) { self.adv(); let args = self.args(); self.adv(); if let Expr::Ident(n) = &node { node = Expr::Call { callee: n.clone(), args }; } else { let callee = expr_to_name(&node); node = Expr::Call { callee, args }; } }
            else { break; }
        }
        node
    }
    
    fn primary(&mut self) -> Expr {
        if self.check(TokenType::Number) { let n = self.cur().value.parse().unwrap_or(0.0); self.adv(); Expr::Num(n) }
        else if self.check(TokenType::String) { let s = self.cur().value.clone(); self.adv(); Expr::Str(s) }
        else if self.check(TokenType::True) { self.adv(); Expr::Bool(true) }
        else if self.check(TokenType::False) { self.adv(); Expr::Bool(false) }
        else if self.check(TokenType::Null) { self.adv(); Expr::Null }
        else if self.check(TokenType::Ident) { let id = self.cur().value.clone(); self.adv(); Expr::Ident(id) }
        else if self.check(TokenType::LParen) { self.adv(); let e = self.expr(); if self.check(TokenType::RParen) { self.adv(); } e }
        else if self.check(TokenType::LBracket) { self.adv(); let a = self.arr(); self.adv(); a }
        else if self.check(TokenType::LBrace) { self.adv(); let o = self.obj(); self.adv(); o }
        else { Expr::Null }
    }
    
    fn args(&mut self) -> Vec<Expr> {
        let mut args = vec![];
        while !self.check(TokenType::RParen) && !self.at() {
            args.push(self.expr());
            if self.check(TokenType::Comma) { self.adv(); }
        }
        args
    }
    
    fn arr(&mut self) -> Expr {
        let mut items = vec![];
        while !self.check(TokenType::RBracket) { items.push(self.expr()); if self.check(TokenType::Comma) { self.adv(); } }
        Expr::Arr(items)
    }
    
    fn obj(&mut self) -> Expr {
        let mut fields = vec![];
        while !self.check(TokenType::RBrace) {
            if self.check(TokenType::Ident) { let k = self.cur().value.clone(); self.adv(); if self.check(TokenType::Colon) { self.adv(); fields.push((k, self.expr())); } }
            if self.check(TokenType::Comma) { self.adv(); }
        }
        Expr::Obj(fields)
    }
}

pub fn stmt_line(s: &Stmt) -> usize {
    match s {
        Stmt::Expr(_, l) | Stmt::Return(_, l)
            | Stmt::Var { line: l, .. } | Stmt::If { line: l, .. }
            | Stmt::While { line: l, .. } | Stmt::For { line: l, .. }
            | Stmt::Break(l) | Stmt::Continue(l) => *l,
    }
}

pub fn expr_to_name(e: &Expr) -> String {
    match e {
        Expr::Ident(s) => s.clone(),
        Expr::Member(obj, member) => format!("{}.{}", expr_to_name(obj), member),
        _ => String::new(),
    }
}
