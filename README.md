# peekls
peekls は、通常の ls コマンドよりも「ファイルやディレクトリの中身の意味」が分かりやすい一覧表示を行う CLI ツールです。

README のタグラインや PDF のタイトルなどを表示し、ディレクトリの内容を素早く把握できることを目的としています。

## Features
- ファイル・ディレクトリ一覧表示
- -l による詳細表示
- -a による隠しファイル表示
- .gitignore を考慮した一覧表示
- --show-ignored による ignore ファイル表示
- -I, --ignore による任意ファイルの非表示
- --readme-tagline による README のタグライン表示
- --pdf-title による PDF タイトル表示

## Usage

一覧表示

bash peekls . 

詳細表示

bash peekls . -l 

隠しファイルを表示

bash peekls . -a 

ignore 対象も表示

bash peekls . --show-ignored 

特定ファイルを非表示

bash peekls . -I README.md 

README タグライン表示

bash peekls . --readme-tagline 

PDF タイトル表示

bash peekls . --pdf-title 

## Example

text Cargo.toml LICENSE README.md icom.pdf src  README: peekls  PDF: icom.pdf   Title: icom.png 

## Test

単体テスト・結合テスト・システムテストを実装しています。

bash cargo test 

静的解析

bash cargo clippy 

## Development

開発方針

- 関数は20行以内を目安に分割
- 処理結果は関数の返り値として返す
- cargo clippy の警告を解消する
- Git のコミット履歴を残しながら開発する
:::  ---  この README なら、  - 何を作ったのか - どんな機能があるのか - どう使うのか - テストしているか  が一目で分かります。  書き換えたら、 bash
cargo test
cargo clippy

git add README.md
git commit -m "Improve project README"
git push
```

=======
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

## 📄 License

This project is licensed under the MIT License.

See the LICENSE file for detail
>>>>>>> origin/main
