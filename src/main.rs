/* 
import env module from the standard library
env: interact with the environment
calling current_dir function to get the current working directory
from the standard library environment module
*/ 
use std::env::current_dir;

/*main function returns a result type that is either Ok
or an error type std::io::Error
std::io: standard input/output library
*/
fn main() -> std::io::Result<()> {

    //? operator handles the results (Ok or Error)
    let path = current_dir()?;
    println!("The current directory is {}", path.display());
    //code exectution is successful if it reaches ok function (success exit)
    Ok(()) 
}
