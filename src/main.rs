use dialoguer::Confirm;

use std::io;

fn main() {
    get_confirmation().unwrap();
}

fn get_confirmation() -> io::Result<bool> {
    if Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()?
    {
        println!("Continuing...");
    } else {
        println!("Cancelling...");
    }
    Ok(true)
}
