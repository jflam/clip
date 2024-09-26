# clip

`clip` is a versatile command-line tool that seamlessly handles both copying the contents of globbed files to the system clipboard and saving clipboard contents (text or images) into a specified file. Designed to enhance productivity, this tool aids in providing file context to AI assistants and managing clipboard data efficiently.

## Features

- **Dual Functionality:**
  - **Copy Files to Clipboard:** Select files using glob patterns and copy their contents to the clipboard with full path context.
  - **Save Clipboard to File:** Save the current clipboard content (text or image) to a specified file.
- **Glob-based File Selection:** Supports various glob patterns for flexible and precise file selection.
- **Token Counting:** Counts the total tokens in the copied text content.
- **Image Support:** Handles image data from the clipboard, saving it in PNG format.
- **Cross-Platform Compatibility:** Works consistently across Windows, macOS, and Linux.

## Prerequisites

To build and run this project, you need to have Rust and Cargo installed on your system. If you haven't already set up a Rust environment, follow these steps:

1. **Install Rust:**
   Follow the official guide to install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. **Verify Installation:**
   Open a new terminal and run:
   ```bash
   rustc --version
   cargo --version
   ```
   If both commands display version information, you're ready to proceed!

## Building the Project

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/yourusername/clip.git
   cd clip
   ```

2. **Build the Project Using Cargo:**
   ```bash
   cargo build --release
   ```
   
3. **Locate the Compiled Binary:**
   The compiled binary will be available in `target/release/clip`

## Usage

`clip` operates in two primary modes based on the provided arguments:

1. **Copying Clipboard Content to a File**
2. **Copying File Contents to the Clipboard**

### General Syntax

```bash
clip [OPTIONS] [PATTERNS...]
```

- **No Arguments:** Saves clipboard content to a file via redirection.
- **With Glob Patterns:** Copies contents of matching files to the clipboard.

### Modes of Operation

#### 1. Copying Clipboard Content to a File

This mode allows you to save the current clipboard content (text or image) into a specified file using shell redirection.

**Usage:**

```bash
clip > filename.extension
```

- **Text Content:** Saves the clipboard text directly into `filename.extension`.
- **Image Content:** Saves the clipboard image as a PNG file (`filename.png`).

**Examples:**

- **Save Text Clipboard to a File:**
  ```bash
  clip > output.txt
  ```
  
- **Save Image Clipboard to a PNG File:**
  ```bash
  clip > image.png
  ```

**Note:** Ensure that the file extension matches the clipboard content type for proper handling (e.g., `.txt` for text, `.png` for images).

#### 2. Copying File Contents to the Clipboard

This mode allows you to select files using glob patterns and copy their contents to the system clipboard. Each file's content is prefixed with its full path to provide context.

**Usage:**

```bash
clip [PATTERNS...]
```

**Examples:**

1. **Glob All `.ts` Files in `src` Directory and Subdirectories:**
   ```bash
   clip src/**/*.ts
   ```

2. **Glob All `.ts` Files in `src` Directory:**
   ```bash
   clip src/*.ts
   ```

3. **Glob Specific Files and Directories:**
   ```bash
   clip package.json src/**/*.ts test/**/*.ts
   ```

4. **Glob Multiple Patterns:**
   ```bash
   clip src/**/*.ts glob2/*.py
   ```

**Behavior:**

- The tool searches for files matching the provided glob patterns.
- For each matched file:
  - Reads its content.
  - Prepends the full file path to the content.
- Concatenates all file contents into a single string.
- Copies the resulting string to the system clipboard.
- Outputs the number of files collected and the total token count.

## Detailed Examples

### 1. Saving Clipboard Content to a File

**Command:**
```bash
clip > saved_clipboard.txt
```

**Behavior:**
- If the clipboard contains text, `saved_clipboard.txt` will contain the text.
- If the clipboard contains an image, `saved_clipboard.txt` will be a PNG file representing the image.

### 2. Copying Files to the Clipboard

**Command:**
```bash
clip src/**/*.ts glob2/*.py
```

**Behavior:**
- Copies the contents of all `.ts` files in the `src` directory and its subdirectories, as well as all `.py` files in `glob2`, to the clipboard.
- Each file's content is separated by its full path.
- Outputs the number of files processed and the total token count.

### 3. Displaying Help

**Command:**
```bash
clip --help
```

**Behavior:**
- Displays detailed usage instructions for both functionalities.

## How It Works

### 1. Copying Clipboard Content to a File

- **Trigger:** No command-line arguments; uses shell redirection (`>`).
- **Process:**
  1. Reads the current clipboard content.
  2. Determines if the content is text or an image.
  3. Writes the content to the specified file:
     - **Text:** Written as plain text.
     - **Image:** Converted and saved as a PNG file.
- **Output:** Confirms the content has been saved and specifies the file path.

### 2. Copying File Contents to the Clipboard

- **Trigger:** Provides glob patterns as command-line arguments.
- **Process:**
  1. Parses and expands the provided glob patterns to identify matching files.
  2. Reads each matched file's content.
  3. Prefixes each content block with the file's full path for context.
  4. Concatenates all contents into a single string.
  5. Counts the total number of tokens using `tiktoken-rs`.
  6. Copies the concatenated string to the system clipboard.
- **Output:** Displays the number of files collected and the total token count.

## Contributing

Contributions are welcome! Whether it's reporting bugs, suggesting features, or submitting pull requests, your input is valuable to enhancing this tool. Please follow these steps to contribute:

1. **Fork the Repository:**
   Click the "Fork" button at the top-right of this page to create a personal copy.

2. **Clone Your Fork:**
   ```bash
   git clone https://github.com/yourusername/clip.git
   cd clip
   ```

3. **Create a New Branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Your Changes:**
   Implement your feature or fix the bug.

5. **Commit Your Changes:**
   ```bash
   git commit -m "Add feature: your-feature-name"
   ```

6. **Push to Your Fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Open a Pull Request:**
   Navigate to the original repository and click "New Pull Request". Provide a clear description of your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
