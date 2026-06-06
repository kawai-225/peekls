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

    for entry in &entries {
        println!("{}", entry.format(args.long));
    }

    if args.pdf_title {
        for entry in &entries {
            if let Some(title) = pdf::read_title(&entry.path) {
                println!("PDF: {}", entry.name);
                println!("  Title: {title}");
            }
        }
    }

    if args.readme_tagline
        && let Some(line) = readme::read_tagline(&args.path)
    {
        println!();
        println!("README: {line}");
    }

    Ok(())
}
