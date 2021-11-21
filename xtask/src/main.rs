mod build;
mod clean;

use structopt::StructOpt;

use crate::{build::build, clean::clean};


#[derive(Debug, StructOpt)]
enum XTaskArgs {
    Build {
        #[structopt(long)]
        release: bool,
    },
    Clean {},
}

fn main() {
    let args = XTaskArgs::from_args();
    match args {
        XTaskArgs::Build { release } => build(release),
        XTaskArgs::Clean {} => clean(),
    }
}
