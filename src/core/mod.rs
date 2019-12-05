use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::{Opt, Result};

pub fn run_repl(opt: &Opt) -> Result<()> {
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(&opt.history);
    loop {
        let readline = rl.readline("rusqly> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == ".exit" {
                    break;
                } else {
                    println!("Unrecognized command '{}'.", line);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => return Err(err.into()),
        }
        rl.save_history(&opt.history)?;
    }

    Ok(())
}
