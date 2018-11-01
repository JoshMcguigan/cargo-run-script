# cargo-run-script

`cargo-run-script` is a Cargo subcommand which allows you to define scripts for common project related tasks within your 'Cargo.toml'. If you are familiar with node, `cargo-run-script` brings the [`npm run`](https://docs.npmjs.com/cli/run-script) functionality to the Rust and Cargo ecosystem. 

## Install

`cargo-run-script` requires Rust in order to build/install. If you need to install Rust, follow [these instructions](https://www.rust-lang.org/en-US/install.html). Once you have Rust installed, `cargo-run-script` can be installed by running the command below:

```bash
cargo install cargo-run-script
```

## Defining Scripts

Scripts are defined by adding a `[package.metadata.scripts]` section to the `Cargo.toml` file of your project, as shown below.

```toml
[package.metadata.scripts]
hello = "echo Hello"
goodbye = "echo Goodbye"
```

A more complete example can be seen by reviewing the `Cargo.toml` for this project. 

## Running Scripts

From the root of your project directory (at the same level as the `Cargo.toml`), you can run your scripts as shown below.

```bash
# to run a script called "hello"
cargo run-script hello
```

The output of this would be

```bash
Running script 'hello': 'echo Hello'
Hello
Finished, status of exit code: 0
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
