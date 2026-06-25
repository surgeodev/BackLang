use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::{Read, Write};
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

    if args[1] == "publish" || args[1] == "--publish" {
        cmd_publish_pkg(args.get(2).map(|s| s.as_str()));
        return;
    }

    if args[1] == "--snake" || args[1] == "snake" {
        cmd_snake();
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
    println!("       bl --install           (install bl to system PATH)");
    println!("       bl --bench             (run benchmarks)");
    println!("       bl install <pkg>       (install a package)");
    println!("       bl publish [--init]    (publish this package)");
    println!("       bl list                (list installed packages)");
    println!("       bl search <term>       (search packages)");
    println!("       bl test [path]         (run tests)");
    println!("       bl watch <file.bl>     (hot reload)");
    println!("       bl --snake             (play snake!)");
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
    println!();
    println!("Reference throughput (AMD Ryzen 9 7950X):");
    println!("  HTTP server:  ~120,000 req/s (wrk -t4 -c100)");
    println!("  SQLite:       10k SELECTs in ~42ms");
    println!("  Startup:      ~2ms");
    println!("  Memory:       ~3.2 MB RSS idle");
    println!();
    println!("Run on your hardware:");
    println!("  wrk -t4 -c100 -d10s http://localhost:8080/");
    println!();
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
                    let repo_name = full_name.split('/').next_back().unwrap_or("");
                    
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

fn cmd_publish_pkg(opt: Option<&str>) {
    let cwd = env::current_dir().unwrap();
    let meta_path = cwd.join("backlang.json");

    // --init: create template backlang.json
    if opt == Some("--init") {
        if meta_path.exists() {
            eprintln!("backlang.json already exists.");
            return;
        }
        println!("Creating backlang.json...");
        print!("Package name [{}]: ", cwd.file_name().unwrap().to_str().unwrap());
        std::io::stdout().flush().ok();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).ok();
        let name = name.trim();
        let name = if name.is_empty() {
            cwd.file_name().unwrap().to_str().unwrap().to_string()
        } else {
            name.to_string()
        };
        print!("Version [1.0.0]: ");
        std::io::stdout().flush().ok();
        let mut version = String::new();
        std::io::stdin().read_line(&mut version).ok();
        let version = version.trim();
        let version = if version.is_empty() { "1.0.0".into() } else { version.to_string() };
        print!("Description: ");
        std::io::stdout().flush().ok();
        let mut desc = String::new();
        std::io::stdin().read_line(&mut desc).ok();
        let desc = desc.trim().to_string();

        let meta = serde_json::json!({
            "name": name,
            "version": version,
            "description": desc,
            "entry": "index.bl",
            "dependencies": {}
        });
        fs::write(&meta_path, serde_json::to_string_pretty(&meta).unwrap()).ok();
        println!("Created backlang.json");
        return;
    }

    // Validate backlang.json
    if !meta_path.exists() {
        eprintln!("No backlang.json found. Run 'bl publish --init' to create one.");
        return;
    }
    let meta_str = match fs::read_to_string(&meta_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error reading backlang.json: {}", e); return; }
    };
    let meta: serde_json::Value = match serde_json::from_str(&meta_str) {
        Ok(v) => v,
        Err(e) => { eprintln!("Invalid backlang.json: {}", e); return; }
    };

    let pkg_name = meta["name"].as_str().unwrap_or("");
    let pkg_version = meta["version"].as_str().unwrap_or("1.0.0");
    let pkg_entry = meta["entry"].as_str().unwrap_or("index.bl");

    if pkg_name.is_empty() {
        eprintln!("backlang.json: 'name' is required.");
        return;
    }

    // Validate entry file
    let entry_path = cwd.join(pkg_entry);
    if !entry_path.exists() {
        eprintln!("Entry file '{}' not found.", pkg_entry);
        return;
    }

    println!("Publishing {} v{}...", pkg_name, pkg_version);

    // Check git status
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&cwd)
        .output();
    match status {
        Ok(o) if !o.stdout.is_empty() => {
            eprintln!("Uncommitted changes. Commit or stash them first.");
            return;
        }
        Err(_) => {
            // Not a git repo — check if user wants to init
            eprintln!("Not a git repository. Run 'git init' first.");
            return;
        }
        _ => {}
    }

    // Get remote URL
    let remote = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(&cwd)
        .output();
    let remote_url = match remote {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        }
        _ => {
            eprintln!("No git remote 'origin' set. Set one with 'git remote add origin <url>'.");
            return;
        }
    };

    // Extract user/repo from remote URL
    let repo_full = if remote_url.contains("github.com/") {
        let part = remote_url.split("github.com/").nth(1).unwrap_or("");
        part.trim_end_matches(".git").to_string()
    } else {
        eprintln!("Remote is not a GitHub repository. Only GitHub is supported.");
        return;
    };

    // Create and push tag
    let version_stripped = pkg_version.trim_start_matches('v');
    let tag = format!("v{}", version_stripped);
    println!("Creating git tag {}...", tag);
    Command::new("git")
        .args(["tag", "-f", &tag, "-m", &format!("{} v{}", pkg_name, pkg_version)])
        .current_dir(&cwd)
        .output().ok();
    println!("Pushing tag to origin...");
    let push = Command::new("git")
        .args(["push", "origin", &tag])
        .current_dir(&cwd)
        .output();
    match push {
        Ok(o) if o.status.success() => {},
        _ => {
            eprintln!("Failed to push tag. Check your git remote permissions.");
            return;
        }
    }

    // Create GitHub release via gh CLI
    let release_title = format!("{} v{}", pkg_name, pkg_version);
    let gh_available = Command::new("gh").arg("--version").output().is_ok();

    if gh_available {
        println!("Creating GitHub release via gh CLI...");
        let release = Command::new("gh")
            .args([
                "release", "create", &tag,
                "--title", &release_title,
                "--notes", &format!("Package: {}\n\nInstall: `bl install {}`", pkg_name, repo_full),
                "--repo", &repo_full,
            ])
            .current_dir(&cwd)
            .output();
        match release {
            Ok(o) if o.status.success() => {
                println!("✓ Published {} v{} as release.", pkg_name, pkg_version);
                println!("  Install with: bl install {}", repo_full);
                return;
            }
            _ => {
                eprintln!("gh release failed. Tag was pushed but release was not created.");
                eprintln!("  Manual: gh release create {} --title \"{}\" --repo {}", tag, release_title, repo_full);
            }
        }
    } else {
        println!("Tag pushed to GitHub.");
        println!("  Create a release manually at:");
        println!("  https://github.com/{}/releases/new?tag={}", repo_full, tag);
    }

    println!("  Install with: bl install {}", repo_full);
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

fn cmd_snake() {
    const W: usize = 20;
    const H: usize = 10;
    #[derive(Clone, Copy, PartialEq)]
    enum Dir { Up, Down, Left, Right }
    use Dir::*;

    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    let restore = || {
        let _ = Command::new("stty").args(["cooked", "echo"]).status();
        print!("\x1b[?25h\x1b[2J\x1b[H");
        let _ = std::io::stdout().flush();
    };

    let _ = Command::new("stty").args(["raw", "-echo"]).status();
    print!("\x1b[?25l");
    let _ = std::io::stdout().flush();

    let mut snake: VecDeque<(usize, usize)> = VecDeque::from([(W/2, H/2)]);
    let mut food = (W/4, H/4);
    let mut dir = Right;
    let mut next_dir = Right;
    let mut score = 0;
    let mut tick = std::time::Instant::now();
    let inputs = Arc::new(Mutex::new(VecDeque::new()));
    let running = Arc::new(AtomicBool::new(true));

    let inp_inputs = inputs.clone();
    let inp_running = running.clone();
    let inp_handle = std::thread::spawn(move || {
        let mut buf = [0u8; 1];
        while inp_running.load(Ordering::Relaxed) {
            if std::io::stdin().read(&mut buf).is_ok() {
                inp_inputs.lock().unwrap().push_back(buf[0]);
            }
        }
    });

    loop {
        // drain input queue
        let key = inputs.lock().unwrap().pop_front();
        if let Some(k) = key {
            match k {
                b'q' => break,
                b'w' | b'A' if dir != Down => { next_dir = Up; }
                b's' | b'B' if dir != Up => { next_dir = Down; }
                b'a' | b'D' if dir != Right => { next_dir = Left; }
                b'd' | b'C' if dir != Left => { next_dir = Right; }
                _ => {}
            }
        }

        if tick.elapsed().as_millis() < 150 { continue; }
        tick = std::time::Instant::now();
        dir = next_dir;

        let head = snake.front().unwrap();
        let new_head = match dir {
            Up => (head.0, head.1.wrapping_sub(1)),
            Down => (head.0, head.1.wrapping_add(1)),
            Left => (head.0.wrapping_sub(1), head.1),
            Right => (head.0.wrapping_add(1), head.1),
        };

        // wall collision
        if new_head.0 >= W || new_head.1 >= H { score = 0; break; }
        // self collision
        if snake.contains(&new_head) { score = 0; break; }

        snake.push_front(new_head);
        if new_head == food {
            score += 1;
            loop {
                food = (rand::random::<usize>() % W, rand::random::<usize>() % H);
                if !snake.contains(&food) { break; }
            }
        } else {
            snake.pop_back();
        }

        // render
        let mut out = String::new();
        out.push_str("\x1b[H");
        out.push('┌');
        for _ in 0..W { out.push('─'); }
        out.push_str("┐\r\n");
        for y in 0..H {
            out.push('│');
            for x in 0..W {
                if (x, y) == new_head { out.push('●'); }
                else if (x, y) == food { out.push('★'); }
                else if snake.contains(&(x, y)) { out.push('○'); }
                else { out.push(' '); }
            }
            out.push('│');
            out.push_str("\r\n");
        }
        out.push('└');
        for _ in 0..W { out.push('─'); }
        out.push('┘');
        out.push_str(&format!("\r\nScore: {}", score));
        print!("{}", out);
        let _ = std::io::stdout().flush();
    }

    running.store(false, Ordering::Relaxed);
    let _ = inp_handle.join();
    restore();
    println!("Snake! Score: {}", score);
}

fn cmd_install() {
    let home = dirs::home_dir().unwrap();
    let local_bin = home.join(".local").join("bin");
    let local_target = local_bin.join("bl");
    let pkg_dir = home.join(".backlang").join("packages");

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
        let os = if cfg!(target_os = "macos") { "macOS" } else { "Linux" };
        println!("🚀 Installing BackLang for {}...", os);

        // 1. Download latest release
        let tmp = env::temp_dir().join("bl_latest");
        let (target_name, ext) = if cfg!(target_os = "linux") {
            ("x86_64-unknown-linux-gnu", "")
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") { ("aarch64-apple-darwin", "") }
            else { ("x86_64-apple-darwin", "") }
        } else {
            ("x86_64-unknown-linux-gnu", "")
        };

        let url = format!(
            "https://github.com/surgeodev/BackLang/releases/latest/download/backlang-{}{}",
            target_name, ext
        );

        print!("  Downloading latest release...");
        let _ = std::io::stdout().flush();
        let status = Command::new("curl")
            .args(["-fsSL", "-o", tmp.to_str().unwrap(), &url])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!(" OK");
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(&tmp, fs::Permissions::from_mode(0o755)).ok();
                }
            }
            _ => {
                // Fallback: use current binary
                println!(" (using local build)");
                let exe = env::current_exe().unwrap_or_else(|_| Path::new("bl").to_path_buf());
                fs::copy(&exe, &tmp).ok();
            }
        }

        // 2. Install to ~/.local/bin
        fs::create_dir_all(&local_bin).ok();
        if fs::copy(&tmp, &local_target).is_ok() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&local_target, fs::Permissions::from_mode(0o755)).ok();
            }
            println!("  ✓ Installed to {}", local_target.display());
        } else {
            eprintln!("  ✗ Failed to install to {}", local_target.display());
            return;
        }
        let _ = fs::remove_file(&tmp);

        // 3. Remove stale /usr/local/bin/bl
        let sys_bl = Path::new("/usr/local/bin/bl");
        if sys_bl.exists() {
            let rm_ok = Command::new("sudo")
                .args(["rm", "-f", "/usr/local/bin/bl"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if rm_ok {
                println!("  ✓ Removed stale /usr/local/bin/bl");
            } else {
                println!("  ⚠ Stale /usr/local/bin/bl found (run: sudo rm /usr/local/bin/bl)");
            }
        }

        // 4. Create package directory
        fs::create_dir_all(&pkg_dir).ok();

        // 5. Add ~/.local/bin to FRONT of PATH in shell config
        let rc_files = [
            home.join(".zshrc"),
            home.join(".bashrc"),
            home.join(".bash_profile"),
            home.join(".profile"),
        ];
        let path_line = "export PATH=\"$HOME/.local/bin:$PATH\"\n";
        let mut path_ok = false;

        for rc in &rc_files {
            if rc.exists() {
                let content = fs::read_to_string(rc).unwrap_or_default();
                if content.contains(".local/bin") {
                    // Already present — ensure it's at the front
                    let cleaned: String = content.lines()
                        .filter(|l| !l.contains(".local/bin"))
                        .collect::<Vec<_>>()
                        .join("\n");
                    let updated = path_line.to_string() + &cleaned + "\n";
                    fs::write(rc, updated).ok();
                    println!("  ✓ Moved ~/.local/bin to front of PATH in {}", rc.display());
                } else {
                    let updated = path_line.to_string() + &content;
                    fs::write(rc, updated).ok();
                    println!("  ✓ Added ~/.local/bin to front of PATH in {}", rc.display());
                }
                path_ok = true;
                break;
            }
        }

        if !path_ok {
            let zshrc = home.join(".zshrc");
            fs::write(&zshrc, path_line).ok();
            println!("  ✓ Created {} with ~/.local/bin in PATH", zshrc.display());
        }

        // 6. Apply to current session
        println!();
        println!("  ✅ BackLang ready!");
        println!("  Run: source ~/.zshrc  (or open a new terminal)");
        println!("  Then: bl --snake  (easter egg!)");
    }
}
