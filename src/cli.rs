use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

use crate::list_directory;

#[derive(Parser)]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(short = 'l', long = "long")]
    pub long: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let entries = list_directory(&args.path)?;

    for entry in entries {
        println!("{}", entry.format(args.long));
    }

    Ok(())
}
