use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::sync::{Mutex, LazyLock, Arc};
use rusqlite::Connection;
use crate::parser::*;
use crate::debugger;

static DB: LazyLock<Mutex<HashMap<String, Connection>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Runtime::new().unwrap()
});

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Value>),
    Obj(HashMap<String, Value>),
    Func(FuncData),
    Task(Arc<Mutex<Option<Value>>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Num(n) => {
                if n.fract() == 0.0 { write!(f, "{}", *n as i64) }
                else { write!(f, "{}", n) }
            }
            Value::Str(s) => write!(f, "{}", s),
            Value::Arr(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Obj(o) => {
                write!(f, "{{")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::Func(_) => write!(f, "<function>"),
            Value::Task(_) => write!(f, "<task>"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Task(_), _) | (_, Value::Task(_)) => false,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncData {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub body: Vec<Stmt>,
    pub exported: bool,
    pub async_: bool,
}

#[derive(Debug, Clone)]
struct VarInfo {
    value: Value,
    mutable: bool,
    exported: bool,
}

#[derive(Debug)]
pub struct Env {
    vars: HashMap<String, VarInfo>,
    parent: Option<Box<Env>>,
}

impl Clone for Env {
    fn clone(&self) -> Self {
        Env {
            vars: self.vars.clone(),
            parent: self.parent.as_ref().map(|p| Box::new(p.as_ref().clone())),
        }
    }
}

impl Env {
    pub fn new() -> Self { Env { vars: HashMap::new(), parent: None } }
    pub fn child(parent: &Env) -> Self { Env { vars: HashMap::new(), parent: Some(Box::new(parent.clone())) } }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        self.vars.get(name).map(|vi| vi.value.clone())
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(name)))
    }

    fn set(&mut self, name: String, val: Value, mutable: bool) {
        if self.vars.contains_key(&name) {
            if let Some(vi) = self.vars.get(&name) {
                if vi.mutable { self.vars.insert(name, VarInfo { value: val, mutable, exported: vi.exported }); }
            }
        } else if let Some(p) = &mut self.parent {
            p.set(name, val, mutable);
        } else {
            self.vars.insert(name, VarInfo { value: val, mutable, exported: false });
        }
    }

    pub fn define(&mut self, name: String, val: Value, mutable: bool) {
        self.vars.insert(name, VarInfo { value: val, mutable, exported: false });
    }

    pub fn define_exported(&mut self, name: String, val: Value, mutable: bool) {
        self.vars.insert(name, VarInfo { value: val, mutable, exported: true });
    }

    pub fn define_full(&mut self, name: String, val: Value, mutable: bool, exported: bool) {
        self.vars.insert(name, VarInfo { value: val, mutable, exported });
    }

    pub fn export_symbols(&self) -> Vec<(String, Value, bool)> {
        let mut result = vec![];
        for (name, vi) in &self.vars {
            if vi.exported {
                result.push((name.clone(), vi.value.clone(), vi.mutable));
            }
        }
        if let Some(p) = &self.parent {
            result.extend(p.export_symbols());
        }
        result
    }

    fn is_mutable(&self, name: &str) -> bool {
        self.vars.get(name).map(|vi| vi.mutable)
            .unwrap_or_else(|| self.parent.as_ref().map_or(false, |p| p.is_mutable(name)))
    }
}

pub enum ExecResult {
    Value(Value),
    Return(Value),
    Break,
    Continue,
}

pub fn execute(source: &str, filename: &str) -> Result<(), String> {
    let mut lexer = crate::lexer::Lexer::new(source.to_string());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse();

    let mut env = Env::new();

    // Set debugger file
    debugger::set_file(filename);

    // Charger les imports
    for imp in &prog.imports {
        load_import(imp, &mut env)?;
    }

    for v in &prog.models {
        let val = Value::Obj(HashMap::new());
        if v.exported { env.define_exported(v.name.clone(), val, false); }
        else { env.define(v.name.clone(), val, false); }
    }
    for f in &prog.functions {
        let fd = FuncData { name: f.name.clone(), params: f.params.clone(), body: f.body.clone(), exported: f.exported, async_: f.async_ };
        if f.exported { env.define_exported(f.name.clone(), Value::Func(fd), false); }
        else { env.define(f.name.clone(), Value::Func(fd), false); }
    }

    // Register middlewares
    for mw in &prog.middlewares {
        let fd = FuncData {
            name: mw.name.clone(),
            params: vec![("req".to_string(), "any".to_string())],
            body: mw.body.clone(),
            exported: mw.exported,
            async_: false,
        };
        if mw.exported { env.define_exported(format!("_middleware_{}", mw.name), Value::Func(fd), false); }
        else { env.define(format!("_middleware_{}", mw.name), Value::Func(fd), false); }
    }

    // Si un serveur est défini, exécuter le body d'initialisation (variables, db.open, etc.)
    // puis lancer le serveur pour que les endpoints aient accès aux variables et connexions
    if !prog.servers.is_empty() {
        execute_stmts(&prog.body, &mut env)?;
    }

    // Si un serveur est défini, lancer Axum (serveur asynchrone)
    if !prog.servers.is_empty() {
        let port = prog.servers.first().map(|s| s.port).unwrap_or(3000) as u16;
        let host = prog.servers.first().map(|s| s.host.clone()).unwrap_or_else(|| "0.0.0.0".into());
        
        println!("BackLang v1.0.0 - Server '{}' on {}:{}", 
            prog.servers.first().map(|s| s.name.as_str()).unwrap_or("app"), 
            host, port);
        
        // Lancer le serveur Axum (bloquant)
        return tokio::runtime::Runtime::new()
            .map_err(|e| e.to_string())?
            .block_on(crate::server::start_server(&host, port, prog, env))
            .map_err(|e| e.to_string());
    }
    
    // Sinon exécution normale du programme
    execute_program(&prog, &mut env)
}

// Charger un module importé
fn load_import(imp: &Import, env: &mut Env) -> Result<(), String> {
    let import_path = &imp.path;
    // Modules standard (std.xxx)
    if import_path.starts_with("std.") {
        return load_std_module(import_path, env);
    }

    // Chercher d'abord dans le dossier std/ local (import "math" -> std/math.bl)
    let local_std_path = format!("std/{}.bl", import_path.replace('.', "/"));
    if Path::new(&local_std_path).exists() {
        return load_file_import(&local_std_path, imp, env);
    }

    // Fichiers locaux : import "mylib" cherche mylib.bl ou ./mylib.bl
    let file_path = format!("{}.bl", import_path.replace('.', "/"));
    let paths_to_try = vec![
        file_path.clone(),
        format!("./{}", file_path),
    ];
    
    for path in paths_to_try {
        if Path::new(&path).exists() {
            return load_file_import(&path, imp, env);
        }
    }
    
    // Chercher dans le dossier de packages installés
    let pkg_path = dirs::home_dir()
        .map(|h| h.join(".backlang/packages").join(import_path).join("index.bl"))
        .filter(|p| p.exists());
    
    if let Some(pkg_file) = pkg_path {
        return load_file_import(pkg_file.to_str().unwrap(), imp, env);
    }
    
    Err(format!("Import '{}' non trouvé", import_path))
}

fn load_file_import(file_path: &str, imp: &Import, env: &mut Env) -> Result<(), String> {
    let source = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Erreur lecture '{}': {}", file_path, e))?;

    let mut lexer = crate::lexer::Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let module_prog = parser.parse();

    // Exécuter le module dans un scope enfant
    let mut module_env = Env::child(env);
    for v in &module_prog.models {
        if v.exported { module_env.define_exported(v.name.clone(), Value::Obj(HashMap::new()), false); }
        else { module_env.define(v.name.clone(), Value::Obj(HashMap::new()), false); }
    }
    for f in &module_prog.functions {
        let fd = FuncData { name: f.name.clone(), params: f.params.clone(), body: f.body.clone(), exported: f.exported, async_: f.async_ };
        if f.exported { module_env.define_exported(f.name.clone(), Value::Func(fd), false); }
        else { module_env.define(f.name.clone(), Value::Func(fd), false); }
    }
    for mw in &module_prog.middlewares {
        let fd = FuncData {
            name: mw.name.clone(),
            params: vec![("req".to_string(), "any".to_string())],
            body: mw.body.clone(),
            exported: mw.exported,
            async_: false,
        };
        if mw.exported { module_env.define_exported(format!("_middleware_{}", mw.name), Value::Func(fd), false); }
        else { module_env.define(format!("_middleware_{}", mw.name), Value::Func(fd), false); }
    }
    execute_stmts(&module_prog.body, &mut module_env)?;

    // Copier les symboles exportés dans l'environnement courant (sauf pour 'import "x" only')
    if !imp.only {
        let alias = imp.alias.as_deref().unwrap_or("");
        for (name, val, mutable) in module_env.export_symbols() {
            let target_name = if !alias.is_empty() {
                format!("{}.{}", alias, name)
            } else {
                name
            };
            env.define(target_name, val, mutable);
        }
    }

    Ok(())
}

// Charger un module standard
fn load_std_module(module_path: &str, env: &mut Env) -> Result<(), String> {
    let module = module_path.strip_prefix("std.").unwrap_or(module_path);
    let fd = |name: &str, params: Vec<(&str, &str)>| -> FuncData {
        FuncData {
            name: name.to_string(),
            params: params.into_iter().map(|(n, t)| (n.to_string(), t.to_string())).collect(),
            body: vec![],
            exported: false,
            async_: false,
        }
    };
    match module {
        "os" => {
            env.define("std.os.getenv".to_string(), Value::Func(fd("getenv", vec![("var", "str")])), false);
            env.define("std.os.exit".to_string(), Value::Func(fd("exit", vec![("code", "num")])), false);
            env.define("std.os.args".to_string(), Value::Func(fd("args", vec![])), false);
        }
        "random" => {
            env.define("std.random.rand".to_string(), Value::Func(fd("rand", vec![])), false);
            env.define("std.random.randint".to_string(), Value::Func(fd("randint", vec![("min", "num"), ("max", "num")])), false);
        }
        "math" => {
            for f in &["sqrt", "abs", "floor", "ceil"] {
                env.define(format!("std.math.{}", f), Value::Func(fd(f, vec![("n", "num")])), false);
            }
        }
        "fs" => {
            env.define("std.fs.readFile".to_string(), Value::Func(fd("readFile", vec![("path", "str")])), false);
            env.define("std.fs.writeFile".to_string(), Value::Func(fd("writeFile", vec![("path", "str"), ("content", "str")])), false);
        }
        "string" => {
            for f in &["split", "trim", "toUpper", "toLower"] {
                env.define(format!("std.string.{}", f), Value::Func(fd(f, vec![("s", "str")])), false);
            }
        }
        "db" => {
            for name in &["open", "query", "execute"] {
                let prefixes = vec![format!("std.db.{}", name), format!("db_{}", name), format!("db.{}", name)];
                for prefix in prefixes {
                    env.define(prefix, Value::Func(fd(name, vec![])), false);
                }
            }
        }
        _ => return Err(format!("Module standard inconnu: {}", module_path)),
    }
    Ok(())
}

fn execute_program(prog: &Program, env: &mut Env) -> Result<(), String> {
    for s in &prog.middlewares { execute_stmts(&s.body, env)?; }
    for s in &prog.endpoints { execute_stmts(&s.body, env)?; }
    execute_stmts(&prog.body, env)?;
    Ok(())
}



pub fn value_to_json(v: &Value) -> String {
    match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Num(n) => {
            if n.fract() == 0.0 { format!("{}", *n as i64) }
            else { n.to_string() }
        }
        Value::Str(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        Value::Arr(a) => {
            let items: Vec<String> = a.iter().map(value_to_json).collect();
            format!("[{}]", items.join(","))
        }
        Value::Obj(o) => {
            let items: Vec<String> = o.iter().map(|(k,v)| format!("\"{}\":{}", k, value_to_json(v))).collect();
            format!("{{{}}}", items.join(","))
        }
        Value::Func(_) => "\"<function>\"".into(),
        Value::Task(_) => "\"<task>\"".into(),
    }
}

pub fn execute_stmts(stmts: &[Stmt], env: &mut Env) -> Result<ExecResult, String> {
    let mut last = Value::Null;
    for s in stmts {
        let line = stmt_line(s);
        debugger::checkpoint("main.bl", line, debugger::depth());
        match execute_stmt(s, env)? {
            ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
            ExecResult::Break => return Ok(ExecResult::Break),
            ExecResult::Continue => return Ok(ExecResult::Continue),
            ExecResult::Value(v) => last = v,
        }
    }
    Ok(ExecResult::Value(last))
}

fn execute_stmt(stmt: &Stmt, env: &mut Env) -> Result<ExecResult, String> {
    match stmt {
        Stmt::Expr(e, _) => eval_expr(e, env).map(ExecResult::Value),
        Stmt::Return(None, _) => Ok(ExecResult::Return(Value::Null)),
        Stmt::Return(Some(e), _) => Ok(ExecResult::Return(eval_expr(e, env)?)),
        Stmt::Var { name, value, mutable, exported, .. } => {
            let val = if let Some(e) = value { eval_expr(e, env)? } else { Value::Null };
            if *exported { env.define_exported(name.clone(), val, *mutable); }
            else { env.define(name.clone(), val, *mutable); }
            Ok(ExecResult::Value(Value::Null))
        }
        Stmt::Export(s, _) => {
            // Export wrapper: mark inner var/func as exported
            match s.as_ref() {
                Stmt::Var { name, value, mutable, .. } => {
                    let val = if let Some(e) = value { eval_expr(e, env)? } else { Value::Null };
                    env.define_exported(name.clone(), val, *mutable);
                    Ok(ExecResult::Value(Value::Null))
                }
                _ => execute_stmt(s, env),
            }
        }
        Stmt::Await(e, _) => {
            let val = eval_expr(e, env)?;
            match val {
                Value::Task(cell) => {
                    let result = loop {
                        let v = cell.lock().unwrap().take();
                        if let Some(v) = v { break v; }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    };
                    Ok(ExecResult::Value(result))
                }
                _ => Ok(ExecResult::Value(val)),
            }
        }
        Stmt::Spawn(e, _) => {
            let val = eval_expr(e, env)?;
            Ok(ExecResult::Value(val))
        }
        Stmt::Task { name, body, .. } => {
            let task_body = body.clone();
            let mut task_env = Env::child(env);
            let cell: Arc<Mutex<Option<Value>>> = Arc::new(Mutex::new(None));
            let cell_clone = cell.clone();
            RUNTIME.spawn(async move {
                let result = match execute_stmts(&task_body, &mut task_env) {
                    Ok(ExecResult::Return(v)) | Ok(ExecResult::Value(v)) => v,
                    _ => Value::Null,
                };
                *cell_clone.lock().unwrap() = Some(result);
            });
            let task_val = Value::Task(cell);
            env.define(name.clone(), task_val, true);
            Ok(ExecResult::Value(Value::Null))
        }
        Stmt::If { cond, then, else_, .. } => {
            if is_truthy(&eval_expr(cond, env)?) {
                execute_stmts(then, env)
            } else {
                execute_stmts(else_, env)
            }
        }
        Stmt::While { cond, body, .. } => {
            while is_truthy(&eval_expr(cond, env)?) {
                match execute_stmts(body, env)? {
                    ExecResult::Break => break,
                    ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                    _ => {}
                }
            }
            Ok(ExecResult::Value(Value::Null))
        }
        Stmt::For { var, iter, body, .. } => {
            let val = eval_expr(iter, env)?;
            match val {
                Value::Arr(a) => {
                    for item in a {
                        env.define(var.clone(), item, true);
                        match execute_stmts(body, env)? {
                            ExecResult::Break => break,
                            ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                            _ => {}
                        }
                    }
                }
                Value::Str(s) => {
                    for c in s.chars() {
                        env.define(var.clone(), Value::Str(c.to_string()), true);
                        match execute_stmts(body, env)? {
                            ExecResult::Break => break,
                            ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                            _ => {}
                        }
                    }
                }
                _ => return Err("for requires array or string".into()),
            }
            Ok(ExecResult::Value(Value::Null))
        }
        Stmt::Break(_) => Ok(ExecResult::Break),
        Stmt::Continue(_) => Ok(ExecResult::Continue),
    }
}

fn eval_expr(e: &Expr, env: &mut Env) -> Result<Value, String> {
    match e {
        Expr::Null => Ok(Value::Null),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Num(n) => Ok(Value::Num(*n)),
        Expr::Str(s) => Ok(Value::Str(s.clone())),
        Expr::Ident(name) => env.get(name).ok_or_else(|| format!("undefined: {}", name)),
        Expr::Arr(items) => {
            let mut vals = vec![];
            for i in items { vals.push(eval_expr(i, env)?); }
            Ok(Value::Arr(vals))
        }
        Expr::Obj(fields) => {
            let mut map = HashMap::new();
            for (k, v) in fields { map.insert(k.clone(), eval_expr(v, env)?); }
            Ok(Value::Obj(map))
        }
        Expr::BinOp { op, left, right } => {
            let l = eval_expr(left, env)?;
            if op == "=" {
                if let Expr::Ident(name) = &**left {
                    let r = eval_expr(right, env)?;
                    if !env.is_mutable(name) { return Err(format!("cannot assign to immutable variable: {}", name)); }
                    env.set(name.clone(), r.clone(), true);
                    return Ok(r);
                }
                if let Expr::Member(obj_expr, key) = &**left {
                    if let Expr::Ident(oname) = &**obj_expr {
                        if let Some(Value::Obj(o)) = env.get(oname) {
                            let mut no = o.clone();
                            no.insert(key.clone(), eval_expr(right, env)?);
                            env.set(oname.clone(), Value::Obj(no), true);
                            return Ok(Value::Null);
                        }
                    }
                }
                if let Expr::Index(obj_expr, idx_expr) = &**left {
                    if let Expr::Ident(name) = &**obj_expr {
                        if let Value::Arr(mut a) = eval_expr(obj_expr, env)? {
                            let i = match eval_expr(idx_expr, env)? {
                                Value::Num(n) => n as usize,
                                _ => return Err("index must be number".into()),
                            };
                            let v = eval_expr(right, env)?;
                            if i < a.len() { a[i] = v.clone(); env.set(name.clone(), Value::Arr(a), true); return Ok(v); }
                        }
                    }
                }
                return Err("invalid assignment".into());
            }
            if op == "+=" || op == "-=" || op == "*=" || op == "/=" {
                if let Expr::Ident(name) = &**left {
                    let cur = env.get(name).ok_or_else(|| format!("undefined: {}", name))?;
                    let r = eval_expr(right, env)?;
                    let res = match op.as_str() {
                        "+=" => add_values(&cur, &r)?,
                        "-=" => sub_values(&cur, &r)?,
                        "*=" => mul_values(&cur, &r)?,
                        "/=" => div_values(&cur, &r)?,
                        _ => unreachable!(),
                    };
                    env.set(name.clone(), res.clone(), true);
                    return Ok(res);
                }
            }
            let r = eval_expr(right, env)?;
            match op.as_str() {
                "+" => add_values(&l, &r),
                "-" => sub_values(&l, &r),
                "*" => mul_values(&l, &r),
                "/" => div_values(&l, &r),
                "%" => {
                    if let (Value::Num(a), Value::Num(b)) = (&l, &r) { Ok(Value::Num(a % b)) }
                    else { Err("mod needs numbers".into()) }
                }
                "==" => Ok(Value::Bool(l == r)),
                "!=" => Ok(Value::Bool(l != r)),
                "<" => cmp_values(&l, &r, |a, b| a < b, |a, b| a < b),
                ">" => cmp_values(&l, &r, |a, b| a > b, |a, b| a > b),
                "<=" => cmp_values(&l, &r, |a, b| a <= b, |a, b| a <= b),
                ">=" => cmp_values(&l, &r, |a, b| a >= b, |a, b| a >= b),
                "&&" => Ok(Value::Bool(is_truthy(&l) && is_truthy(&r))),
                "||" => Ok(Value::Bool(is_truthy(&l) || is_truthy(&r))),
                _ => Err(format!("unknown op: {}", op)),
            }
        }
        Expr::UnOp { op, val } => {
            let v = eval_expr(val, env)?;
            match op.as_str() {
                "!" => Ok(Value::Bool(!is_truthy(&v))),
                "-" => {
                    if let Value::Num(n) = v { Ok(Value::Num(-n)) }
                    else { Err("neg needs number".into()) }
                }
                _ => Err(format!("unknown unop: {}", op)),
            }
        }
        Expr::Call { callee, args } => {
            let vals: Vec<Value> = args.iter().map(|a| eval_expr(a, env)).collect::<Result<_,_>>()?;
            if callee == "print" {
                if vals.is_empty() { println!(); } else { println!("{}", vals[0]); }
                return Ok(Value::Null);
            }
            if callee == "len" {
                if vals.is_empty() { return Err("len needs 1 arg".into()); }
                return match &vals[0] {
                    Value::Str(s) => Ok(Value::Num(s.len() as f64)),
                    Value::Arr(a) => Ok(Value::Num(a.len() as f64)),
                    _ => Err("len needs string or array".into()),
                };
            }
            if callee == "push" {
                if vals.len() < 2 { return Err("push needs 2 args".into()); }
                if let Expr::Ident(name) = &args[0] {
                    if let Value::Arr(mut a) = eval_expr(&args[0], env)? {
                        a.push(vals[1].clone());
                        env.set(name.clone(), Value::Arr(a.clone()), true);
                        return Ok(Value::Arr(a));
                    }
                }
                return Err("push first arg must be array".into());
            }
            if callee == "pop" {
                if vals.is_empty() { return Err("pop needs 1 arg".into()); }
                if let Expr::Ident(name) = &args[0] {
                    if let Value::Arr(mut a) = eval_expr(&args[0], env)? {
                        let v = a.pop().unwrap_or(Value::Null);
                        env.set(name.clone(), Value::Arr(a), true);
                        return Ok(v);
                    }
                }
                return Err("pop needs array".into());
            }
            if callee == "keys" {
                if vals.is_empty() { return Err("keys needs 1 arg".into()); }
                if let Value::Obj(o) = &vals[0] {
                    return Ok(Value::Arr(o.keys().map(|k| Value::Str(k.clone())).collect()));
                }
                return Err("keys needs object".into());
            }
            if callee == "str" {
                if vals.is_empty() { return Ok(Value::Str("".into())); }
                return Ok(Value::Str(vals[0].to_string()));
            }
            if callee == "num" {
                if vals.is_empty() { return Ok(Value::Num(0.0)); }
                return match &vals[0] {
                    Value::Str(s) => s.parse().map(Value::Num).map_err(|_| "invalid num".into()),
                    Value::Num(n) => Ok(Value::Num(*n)),
                    _ => Err("cannot convert to num".into()),
                };
            }
            if callee == "type" {
                if vals.is_empty() { return Ok(Value::Str("null".into())); }
                let t = match &vals[0] {
                    Value::Null => "null", Value::Bool(_) => "bool", Value::Num(_) => "num",
                    Value::Str(_) => "str", Value::Arr(_) => "array", Value::Obj(_) => "object",
                    Value::Func(_) => "function",
                    Value::Task(_) => "task",
                };
                return Ok(Value::Str(t.into()));
            }

            if callee == "str" {
                if vals.is_empty() { return Ok(Value::Str("".into())); }
                return Ok(Value::Str(vals[0].to_string()));
            }

            // Standard library : std.os.*
            if callee.starts_with("std.os.") {
                let func = callee.strip_prefix("std.os.").unwrap();
                return match func {
                    "getenv" => {
                        let var = vals.get(0).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("getenv requires string arg")?;
                        Ok(std::env::var(var).map(Value::Str).unwrap_or(Value::Null))
                    }
                    "exit" => {
                        let code = vals.get(0).and_then(|v| if let Value::Num(n) = v { Some(*n as i32) } else { None })
                            .unwrap_or(0);
                        std::process::exit(code);
                    }
                    "args" => {
                        let args: Vec<Value> = std::env::args().map(Value::Str).collect();
                        Ok(Value::Arr(args))
                    }
                    _ => Err(format!("unknown std.os function: {}", func)),
                };
            }

            // Standard library : std.random.*
            if callee.starts_with("std.random.") {
                let func = callee.strip_prefix("std.random.").unwrap();
                return match func {
                    "rand" => Ok(Value::Num(rand::random::<f64>())),
                    "randint" => {
                        let min = vals.get(0).and_then(|v| if let Value::Num(n) = v { Some(*n as i64) } else { None })
                            .unwrap_or(0);
                        let max = vals.get(1).and_then(|v| if let Value::Num(n) = v { Some(*n as i64) } else { None })
                            .unwrap_or(100);
                        let r = rand::random::<i64>().abs() % (max - min + 1) + min;
                        Ok(Value::Num(r as f64))
                    }
                    _ => Err(format!("unknown std.random function: {}", func)),
                };
            }

            // Standard library : std.math.*
            if callee.starts_with("std.math.") {
                let func = callee.strip_prefix("std.math.").unwrap();
                let n = vals.get(0).and_then(|v| if let Value::Num(n) = v { Some(*n) } else { None })
                    .ok_or("math function requires number arg")?;
                return match func {
                    "sqrt" => Ok(Value::Num(n.sqrt())),
                    "abs" => Ok(Value::Num(n.abs())),
                    "floor" => Ok(Value::Num(n.floor())),
                    "ceil" => Ok(Value::Num(n.ceil())),
                    _ => Err(format!("unknown std.math function: {}", func)),
                };
            }

            // Standard library : std.fs.*
            if callee.starts_with("std.fs.") {
                let func = callee.strip_prefix("std.fs.").unwrap();
                return match func {
                    "readFile" => {
                        let path = vals.get(0).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("readFile requires path string")?;
                        std::fs::read_to_string(path)
                            .map(Value::Str)
                            .map_err(|e| format!("readFile error: {}", e))
                    }
                    "writeFile" => {
                        let path = vals.get(0).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("writeFile requires path string")?;
                        let content = vals.get(1).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("writeFile requires content string")?;
                        std::fs::write(path, content)
                            .map(|_| Value::Null)
                            .map_err(|e| format!("writeFile error: {}", e))
                    }
                    _ => Err(format!("unknown std.fs function: {}", func)),
                };
            }

            // Standard library : std.string.*
            if callee.starts_with("std.string.") {
                let func = callee.strip_prefix("std.string.").unwrap();
                let s = vals.get(0).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                    .ok_or("string function requires string arg")?;
                return match func {
                    "split" => {
                        let sep = vals.get(1).and_then(|v| if let Value::Str(s) = v { Some(s.as_str()) } else { None })
                            .unwrap_or("");
                        let parts: Vec<Value> = s.split(sep).map(|p| Value::Str(p.to_string())).collect();
                        Ok(Value::Arr(parts))
                    }
                    "trim" => Ok(Value::Str(s.trim().to_string())),
                    "toUpper" => Ok(Value::Str(s.to_uppercase())),
                    "toLower" => Ok(Value::Str(s.to_lowercase())),
                    _ => Err(format!("unknown std.string function: {}", func)),
                };
            }
            // Standard library : std.db.* / db_* / db.*
            if callee.starts_with("std.db.") || callee.starts_with("db_") || callee.starts_with("db.") {
                let func = if callee.starts_with("std.db.") { callee.strip_prefix("std.db.").unwrap().to_string() }
                           else if callee.starts_with("db_") { callee.strip_prefix("db_").unwrap().to_string() }
                           else { callee.strip_prefix("db.").unwrap().to_string() };
                return match func.as_str() {
                    "open" => {
                        let path = vals.get(0).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("db.open requires a path string")?;
                        let conn = Connection::open(&path).map_err(|e| e.to_string())?;
                        DB.lock().unwrap().insert(path.clone(), conn);
                        Ok(Value::Str(path))
                    }
                    "query" => {
                        let path = vals.get(0).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("db.query requires path string")?;
                        let sql = vals.get(1).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("db.query requires SQL string")?;
                        let db = DB.lock().unwrap();
                        let conn = db.get(&path).ok_or("Database not opened. Call db.open() first")?;
                        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
                        let col_count = stmt.column_count();
                        let col_names: Vec<String> = (0..col_count).map(|i| stmt.column_name(i).unwrap_or("?").to_string()).collect();
                        let mut results: Vec<Value> = vec![];
                        let rows = stmt.query_map([], |row| {
                            let mut map = HashMap::new();
                            for i in 0..col_count {
                                match row.get::<_, rusqlite::types::Value>(i) {
                                    Ok(rusqlite::types::Value::Null) => { map.insert(col_names[i].clone(), Value::Null); }
                                    Ok(rusqlite::types::Value::Integer(n)) => { map.insert(col_names[i].clone(), Value::Num(n as f64)); }
                                    Ok(rusqlite::types::Value::Real(f)) => { map.insert(col_names[i].clone(), Value::Num(f)); }
                                    Ok(rusqlite::types::Value::Text(s)) => { map.insert(col_names[i].clone(), Value::Str(s)); }
                                    _ => { map.insert(col_names[i].clone(), Value::Null); }
                                }
                            }
                            Ok(map)
                        }).map_err(|e| e.to_string())?;
                        for row in rows {
                            results.push(Value::Obj(row.map_err(|e| e.to_string())?));
                        }
                        Ok(Value::Arr(results))
                    }
                    "execute" => {
                        let path = vals.get(0).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("db.execute requires path string")?;
                        let sql = vals.get(1).and_then(|v| if let Value::Str(s) = v { Some(s.clone()) } else { None })
                            .ok_or("db.execute requires SQL string")?;
                        let db = DB.lock().unwrap();
                        let conn = db.get(&path).ok_or("Database not opened. Call db.open() first")?;
                        let count = conn.execute(&sql, []).map_err(|e| e.to_string())?;
                        Ok(Value::Num(count as f64))
                    }
                    _ => Err(format!("unknown std.db function: {}", func)),
                };
            }
            if let Some(Value::Func(fd)) = env.get(callee) {
                if fd.async_ {
                    // Async call — spawn as task
                    let body = fd.body.clone();
                    let mut task_env = Env::child(env);
                    for (i, (pname, _)) in fd.params.iter().enumerate() {
                        task_env.define(pname.clone(), vals.get(i).cloned().unwrap_or(Value::Null), false);
                    }
                    let cell: Arc<Mutex<Option<Value>>> = Arc::new(Mutex::new(None));
                    let cell_clone = cell.clone();
                    RUNTIME.spawn(async move {
                        let result = match execute_stmts(&body, &mut task_env) {
                            Ok(ExecResult::Return(v)) | Ok(ExecResult::Value(v)) => v,
                            _ => Value::Null,
                        };
                        *cell_clone.lock().unwrap() = Some(result);
                    });
                    Ok(Value::Task(cell))
                } else {
                    let mut scope = Env::child(env);
                    for (i, (pname, _)) in fd.params.iter().enumerate() {
                        scope.define(pname.clone(), vals.get(i).cloned().unwrap_or(Value::Null), false);
                    }
                    debugger::inc_depth();
                    let result = match execute_stmts(&fd.body, &mut scope)? {
                        ExecResult::Return(v) => Ok(v),
                        _ => Ok(Value::Null),
                    };
                    debugger::dec_depth();
                    result
                }
            } else {
                Err(format!("undefined function: {}", callee))
            }
        }
        Expr::Member(obj_expr, key) => {
            if let Expr::Ident(name) = &**obj_expr {
                // Try as an object first
                if let Some(Value::Obj(o)) = env.get(name) {
                    return Ok(o.get(key).cloned().unwrap_or(Value::Null));
                }
                // Try as a qualified name (alias.exported_name)
                let qualified = format!("{}.{}", name, key);
                if let Some(v) = env.get(&qualified) {
                    return Ok(v);
                }
            }
            Err("invalid member access".into())
        }
        Expr::Spawn(e) => {
            let expr = e.as_ref();
            if let Expr::Call { callee, args } = expr {
                let vals: Vec<Value> = args.iter().map(|a| eval_expr(a, env)).collect::<Result<_,_>>()?;
                if let Some(Value::Func(fd)) = env.get(callee) {
                    let body = fd.body.clone();
                    let mut task_env = Env::child(env);
                    for (i, (pname, _)) in fd.params.iter().enumerate() {
                        task_env.define(pname.clone(), vals.get(i).cloned().unwrap_or(Value::Null), false);
                    }
                    let cell: Arc<Mutex<Option<Value>>> = Arc::new(Mutex::new(None));
                    let cell_clone = cell.clone();
                    RUNTIME.spawn(async move {
                        let result = match execute_stmts(&body, &mut task_env) {
                            Ok(ExecResult::Return(v)) | Ok(ExecResult::Value(v)) => v,
                            _ => Value::Null,
                        };
                        *cell_clone.lock().unwrap() = Some(result);
                    });
                    return Ok(Value::Task(cell));
                }
                return Err(format!("spawn: undefined function '{}'", callee));
            }
            Err("spawn requires a function call".into())
        }
        Expr::Await(e) => {
            let val = eval_expr(e, env)?;
            match val {
                Value::Task(cell) => {
                    let result = loop {
                        let v = cell.lock().unwrap().take();
                        if let Some(v) = v { break v; }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    };
                    Ok(result)
                }
                _ => Ok(val),
            }
        }
        Expr::Index(obj_expr, idx_expr) => {
            let val = eval_expr(obj_expr, env)?;
            let i = eval_expr(idx_expr, env)?;
            match (&val, &i) {
                (Value::Arr(a), Value::Num(n)) => {
                    let ix = *n as usize;
                    Ok(a.get(ix).cloned().unwrap_or(Value::Null))
                }
                (Value::Str(s), Value::Num(n)) => {
                    let ix = *n as usize;
                    Ok(s.chars().nth(ix).map(|c| Value::Str(c.to_string())).unwrap_or(Value::Null))
                }
                _ => Err("invalid index".into()),
            }
        }
    }
}

fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Num(n) => *n != 0.0,
        Value::Str(s) => !s.is_empty(),
        _ => true,
    }
}

fn add_values(a: &Value, b: &Value) -> Result<Value, String> {
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => Ok(Value::Num(x + y)),
        (Value::Str(x), Value::Str(y)) => Ok(Value::Str(x.clone() + y)),
        (Value::Arr(x), Value::Arr(y)) => {
            let mut a = x.clone(); a.extend(y.clone()); Ok(Value::Arr(a))
        }
        (Value::Str(x), _) => Ok(Value::Str(x.clone() + &b.to_string())),
        (_, Value::Str(y)) => Ok(Value::Str(a.to_string() + y)),
        _ => Err("invalid add".into()),
    }
}

fn sub_values(a: &Value, b: &Value) -> Result<Value, String> {
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => Ok(Value::Num(x - y)),
        _ => Err("invalid sub".into()),
    }
}

fn mul_values(a: &Value, b: &Value) -> Result<Value, String> {
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => Ok(Value::Num(x * y)),
        _ => Err("invalid mul".into()),
    }
}

fn div_values(a: &Value, b: &Value) -> Result<Value, String> {
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => {
            if *y == 0.0 { Err("division by zero".into()) }
            else { Ok(Value::Num(x / y)) }
        }
        _ => Err("invalid div".into()),
    }
}

fn cmp_values<F, G>(a: &Value, b: &Value, fn_num: F, fn_str: G) -> Result<Value, String>
where F: Fn(f64, f64) -> bool, G: Fn(&str, &str) -> bool
{
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => Ok(Value::Bool(fn_num(*x, *y))),
        (Value::Str(x), Value::Str(y)) => Ok(Value::Bool(fn_str(x, y))),
        _ => Err("invalid comparison".into()),
    }
}
