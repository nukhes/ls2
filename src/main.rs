use std::fs;
use std::env;

fn main() {
    let root_path = env::current_dir().expect("ERROR: Unable to get current directory");
    let root_path_str = root_path.to_str().unwrap();
    list_dir(root_path_str, 0);
    println!("{}", root_path_str);
}

fn list_dir(path: &str, level: usize) {
    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(_) => return,
    };

    for entry in paths {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let entry_path = entry.path();
        let entry_name = entry.file_name();

        println!("{}{}", "  ".repeat(level), entry_name.to_string_lossy());

        if entry_path.is_dir() {
            list_dir(&entry_path.to_string_lossy(), level + 1);
        }
    }
}
