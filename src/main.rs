use clap::Parser;
use core::panic;
use std::fs::{self, File};

/// Command line tool for C projects
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the project
    project_name: String,
}

#[derive(Clone)]
struct Cinit(Args);

impl Cinit {
    fn new() -> Self {
        let cinit = Cinit(Args::parse());
        let _ = fs::create_dir(cinit.clone().0.project_name);

        cinit
    }

    fn create_sub_directory(&self, sub_dir: &str) {
        let full_path = format!("./{}/{}", &self.0.project_name, &sub_dir);

        match fs::create_dir_all(&full_path) {
            Ok(_) => println!("dir: {} created", full_path),
            Err(e) => panic!("{e}"),
        }
    }

    fn create_file(&self, sub_dir: &str, name: &str) {
        let file_dir = if sub_dir.is_empty() {
            format!("./{}/{}", self.0.project_name, name)
        } else {
            format!("./{}/{}/{}", self.0.project_name, sub_dir, name)
        };

        match File::create(&file_dir) {
            Ok(_) => println!("File: {} created", file_dir),
            Err(e) => eprintln!("Error creating file: {} - {}", file_dir, e),
        }

        // Writting deafult code
        if name == "main.c" {
            self.write_main_c(&file_dir);
        } else if name == "Makefile" {
            self.write_makefile(&file_dir);
        } else if name == "README.md" {
            self.write_readme(&file_dir);
        } else {
            self.write_default_content(&file_dir);
        }
    }

    fn write_main_c(&self, file_dir: &str) {
        let content = "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, world!\\n\");\n    return 0;\n}\n";
        let _ = fs::write(file_dir, content);
        println!("Content written to {}", file_dir);
    }

    fn write_makefile(&self, file_dir: &str) {
        let content = "CC = gcc\nCFLAGS = -Wall\n\nmain:\n\t$(CC) $(CFLAGS) -o main ./src/main.c\n";
        let _ = fs::write(file_dir, content);
        println!("Content written to {}", file_dir);
    }

    fn write_readme(&self, file_dir: &str) {
        let content = format!(
            "# {}\n\nA simple C project generated with cinit.\n\n## Structure\n\n- `main.c`: Entry point of the program.\n- `Makefile`: Build system configuration.\n- `README.md`: Project description.\n\n## Build\n\nTo build the project, run:\n\n```bash\nmake\n```\n\n## Run\n\nAfter building, run the program:\n\n```bash\n./main\n```\n",
            self.0.project_name
        );
        let _ = fs::write(file_dir, content);
        println!("README.md created and content written to {}", file_dir);
    }

    fn write_default_content(&self, file_dir: &str) {
        let content = "// Default content\n";
        let _ = fs::write(file_dir, content);
        println!("Content written to {}", file_dir);
    }
}

fn create_project() {
    let cinit = Cinit::new();

    cinit.create_sub_directory("src");
    cinit.create_file("src", "main.c");
    cinit.create_file("", "README.md");
    cinit.create_file("", "Makefile");
}

fn main() {
    create_project();
}
