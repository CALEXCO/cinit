use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the project
    project_name: String,
}

fn main() {
    let _args = Args::parse();
    
}