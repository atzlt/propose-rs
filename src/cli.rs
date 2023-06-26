use crate::interpreter::interpret::InterpreterState;
use clap::Parser;
use std::{env, fs, io, path::PathBuf};

#[derive(Parser)]
#[command(name = "Propose")]
#[command(about, version, long_about = None)]
struct Cli {
    /// Path to input file or directory.
    ///
    /// Will change working directory to this if it is a directory.
    input: PathBuf,
    /// Path to output file.
    ///
    /// Only have effect when input given is a file.
    /// If none is given, the output file name is implied from input.
    #[arg(long, short)]
    output: Option<PathBuf>,
    /// Extension to detect.
    ///
    /// Only have effect when input given is a directory.
    #[arg(long = "ext", short, default_value_t = String::from("prs"))]
    extension: String,
    /// Do not save file.
    ///
    /// Only have effect when input given is a file.
    /// Ignores `save` statements.
    #[arg(long = "no-save", short = 'S')]
    no_save: bool,
}

macro_rules! ok_or_print_ln_number {
    ($res:expr, $str:literal) => {
        let result = $res;
        if let Err(e) = result {
            println!("On interpreting line {}:", e.0);
            println!($str, e.1);
            return Ok(());
        }
    };
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
    let cli = Cli::parse();
    let input = cli.input;

    if input.is_file() {
        // TODO: Change working directory.
        let output = match cli.output {
            Some(path) => path,
            None => input.with_extension("svg"),
        };
        let file = fs::read_to_string(input)?;
        let mut interpreter = InterpreterState::new();
        ok_or_print_ln_number!(interpreter.interpret(&file), "Cannot interpret file: {}");
        if !cli.no_save {
            ok_or_print!(interpreter.save(output), "Cannot save to output: {}");
        }
    } else if input.is_dir() {
        env::set_current_dir(&input)?;
        let entries = fs::read_dir(".")?;
        let mut interpreter = InterpreterState::new();
        for entry in entries {
            let input = entry?.path();
            if let Some(ext) = input.extension() {
                if ext == cli.extension.as_str() {
                    let output = input.with_extension("svg");
                    let file = fs::read_to_string(input)?;
                    ok_or_print_ln_number!(
                        interpreter.interpret(&file),
                        "Cannot interpret file: {}"
                    );
                    ok_or_print!(interpreter.save(output), "Cannot save to output: {}");
                    interpreter.clear();
                }
            }
        }
    } else {
        println!("Input path is neither a file nor a directory");
    }
    Ok(())
}
