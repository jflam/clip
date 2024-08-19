use std::env;
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use clipboard::{ClipboardContext, ClipboardProvider};
use glob::glob;
use tiktoken_rs::cl100k_base;

fn collect_ts_files(patterns: &[String]) -> io::Result<Vec<PathBuf>> {
    let mut ts_files = Vec::new();
    for pattern in patterns {
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => ts_files.push(path),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
    Ok(ts_files)
}

fn process_files(ts_files: &[PathBuf]) -> io::Result<String> {
    let mut content = String::new();
    for file_path in ts_files {
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
    let patterns: Vec<String> = env::args().skip(1).collect();
    let patterns = if patterns.is_empty() {
        vec!["*.ts".to_string()]
    } else {
        patterns
    };

    let ts_files = collect_ts_files(&patterns)?;

    if ts_files.is_empty() {
        println!("No TypeScript files found matching the provided patterns.");
        return Ok(());
    }

    let content = process_files(&ts_files)?;
    let token_count = count_tokens(&content);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(content).unwrap();

    println!("Collected {} TypeScript files.", ts_files.len());
    println!("Total tokens: {}", token_count);
    println!("File contents have been copied to the clipboard.");

    Ok(())
}