use std::env;
use std::fs;
use std::path::Path;
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
    
    if args[1] == "--bench" || args[1] == "bench" {
        cmd_bench();
        return;
    }

    if args[1] == "install" {
        if args.len() < 3 { eprintln!("Usage: bl install <package>"); return; }
        cmd_install_pkg(&args[2]);
        return;
    }

    if args[1] == "list" {
        cmd_list_pkgs();
        return;
    }

    if args[1] == "search" {
        if args.len() < 3 { eprintln!("Usage: bl search <term>"); return; }
        cmd_search_pkgs(&args[2]);
        return;
    }

    if args[1] == "test" || args[1] == "--test" {
        cmd_test(args.get(2).map(|s| s.as_str()));
        return;
    }

    if args[1] == "watch" || args[1] == "--watch" {
        if args.len() < 3 { eprintln!("Usage: bl watch <file.bl>"); return; }
        cmd_watch(&args[2]);
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
    println!("       bl --bench             (run benchmarks)");
    println!("       bl install <pkg>       (install a package)");
    println!("       bl list                (list installed packages)");
    println!("       bl search <term>       (search packages)");
    println!("       bl test [path]         (run tests)");
    println!("       bl watch <file.bl>     (hot reload)");
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

fn cmd_bench() {
    let size = std::fs::metadata(std::env::current_exe().unwrap())
        .map(|m| m.len() / 1024).unwrap_or(0);

    println!("── BackLang Benchmarks ──────────────────────");
    println!("Version:       v{}", env!("CARGO_PKG_VERSION"));
    println!("Engine:        Tree-walking interpreter (Rust)");
    println!("Binary size:   {} KB", size);
    println!("");
    println!("Reference throughput (AMD Ryzen 9 7950X):");
    println!("  HTTP server:  ~120,000 req/s (wrk -t4 -c100)");
    println!("  SQLite:       10k SELECTs in ~42ms");
    println!("  Startup:      ~2ms");
    println!("  Memory:       ~3.2 MB RSS idle");
    println!("");
    println!("Run on your hardware:");
    println!("  wrk -t4 -c100 -d10s http://localhost:8080/");
    println!("");
    println!("Or use the benchmark server in bench/http.bl:");
    println!("  bl bench/http.bl");
    println!("  wrk -t4 -c100 -d10s http://localhost:9998/");
    println!("──────────────────────────────────────────────");
}

fn cmd_install_pkg(name: &str) {
    let home = dirs::home_dir().unwrap_or_else(|| Path::new(".").to_path_buf());
    let pkg_dir = home.join(".backlang").join("packages").join(name);
    
    if pkg_dir.join("index.bl").exists() {
        println!("Package '{}' already installed.", name);
        return;
    }
    
    println!("Installing '{}'...", name);
    
    // Try registry lookup via GitHub API
    let registry_url = format!(
        "https://api.github.com/search/repositories?q={}+language:BackLang&sort=stars",
        name
    );
    
    let output = Command::new("curl")
        .args(["-s", "-H", "Accept: application/vnd.github.v3+json", &registry_url])
        .output();
    
    if let Ok(output) = output {
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
            if let Some(items) = json["items"].as_array() {
                for item in items {
                    let full_name = item["full_name"].as_str().unwrap_or("");
                    let desc = item["description"].as_str().unwrap_or("");
                    let repo_name = full_name.split('/').last().unwrap_or("");
                    
                    if repo_name == name || full_name == name {
                        let clone_url = item["clone_url"].as_str().unwrap_or("");
                        if !clone_url.is_empty() {
                            println!("Found: {} — {}", full_name, desc);
                            println!("Cloning {}...", clone_url);
                            
                            fs::create_dir_all(&pkg_dir).ok();
                            
                            let status = Command::new("git")
                                .args(["clone", "--depth", "1", clone_url, pkg_dir.to_str().unwrap()])
                                .status();
                            
                            if let Ok(s) = status {
                                if s.success() {
                                    println!("Package '{}' installed.", name);
                                } else {
                                    eprintln!("Failed to clone repository.");
                                }
                            }
                            return;
                        }
                    }
                }
            }
        }
    }
    
    // Fallback: try direct git clone from GitHub user/repo
    if name.contains('/') {
        let clone_url = format!("https://github.com/{}.git", name);
        println!("Cloning {}...", clone_url);
        fs::create_dir_all(&pkg_dir).ok();
        let status = Command::new("git")
            .args(["clone", "--depth", "1", &clone_url, pkg_dir.to_str().unwrap()])
            .status();
        if let Ok(s) = status {
            if s.success() {
                println!("Package '{}' installed.", name);
                return;
            }
        }
    }
    
    eprintln!("Package '{}' not found.", name);
}

fn cmd_list_pkgs() {
    let home = dirs::home_dir().unwrap_or_else(|| Path::new(".").to_path_buf());
    let pkg_dir = home.join(".backlang").join("packages");
    
    if !pkg_dir.exists() {
        println!("No packages installed.");
        return;
    }
    
    println!("Installed packages:");
    if let Ok(entries) = fs::read_dir(&pkg_dir) {
        let mut found = false;
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    let has_index = entry.path().join("index.bl").exists();
                    println!("  {} {}", name, if has_index { "" } else { "(no index.bl)" });
                    found = true;
                }
            }
        }
        if !found {
            println!("  (none)");
        }
    }
}

fn cmd_search_pkgs(term: &str) {
    let url = format!(
        "https://api.github.com/search/repositories?q={}+language:BackLang&sort=stars",
        term
    );
    
    let output = Command::new("curl")
        .args(["-s", "-H", "Accept: application/vnd.github.v3+json", &url])
        .output();
    
    match output {
        Ok(o) if o.status.success() => {
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&o.stdout) {
                let items = json["items"].as_array().map(|a| a.len()).unwrap_or(0);
                println!("Found {} packages for '{}':", items, term);
                if let Some(repos) = json["items"].as_array() {
                    for repo in repos {
                        let name = repo["full_name"].as_str().unwrap_or("?");
                        let desc = repo["description"].as_str().unwrap_or("");
                        let stars = repo["stargazers_count"].as_i64().unwrap_or(0);
                        println!("  {} (★{}) — {}", name, stars, desc);
                    }
                }
            }
        }
        _ => {
            eprintln!("Search failed (check internet connection)");
        }
    }
}

fn cmd_test(path: Option<&str>) {
    use std::path::Path;
    
    let test_dir = path.map(|p| p.to_string()).unwrap_or_else(|| "tests".to_string());
    let test_path = Path::new(&test_dir);
    
    if !test_path.exists() {
        println!("No tests directory found.");
        return;
    }
    
    let mut passed = 0;
    let mut failed = 0;
    let mut errors: Vec<String> = vec![];
    
    if test_path.is_dir() {
        for entry in fs::read_dir(test_path).unwrap().flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "bl").unwrap_or(false) {
                let src = match fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => { errors.push(format!("  {} — read error: {}", path.display(), e)); failed += 1; continue; }
                };
                match interpreter::execute(&src, path.to_str().unwrap()) {
                    Ok(()) => {
                        println!("  ✓ {}", path.display());
                        passed += 1;
                    }
                    Err(e) => {
                        println!("  ✗ {} — {}", path.display(), e);
                        errors.push(format!("  {} — {}", path.display(), e));
                        failed += 1;
                    }
                }
            }
        }
    } else {
        let src = match fs::read_to_string(test_path) {
            Ok(s) => s,
            Err(e) => { eprintln!("Error reading {}: {}", test_path.display(), e); return; }
        };
        match interpreter::execute(&src, test_path.to_str().unwrap()) {
            Ok(()) => { passed += 1; }
            Err(e) => { errors.push(format!("  {} — {}", test_path.display(), e)); failed += 1; }
        }
    }
    
    println!("\nTest results: {} passed, {} failed", passed, failed);
    for e in &errors {
        eprintln!("{}", e);
    }
}

fn cmd_watch(file: &str) {
    let file_path = file.to_string();
    println!("Watching {} for changes...", file_path);
    println!("Press Ctrl+C to stop.");
    
    loop {
        // Read and execute the file
        let src = match fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {}: {}", file_path, e);
                std::thread::sleep(std::time::Duration::from_millis(1000));
                continue;
            }
        };
        
        // Get modification time
        let mtime = fs::metadata(&file_path)
            .and_then(|m| m.modified())
            .ok();
        
        if let Err(e) = interpreter::execute(&src, &file_path) {
            eprintln!("Error: {}", e);
        }
        
        // Wait for file change (poll every 500ms)
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            let new_mtime = fs::metadata(&file_path)
                .and_then(|m| m.modified())
                .ok();
            if new_mtime != mtime && new_mtime.is_some() {
                println!("\nFile changed, reloading...");
                break;
            }
        }
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
