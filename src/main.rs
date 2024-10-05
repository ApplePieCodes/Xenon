use std::{env, fs::{self, File}, io::Write, path::{Path, PathBuf}};


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        print_info();
        return;
    }

    if args.len() > 1 {
        if args[1] == "new" {
            make_project(args);
        }
    }
}

fn make_file(path: PathBuf) -> File {
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => return file,
    };
}

fn make_project(args: Vec<String>) {
    let project_dir: PathBuf = if args.len() > 2 {
        // Get the base path from args and current directory as PathBuf
        let base_path = Path::new(&args[2]);
        let current_path = env::current_dir().unwrap_or_else(|_| {
            eprintln!("Failed to get current directory");
            PathBuf::from(".")
        });
        current_path.join(base_path)  // Combine the paths
    } else {
        env::current_dir().unwrap_or_else(|_| {
            eprintln!("Failed to get current directory");
            PathBuf::from(".")  // Use PathBuf for the fallback
        })
    };

    if ! fs::exists(project_dir.clone()).unwrap() {
        fs::create_dir(project_dir.clone()).unwrap();
    }

    let mut project_file = make_file(project_dir.join(Path::new("Project.toml")));
    project_file.write_fmt(format_args!("[project]\nname = \"{}\"\nversion = \"0.0.1\"", args[2])).unwrap();
    fs::create_dir(project_dir.join(Path::new("src"))).unwrap();
    let mut main_file = make_file(project_dir.join(Path::new("src/main.xe")));
    main_file.write_fmt(format_args!("use std;\n\nnamespace {} {{\n    void main() {{\n    \n    }}\n}}", args[2])).unwrap();

    println!("Project {} created", args[2]);
}


fn print_info() {
    println!("Xenon Toolchain Version {}", env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Created by {}", env!("CARGO_PKG_AUTHORS"));
    println!("Source Code at {}", env!("CARGO_PKG_REPOSITORY"));
}