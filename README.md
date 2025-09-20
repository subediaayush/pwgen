# cls-capp-pwgen

A command-line password generator written in Rust.
This tool allows you to generate secure, customizable passwords directly from your terminal.
It supports various character sets, password lengths, and can copy generated passwords to your clipboard for convenience.

## Features
- Generate strong, random passwords
- Customize password length and character sets (uppercase, lowercase, digits, symbols)
- Copy passwords to clipboard
- Simple CLI interface

## Prerequisites
- Rust (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)

## Usage

1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd cls-capp-pwgen
   ```

2. **Build the project:**
   ```sh
   cargo build --release
   ```

3. **Run the password generator:**
   ```sh
   cargo run --release -- [OPTIONS]
   ```
   Example:
   ```sh
   cargo run --release -- -l 16 -f -p snlu
   ```
   This generates a 16-character password with 4 symbols, 4 digits, 4 lower case and 4 uppercase characters.
   You can also use parentheses to define extendable regions of your password. For eg for a length of 13 and pattern `s(nlu)` you will get 1 symbol, 4 digits, 4 lower case and 4 uppercase characters.

4. **Copy to clipboard:**
   Use the `-c` flag to copy the generated password to your clipboard.

5. **Run tests:**
   ```sh
   cargo test
   ```

## CLI Options
- `-l`, `--length <length>`: Set password length
- `-u`, `--uppercase`: Include uppercase letters
- `-d`, `--digits`: Include digits
- `-s`, `--symbols`: Include symbols
- `-c`, `--copy`: Copy password to clipboard
- `-h`, `--help`: Show help message

## License
MIT License

---
Replace `<repository-url>` with your repository's URL if sharing.

