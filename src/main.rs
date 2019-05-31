#[macro_use]
extern crate log;
extern crate env_logger;

mod create;

use structopt::StructOpt;
use crate::create::create_database;
use log::Level;
use std::path::PathBuf;
use env_logger::Builder;

#[derive(StructOpt, Debug)]
#[structopt(name = "mtsunday", about = "tool to create or manage mt sunday papers")]
struct Opt {
    #[structopt(name = "logging level", short = "l", long = "logging-level", default_value = "Error")]
    /// Set the logging level in order to print information.
    log: Level,
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "serve")]
    /// This start a web server to use MT Sunday tool.
    Serve {
    },
    #[structopt(name = "create")]
    /// This will create a SQLite DB required in order to start the web server.
    Create {
        #[structopt(short = "i", long = "input", parse(from_os_str))]
        /// The input directory where all score and music files a located.
        input: PathBuf,
        #[structopt(short = "o", long = "output-db-name", parse(from_os_str))]
        /// The name of the SQLite database.
        output_db_name: PathBuf
    }
}

fn main() {
    let opt: Opt = Opt::from_args();

    Builder::from_default_env()
        .filter_level(opt.log.to_level_filter())
        .init();

    let command: Command = opt.cmd;
    match command {
        Command::Create {input, output_db_name} => create_database(input, output_db_name),
        Command::Serve {} => unimplemented!(),
    }
}
