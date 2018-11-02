pub struct Args {
    pub script: Option<String>,
    others: Vec<String>
}

pub fn parse<T>(mut args: Vec<T>) -> Args
    where
        T: Into<String>,
        Option<T>: PartialEq
{
    args.reverse(); // dirty trick to make it easier to skip the first argument or two
    args.pop(); // skip first arg always

    let script;
    let second_arg = args.pop().map(|x| x.into());
    // if the second arg is `run-script`, skip that too
    if second_arg == Some("run-script".into()) {
        script = args.pop().map(|x| x.into());
    } else {
        script = second_arg;
    }

    Args {
        script,
        others: Vec::new()
    }
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
        assert_eq!(0, args.others.len());
    }

    #[test]
    fn development_no_script() {
        // Testing during development, without specifying script name
        // Command: cargo run

        let incoming_args = vec!["target/debug/cargo-run-script"];

        let args = parse(incoming_args);

        assert_eq!(None, args.script);
        assert_eq!(0, args.others.len());
    }

    #[test]
    fn cargo_subcommand_script_no_args() {
        // Running as a cargo subcommand, with script name and no additional args
        // cargo run-script hello

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script", "hello"];

        let args = parse(incoming_args);

        assert_eq!(Some("hello".into()), args.script);
        assert_eq!(0, args.others.len());
    }

    #[test]
    fn development_script_no_args() {
        // Testing during development, with script name and no additional args
        // cargo run hello

        let incoming_args = vec!["target/debug/cargo-run-script", "hello"];

        let args = parse(incoming_args);

        assert_eq!(Some("hello".into()), args.script);
        assert_eq!(0, args.others.len());
    }
}
