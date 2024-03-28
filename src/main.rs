use arg_parse::Args;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::Command;

mod arg_parse;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Config {
    Workspace { workspace: MetadataSection },
    Package { package: MetadataSection },
}

#[derive(Deserialize, Debug)]
struct MetadataSection {
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    scripts: HashMap<String, String>,
}

impl Metadata {
    fn print_script_names(&self) {
        self.scripts
            .keys()
            .for_each(|script_name| println!("{}", script_name));
    }
}

fn main() -> Result<(), String> {
    let metadata = parse_toml_file("Cargo.toml");

    let args = arg_parse::parse(env::args().collect()).or_else(|err| {
        metadata.print_script_names();
        Err(err)
    })?;

    match metadata.scripts.get(&args.script_name) {
        Some(script) => {
            run_script(script, args);
            Ok(())
        }
        None => {
            metadata.print_script_names();
            Err("script name is invalid".into())
        }
    }
}

fn parse_toml_file(file_path: &str) -> Metadata {
    let mut f = File::open(file_path).unwrap_or_else(|_| panic!("{} file not found.", file_path));

    let mut toml = String::new();
    f.read_to_string(&mut toml)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path));

    let config: Config = toml::from_str(&toml)
        .expect("Expected toml file to contain package.metadata.scripts or workspace.metadata.scripts table.");

    match config {
        Config::Workspace { workspace } => workspace.metadata,
        Config::Package { package } => package.metadata,
    }
}

fn run_script(script: &str, args: Args) {
    let mut shell = if cfg!(target_os = "windows") {
        let mut shell = Command::new("cmd");
        shell.arg("/C");

        shell
    } else {
        let mut shell = Command::new("sh");
        shell.arg("-c");

        shell
    };

    shell.arg(script);
    args.script_arguments.iter().for_each(|arg| {
        shell.arg(arg);
    });

    let mut child = shell.spawn().expect("Failed to run script");
    match child.wait() {
        Ok(status) => println!("Finished, status of {}", status),
        Err(e) => println!("Failed, error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workspace_toml() {
        let result = parse_toml_file("test-files/workspace-cargo.toml");
        assert!(result.scripts.contains_key("hello"));
        assert!(result.scripts.contains_key("goodbye"));
    }

    #[test]
    fn test_parse_package_toml() {
        let result = parse_toml_file("test-files/package-cargo.toml");
        assert!(result.scripts.contains_key("hello"));
        assert!(result.scripts.contains_key("goodbye"));
    }
}
