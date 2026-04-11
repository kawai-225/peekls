# peekls
This is a CLI tool that reimagines ls with smarter file listing and lightweight previews.



This is a CLI tool that 

## Description
peekls is a command-line tool that enhances `ls` by providing ignore-aware file listing and lightweight previews such as README taglines and PDF titles. It helps users quickly understand directory contents without opening files.It is developed in Rust.

## Usage
```
-l, --long            Display detailed file information
-a, --all             　Show hidden files
-I, --ignore          Respect .gitignore and .dockerignore
--show-ignored        Show ignored files as well

--readme-tagline      Show tagline from README.md in directories
--pdf-title           Show title of PDF files

-h, --help            Show help message
-V, --version         Show version
```
