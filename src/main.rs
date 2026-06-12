use std::process::ExitCode;

fn main() -> ExitCode {
    match peekls::cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
