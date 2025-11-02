use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::{env, path::Path, process};
use std::fs::{self, File};
use copypasta::{ClipboardContext, ClipboardProvider as _};

const EXTENSIONS: [&str; 12] = ["go", "py", "cs", "csproj", "axaml", "xaml", "rs", "lua", "js", "json", "txt", "ipynb"];

fn main() {
    println!("[INFO] Starting code aggregator");
    
    let args: Vec<String> = env::args().collect();
    let (path_arg, exclude_folders) = check_args(&args);
    
    println!("[INFO] Target path: {}", path_arg);
    if !exclude_folders.is_empty() {
        println!("[INFO] Excluding folders: {:?}", exclude_folders);
    }
    
    let path = Path::new(&path_arg);
    
    if !path.exists() {
        eprintln!("[ERROR] Path does not exist: {}", path_arg);
        process::exit(1);
    }
    
    println!("[INFO] Creating output file 'code'");
    let code = match File::create("code") {
        Ok(f) => {
            println!("[SUCCESS] Output file created");
            f
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to create output file: {}", e);
            process::exit(1);
        }
    };
    
    println!("[INFO] Walking directory tree...");
    let file_count = walk_dir(path, &code, &exclude_folders);
    println!("[SUCCESS] Processed {} files", file_count);
    
    println!("[INFO] Initializing clipboard context");
    let mut ctx = match ClipboardContext::new() {
        Ok(c) => {
            println!("[SUCCESS] Clipboard context initialized");
            c
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to initialize clipboard: {}", e);
            process::exit(1);
        }
    };
    
    println!("[INFO] Reading aggregated code from file");
    let content = match fs::read_to_string("code") {
        Ok(c) => {
            println!("[SUCCESS] Read {} bytes", c.len());
            c
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to read output file: {}", e);
            process::exit(1);
        }
    };
    
    println!("[INFO] Copying content to clipboard");
    match ctx.set_contents(content) {
        Ok(_) => println!("[SUCCESS] Content copied to clipboard"),
        Err(e) => {
            eprintln!("[ERROR] Failed to copy to clipboard: {}", e);
            process::exit(1);
        }
    }
    
    println!("code copied to clipboard and will be there for 10 seconds");
    for i in (0..10).rev() {
        sleep(Duration::new(1, 0));
        println!("{}", i);
    }
    
    println!("[INFO] Program completed successfully");
}

fn walk_dir(path: &Path, mut output: &File, exclude_folders: &[String]) -> usize {
    let mut file_count = 0;
    
    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!("[WARN] Failed to read directory {}: {}", path.display(), e);
            return file_count;
        }
    };
    
    for dir_entry in read_dir {
        let dir_entry = match dir_entry {
            Ok(de) => de,
            Err(e) => {
                eprintln!("[WARN] Failed to read directory entry: {}", e);
                continue;
            }
        };
        
        let file_type = match dir_entry.file_type() {
            Ok(ft) => ft,
            Err(e) => {
                eprintln!("[WARN] Failed to get file type for {}: {}", dir_entry.path().display(), e);
                continue;
            }
        };
        
        let dir_entry_path = dir_entry.path();
        
        if file_type.is_dir() {
            let folder_name = dir_entry_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            
            if exclude_folders.contains(&folder_name.to_string()) {
                println!("[INFO] Skipping excluded folder: {}", dir_entry_path.display());
                continue;
            }
            
            println!("[DEBUG] Entering directory: {}", dir_entry_path.display());
            file_count += walk_dir(&dir_entry_path, output, exclude_folders);
        } else if file_type.is_file() {
            let extension = dir_entry_path.extension();
            if extension.is_none() {
                continue;
            }
            
            let extension = extension.unwrap();
            if EXTENSIONS.contains(&extension.to_str().unwrap()) {
                println!("[INFO] Processing file: {}", dir_entry_path.display());
                
                let contents = match fs::read_to_string(&dir_entry_path) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("[WARN] Failed to read file {}: {}", dir_entry_path.display(), e);
                        continue;
                    }
                };
                
                match writeln!(output, "{}", dir_entry_path.display()) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("[ERROR] Failed to write path to output: {}", e);
                        continue;
                    }
                }
                
                match writeln!(output, "{}\n", contents) {
                    Ok(_) => {
                        println!("[SUCCESS] Added {} ({} bytes)", dir_entry_path.display(), contents.len());
                        file_count += 1;
                    }
                    Err(e) => {
                        eprintln!("[ERROR] Failed to write contents to output: {}", e);
                    }
                }
            }
        }
    }
    
    file_count
}

fn check_args(args: &Vec<String>) -> (String, Vec<String>) {
    if args.len() < 2 {
        eprintln!("[ERROR] Missing required argument");
        eprintln!("Usage: {} <path> [--exclude folder1 folder2 ...]", args[0]);
        eprintln!("Example: {} ./myproject --exclude target node_modules .git", args[0]);
        process::exit(1);
    }
    
    let path = args[1].clone();
    let mut exclude_folders = Vec::new();
    
    if let Some(exclude_pos) = args.iter().position(|arg| arg == "--exclude") {
        exclude_folders = args[exclude_pos + 1..].to_vec();
        println!("[INFO] Found {} folders to exclude", exclude_folders.len());
    }
    
    println!("[INFO] Arguments validated");
    (path, exclude_folders)
}
