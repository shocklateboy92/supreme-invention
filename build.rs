use std::io::Read;
use std::fs;
use std::process::Command;

// Generic commands for running commands and lazy building
//
fn should_rebuild(target: &str, sources: &[&str]) -> bool {
    macro_rules! get_mtime {
        ($n:ident) => {{
            let meta = match fs::metadata($n) {
                Ok(v) => v,
                Err(_) => return true
            };
            match meta.modified() {
                Ok(v) => v,
                Err(_) => return true
            }
        }}
    }
    let target_mtime = get_mtime!(target);
    for source in sources {
        let source_mtime = get_mtime!(source);
        if target_mtime < source_mtime {
            return true;
        }
    }
    false
}

fn run_command(command_str: &str) {
    println!("{}", command_str);
    let mut cmd_parts = command_str.split_whitespace();
    let cmd = cmd_parts.next().unwrap();
    let cmd_args = cmd_parts;
    Command::new(cmd).args(cmd_args).spawn().expect(&format!(
        "Failed to run command {}",
        command_str
    ));
}

fn build_schema() {
    let input_file = "src/contracts/backend.schema.json";
    let output_file = "src/backend/schema.rs";
    run_command(&format!(
        "yarn run quicktype --lang=rust --derive-debug --src-lang=schema {} -o {}",
        input_file,
        output_file
    ));
}

fn main() {
    let rules = [
        (
            "src/schema.rs",
            vec!["contracts/backend.schema.json", "build.rs"],
            build_schema,
        ),
    ];
    for rule in rules.iter() {
        if should_rebuild(rule.0, &rule.1) {
            rule.2();
        }
    }
}
