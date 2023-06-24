use propose::cli::cli_main;

fn main() {
    cli_main().unwrap_or_else(|e| println!("File I/O Error: {}", e));
}
