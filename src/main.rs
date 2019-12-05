use std::process;
use structopt::StructOpt;

fn main() {
    let result = {
        let opt = rusqly::Opt::from_args();
        rusqly::run(&opt)
    };

    process::exit(match result {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{}", err);
            eprintln!("{:?}", err);
            1
        }
    })
}
