# 📂 peekls

peekls is a directory listing tool written in Rust that provides more meaningful information than the standard ls command.

Instead of displaying only file and directory names, peekls can show README taglines and PDF titles, helping users quickly understand the contents of a directory.

This project was developed as a learning project for Rust, software testing, and CLI application development.

## 🗣️ Overview

peekls is designed to help users understand the purpose and contents of files without opening them.

The tool extends traditional directory listing by providing lightweight previews such as:

- README taglines
- PDF titles
- Ignore-aware listings

## 🚀 Features

- Directory Listing: Display files and directories.
- Long Format: Show detailed information with -l.
- Hidden Files: Display hidden files with -a.
- Gitignore Support: Respects .gitignore rules by default.
- Show Ignored Files: Display ignored files with --show-ignored.
- Custom Ignore Rules: Hide specific files using -I.
- README Preview: Show README taglines with --readme-tagline.
- PDF Preview: Show PDF titles with --pdf-title.

## 📦 Installation

```bash git clone https://github.com/kawai-225/peekls.git cd peekls cargo build ```

## 💻 Usage

Display files and directories:

```bash
cargo run -- .
```

Display detailed information:

```bash
cargo run -- . -l
```

Show hidden files:

```bash
cargo run -- . -a
```

Show ignored files:

```bash
cargo run -- . --show-ignored
```

Ignore a specific file:

```bash
cargo run -- . -I README.md
```

Display the README tagline:

```bash
cargo run -- . --readme-tagline
```

Display PDF titles:

```bash
cargo run -- . --pdf-title
```

## 📖 Example

text Cargo.toml LICENSE README.md sample.pdf src  README: peekls  PDF: sample.pdf   Title: Sample Document 

## 🧪 Testing

### Run all tests

bash cargo test 

### Run static analysis

bash cargo clippy 

### Format source code

bash cargo fmt 

Current test coverage includes:

- Unit Tests
  - clean_line
  - matches_custom_ignore
  - is_hidden
- Integration Tests
  - list_directory
- System Tests
  - CLI execution

## 📁 Project Structure

text src/ ├── main.rs ├── cli.rs ├── lib.rs ├── entry.rs ├── ignore.rs ├── readme.rs └── pdf.rs  tests/ ├── integration_test.rs └── system_cli.rs 

## 👨‍💻 Development Policy

- Keep functions small and focused.
- Return results from functions whenever possible.
- Eliminate all cargo clippy warnings.
- Maintain a clear Git commit history.
- Add tests for new functionality.

## 📄 License

This project is licensed under the MIT License.

See the LICENSE file for detail
