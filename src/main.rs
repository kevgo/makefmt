mod error;
mod fixes;
mod makefile;

use error::UserError;
use makefile::Makefile;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!("ERROR: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<(), UserError> {
    let makefile = Makefile::read()?;
    let makefile = fixes::double_empty_lines(makefile);
    makefile.save()
}
