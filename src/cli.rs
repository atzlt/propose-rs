use crate::interpreter::interpret::InterpreterState;
use clap::Parser;
use std::{fs, io, path::PathBuf};

#[derive(Parser)]
#[command(name = "Propose")]
#[command(about, version, long_about = None)]
struct Cli {
    /// Path to input file
    input: PathBuf,
    /// Path to output file
    #[arg(long, short)]
    output: Option<PathBuf>,
}

macro_rules! ok_or_print {
    ($res:expr, $str:literal) => {
        let result = $res;
        if let Err(e) = result {
            println!($str, e);
            return Ok(());
        }
    };
}

pub fn cli_main() -> io::Result<()> {
    // TODO: Change working directory.
    let cli = Cli::parse();
    let input = cli.input;

    if input.is_file() {
        let output = match cli.output {
            Some(path) => path,
            None => input.with_extension("svg"),
        };
        let file = fs::read_to_string(input)?;

        let mut interpreter = InterpreterState::new();
        ok_or_print!(interpreter.interpret(&file), "Cannot interpret file: {}");
        ok_or_print!(interpreter.save(output), "Cannot save to output: {}");
    } else if input.is_dir() {
        let entries = fs::read_dir(input)?;
        let mut interpreter = InterpreterState::new();
        for entry in entries {
            let input = entry?.path();
            let output = input.with_extension("svg");
            let file = fs::read_to_string(input)?;
            ok_or_print!(interpreter.interpret(&file), "Cannot interpret file: {}");
            ok_or_print!(interpreter.save(output), "Cannot save to output: {}");
            interpreter.clear();
        }
    } else {
        println!("Input path is neither a file nor a directory");
    }
    Ok(())
}
