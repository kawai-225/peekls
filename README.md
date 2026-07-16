# peekls
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
[![Coverage Status](https://coveralls.io/repos/github/kawai-225/peekls/badge.svg?branch=main)](https://coveralls.io/github/kawai-225/peekls?branch=main)


This is a CLI tool that reimagines ls with smarter file listing and lightweight previews.

## Description
peekls is a command-line tool that enhances `ls` by providing ignore-aware file listing and lightweight previews such as README taglines and PDF titles. It helps users quickly understand directory contents without opening files.It is developed in Rust.

## Usage
```
-l, --long              Show detailed file information
-a, --all               Show hidden files
    --show-ignored      Show files ignored by .gitignore
-I <PATTERN>            Hide files or directories matching the pattern
    --readme-tagline    Show tagline from README.md in directories
    --pdf-title         Show title of PDF files
-h, --help              Show help message
-V, --version           Show version
```

## Docker

Build the Docker image:

```bash
docker build -t peekls:local -f Containerfile .

```
