use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::env;
use std::process::Command;

mod arg_parse;

#[derive(Deserialize, Debug)]
struct Config {
    package: Package
}

#[derive(Deserialize, Debug)]
struct Package {
    metadata: Metadata
}

#[derive(Deserialize, Debug)]
struct Metadata {
    scripts: HashMap<String, String>
}

fn main() {
    let mut f = File::open("Cargo.toml").expect("Cargo.toml file not found.");

    let mut toml = String::new();
    f.read_to_string(&mut toml)
        .expect("Failed to read Cargo.toml.");

    let table : Config = toml::from_str(&toml)
        .expect("Expected Cargo.toml to contain package.metadata.scripts table.");

    let args = arg_parse::parse(env::args().collect());

    // TODO handle passing args through to scripts
    match args.script {
        None => {
            // display the name of all scripts
            table.package.metadata.scripts.keys()
                .for_each(|script_name| println!("{}", script_name));
        },
        Some(script_name) => {
            // run the script

            let script = table.package.metadata.scripts.get(&script_name)
                .expect("Script not found");

            println!("Running script '{}': '{}'", script_name, script);

            let mut shell = if cfg!(target_os = "windows") {
                let mut shell = Command::new("cmd");
                shell.arg("/C");

                shell
            } else {
                let mut shell = Command::new("sh");
                shell.arg("-c");

                shell
            };

            let mut child = shell.arg(script).spawn().expect("Failed to run script");

            match child.wait() {
                Ok(status) => println!("Finished, status of {}", status),
                Err(e)     => println!("Failed, error: {}", e)
            }
        },
    }
}
