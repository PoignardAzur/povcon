use clap::Clap;

/// A version control system
#[derive(Clap)]
#[clap(version = "0.1", author = "Olivier FAURE <couteaubleu@gmail.com>")]
struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
  Hello(Hello),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Hello;

fn main() {
  let opts: Opts = Opts::parse();

  match opts.subcmd {
    SubCommand::Hello(Hello {}) => {
      println!("Hello world!");
    }
  }
}
