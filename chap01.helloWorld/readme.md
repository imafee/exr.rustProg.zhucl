# 认识

## 工具集

```shell
rustc # Compiler
rustup # toolchain
cargo # package manager
```

## 安装方法

[官方文档](https://www.rust-lang.org/tools/install)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
rustup --version
cargo --version
```

## 命令 rustup

Usage: `rustup [OPTIONS] [+toolchain] <SUBCOMMAND>`

```shell
# OPTIONS:
-v, --verbose    Enable verbose output
-q, --quiet      Disable progress output
-h, --help       Print help information
-V, --version    Print version information

# SUBCOMMANDS:
show           Show the active and installed toolchains or profiles
update         Update Rust toolchains and rustup
check          Check for updates to Rust toolchains and rustup
default        Set the default toolchain
toolchain      Modify or query the installed toolchains
target         Modify a toolchain's supported targets
component      Modify a toolchain's installed components
override       Modify directory toolchain overrides
run            Run a command with an environment configured for a given toolchain
which          Display which binary will be run for a given command
doc            Open the documentation for the current toolchain
man            View the man page for a given command
self           Modify the rustup installation
set            Alter rustup settings
completions    Generate tab-completion scripts for your shell
help           Print this message or the help of the given subcommand(s)
```

## 命令 cargo

Usage:
`cargo [+toolchain] [OPTIONS] [COMMAND]`
`cargo [+toolchain] [OPTIONS] -Zscript <MANIFEST_RS> [ARGS]...`

```shell
# Options:
-V, --version             Print version info and exit
--list                List installed commands
--explain <CODE>      Provide a detailed explanation of a rustc error message
-v, --verbose...          Use verbose output (-vv very verbose/build.rs output)
-q, --quiet               Do not print cargo log messages
--color <WHEN>        Coloring: auto, always, never
-C <DIRECTORY>            Change to DIRECTORY before doing anything (nightly-only)
--frozen              Require Cargo.lock and cache are up to date
--locked              Require Cargo.lock is up to date
--offline             Run without accessing the network
--config <KEY=VALUE>  Override a configuration value
-Z <FLAG>                 Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
-h, --help                Print help

# Commands:
new         Create a new cargo package
init        Create a new cargo package in an existing directory
publish     Package and upload this package to the registry

add         Add dependencies to a manifest file
remove      Remove dependencies from a manifest file
update      Update dependencies listed in Cargo.lock

search      Search registry for crates
install     Install a Rust binary. Default location is $HOME/.cargo/bin
uninstall   Uninstall a Rust binary

run, r      Run a binary or example of the local package
test, t     Run the tests
bench       Run the benchmarks
build, b    Compile the current package
check, c    Analyze the current package and report errors, but don\'t build object files
clean       Remove the target directory
doc, d      Build this package's and its dependencies\' documentation






```
