use std::env;
use std::fs;
use std::process::Command;
mod lexer;
mod parser;
mod interpreter;
pub mod server;
mod debugger;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        return;
    }
    
    if args[1] == "--version" || args[1] == "-v" {
        println!("BackLang v{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    if args[1] == "--update" || args[1] == "-u" {
        cmd_update();
        return;
    }
    
    if args[1] == "--install" {
        cmd_install();
        return;
    }
    
    let debug_mode = args[1] == "--debug";
    let check_only = args[1] == "--check";
    let file = if check_only || debug_mode { &args[2] } else { &args[1] };
    let src = fs::read_to_string(file).unwrap_or_else(|_| { eprintln!("Cannot read file: {}", file); std::process::exit(1); });
    
    if check_only {
        let tokens = lexer::Lexer::new(src).tokenize();
        let mut p = parser::Parser::new(tokens);
        p.parse();
        println!("OK");
        return;
    }
    
    if debug_mode {
        debugger::init();
    }
    
    if let Err(e) = interpreter::execute(&src, file) {
        eprintln!("Error: {}", e);
    }
}

fn print_usage() {
    println!("BackLang v{} — Independent", env!("CARGO_PKG_VERSION"));
    println!("Usage: bl <file.bl>");
    println!("       bl --check <file.bl>  (parse only, no execution)");
    println!("       bl --debug <file.bl>  (debug mode)");
    println!("       bl --version           (show version)");
    println!("       bl --update            (update to latest release)");
    println!("       bl --install           (add to PATH on Windows)");
}

fn cmd_update() {
    let current = env!("CARGO_PKG_VERSION");
    
    println!("Checking for updates... (current v{})", current);
    
    let output = match Command::new("curl")
        .args(["-s", "https://api.github.com/repos/surgeodev/BackLang/releases/latest"])
        .output()
    {
        Ok(o) => o,
        Err(e) => { eprintln!("Error: curl not found ({})", e); return; }
    };
    
    if !output.status.success() {
        eprintln!("Error: failed to fetch latest release (check your internet)");
        return;
    }
    
    let json: serde_json::Value = match serde_json::from_slice(&output.stdout) {
        Ok(v) => v,
        Err(_) => { eprintln!("Error: invalid response from GitHub"); return; }
    };
    
    let latest_tag = json["tag_name"].as_str().unwrap_or("v0.0.0");
    let latest = latest_tag.trim_start_matches('v');
    
    if latest == current {
        println!("Already up to date (v{})", current);
        return;
    }
    
    println!("Updating BackLang v{} -> {}...", current, latest_tag);
    
    let (target, ext) = if cfg!(target_os = "linux") {
        ("x86_64-unknown-linux-gnu", "")
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") { ("aarch64-apple-darwin", "") }
        else { ("x86_64-apple-darwin", "") }
    } else if cfg!(target_os = "windows") {
        ("x86_64-pc-windows-msvc", ".exe")
    } else {
        eprintln!("Unsupported platform"); return;
    };
    
    let tmp = env::temp_dir().join(format!("bl_new{}", ext));
    let url = format!(
        "https://github.com/surgeodev/BackLang/releases/latest/download/backlang-{}{}",
        target, ext
    );
    
    let status = Command::new("curl")
        .args(["-fsSLo", tmp.to_str().unwrap(), &url])
        .status();
    
    match status {
        Ok(s) if s.success() => {},
        _ => { eprintln!("Download failed"); return; }
    }
    
    let current_exe = match env::current_exe() {
        Ok(p) => p,
        Err(_) => { eprintln!("Cannot determine executable path"); return; }
    };
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&tmp, fs::Permissions::from_mode(0o755)).ok();
        if fs::rename(&tmp, &current_exe).is_ok() {
            println!("Updated to {}!", latest_tag);
            return;
        }
        // Fallback: try with sudo cp
        let status = Command::new("sudo")
            .args(["cp", tmp.to_str().unwrap(), current_exe.to_str().unwrap()])
            .status();
        if let Ok(s) = status { if s.success() {
            fs::remove_file(&tmp).ok();
            println!("Updated to {}!", latest_tag);
            return;
        }}
        eprintln!("Update failed: permission denied. Try: sudo bl --update");
    }
    
    #[cfg(windows)]
    {
        let bat = env::temp_dir().join("bl_update.bat");
        let script = format!(
            "@echo off\ntimeout /t 2 /nobreak >nul\ncopy /y \"{}\" \"{}\" >nul\ndel \"{}\"\n",
            tmp.to_str().unwrap(),
            current_exe.to_str().unwrap(),
            tmp.to_str().unwrap()
        );
        fs::write(&bat, script).ok();
        Command::new("cmd").args(["/c", "start", "/b", bat.to_str().unwrap()]).spawn().ok();
        println!("Updated to {}! Restart your terminal.", latest_tag);
    }
}

fn cmd_install() {
    #[cfg(windows)]
    {
        let exe = match env::current_exe() {
            Ok(p) => p,
            Err(_) => { eprintln!("Cannot determine executable path"); return; }
        };
        let dir = exe.parent().unwrap().to_str().unwrap();
        let ps = format!(
            "$path = [Environment]::GetEnvironmentVariable('Path', 'User'); \
             if ($path -notlike '*{}*') {{ \
                 [Environment]::SetEnvironmentVariable('Path', \"$path;{}\", 'User'); \
                 Write-Host 'Added {} to PATH' \
             }} else {{ \
                 Write-Host 'Already in PATH' \
             }}",
            dir.replace('\'', "''"),
            dir.replace('\'', "''"),
            dir
        );
        let status = Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps])
            .status();
        match status {
            Ok(s) if s.success() => {
                println!("bl is now in your PATH. Restart your terminal.");
            }
            _ => { eprintln!("Failed to add to PATH. Try running as Administrator."); }
        }
    }
    
    #[cfg(not(windows))]
    {
        println!("On Linux/macOS, bl is installed to /usr/local/bin which is already in PATH.");
        println!("If not, run: export PATH=\"$PATH:$(dirname $(which bl))\"");
    }
}
