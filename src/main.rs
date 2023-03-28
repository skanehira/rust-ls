use clap::Parser;
use rust_ls::{current_dir, print_recursively_output, visit_dirs};
use std::{io, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    dir: Option<String>,

    #[arg(short, default_value_t = false)]
    recursively: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let dir = args.dir.unwrap_or(".".into());

    if args.recursively {
        let mut output = visit_dirs(Path::new(&dir))?;
        output[0].dir = None;
        print_recursively_output(output);
    } else {
        let dirs = current_dir(Path::new(&dir))?;
        for d in dirs {
            println!("{}", d);
        }
    }
    Ok(())
}
