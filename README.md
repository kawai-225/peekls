# peekls

peekls is a CLI tool that provides a more meaningful directory listing than the standard ls command.

Instead of only displaying file and directory names, peekls can show additional information such as README taglines and PDF titles, helping users quickly understand the contents of a directory.

## Features

- Display files and directories
- Long format listing (-l, --long)
- Show hidden files (-a, --all)
- Respect .gitignore rules
- Show ignored files (--show-ignored)
- Ignore specific files (-I, --ignore)
- Display README taglines (--readme-tagline)
- Display PDF titles (--pdf-title)

## Installation

bash git clone https://github.com/kawai-225/peekls.git cd peekls cargo build 

## Usage

Display files and directories:

bash cargo run -- . 

Display detailed information:

bash cargo run -- . -l 

Show hidden files:

bash cargo run -- . -a 

Show ignored files:

bash cargo run -- . --show-ignored 

Ignore a specific file:

bash cargo run -- . -I README.md 

Display the README tagline:

bash cargo run -- . --readme-tagline 

Display PDF titles:

bash cargo run -- . --pdf-title 

## Example

text Cargo.toml LICENSE README.md sample.pdf src  README: peekls  PDF: sample.pdf   Title: Sample Document 

## Testing

Run all tests:

bash cargo test 

Run static analysis:

bash cargo clippy 

Format source code:

bash cargo fmt 

## Project Structure

text src/ ├── main.rs      # CLI entry point ├── cli.rs       # Command-line argument handling ├── lib.rs       # Core functionality ├── entry.rs     # File and directory information ├── ignore.rs    # .gitignore support ├── readme.rs    # README tagline extraction └── pdf.rs       # PDF title extraction  tests/ ├── integration_test.rs └── system_cli.rs 

## Development Policy

- Keep functions short and focused (approximately 20 lines or less)
- Return results from functions whenever possible
- Resolve all cargo clippy warnings
- Maintain a clear Git commit history
- Add tests for new functionality
```
:::

これなら課題で要求されている

- CLIツールの説明
- 使用方法
- テスト方法
- ディレクトリ構成
- 開発方針

がすべて含まれていて、GitHubのREADMEとして十分見栄えがします。
