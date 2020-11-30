#[macro_use]
extern crate serde_derive;

mod commands;
mod metadata;

use crate::metadata::global_metadata;
use crate::metadata::global_store;
use crate::metadata::working_state;
use crate::metadata::file_blob;
use crate::metadata::commit;

use clap::Clap;

/// A version control system
#[derive(Clap)]
#[clap(version = "0.1", author = "Olivier FAURE <couteaubleu@gmail.com>")]
struct Opts {
  /// Serialize json version of metadata in addition to RMP
  #[clap(short, long)]
  json: bool,

  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
  Hello(HelloCommand),
  Init(InitCommand),
}

/// Print a dumb message
#[derive(Clap)]
struct HelloCommand;

/// Init a new repository
#[derive(Clap)]
struct InitCommand;

fn main() {
  let opts: Opts = Opts::parse();

  match opts.subcmd {
    SubCommand::Hello(HelloCommand {}) => {
      println!("Hello world!");
    },
    SubCommand::Init(InitCommand {}) => {
      // TODO - handle errors
      crate::commands::pov_init::pov_init(opts.json).unwrap();
    },
  }
}
