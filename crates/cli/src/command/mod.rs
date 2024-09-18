mod version;

pub use version::version;

use clap::Command;

pub fn cli() -> Command {
    Command::new("ezcfg")
        .about("A simple configuration tool")
        .subcommand_required(false)
        .arg_required_else_help(false)
        // Add args
        .arg(version::version_args())
}
