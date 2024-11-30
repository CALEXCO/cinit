use clap::Parser;

/// Command line tool for C projects
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the project
    project_name: String,
}

fn main() {
    let _args = Args::parse();
}
