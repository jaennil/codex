use std::io::Write;
use std::{env, path::Path, process};
use std::fs::{self, File};

const EXTENSIONS: [&str; 9] = ["py", "cs", "csproj", "axaml", "xaml", "rs", "lua", "js", "json"];

fn main() {
    let args: Vec<String> = env::args().collect();

    check_args(&args);

    let path_arg = &args[1];
    let path = Path::new(path_arg);

    let code = File::create("code").unwrap();
    walk_dir(path, &code);

}

fn walk_dir(path: &Path, mut output: &File) {
    let read_dir = fs::read_dir(path).unwrap();

    for dir_entry in read_dir {
        let dir_entry = dir_entry.unwrap();
        let file_type = dir_entry.file_type().unwrap();
        let dir_entry_path = dir_entry.path();
        if file_type.is_dir() {
            walk_dir(&dir_entry_path, output);
        } else if file_type.is_file() {
            let extension = dir_entry_path.extension();
            if extension.is_none() {
                continue;
            }
            let extension = extension.unwrap();
            if EXTENSIONS.contains(&extension.to_str().unwrap()) {
                let contents = fs::read_to_string(&dir_entry_path).unwrap();
                writeln!(output, "{}", dir_entry_path.display());
                writeln!(output, "{}\n", contents);
            }
        }
    }
}

fn check_args(args: &Vec<String>) {
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        process::exit(1);
    }
}
