use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: file_organizer <source>");
        return;
    }

    let source = &args[1];

    match fs::read_dir(source) {
        Ok(dir_entries) => {
            for entry in dir_entries {
                match entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.path();
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        let extension = path.extension().unwrap().to_str().unwrap();

                        let dest_path = format!("organized/{}.{}", file_name, extension);
                        match fs::create_dir_all(Path::new("organized")) {
                            Ok(_) => (),
                            Err(e) => panic!("Error creating directory: {}", e),
                        }
                        match fs::copy(&path, Path::new(&dest_path)) {
                            Ok(_) => println!("Copied file: {}", file_name),
                            Err(e) => panic!("Error copying file: {}", e),
                        }
                    }
                    Err(e) => panic!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => panic!("Error reading directory: {}", e),
    }
}