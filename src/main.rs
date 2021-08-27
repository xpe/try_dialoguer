use dialoguer::Confirm;

use std::io;

#[tokio::main]
async fn main() {
    get_conf_async().await.unwrap();
}

pub async fn get_conf_async() -> io::Result<bool> {
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

pub fn main_sync() {
    get_conf().unwrap();
}

pub fn get_conf() -> io::Result<bool> {
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
