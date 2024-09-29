use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use arboard::Clipboard;
use glob::glob;
use tiktoken_rs::cl100k_base;

use image::{ImageBuffer, RgbaImage, ImageFormat};
use std::convert::TryInto;
use std::io::Cursor;

/// Print help message for the combined clip program
fn print_help() {
    println!("clip - Combined Clipboard Utility");
    println!("\nUsage:");
    println!("  clip [OPTIONS] [PATTERNS...]");
    println!("\nOptions:");
    println!("  -h, --help    Show this help message");
    println!("\nUsage Patterns:");
    println!("  1. Copy Clipboard Content to a File:");
    println!("     clip > filename.extension");
    println!("\n     - Saves the current clipboard content (text or image) to the specified file.");
    println!("\n  2. Copy File Contents to Clipboard:");
    println!("     clip glob1/**/* glob2/*.py");
    println!("\n     - Copies the contents of files matching the provided glob patterns to the clipboard.");
    println!("\nExamples:");
    println!("  clip src/**/*.ts                            # Glob all .ts files in src directory and subdirectories");
    println!("  clip src/*.ts                               # Glob all .ts files in src directory");
    println!("  clip package.json src/**/*.ts test/**/*.ts  # Glob specific files and directories");
    println!("  clip > output.txt                           # Save clipboard contents to output.txt");
}

/// Collect files matching the given glob patterns
fn collect_files(patterns: &[String]) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for pattern in patterns {
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => files.push(path),
                Err(e) => eprintln!("Glob pattern error: {:?}", e),
            }
        }
    }
    Ok(files)
}

/// Process files by concatenating their paths and contents
fn process_files(files: &[PathBuf]) -> io::Result<String> {
    let mut content = String::new();
    for file_path in files {
        content.push_str(&file_path.to_string_lossy());
        content.push_str("\n\n");
        // Read as text; if binary files are expected, adjust accordingly
        let file_content = fs::read_to_string(file_path)?;
        content.push_str(&file_content);
        content.push_str("\n\n");
    }
    Ok(content)
}

/// Count tokens using tiktoken_rs
fn count_tokens(text: &str) -> usize {
    let bpe = cl100k_base().unwrap();
    bpe.encode_with_special_tokens(text).len()
}

/// Perform "clip" functionality: copy files to clipboard
fn clip_files_to_clipboard(patterns: &[String]) -> io::Result<()> {
    let files = collect_files(patterns)?;
    if files.is_empty() {
        println!("No files found matching the provided patterns.");
        return Ok(());
    }

    let content = process_files(&files)?;
    let token_count = count_tokens(&content);

    let mut clipboard = Clipboard::new().map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to initialize clipboard: {}", e))
    })?;

    clipboard.set_text(content).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to set clipboard text: {}", e))
    })?;

    println!("Collected {} files.", files.len());
    println!("Total tokens: {}", token_count);
    println!("File contents have been copied to the clipboard.");
    Ok(())
}

/// Perform "clippa" functionality: read from clipboard and write to stdout
fn clippa_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the clipboard
    let mut clipboard = Clipboard::new()?;

    // Attempt to retrieve text from the clipboard
    if let Ok(text) = clipboard.get_text() {
        // Write the text to stdout
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(text.as_bytes())?;
        handle.flush()?;
    }
    // If no text is found, attempt to retrieve an image
    else if let Ok(image_data) = clipboard.get_image() {
        // Convert width and height from usize to u32
        let width: u32 = image_data.width.try_into().map_err(|_| "Width is too large")?;
        let height: u32 = image_data.height.try_into().map_err(|_| "Height is too large")?;

        // Convert image data from BGRA to RGBA format
        let mut rgba_bytes = Vec::with_capacity(image_data.bytes.len());

        for chunk in image_data.bytes.chunks_exact(4) {
            rgba_bytes.push(chunk[2]); // R
            rgba_bytes.push(chunk[1]); // G
            rgba_bytes.push(chunk[0]); // B
            rgba_bytes.push(chunk[3]); // A
        }

        // Create an ImageBuffer from the RGBA bytes
        let buffer: RgbaImage = ImageBuffer::from_raw(width, height, rgba_bytes)
            .ok_or("Failed to create image buffer from clipboard data")?;

        // Encode the image as PNG using a Cursor to satisfy Write + Seek
        let mut cursor = Cursor::new(Vec::new());
        buffer
            .write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| format!("Failed to encode image as PNG: {}", e))?;

        // Extract the PNG bytes from the Cursor
        let png_bytes = cursor.into_inner();

        // Write the PNG bytes to stdout as binary
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(&png_bytes)?;
        handle.flush()?;
    }
    // If neither text nor image is present in the clipboard
    else {
        eprintln!("Clipboard does not contain text or image data.");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    // If no arguments, perform "clippa" functionality (clipboard to file via stdout)
    if args.is_empty() {
        clippa_to_stdout()
    }
    // If help flag is present, display help
    else if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        Ok(())
    }
    // Otherwise, perform "clip" functionality (copy files to clipboard)
    else {
        clip_files_to_clipboard(&args)?;
        Ok(())
    }
}
