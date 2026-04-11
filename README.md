# peekls
This is a CLI tool that reimagines ls with smarter file listing and lightweight previews.

![GitHub License](https://img.shields.io/github/license/oriduruMaho/tallylingo)


This is a CLI tool that 

## Description
peekls is a command-line tool for listing files in a directory with additional useful features.

Unlike the standard ls command, peekls provides:
	•	Ignore-aware listing using .gitignore and .dockerignore
	•	README tagline preview for directories
	•	PDF title preview for PDF files

This tool helps users quickly understand directory contents without opening files.



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
