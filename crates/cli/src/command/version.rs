use clap::{Arg, ArgAction};

pub fn version_args() -> Arg {
    Arg::new("version")
        .long("version")
        .short('v')
        .help("Prints the current version")
        .action(ArgAction::SetTrue)
}

pub fn version(digit: &str) {
    println!("v{}", digit);
}
