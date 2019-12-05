#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

mod core;

use dirs::home_dir;
use rustyline::error::ReadlineError;
use std::path::PathBuf;
use std::{fs, io, result};
use structopt::StructOpt;

pub type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(io::Error),
    #[fail(display = "{}", _0)]
    Readline(ReadlineError),
    #[fail(display = "Unknown")]
    Unknown,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<ReadlineError> for Error {
    fn from(err: ReadlineError) -> Self {
        Error::Readline(err)
    }
}

lazy_static! {
    static ref CACHE_DIR: String = {
        let mut path = home_dir().unwrap();
        path.push(".cache");
        path.push("rusqly");
        String::from(path.to_str().unwrap())
    };
    static ref DEFAULT_HISTORY_PATH: String = {
        let mut path = PathBuf::from(&CACHE_DIR as &str);
        path.push("history.txt");
        String::from(path.to_str().unwrap())
    };
}

#[derive(StructOpt, Debug, Clone, PartialEq, Eq, Hash)]
#[structopt()]
pub struct Opt {
    /// path to history file
    #[structopt(long, parse(from_os_str), raw(default_value = "&DEFAULT_HISTORY_PATH"))]
    pub history: PathBuf,
}

pub fn run(opt: &Opt) -> Result<()> {
    eprintln!("{:?}", opt);

    // create cache dir if not exists
    fs::create_dir_all(&CACHE_DIR as &str)?;

    core::run_repl(opt)?;
    println!("Bye.");

    Ok(())
}
