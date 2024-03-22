mod pack_multi;
mod pack_single;
mod utils;

use clap::{Parser, ValueEnum};
use pack_multi::pack_multi;
use pack_single::pack_single;

fn main() {
    match Args::parse().pack_mode {
        PackMode::Multi => pack_multi(),
        PackMode::Single => pack_single(),
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// The mode/format the packer will use
    #[arg(long, value_enum, default_value_t = Default::default())]
    pack_mode: PackMode,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PackMode {
    /// Pack resources into a `resources` folder
    #[default]
    Multi,
    /// Pack resources into a single `resources.pck` file
    Single,
}
