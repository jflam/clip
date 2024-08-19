# clip

`clip` is a command-line tool that copies the contents of globbed files to the system clipboard, with each file separated by its full path. This tool is designed to help provide file context to AI assistants easily and quickly.

## Features

- Glob-based file selection
- Copies file contents to clipboard with full path context
- Counts total tokens in copied content
- Supports various glob patterns for flexible file selection

## Prerequisites

To build and run this project, you need to have Rust and Cargo installed on your system. If you haven't already set up a Rust environment, follow these steps:

1. Install Rust by following the official guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Verify the installation by opening a new terminal and running:
   ```
   rustc --version
   cargo --version
   ```

   If both commands display version information, you're good to go!

## Building the Project

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/clip.git
   cd clip
   ```

2. Build the project using Cargo:
   ```
   cargo build --release
   ```

3. The compiled binary will be available in `target/release/clip`

## Usage

Run the tool using Cargo:

```
cargo run -- [OPTIONS] [PATTERNS...]
```

Or, if you've built the release version, you can run the binary directly:

```
./target/release/clip [OPTIONS] [PATTERNS...]
```

### Options

- `-h, --help`: Show the help message

### Examples

1. Glob all `.ts` files in the `src` directory and subdirectories:
   ```
   cargo run -- src/**/*.ts
   ```

2. Glob all `.ts` files in the `src` directory:
   ```
   cargo run -- src/*.ts
   ```

3. Glob specific files and directories:
   ```
   cargo run -- package.json src test
   ```

## How It Works

1. The tool takes glob patterns as command-line arguments.
2. It searches for files matching the provided patterns.
3. For each matched file, it reads the content and prepends the full file path.
4. All file contents are concatenated into a single string.
5. The resulting string is copied to the system clipboard.
6. The tool outputs the number of files collected and the total token count.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.