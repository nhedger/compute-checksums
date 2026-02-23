mod error;
mod input;
mod output;

use checksum_core::{ChecksumOptions, compute_checksums};

use crate::{error::ActionError, input::ActionInput};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        println!("::error::{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), ActionError> {
    let input = ActionInput::from_env()?;
    let options = ChecksumOptions {
        algorithms: input.algorithms,
        follow_symlinks: input.follow_symlinks,
    };
    let selected_algorithms = options.normalized_algorithms();

    let checksums = compute_checksums(&input.root, &input.patterns, &options)?;
    output::write_action_outputs(&checksums, &selected_algorithms)?;

    Ok(())
}
