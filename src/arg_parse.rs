use std::fmt::Debug;

#[derive(Debug)]
pub struct Args {
    /// binary path to this executable
    pub binary_path: String,
    /// name of the script to be executed as specified in Cargo.toml
    pub script_name: String,
    /// arguments to be passed to the script
    pub script_arguments: Vec<String>,
}

pub fn parse(args: Vec<String>) -> Result<Args, &'static str> {
    let mut iter = args.iter();

    let binary_path = iter.next().expect("binary path is always specified");

    // skip "run-script" subcommand if present
    let possible_script_name = match iter.next() {
        Some(value) => match value.as_str() {
            "run-script" => iter.next(),
            _ => Some(value),
        },
        None => None,
    };

    let script_name = match possible_script_name {
        Some(value) => Ok(value),
        None => Err("script name not specified"),
    }?;

    Ok(Args {
        binary_path: binary_path.clone(),
        script_name: script_name.clone(),
        script_arguments: iter.map(|arg| arg.into()).collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_subcommand_no_script() {
        // Running as a cargo subcommand, without specifying script name
        // Command: cargo run-script

        let incoming_args = vec![
            "/Users/josh/.cargo/bin/cargo-run-script".into(),
            "run-script".into(),
        ];

        let args = parse(incoming_args).unwrap_err();
        assert_eq!(args, "script name not specified");
    }

    #[test]
    fn development_no_script() {
        // Testing during development, without specifying script name
        // Command: cargo run

        let incoming_args = vec!["target/debug/cargo-run-script".into()];

        let args = parse(incoming_args).unwrap_err();
        assert_eq!(args, "script name not specified");
    }

    #[test]
    fn cargo_subcommand_script_no_args() {
        // Running as a cargo subcommand, with script name and no additional args
        // cargo run-script hello

        let incoming_args = vec![
            "/Users/josh/.cargo/bin/cargo-run-script".into(),
            "run-script".into(),
            "hello".into(),
        ];

        let args = parse(incoming_args).unwrap();
        assert_eq!(args.script_name, "hello");
        assert!(args.script_arguments.is_empty());
    }

    #[test]
    fn development_script_no_args() {
        // Testing during development, with script name and no additional args
        // cargo run hello

        let incoming_args = vec!["target/debug/cargo-run-script".into(), "hello".into()];

        let args = parse(incoming_args).unwrap();
        assert_eq!(args.script_name, "hello");
        assert!(args.script_arguments.is_empty());
    }

    #[test]
    fn cargo_subcommand_script_args() {
        // Running as a cargo subcommand, with script name and no additional args
        // cargo run-script hello

        let incoming_args = vec![
            "/Users/josh/.cargo/bin/cargo-run-script".into(),
            "run-script".into(),
            "hello".into(),
            "argument one".into(),
            "argument two".into(),
        ];

        let args = parse(incoming_args).unwrap();
        assert_eq!(args.script_name, "hello");
        assert_eq!(args.script_arguments.len(), 2);
        assert_eq!(args.script_arguments[0], "argument one");
        assert_eq!(args.script_arguments[1], "argument two");
    }

    #[test]
    fn development_script_args() {
        // Testing during development, with script name and no additional args
        // cargo run hello

        let incoming_args = vec![
            "target/debug/cargo-run-script".into(),
            "hello".into(),
            "argument one".into(),
            "argument two".into(),
        ];

        let args = parse(incoming_args).unwrap();
        assert_eq!(args.script_name, "hello");
        assert_eq!(args.script_arguments.len(), 2);
        assert_eq!(args.script_arguments[0], "argument one");
        assert_eq!(args.script_arguments[1], "argument two");
    }
}
