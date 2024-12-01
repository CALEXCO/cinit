use clap::Parser;
use std::fs::{self, File};

/// Command line tool for C projects
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the project
    project_name: String,

    /// Create lib directory whith files named <FILE>.c and <FILE>.h
    #[arg(long, value_name = "FILE")]
    lib: Option<String>,
}

#[derive(Clone)]
struct Cinit(Args);

impl Cinit {
    fn new() -> Self {
        let cinit = Cinit(Args::parse());
        let _ = fs::create_dir(cinit.clone().0.project_name);

        cinit
    }

    fn create_sub_directory(&self, sub_dir: &str) -> Result<(), std::io::Error> {
        let full_path = format!("./{}/{}", &self.0.project_name, &sub_dir);
        fs::create_dir_all(&full_path)?;
        Ok(())
    }

    fn create_file(&self, sub_dir: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_dir = if sub_dir.is_empty() {
            format!("./{}/{}", self.0.project_name, name)
        } else {
            format!("./{}/{}/{}", self.0.project_name, sub_dir, name)
        };

        File::create(&file_dir)?;

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

        Ok(())
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

    fn create_lib_folder(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.create_sub_directory("lib")?;
        self.create_file("lib", format!("{file_name}.c").as_str())?;
        self.create_file("lib", format!("{file_name}.h").as_str())?;
        Ok(())
    }
}

fn create_project() -> Result<(), Box<dyn std::error::Error>> {
    let cinit = Cinit::new();

    cinit.create_sub_directory("src")?;

    if let Some(file_name) = &cinit.0.lib {
        cinit.create_lib_folder(file_name)?;
    }

    cinit.create_file("src", "main.c")?;
    cinit.create_file("", "README.md")?;
    cinit.create_file("", "Makefile")?;

    Ok(())
}

fn main() {
    match create_project() {
        Ok(_) => println!("Enjoy coding :)"),
        Err(err) => eprintln!("ERROR: {}", err),
    }
}
