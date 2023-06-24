use crate::interpreter::interpret::InterpreterState;
use clap::Parser;
use rayon::prelude::*;
use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "Propose")]
#[command(about, version, long_about = None)]
struct Cli {
    /// Path to input file
    input: PathBuf,
    /// Path to output file
    #[arg(long, short)]
    output: Option<PathBuf>,
    /// Whether to parallelize
    #[arg(long, short)]
    parallel: bool,
}

macro_rules! ok_or_print {
    ($res:expr, $str:literal) => {
        let result = $res;
        if let Err(e) = result {
            println!($str, e);
            return;
        }
    };
}

fn interpret_file(input: PathBuf, output: Option<PathBuf>) {
    let parent = input.parent();
    if let Some(parent) = parent {
        ok_or_print!(
            env::set_current_dir(parent),
            "Cannot change working directory: {}"
        );
    }
    let output = match output {
        Some(path) => path,
        None => input.with_extension("svg"),
    };
    let file = fs::read_to_string(input);
    if let Err(e) = file {
        println!("Cannot open file: {}", e);
        return;
    }
    let file = file.unwrap();
    let mut interpreter = InterpreterState::new();
    ok_or_print!(interpreter.interpret(&file), "Cannot interpret file: {}");
    ok_or_print!(interpreter.save(output), "Cannot save to output: {}");
}

fn interpret_dir_entry(entry: std::io::Result<DirEntry>) {
    if let Err(e) = entry {
        println!("Cannot open directory entry: {}", e);
        return;
    }
    let entry = entry.unwrap();
    interpret_file(entry.path(), None);
}

pub fn cli_main() {
    let cli = Cli::parse();
    let input = cli.input;

    if input.is_file() {
        interpret_file(input, cli.output);
    } else if input.is_dir() {
        let result = fs::read_dir(input);
        if let Err(e) = result {
            println!("Cannot open directory: {}", e);
            return;
        }
        let entries = result.unwrap();
        if cli.parallel {
            entries.par_bridge().for_each(interpret_dir_entry);
        } else {
            entries.for_each(interpret_dir_entry);
        }
    } else {
        println!("Input path is neither a file nor a directory");
        return;
    }
}
