
# CInit - Command Line Tool for C Projects

**CInit** is a command-line utility designed to simplify the workflow for creating, building, and running C projects. It provides a set of commands to automate common tasks, such as initializing project structures, compiling source files, and managing binaries.

## Features

- Quickly create new C projects with standard folder structures.
- Build all `.c` files in the `./src/` directory.
- Move compiled binaries to the `./bin/` directory.
- Optionally, create a library directory with `.c` and `.h` files.
- Build and immediately run a project.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/cinit.git
   cd cinit
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. (Optional) Move the binary to a directory in your `$PATH`:
   ```bash
   mv target/release/cinit /usr/local/bin/
   ```

## Usage

To see the list of available commands, run:

```bash
cinit help
```

### Commands

#### `new`
Create a new C project with the specified name.

**Usage:**

```bash
cinit new [OPTIONS] <PROJECT_NAME>
```

**Arguments:**
- `<PROJECT_NAME>`: Name of the new C project.

**Options:**
- `--lib <FILE>`: Creates a `lib` directory with files named `<FILE>.c` and `<FILE>.h`.

**Example:**
```bash
cinit new my_project --lib my_library
```

This will create a directory `my_project` with the following structure:

```
my_project/
├── src/
├── include/
├── lib/
│   ├── my_library.c
│   └── my_library.h
└── bin/
```

#### `build`
Compile all `.c` files in the `src/` directory and move the binary to the `bin/` directory.

**Usage:**

```bash
cinit build
```

**Description:**
- Looks for `.c` files in the `src/` directory.
- Compiles them using `gcc` with the `-Wall` flag.
- Places the compiled binary in the `bin/` directory.

**Example:**
```bash
cinit build
```

#### `build-run`
Build and run your project in a single command.

**Usage:**

```bash
cinit build-run
```

**Description:**
- Builds the project as described in `build`.
- Immediately executes the resulting binary.

#### `help`
Prints the help message for the main command or any subcommand.

**Usage:**

```bash
cinit help
```

**Example:**
To see help for a specific command, use:
```bash
cinit help new
```

---

## Example Workflow

1. **Create a new project:**
   ```bash
   cinit new my_project
   cd my_project
   ```

2. **Build the project:**
   ```bash
   cinit build
   ```

3. **Run the binary:**
   ```bash
   ./bin/main
   ```

---