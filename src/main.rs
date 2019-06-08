#[macro_use]
extern crate log;
extern crate env_logger;

mod create;
mod serve;

use structopt::StructOpt;
use crate::create::create_database;
use crate::serve::serve;
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
        #[structopt(short = "c", long = "certificate", parse(from_os_str))]
        /// The path of the certificate.
        certificate: Option<PathBuf>,
        #[structopt(short = "k", long = "rsa-key", parse(from_os_str))]
        /// The path of the rsa key.
        key: Option<PathBuf>,
        #[structopt(short = "s", long = "secured")]
        /// Start server in http or in https.
        secured: bool,

    },
    #[structopt(name = "create")]
    /// This will create a SQLite DB required in order to start the web server.
    Create {
        #[structopt(short = "i", long = "input", parse(from_os_str))]
        /// The input directory where all score and music files a located.
        input: PathBuf,
        #[structopt(short = "o", long = "output-db-name", parse(from_os_str))]
        /// The name of the SQLite database.
        output_db_name: PathBuf,
        #[structopt(short = "p", long = "data-path", parse(from_os_str))]
        /// The directory where all files will be located.
        data_path: PathBuf,

    }
}

fn main() {
    let opt: Opt = Opt::from_args();
    trace!("Starting app with parameters {:?}.", opt);

    Builder::from_default_env()
        .filter_level(opt.log.to_level_filter())
        .init();

    let command: Command = opt.cmd;
    match command {
        Command::Create {input, output_db_name, data_path} => create_database(input, output_db_name, data_path),
        Command::Serve {certificate, key, secured} => serve(certificate, key, secured),
    }
}
