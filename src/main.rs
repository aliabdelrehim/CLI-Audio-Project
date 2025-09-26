use std::fs::File;
use rodio::{Decoder};

/*main function returns a result type that is either Ok
or an error type std::io::Error
std::io: standard input/output library
*/
fn main() -> std::io::Result<()> {

    // Get an output stream handle to the default physical sound device.
    // Note that the playback stops when the stream_handle is dropped.//!
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
    let _sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // Load a sound from a file, using a path relative to Cargo.toml
    let file1 = File::open("Blue_One_Love.mp3").unwrap();
    let file2 = File::open("coldplay_a-sky-full-of-stars-coldplay.mp3").unwrap();
    let file3 = File::open("Show_Me_The_Meaning_Of_Being_Lonely.mp3").unwrap();
    // Decode that sound file into a source
    let source1 = Decoder::try_from(file1).unwrap();
    let source2 = Decoder::try_from(file2).unwrap();
    let source3 = Decoder::try_from(file3).unwrap();
    // Play the sound directly on the device
    stream_handle.mixer().add(source1);
    stream_handle.mixer().add(source2);
    stream_handle.mixer().add(source3);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    
    //code exectution is successful if it reaches ok function (success exit)
    Ok(()) 
}