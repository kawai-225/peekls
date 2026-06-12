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

で良いと思います。
