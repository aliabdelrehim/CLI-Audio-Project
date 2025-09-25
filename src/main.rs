/* 
import env module from the standard library
env: interact with the environment
calling current_dir function to get the current working directory
from the standard library environment module
*/ 
use std::env::current_dir;
use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source};

/*main function returns a result type that is either Ok
or an error type std::io::Error
std::io: standard input/output library
*/
fn main() -> std::io::Result<()> {

    //? operator handles the results (Ok or Error)
    let path = current_dir()?;
    println!("The current directory is {}", path.display());

    // Get an output stream handle to the default physical sound device.
    // Note that the playback stops when the stream_handle is dropped.//!
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open("Blue_One_Love.mp3").unwrap();
    // Decode that sound file into a source
    let source = Decoder::try_from(file).unwrap();
    // Play the sound directly on the device
    stream_handle.mixer().add(source);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    println!("Playing sound for 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(20));
    println!("Playing sound for 5 seconds...");
    //code exectution is successful if it reaches ok function (success exit)
    Ok(()) 
}
