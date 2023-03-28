use clap::Parser;
use rust_ls::current_dir;
use std::{env, io, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // TODO: implment -a
    //#[arg(short, default_value_t = false)]
    //all: bool,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];

    let dirs = current_dir(Path::new(&dir))?;
    for d in dirs {
        println!("{}", d);
    }
    Ok(())
}

