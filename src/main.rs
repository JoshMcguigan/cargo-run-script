use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::env;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Config {
    scripts: HashMap<String, String>
}

fn main() {
    let mut f = File::open("Cargo.toml").expect("Cargo.toml file not found.");

    let mut toml = String::new();
    f.read_to_string(&mut toml)
        .expect("Failed to read Cargo.toml.");

    let table : Config = toml::from_str(&toml).expect("Expected Cargo.toml to contain scripts table.");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // run the script
        let script_name = &args[args.len() - 1];

        let script = table.scripts.get(script_name).expect("Script not found");

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

        let output = shell.arg(script).output().expect("Failed to run script");

        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("Script exited with status code {}", output.status);
            println!("stdout:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("stderr:");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    } else {
        // display the name of all scripts
        table.scripts.keys().for_each(|script_name| println!("{}", script_name));
    }

}
