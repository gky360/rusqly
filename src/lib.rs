#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

use dirs::home_dir;
use std::path::PathBuf;
use std::result;
use structopt::StructOpt;

pub type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    #[fail(display = "Unknown")]
    Unknown,
}

lazy_static! {
    static ref DEFAULT_HISTORY: PathBuf = {
        let mut path = home_dir().unwrap();
        path.push(".cache");
        path.push("rusqly");
        path.push("history.txt");
        path
    };
}

#[derive(StructOpt, Debug, Clone, PartialEq, Eq, Hash)]
#[structopt()]
pub struct Opt {
    /// path to history file
    #[structopt(
        long,
        parse(from_os_str),
        raw(default_value = "DEFAULT_HISTORY.to_str().unwrap()")
    )]
    pub history: PathBuf,
}

pub fn run(opt: &Opt) -> Result<()> {
    println!("{:?}", opt);
    Ok(())
}
