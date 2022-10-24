use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use std::env;
use std::path::PathBuf;
use std::str;
use tempfile::TempDir;
use tokio::fs;
use tokio::process::Command;

#[derive(Debug, World)]
pub struct MakefmtWorld {
    dir: TempDir,
}

impl MakefmtWorld {
    fn filepath(&self, filename: &str) -> PathBuf {
        self.dir.path().join(filename)
    }
}

impl Default for MakefmtWorld {
    fn default() -> Self {
        Self {
            dir: TempDir::new().expect("cannot create temp dir"),
        }
    }
}

#[given("a Makefile:")]
async fn create_makefile(world: &mut MakefmtWorld, step: &Step) {
    let content = step.docstring().expect("no docstring");
    fs::write(world.filepath("Makefile"), content)
        .await
        .expect("cannot write Makefile");
}

#[when(expr = "running {string}")]
async fn run_makefmt(world: &mut MakefmtWorld, command: String) {
    let mut argv = command.split_ascii_whitespace();
    match argv.next() {
        Some("makefmt") => {}
        _ => panic!("The end-to-end tests can only run the 'makefmt' command"),
    }
    let cwd = env::current_dir().expect("cannot determine current dir");
    let makefmt_path = cwd.join("target").join("debug").join("makefmt");
    let output = Command::new(makefmt_path)
        .args(argv)
        .current_dir(&world.dir)
        .output()
        .await
        .expect("cannot find the 'makefmt' executable");
    assert!(output.status.success());
}

#[then("the Makefile should contain:")]
async fn verify_makefile_content(world: &mut MakefmtWorld, step: &Step) {
    let want = step.docstring().expect("step has no docstring");
    let have = fs::read_to_string(world.filepath("Makefile"))
        .await
        .expect("cannot read Makefile");
    pretty::assert_eq!(&have, want);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    MakefmtWorld::run("features").await;
}
