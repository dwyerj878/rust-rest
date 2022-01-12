use std::path::PathBuf;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
pub struct Cli {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Set port
    #[structopt(short, long, default_value = "5000")]
    pub port: u16,


    /// bind address
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub bind: String,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
