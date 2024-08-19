use std::env;
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use clipboard::{ClipboardContext, ClipboardProvider};
use glob::glob;
use tiktoken_rs::cl100k_base;

fn print_help() {
    println!("clip - Copy file contents to clipboard with path context");
    println!("\nUsage:");
    println!("  clip [OPTIONS] [PATTERNS...]");
    println!("\nOptions:");
    println!("  -h, --help    Show this help message");
    println!("\nExamples:");
    println!("  clip src/**/*.ts                            # Glob all .ts files in src directory and subdirectories");
    println!("  clip src/*.ts                               # Glob all .ts files in src directory");
    println!("  clip package.json src/**/*.ts test/**/*.ts  # Glob specific files and directories");
    println!("\nDescription:");
    println!("  This program copies the contents of globbed files to the system clipboard,");
    println!("  where each file is separated by its full path. This is intended to help");
    println!("  provide file context to AIs easily and quickly.");
}

fn collect_files(patterns: &[String]) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for pattern in patterns {
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => files.push(path),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
    Ok(files)
}

fn process_files(files: &[PathBuf]) -> io::Result<String> {
    let mut content = String::new();
    for file_path in files {
        content.push_str(&file_path.to_string_lossy());
        content.push_str("\n\n");
        content.push_str(&fs::read_to_string(file_path)?);
        content.push_str("\n\n");
    }
    Ok(content)
}

fn count_tokens(text: &str) -> usize {
    let bpe = cl100k_base().unwrap();
    bpe.encode_with_special_tokens(text).len()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        return Ok(());
    }

    let files = collect_files(&args)?;
    if files.is_empty() {
        println!("No files found matching the provided patterns.");
        return Ok(());
    }

    let content = process_files(&files)?;
    let token_count = count_tokens(&content);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(content).unwrap();

    println!("Collected {} files.", files.len());
    println!("Total tokens: {}", token_count);
    println!("File contents have been copied to the clipboard.");
    Ok(())
}