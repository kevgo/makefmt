use crate::error::UserError;
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Makefile {
    sections: Vec<Section>,
}

impl Makefile {
    pub fn read() -> Result<Makefile, UserError> {
        let content = fs::read_to_string("Makefile").map_err(|err| UserError::NoMakefile {
            guidance: err.to_string(),
        })?;
        Ok(Makefile::parse(&content))
    }

    pub fn parse(text: &str) -> Makefile {
        let mut sections = Vec::new();
        let mut current_target: Option<Target> = None;
        let target_def_regex = Regex::new("^([a-zA-Z]+):").unwrap();
        for line in text.lines() {
            if line.is_empty() {
                sections.push(Section::EmptyLine);
                if let Some(target) = current_target {
                    sections.push(Section::Target(target));
                    current_target = None;
                }
                continue;
            }
            if line.starts_with("#") {
                sections.push(Section::Comment(line.into()));
                continue;
            }
            if line.starts_with("\t") {
                if let Some(mut target) = current_target {
                    target.commands.push(line.into());
                    current_target = Some(target);
                } else {
                    sections.push(Section::TopLevel(line.into()));
                }
                continue;
            }
            if let Some(captures) = target_def_regex.captures(text) {
                if let Some(target) = current_target {
                    sections.push(Section::Target(target));
                }
                current_target = Some(Target {
                    name: captures.get(1).map(|m| m.as_str().to_string()).unwrap(),
                    deps: match captures.get(2) {
                        Some(deps) => deps.as_str().split(" ").map(|s| s.to_string()).collect(),
                        None => vec![],
                    },
                    comment: captures.get(3).map(|m| m.as_str().to_string()),
                    commands: vec![],
                });
                continue;
            }
            sections.push(Section::TopLevel(line.into()))
        }
        if let Some(target) = current_target {
            sections.push(Section::Target(target));
        }
        Makefile { sections }
    }

    pub fn save(self) -> Result<(), UserError> {
        fs::write("Makefile", self.serialize()).map_err(|err| UserError::CannotWriteMakefile {
            guidance: err.to_string(),
        })
    }

    pub fn serialize(self) -> String {
        let mut result = String::new();
        for line in self.sections {
            result.push_str(&line.to_string())
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub enum Section {
    /// a line consisting only of a comment
    Comment(String),
    /// an empty line
    EmptyLine,
    /// a Make target
    Target(Target),
    /// a line of content outside of a target definition
    TopLevel(String),
}

impl Section {
    pub fn to_string(self) -> String {
        match self {
            Section::Comment(text) => text,
            Section::EmptyLine => "\n".into(),
            Section::Target(target) => {
                let mut result = target.name;
                for dep in target.deps {
                    result.push_str(" ");
                    result.push_str(&dep);
                }
                if let Some(comment) = target.comment {
                    result.push_str(" ");
                    result.push_str(&comment);
                }
                result
            }
            Section::TopLevel(text) => text,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Target {
    /// name of the target
    name: String,
    /// dependencies of the target
    deps: Vec<String>,
    /// optional comment at the end of the target definition
    comment: Option<String>,
    /// the rules how to build this target
    commands: Vec<String>,
}

#[cfg(test)]
mod tests {

    mod makefile {

        mod parse {
            use crate::makefile::{Makefile, Section, Target};

            #[test]
            fn empty() {
                let give = "";
                let want = Makefile { sections: vec![] };
                let have = Makefile::parse(give);
                assert_eq!(have, want);
            }

            #[test]
            fn comments() {
                let give = "# one\n# two\n";
                let want = Makefile {
                    sections: vec![
                        Section::Comment("# one".into()),
                        Section::Comment("# two".into()),
                    ],
                };
                let have = Makefile::parse(give);
                assert_eq!(have, want);
            }

            #[test]
            fn simple_target() {
                let give = "alpha: beta  # a target\n\techo one\n\techo two\n";
                let want = Makefile {
                    sections: vec![Section::Target(Target {
                        name: "alpha".into(),
                        deps: vec!["beta".into()],
                        comment: Some("a target".into()),
                        commands: vec!["\techo one".into(), "\techo two".into()],
                    })],
                };
                let have = Makefile::parse(give);
                assert_eq!(have, want);
            }

            #[test]
            fn multiple_targets() {
                let give = "alpha:\n\techo one\n\techo two\n";
                let want = Makefile {
                    sections: vec![Section::Target(Target {
                        name: "alpha".into(),
                        deps: vec![],
                        comment: None,
                        commands: vec!["\techo one".into(), "\techo two".into()],
                    })],
                };
                let have = Makefile::parse(give);
                assert_eq!(have, want);
            }
        }
    }
}
