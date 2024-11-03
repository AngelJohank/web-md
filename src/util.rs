use std::env;

/// Retrieves the command-line arguments.
/// If no arguments are provided, a message is printed and the program exits.
pub fn get_cmd_args() -> Vec<String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("No files were provided.");
        std::process::exit(0);
    }

    args
}
