use std::fmt::Debug;

pub struct Args {
    /// name of the script to be executed as specified in Cargo.toml
    pub script: Option<String>,
}

pub fn parse<T>(mut args: Vec<T>) -> Args
    where
        T: Into<String> + Debug,
        Option<T>: PartialEq
{
    match args.pop() {
        Some(script) => {
            let script : String = script.into();
            if script.contains("run-script") {
                // no script was passed, so the name of the binary, "run-script", was the last arg
                // contains is used rather than equals because in development the binary name is "cargo-run-script"
                return Args { script: None};
            };
            return Args {script: Some(script)};
        },
        None => return Args {script: None} 
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_subcommand_no_script() {
        // Running as a cargo subcommand, without specifying script name
        // Command: cargo run-script

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script"];

        let args = parse(incoming_args);

        assert_eq!(None, args.script);
    }

    #[test]
    fn development_no_script() {
        // Testing during development, without specifying script name
        // Command: cargo run

        let incoming_args = vec!["target/debug/cargo-run-script"];

        let args = parse(incoming_args);

        assert_eq!(None, args.script);
    }

    #[test]
    fn cargo_subcommand_script_no_args() {
        // Running as a cargo subcommand, with script name and no additional args
        // cargo run-script hello

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script", "hello"];

        let args = parse(incoming_args);

        assert_eq!(Some("hello".into()), args.script);
    }

    #[test]
    fn development_script_no_args() {
        // Testing during development, with script name and no additional args
        // cargo run hello

        let incoming_args = vec!["target/debug/cargo-run-script", "hello"];

        let args = parse(incoming_args);

        assert_eq!(Some("hello".into()), args.script);
    }
}
