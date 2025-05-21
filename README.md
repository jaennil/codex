# Codex

This Rust program, `codex`, recursively traverses a specified directory and concatenates the contents of files with code extensions into a single output file named `code`. It's useful for collecting code to paste into large language models (LLMs) for analysis or debugging.

## Installation

1. Ensure you have [Rust](https://rustup.rs/) installed.
2. Clone the repository or copy the source code.
3. Run the following command to build and install the program:
   ```bash
   make build-install
   ```
   This compiles the program and copies binary to `/usr/local/bin` making it available for use.

## Usage

1. **Run the program**:
   ```bash
   codex <directory_path>
   ```
   Replace `<directory_path>` with the path to the directory you want to scan.

2. **Output**:
   - Creates a `code` file in the current directory.
   - For each file with a matching extension, it writes:
     - The full path of the file.
     - The file's contents.
     - A blank line separator.

3. **Example**:
   ```bash
   codex ./src
   ```
   Scans the `./src` directory and writes contents of matching files to `code`.

## Requirements

- **Rust**: Install via [rustup](https://rustup.rs/).
- A valid directory path as a command-line argument.

## TODO

- Add support for processing GitHub URLs to fetch and include code from repositories.
- Implement a feature to copy the contents of the `code` file to the clipboard for easy pasting into LLMs.
- Update the search to ignore `.gitignore` files.
- Add support for including `Makefile` and other usefull files without extension in the output.
