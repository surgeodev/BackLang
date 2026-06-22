use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::Mutex;

static DBG: Mutex<Option<DebugState>> = Mutex::new(None);

struct DebugState {
    breakpoints: HashMap<String, Vec<usize>>,
    state: State,
    step_depth: usize,
    call_depth: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Running,
    SteppingOver,
    SteppingIn,
    SteppingOut,
}

fn send(msg: &str) {
    eprintln!("{}", msg);
    let _ = io::stderr().flush();
}

fn read_line() -> String {
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_ok() {
        line.trim().to_string()
    } else {
        String::new()
    }
}

/// Read initial config from stdin (breakpoints).
/// Called once from main.rs before execution begins.
pub fn init() {
    let d = DBG.lock().unwrap();
    if d.is_some() { return; }
    drop(d);

    // Read lines until "start"
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut bps: HashMap<String, Vec<usize>> = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        let line = line.trim().to_string();
        if line == "start" { break; }
        if line.starts_with("setBreakpoints") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                let file = parts[1].to_string();
                let lines: Vec<usize> = parts[2].split_whitespace()
                    .filter_map(|s| s.parse().ok()).collect();
                bps.insert(file, lines);
            }
        }
    }

    let mut d = DBG.lock().unwrap();
    *d = Some(DebugState {
        breakpoints: bps,
        state: State::Running,
        step_depth: 0,
        call_depth: 0,
    });
}

/// Checkpoint called before each statement during execution.
static SOURCE_FILE: Mutex<String> = Mutex::new(String::new());

/// Set the source file being debugged
pub fn set_file(path: &str) {
    let mut f = SOURCE_FILE.lock().unwrap();
    *f = path.to_string();
}

pub fn checkpoint(_file: &str, line: usize, depth: usize) {
    // Use the actual source file path for breakpoint matching
    let source_file = SOURCE_FILE.lock().unwrap().clone();

    let should_pause = {
        let d = DBG.lock().unwrap();
        let d = match d.as_ref() { Some(d) => d, _ => return };
        match d.state {
            State::Running => {
                // Check breakpoints for both the hardcoded filename and the actual source file
                d.breakpoints.get(&source_file)
                    .map(|lines| lines.contains(&line))
                    .unwrap_or(false)
            }
            State::SteppingOver => depth <= d.step_depth,
            State::SteppingIn => true,
            State::SteppingOut => depth < d.step_depth,
        }
    };

    if !should_pause { return; }

    // Update state to Paused and notify
    {
        let mut d = DBG.lock().unwrap();
        let d = d.as_mut().unwrap();
        d.state = State::Running; // reset while we handle pause
    }

    send(&format!(
        r#"{{"event":"paused","reason":"breakpoint","file":"{}","line":{},"depth":{}}}"#,
        source_file, line, depth
    ));

    // Loop reading commands until continue/step
    loop {
        let cmd = read_line();
        let mut d = DBG.lock().unwrap();
        let d = d.as_mut().unwrap();
        match cmd.as_str() {
            "continue" | "c" => {
                d.state = State::Running;
                send(r#"{"event":"continued"}"#);
                break;
            }
            "next" | "n" => {
                d.state = State::SteppingOver;
                d.step_depth = depth;
                send(r#"{"event":"continued"}"#);
                break;
            }
            "step" | "s" => {
                d.state = State::SteppingIn;
                d.step_depth = depth;
                send(r#"{"event":"continued"}"#);
                break;
            }
            "finish" | "f" => {
                d.state = State::SteppingOut;
                d.step_depth = depth;
                send(r#"{"event":"continued"}"#);
                break;
            }
            "terminate" | "exit" => {
                send(r#"{"event":"terminated","exitCode":0}"#);
                std::process::exit(0);
            }
            _ => {
                send(&format!(r#"{{"event":"error","message":"unknown cmd: {}"}}"#, cmd));
            }
        }
    }
}

pub fn inc_depth() {
    let mut d = DBG.lock().unwrap();
    if let Some(d) = d.as_mut() { d.call_depth += 1; }
}

pub fn dec_depth() {
    let mut d = DBG.lock().unwrap();
    if let Some(d) = d.as_mut() {
        if d.call_depth > 0 { d.call_depth -= 1; }
    }
}

pub fn depth() -> usize {
    let d = DBG.lock().unwrap();
    d.as_ref().map(|d| d.call_depth).unwrap_or(0)
}


