/* 
import env module from the standard library
env: interact with the environment
*/ 
use std::env::current_dir;

fn main() -> std::io::Result<()> {
    let path = current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(()) 
}
