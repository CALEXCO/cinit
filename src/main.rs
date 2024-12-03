use clap::{Parser, Subcommand};
use std::{
    fs::{self, File},
    process::Command,
};

/// Command line tool for C projects
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the project
    // project_name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Create new C Proejct with name
    CNew {
        project_name: String,

        /// Create lib directory whith files named <FILE>.c and <FILE>.h
        #[arg(long, value_name = "FILE")]
        lib: Option<String>,
    },

    /// Build ./src/*.c files and moved to ./bin/
    CBuild,

    /// Build and run project
    CBulidRun,
}

#[derive(Clone)]
struct Cinit(Args);

impl Cinit {
    fn new() -> Self {
        Cinit(Args::parse())
    }
}

fn c_new(project_name: &String, lib: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    create_sub_directory(project_name, "src")?;

    if let Some(lib_file_name) = lib {
        create_lib_folder(project_name, &lib_file_name)?;
    }

    create_file(project_name, "src", "main.c")?;
    create_file(project_name, "", "README.md")?;
    create_file(project_name, "", "Makefile")?;

    Ok(())
}

fn c_build() -> Result<(), std::io::Error> {
    // Ejecutar GCC para compilar el programa
    let gcc_status = Command::new("gcc")
        .args(["src/main.c", "-Wall", "-o", "main"])
        .status()
        .expect("Failed to execute GCC");

    if let Ok(actual_path) = std::env::current_dir() {
        fs::create_dir(format!("{}/bin", actual_path.display()).as_str())?;
    }

    if gcc_status.success() {
        println!("Compilation successful!");

        // Intentar mover el binario al directorio /bin
        match fs::rename("main", "./bin/main") {
            Ok(_) => {
                println!("Binary moved to ./bin successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to move binary to ./bin: {}", e);
                Err(e)
            }
        }
    } else {
        eprintln!("Compilation failed!");
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "GCC compilation failed",
        ))
    }
}

fn create_sub_directory(project_name: &String, sub_dir: &str) -> Result<(), std::io::Error> {
    let full_path = format!("./{}/{}", project_name, &sub_dir);
    fs::create_dir_all(&full_path)?;
    Ok(())
}

fn create_file(
    project_name: &String,
    sub_dir: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_dir = if sub_dir.is_empty() {
        format!("./{}/{}", project_name, name)
    } else {
        format!("./{}/{}/{}", project_name, sub_dir, name)
    };

    File::create(&file_dir)?;

    if name == "main.c" {
        write_main_c(&file_dir);
    } else if name == "Makefile" {
        write_makefile(&file_dir);
    } else if name == "README.md" {
        write_readme(project_name, &file_dir);
    } else {
        write_default_content(&file_dir);
    }

    Ok(())
}

fn write_main_c(file_dir: &str) {
    let content =
        "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, world!\\n\");\n    return 0;\n}\n";
    let _ = fs::write(file_dir, content);
    println!("Content written to {}", file_dir);
}

fn write_makefile(file_dir: &str) {
    let content = "CC = gcc\nCFLAGS = -Wall\n\nmain:\n\t$(CC) $(CFLAGS) -o main ./src/main.c\n";
    let _ = fs::write(file_dir, content);
    println!("Content written to {}", file_dir);
}

fn write_readme(project_name: &String, file_dir: &str) {
    let content = format!(
            "# {}\n\nA simple C project generated with cinit.\n\n## Structure\n\n- `main.c`: Entry point of the program.\n- `Makefile`: Build system configuration.\n- `README.md`: Project description.\n\n## Build\n\nTo build the project, run:\n\n```bash\nmake\n```\n\n## Run\n\nAfter building, run the program:\n\n```bash\n./main\n```\n",
            project_name
        );
    let _ = fs::write(file_dir, content);
    println!("README.md created and content written to {}", file_dir);
}

fn write_default_content(file_dir: &str) {
    let content = "// Default content\n";
    let _ = fs::write(file_dir, content);
    println!("Content written to {}", file_dir);
}

fn create_lib_folder(
    project_name: &String,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    create_sub_directory(project_name, "lib")?;
    create_file(project_name, "lib", format!("{file_name}.c").as_str())?;
    create_file(project_name, "lib", format!("{file_name}.h").as_str())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(command) = Cinit::new().0.command {
        match command {
            Commands::CNew { project_name, lib } => {
                c_new(&project_name, lib)?;
            }
            Commands::CBuild => {
                c_build()?;
                //println!("{:?}", std::env::current_dir())
            }
            Commands::CBulidRun => eprintln!("Developing"),
        }
    }

    Ok(())
}
