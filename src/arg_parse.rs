use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, crate_version, SubCommand};
use std::fmt::Debug;
use std::ffi::OsString;

pub struct Args {
    /// name of the script to be executed as specified in Cargo.toml
    pub script: Option<String>,
    /// arguments to be passed on to the script specified in Cargo.toml
    script_args: Vec<String>,
    /// arguments to cargo-run-script
    system_args: Vec<String>,
}

/// Normalizes arguments to workaround cargo subcommand args inconsistency
/// https://github.com/rust-lang/cargo/issues/6127
// TODO move this into separate module
fn get_normalized_args<T>(mut args: Vec<T>) -> Vec<String>
    where
        T: Into<String> + Debug,
        Option<T>: PartialEq
{
    // convert to Vec<String>
    let mut args : Vec<String> = args.into_iter().map(|x| x.into()).collect();

    if args.get(1) != Some(&"run-script".to_string()) {
        args.insert(1, "run-script".to_string());
    }

    args
}

// TODO migrate this to structop
pub fn parse<'a, T>(mut args: Vec<T>) -> ArgMatches<'a>
    where
        T: Into<String> + Debug + Clone,
        Option<T>: PartialEq,
        OsString: From<T>
{
    let version = crate_version!();
    let about = "Runs shell scripts from your `Cargo.toml`.";

    let args = App::new("cargo")
        .bin_name("cargo")
        .version(version)
        .about(about)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("run-script")
            .version(version)
            .about(about)
            .usage("cargo run-script <script> [FLAGS OPTIONS] [--] <args>...")


            .arg(Arg::with_name("script")
                .help("Script to execute")
                .index(1)
            )
            .arg(Arg::with_name("args")
                .help("Additional arguments passed to the script")
                .index(2)
                .multiple(true)
            )
            .arg(Arg::with_name("if-present")
                .help("Avoid exiting with a non-zero exit code when the script is undefined")
                .long("if-present")
                .requires("script")
            )
        )
        .get_matches_from(get_normalized_args(args));

    println!("{:?}", args);

    args.subcommand_matches("run-script").unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_subcommand_no_script() {
        // Running as a cargo subcommand, without specifying script name
        // Command: cargo run-script

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script"];

        let args = get_normalized_args(incoming_args);

        assert_eq!(2, args.len());
    }

    #[test]
    fn development_no_script() {
        // Testing during development, without specifying script name
        // Command: cargo run

        let incoming_args = vec!["target/debug/cargo-run-script"];

        let args = get_normalized_args(incoming_args);

        assert_eq!(2, args.len());
        assert_eq!("run-script".to_string(), args[1]);
    }

    #[test]
    fn cargo_subcommand_script_no_args() {
        // Running as a cargo subcommand, with script name and no additional args
        // cargo run-script hello

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script", "hello"];

        let args = get_normalized_args(incoming_args.clone());

        assert_eq!(incoming_args, args);
    }

    #[test]
    fn development_script_no_args() {
        // Testing during development, with script name and no additional args
        // cargo run hello

        let incoming_args = vec!["target/debug/cargo-run-script", "hello"];

        let args = get_normalized_args(incoming_args);

        assert_eq!(3, args.len());
        assert_eq!("run-script", args[1]);
        assert_eq!("hello", args[2]);
    }

    #[test]
    fn no_script() {
        // cargo run-script

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script"];

        let args = parse(incoming_args);

        assert_eq!(None, args.value_of("script"));
        assert!(!args.is_present("if-present"));
    }

    #[test]
    fn with_script() {
        // cargo run-script hello

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script", "hello"];

        let args = parse(incoming_args);

        assert_eq!(Some("hello"), args.value_of("script"));
        assert!(!args.is_present("if-present"));
    }

    #[test]
    fn with_script_if_present() {
        // cargo run-script hello --if-present

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script", "hello", "--if-present"];

        let args = parse(incoming_args);

        assert_eq!(Some("hello"), args.value_of("script"));
        assert!(args.is_present("if-present"));
    }

    #[test]
    fn with_script_if_present_script_args() {
        // cargo run-script hello --if-present -- --script-flag

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script", "hello", "--if-present", "--", "--script-flag"];

        let args = parse(incoming_args);

        assert_eq!(Some("hello"), args.value_of("script"));
        assert!(args.is_present("if-present"));

        let mut script_args = args.values_of("args").unwrap();

        assert_eq!(Some("--script-flag"), script_args.next());
        assert_eq!(None, script_args.next());
    }

    #[test]
    #[ignore] // this test does the correct thing (it fails and alerts the user), but it needs to be an integration test
    fn with_script_if_present_script_args_with_misplaced_script_arg() {
        // cargo run-script hello --if-present -- --script-flag

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script", "hello", "--if-present", "--misplaced-arg", "--", "--script-flag"];

        let args = parse(incoming_args);
    }

    #[test]
    #[ignore] // this test shows clap adding the misplaced-arg to the script args, which is not desirable
    fn with_script_if_present_script_args_with_misplaced_script_arg_in_front_of_if_present() {
        // cargo run-script hello --if-present -- --script-flag

        let incoming_args = vec!["/Users/josh/.cargo/bin/cargo-run-script", "run-script", "hello", "misplaced-arg", "--if-present", "--", "--script-flag"];

        let args = parse(incoming_args);

        assert_eq!(Some("hello"), args.value_of("script"));
        assert!(args.is_present("if-present"));

        let mut script_args = args.values_of("args").unwrap();

        assert_eq!(Some("--script-flag"), script_args.next());
        assert_eq!(None, script_args.next());
    }
}
