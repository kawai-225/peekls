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
:::
bash cargo run -- . 
:::

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



