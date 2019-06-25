extern crate which;

fn main() {
    // Get the command-line argument iterator.
    let mut args = std::env::args_os();

    // Skip the first argument, since it is usually the path of this executable.
    args.next();

    // Initialize the exit code for success.
    let mut exit_code = 0;

    // Try each arg in turn.
    for ref arg in args {
        match which::which(arg) {
            Ok(path) => {
                // On success, print the path.
                println!("{}", path.to_string_lossy());
            }
            Err(err) => {
                // On failure, print the error and set the exit code. Do not break out of the loop.
                println!("Error: {} ({})", err, arg.to_string_lossy());
                exit_code = 1;
            }
        }
    }

    // Finish with the exit code.
    std::process::exit(exit_code);
}
