use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

use crate::list_directory;
use crate::pdf;
use crate::readme;

#[derive(Parser)]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(short = 'l', long = "long")]
    pub long: bool,

    #[arg(short = 'a', long = "all")]
    pub all: bool,

    #[arg(long = "show-ignored")]
    pub show_ignored: bool,

    #[arg(short = 'I', long = "ignore")]
    pub ignore_patterns: Vec<String>,

    #[arg(long = "readme-tagline")]
    pub readme_tagline: bool,

    #[arg(long = "pdf-title")]
    pub pdf_title: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let entries = list_directory(
        &args.path,
        args.all,
        args.show_ignored,
        &args.ignore_patterns,
    )?;

    let output = format_output(&args, &entries);
    println!("{output}");

    Ok(())
}

fn format_output(args: &Args, entries: &[crate::entry::Entry]) -> String {
    let mut lines = format_entries(entries, args.long);

    if let Some(readme_line) = format_readme(args) {
        lines.push(String::new());
        lines.push(readme_line);
    }

    let pdf_lines = format_pdf_titles(args, entries);

    if !pdf_lines.is_empty() {
        lines.push(String::new());
        lines.extend(pdf_lines);
    }

    lines.join("\n")
}

fn format_entries(entries: &[crate::entry::Entry], long: bool) -> Vec<String> {
    entries.iter().map(|entry| entry.format(long)).collect()
}

fn format_readme(args: &Args) -> Option<String> {
    if !args.readme_tagline {
        return None;
    }

    readme::read_tagline(&args.path).map(|line| format!("README: {line}"))
}

fn format_pdf_titles(args: &Args, entries: &[crate::entry::Entry]) -> Vec<String> {
    if !args.pdf_title {
        return Vec::new();
    }

    entries
        .iter()
        .filter_map(format_pdf_title)
        .flatten()
        .collect()
}

fn format_pdf_title(entry: &crate::entry::Entry) -> Option<Vec<String>> {
    let title = pdf::read_title(&entry.path)?;

    Some(vec![
        format!("PDF: {}", entry.name),
        format!("  Title: {title}"),
    ])
}
