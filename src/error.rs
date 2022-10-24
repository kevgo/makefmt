use std::fmt::Display;

pub enum UserError {
    CannotWriteMakefile { guidance: String },
    NoMakefile { guidance: String },
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("No Makefile found")
    }
}
